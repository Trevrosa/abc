use serenity::all::{Context, Message};

use crate::utils::context::Ext;
use crate::TrackHandleKey;

pub async fn status(ctx: Context, msg: Message) {
    let global = ctx.data.try_read().unwrap();

    if global.contains_key::<TrackHandleKey>() {
        let Some(track) = global.get::<TrackHandleKey>() else {
            return;
        };

        ctx.reply(format!("```rust\n{:#?}\n```", track.get_info().await), &msg)
            .await;
    } else {
        ctx.reply("im not play anything", &msg).await;
    }
}
