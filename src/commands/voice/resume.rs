use serenity::all::{Context, Message};

use super::Reply;
use crate::TrackHandleKey;

pub async fn resume(ctx: Context, msg: Message) {
    let global = ctx.data.read().await;

    if global.contains_key::<TrackHandleKey>() {
        let Some(track) = global.get::<TrackHandleKey>() else {
            ctx.reply("faild to pause", &msg).await;
            return;
        };

        track.play().unwrap();
        ctx.reply("resumd", &msg).await;
    } else {
        ctx.reply("im not play anything", &msg).await;
    }
}
