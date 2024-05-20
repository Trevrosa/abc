use serenity::all::{Context, Message};

use super::Reply;
use crate::TrackHandleKey;

pub async fn resume(ctx: Context, msg: Message) {
    let global_track = ctx.data.read().await;

    if global_track.is_empty() {
        ctx.reply("im not play anything", &msg).await;
        return;
    }

    let Some(track) = global_track.get::<TrackHandleKey>().cloned() else {
        return;
    };

    track.play().unwrap();
    ctx.reply("resumed", &msg).await;
}
