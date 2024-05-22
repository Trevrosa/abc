use serenity::all::{Context, Message};

use crate::TrackHandleKey;

use super::Utils;

pub async fn status(ctx: Context, msg: Message) {
    let global = ctx.data.read().await;

    if global.contains_key::<TrackHandleKey>() {
        let Some(track) = global.get::<TrackHandleKey>() else {
            return;
        };

        ctx.reply(
            &format!("```rust\n{:#?}\n```", track.get_info().await),
            &msg,
        )
        .await;
    } else {
        ctx.reply("im not play anything", &msg).await;
    }
}
