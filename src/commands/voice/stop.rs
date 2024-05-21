use serenity::all::{Context, Message};

use crate::TrackHandleKey;

use super::Reply;

pub async fn stop(ctx: Context, msg: Message) {
    let global = ctx.data.read().await;

    if global.contains_key::<TrackHandleKey>() {
        let Some(track) = global.get::<TrackHandleKey>().cloned() else {
            ctx.reply("faild to stop", &msg).await;
            return;
        };

        track.stop().unwrap();
        ctx.reply("stopd", &msg).await;
    } else {
        ctx.reply("im not play anything", &msg).await;
    }
}
