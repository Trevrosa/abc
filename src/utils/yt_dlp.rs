use std::{
    path::{Path, PathBuf},
    process::Stdio,
};

use serenity::all::{Context, Message};
use tokio::process::Command;
use tracing::{info, warn};

use crate::utils::{context::Ext, status::do_status};

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

    let args = [url, "-o", &output.to_string_lossy(), "-f", download_format];
    let extra_args = extra_args.unwrap_or(&[]);

    let downloader = Command::new("/usr/bin/yt-dlp")
        // ba* = choose best quality format with audio, which might be video
        // see: https://github.com/yt-dlp/yt-dlp?tab=readme-ov-file#format-selection
        .args(args)
        .args(extra_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    let Ok(mut downloader) = downloader else {
        ctx.edit_msg("faild to start download", status_msg).await;
        return Err("");
    };

    let filter = |line: &str| {
        if line.starts_with("Input") {
            true
        } else if line.starts_with("[hls") {
            true
        } else if line.starts_with("WARNING") {
            warn!("{line}");
            true
        } else if line.trim_start().starts_with("n =") {
            true
        } else if line.trim_start().starts_with("Please report") {
            true
        } else {
            false
        }
    };

    do_status(ctx, status_msg, &mut downloader, Some(filter)).await;

    if !downloader.wait().await.unwrap().success() {
        return Err("download faild");
    }

    info!("downloaded {url} with yt-dlp");

    Ok(())
}
