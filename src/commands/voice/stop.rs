use anyhow::Result;
use serenity::all::{Context, Message};

use crate::TrackHandleKey;

use super::Reply;

pub async fn stop(ctx: Context, msg: Message) {
    let global_track = ctx.data.read().await;

    if global_track.is_empty() {
        ctx.reply("im not play anything", &msg).await;
    } else {
        let Some(track) = global_track.get::<TrackHandleKey>().cloned() else {
            ctx.reply("faild to stop", &msg).await;
            return;
        };

        track.stop().unwrap();
        ctx.reply("stopd", &msg).await;
    }
}