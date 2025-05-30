use std::{env, path::Path};

use serenity::all::{
    CommandOptionType, Context, CreateAttachment, CreateCommand, CreateCommandOption,
};
use tokio::fs;
use tracing::{error, info};

use crate::utils::{
    context::CtxExt,
    reply::{CreateReply, Replyer},
    spotify::extract_spotify,
    ArgValue, Args, DeleteWhenDone, Get, Is,
};

/// discord's free upload limit in bytes
const DISCORD_UPLOAD_LIMIT: u64 = 10 * 1000 * 1000;

pub async fn get_song(
    ctx: &Context,
    replyer: &Replyer<'_>,
    args: Args<'_>,
) -> Result<(), &'static str> {
    if args.is_empty() {
        return Err("expected at least 1 argument");
    }

    let Some(ArgValue::String(url)) = args.first_value() else {
        return Err("no string url in args");
    };

    // naive check
    if !url.contains("http") && !url.contains(".com") {
        return Err("did not find url (did u forget `http`?)");
    }

    let url = if url.contains("spotify.com") {
        ctx.reply(
            "this is a spotify url, we need to do some stuff first.",
            replyer,
        )
        .await;
        extract_spotify(ctx, replyer, url).await?
    } else {
        (*url).to_string()
    };

    let mut greet = ctx.reply("downloading ", replyer).await;

    let idstring = match replyer {
        Replyer::Prefix(msg) => msg.author.id.get().to_string(),
        Replyer::Slash(int) => int.user.id.get().to_string(),
    };
    let download_path = Path::new(&idstring);

    if download_path.exists() {
        ctx.edit_msg("u already downloading, pls wait", &mut greet)
            .await;
        return Err("");
    }

    let _cleanup = DeleteWhenDone::new(download_path);

    if let Err(err) = fs::create_dir(download_path).await {
        error!("failed to create path {download_path:?}: {err:#?}");
        return Err("could not create download folder");
    }

    // we use yt-dlp output templates (https://github.com/yt-dlp/yt-dlp?tab=readme-ov-file#output-template)
    let output = download_path.join("%(title)s [%(id)s].%(ext)s");

    let no_video =
        args.get("novid").is_some_and(|a| a.is(true)) || args.get(1).is_some_and(|a| a.is("novid"));
    let mp3 =
        args.get("mp3").is_some_and(|a| a.is(true)) || args.get(2).is_some_and(|a| a.is("mp3"));

    // `ba*` by default, `ba` if the user wants it.
    let download_format = if no_video { "ba" } else { "ba*" };
    let audio_only_args: Option<&[&str]> = if mp3 {
        // ensure we get mp3 so it embeds on discord properly
        Some(&["--extract-audio", "--audio-format", "mp3"])
    } else {
        None
    };

    ctx.yt_dlp(
        url.as_str(),
        Some(output),
        download_format,
        audio_only_args,
        &mut greet,
    )
    .await?;

    ctx.msg_new_line("finished download, checking size", &mut greet)
        .await;

    let Ok(files) = download_path.read_dir() else {
        return Err("could not find download folder");
    };

    for file in files {
        let Ok(file) = file else {
            return Err("could not read downloaded file");
        };

        if file
            .metadata()
            .is_ok_and(|m| m.len() < DISCORD_UPLOAD_LIMIT)
        {
            ctx.msg_new_line("uploading as attachment", &mut greet)
                .await;

            // we can upload to discord
            let Ok(attachment) = CreateAttachment::path(file.path()).await else {
                return Err("failed to create attachment");
            };

            let message = CreateReply::new().content("done!").add_file(attachment);
            ctx.reply(message, replyer).await;
        } else if let Some(shared_dir) = env::var_os("ABC_SHARED_DIR") {
            ctx.msg_new_line("file >10mb, moving to shared dir", &mut greet)
                .await;

            let shared_dir = Path::new(&shared_dir);
            if let Err(err) = fs::rename(file.path(), shared_dir.join(file.file_name())).await {
                if err.kind() != std::io::ErrorKind::CrossesDevices {
                    error!("error moving file to shared dir {shared_dir:?}: {err:#?}");
                    return Err("could not move file to shared dir");
                }

                ctx.msg_new_line("need to copy over mount point, wait..", &mut greet)
                    .await;
                info!("copying file {:?} over mount point", file.path());
                if let Err(err) = fs::copy(file.path(), shared_dir.join(file.file_name())).await {
                    error!("error copying file to shared dir {shared_dir:?}: {err:#?}");
                    return Err("could not copy file to shared dir");
                }
            }

            let external_host = include_str!("../../external_host").trim();
            if external_host.is_empty() {
                ctx.reply("uploaded to shared dir. (file was >10mb)", replyer)
                    .await;
            } else {
                let url = Path::new(external_host).join(file.file_name());
                let url = url.to_string_lossy().replace(' ', "%20");
                ctx.reply(format!("done! {url}"), replyer).await;
            }
        } else {
            return Err("could not upload file (was >10mb)");
        }
    }

    Ok(())
}

inventory::submit! {
    crate::CrateCommand::new("getsong")
}

pub fn register() -> CreateCommand {
    CreateCommand::new("getsong")
        .description("get a song from its url, supporting spotify")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "songurl", "song's url")
                .required(true),
        )
        .add_option(CreateCommandOption::new(
            CommandOptionType::Boolean,
            "novideo",
            "only download audio",
        ))
        .add_option(CreateCommandOption::new(
            CommandOptionType::Boolean,
            "mp3",
            "convert to mp3, might take a while",
        ))
}
