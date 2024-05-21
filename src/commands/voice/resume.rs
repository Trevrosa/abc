use serenity::all::{Context, Message};

use super::Reply;
use crate::TrackHandleKey;

pub async fn resume(ctx: Context, msg: Message) {
    let global = ctx.data.read().await;

    if !global.contains_key::<TrackHandleKey>() {
        ctx.reply("im not play anything", &msg).await;
        return;
    }

    let Some(track) = global.get::<TrackHandleKey>() else {
        return;
    };

    track.play().unwrap();
    ctx.reply("resumed", &msg).await;
}
