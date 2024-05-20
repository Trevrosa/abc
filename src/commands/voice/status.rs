use anyhow::Result;
use serenity::all::{Context, Message};

use crate::TrackHandleKey;

use super::Reply;

pub async fn status(ctx: Context, msg: Message) {
    let global_track = ctx.data.read().await;

    if global_track.is_empty() {
        ctx.reply("im not play anything", &msg).await;
    } else {
        let Some(track) = global_track.get::<TrackHandleKey>().cloned() else {
            return;
        };

        ctx.reply(
            &format!("```rust\n{:#?}\n```", track.get_info().await),
            &msg,
        )
        .await;
    }
}
