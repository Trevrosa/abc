use serenity::all::{Context, Message};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Child,
};
use tokio_stream::{wrappers::LinesStream, StreamExt};

use super::context::CtxExt;

/// Constantly update message `status_msg` with the output of the passed [`Child`] with optional filtering.
///
/// For each line, `filter(line)` is run. If it returns `true`, that line is skipped.
///
/// # Errors
///
/// Will panic if `process.stdout.take()` or `process.stderr.take()` returns `None`.
#[inline]
pub async fn do_status(
    ctx: &Context,
    status_msg: &mut Message,
    process: &mut Child,
    filter: Option<fn(&str) -> bool>,
) {
    let filter = filter.unwrap_or(|_| false);

    let stdout = process.stdout.take().unwrap();
    let stderr = process.stderr.take().unwrap();

    let stdout = LinesStream::new(BufReader::new(stdout).lines());
    let stderr = LinesStream::new(BufReader::new(stderr).lines());

    let mut lines = stdout.merge(stderr);

    let mut first = true;
    while let Some(Ok(line)) = lines.next().await {
        if filter(&line) {
            continue;
        }

        let new_msg = if first {
            first = false;
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
    }
}
