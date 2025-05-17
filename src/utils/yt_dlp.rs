use std::{
    path::{Path, PathBuf},
    process::Stdio,
};

use serenity::all::{Context, Message};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};
use tracing::info;

use crate::utils::context::Ext;

const DEFAULT_OUTPUT_TEMPLATE: &str = "%(title)s [%(id)s].%(ext)s";

pub(super) async fn download<P: AsRef<Path>, S: AsRef<str>>(
    ctx: &Context,
    url: S,
    output: Option<P>,
    download_format: S,
    extra_args: Option<&[&str]>,
    status_msg: &mut Message,
) -> Result<(), &'static str> {
    let url = url.as_ref();
    let output = output.map_or(PathBuf::from(DEFAULT_OUTPUT_TEMPLATE), |p| {
        p.as_ref().to_owned()
    });
    let download_format = download_format.as_ref();
    let extra_args = extra_args.unwrap_or(&[]);

    let downloader = Command::new("/usr/bin/yt-dlp")
        // ba* = choose best quality format with audio, which might be video
        // see: https://github.com/yt-dlp/yt-dlp?tab=readme-ov-file#format-selection
        .args([url, "-o", &output.to_string_lossy(), "-f", download_format])
        .args(extra_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    let Ok(mut downloader) = downloader else {
        ctx.edit_msg("faild to start download", status_msg).await;
        return Err("");
    };

    // we want to drop reader after we finish
    {
        let output = downloader.stdout.take().unwrap();
        let reader = BufReader::new(output);

        let mut i = 0;
        let mut lines = reader.lines();

        while let Ok(Some(line)) = lines.next_line().await {
            let new_msg = if i == 0 {
                format!("```{}```", line.trim())
            } else {
                // should work since we put ``` already at the start of msg
                format!(
                    "{}\n{}```",
                    &status_msg.content.strip_suffix("```").unwrap(),
                    line.trim()
                )
            };

            ctx.edit_msg(new_msg, status_msg).await;
            i += 1;
        }
    }

    if !downloader.wait().await.unwrap().success() {
        return Err("download faild");
    }

    info!("downloaded {url} with yt-dlp");

    Ok(())
}
