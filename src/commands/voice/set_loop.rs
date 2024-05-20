use anyhow::Result;
use serenity::all::{Context, Message};
use songbird::tracks::LoopState;

use crate::TrackHandleKey;

use super::Reply;

pub async fn set_loop(ctx: Context, msg: Message) {
    let global_track = ctx.data.read().await;

    if global_track.is_empty() {
        ctx.reply("im not play anything", &msg).await;
    } else {
        let Some(track) = global_track.get::<TrackHandleKey>().cloned() else {
            ctx.reply("faild to loop", &msg).await;
            return;
        };

        if track.get_info().await.unwrap().loops == LoopState::Infinite {
            track.disable_loop().unwrap();
            ctx.reply("stopd looping", &msg).await;
        } else {
            track.enable_loop().unwrap();
            ctx.reply("looping", &msg).await;
        }
    }
}
