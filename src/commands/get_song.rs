use std::{
    env, io::{stdout, BufRead, BufReader}, path::Path, process::{Command, Stdio}
};

use serenity::all::{Context, CreateAttachment, CreateMessage, Message};
use tokio::fs;
use tracing::info;

use crate::utils::context::Ext;

pub async fn get_song(ctx: &Context, msg: &Message) -> Result<(), &'static str> {
    let args = msg.content.trim().split(' ').collect::<Vec<&str>>();
    if args.len() != 2 {
        return Err("expected 1 argument");
    }

    let url = args[1];
    // naive check
    if !url.contains("http") && !url.contains(".com") {
        return Err("did not find url (did u forget `http`?)");
    }

    let mut greet = ctx.reply("downloading ", msg).await;

    let idstring = msg.author.id.get().to_string();
    let idpath = Path::new(&idstring);

    if idpath.exists() {
        if fs::remove_dir_all(idpath).await.is_err() {
            return Err("failed to clear old download folder");
        }
    }

    if fs::create_dir(idpath).await.is_err() {
        return Err("could not create download folder");
    }

    // we use yt-dlp output templates (https://github.com/yt-dlp/yt-dlp?tab=readme-ov-file#output-template)
    let output = idpath.join("%(title)s [%(id)s].%(ext)s");
    let output = output.to_string_lossy();

    let downloader = Command::new("/usr/bin/yt-dlp")
        // ba* = choose best quality format with audio, which might be video
        // see: https://github.com/yt-dlp/yt-dlp?tab=readme-ov-file#format-selection
        .args([args[1], "-o", &output, "-f", "ba*"])
        .stdout(Stdio::piped())
        .stderr(stdout())
        .spawn();

    let Ok(mut downloader) = downloader else {
        ctx.edit_msg("faild to start download", &mut greet).await;
        return Err("");
    };

    // we want to drop reader after we finish
    {
        let output = downloader.stdout.as_mut().unwrap();
        let reader = BufReader::new(output);

        for (i, chunk) in reader.lines().enumerate() {
            let new_msg = if i == 0 {
                format!("```{}```", chunk.unwrap().trim())
            } else {
                // should work since we put ``` already at the start of msg
                format!(
                    "{}\n{}```",
                    &greet.content.strip_suffix("```").unwrap(),
                    chunk.unwrap().trim()
                )
            };

            ctx.edit_msg(new_msg, &mut greet).await;
        }
    }

    if !downloader.wait().unwrap().success() {
        return Err("download faild");
    }

    info!("downloaded {} with yt-dlp", args[1]);

    let Ok(mut files) = idpath.read_dir() else {
        return Err("could not find download folder");
    };
    let Some(Ok(file)) = files.next() else {
        return Err("could not find download file");
    };

    const DISCORD_LIMIT: u64 = 10 * 1000 * 1000;
    if file.metadata().is_ok_and(|m|m.len() < DISCORD_LIMIT) {
        // we can upload to discord
        let Ok(attachment) = CreateAttachment::path(file.path()).await else {
            return Err("failed to create attachment");
        };

        let message = CreateMessage::new().content("done!").add_file(attachment);
        ctx.reply(message, msg).await;
    } else if let Some(shared_dir) = env::var_os("ABC_SHARED_DIR") {
        let shared_dir = Path::new(&shared_dir);
        if fs::rename(file.path(), shared_dir.join(file.file_name())).await.is_err() {
            return Err("could not move file to shared dir");
        }
        
        let external_host = include_str!("../../external_host").trim();
        if external_host.is_empty() {
            ctx.reply("uploaded to shared dir. (file was >10mb)", msg).await;
        } else {
            let url = Path::new(external_host).join(file.file_name());
            ctx.reply(format!("done! {}", url.to_string_lossy()), msg).await;
        }
    } else {
        return Err("could not upload file (was >10mb)");
    }

    if let Err(err) = fs::remove_dir_all(idpath).await {
        tracing::error!("error removing dir {idpath:?}: {err:#?}");
    }

    Ok(())
}
