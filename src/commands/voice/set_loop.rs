use serenity::all::{Context, Message};
use songbird::tracks::LoopState;

use crate::TrackHandleKey;

use super::Reply;

pub async fn set_loop(ctx: Context, msg: Message) {
    let global = ctx.data.read().await;

    if global.contains_key::<TrackHandleKey>() {
        let Some(track) = global.get::<TrackHandleKey>() else {
            ctx.reply("faild to loop", &msg).await;
            return;
        };
        let Ok(track_info) = track.get_info().await else {
            ctx.reply("im not play anything", &msg).await;
            return;
        };

        if track_info.loops == LoopState::Infinite {
            track.disable_loop().unwrap();
            ctx.reply("stopd looping", &msg).await;
        } else {
            track.enable_loop().unwrap();
            ctx.reply("looping", &msg).await;
        }
    } else {
        ctx.reply("im not play anything", &msg).await;
    }
}
