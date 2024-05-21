use serenity::all::{Context, Message};

use crate::TrackHandleKey;

use super::Reply;

pub async fn pause(ctx: Context, msg: Message) {
    let global = ctx.data.read().await;

    if global.contains_key::<TrackHandleKey>() {
        let Some(track) = global.get::<TrackHandleKey>() else {
            ctx.reply("faild to pause", &msg).await;
            return;
        };

        track.pause().unwrap();
        ctx.reply("pausd", &msg).await;
    } else {
        ctx.reply("im not play anything", &msg).await;
    }
}
