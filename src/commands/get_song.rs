use std::{env, path::Path};

use serenity::all::{Context, CreateAttachment, CreateMessage, Message};
use tokio::fs;
use tracing::{error, info};

use crate::utils::context::Ext;

/// discord's free upload limit in bytes
const DISCORD_UPLOAD_LIMIT: u64 = 10 * 1000 * 1000;

/// A guard struct that removes `self.path` after `self` is [`drop()`]ped.
struct DeleteWhenDone<'a> {
    path: &'a Path
}

impl<'a> Drop for DeleteWhenDone<'a> {
    fn drop(&mut self) {
        let path = self.path.to_owned();
        tokio::task::spawn_blocking(move || {
            if let Err(err) = std::fs::remove_dir_all(&path) {
                // we don't care if `path` wasn't found.
                if !matches!(err.kind(), std::io::ErrorKind::NotFound) {
                    error!("failed to clean {path:?}: {err:#?}");
                }
            } else {
                info!("cleaned path {path:?}");
            }
        });
    }
}

pub async fn get_song(ctx: &Context, msg: &Message) -> Result<(), &'static str> {
    let args = msg.content.trim().split(' ').collect::<Vec<&str>>();
    if args.len() < 2 {
        return Err("expected at least 1 argument");
    }

    let url = args[1];
    // naive check
    if !url.contains("http") && !url.contains(".com") {
        return Err("did not find url (did u forget `http`?)");
    }

    let mut greet = ctx.reply("downloading ", msg).await;
    
    let idstring = msg.author.id.get().to_string();
    let download_path = Path::new(&idstring);
    
    if download_path.exists() {
        return Err("u already downloading, pls wait");
    }

    let _cleanup = DeleteWhenDone { path: download_path };

    if let Err(err) = fs::create_dir(download_path).await {
        error!("failed to create path {download_path:?}: {err:#?}");
        return Err("could not create download folder");
    }

    // we use yt-dlp output templates (https://github.com/yt-dlp/yt-dlp?tab=readme-ov-file#output-template)
    let output = download_path.join("%(title)s [%(id)s].%(ext)s");

    // `ba*` by default, `ba` if the user wants it.
    let download_format = args
        .get(2)
        .iter()
        .find_map(|c| if c == &&"novid" { Some("ba") } else { None })
        .unwrap_or("ba*");

    ctx.yt_dlp(url, Some(output), download_format, &mut greet)
        .await?;

    let Ok(mut files) = download_path.read_dir() else {
        return Err("could not find download folder");
    };
    let Some(Ok(file)) = files.next() else {
        return Err("could not find download file");
    };

    if file
        .metadata()
        .is_ok_and(|m| m.len() < DISCORD_UPLOAD_LIMIT)
    {
        // we can upload to discord
        let Ok(attachment) = CreateAttachment::path(file.path()).await else {
            return Err("failed to create attachment");
        };

        let message = CreateMessage::new().content("done!").add_file(attachment);
        ctx.reply(message, msg).await;
    } else if let Some(shared_dir) = env::var_os("ABC_SHARED_DIR") {
        let shared_dir = Path::new(&shared_dir);
        if let Err(err) = fs::rename(file.path(), shared_dir.join(file.file_name())).await {
            error!("error moving file to shared dir {shared_dir:?}: {err:#?}");
            return Err("could not move file to shared dir");
        }

        let external_host = include_str!("../../external_host").trim();
        if external_host.is_empty() {
            ctx.reply("uploaded to shared dir. (file was >10mb)", msg)
                .await;
        } else {
            let url = Path::new(external_host).join(file.file_name());
            ctx.reply(format!("done! {}", url.to_string_lossy()), msg)
                .await;
        }
    } else {
        return Err("could not upload file (was >10mb)");
    }

    Ok(())
}
