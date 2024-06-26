use serenity::all::{Context, Message};
use songbird::tracks::LoopState;

use crate::utils::context::Ext;
use crate::TrackHandleKey;

pub async fn set_loop(ctx: &Context, msg: &Message) -> Result<(), &'static str> {
    let global = ctx.data.try_read().unwrap();

    if global.contains_key::<TrackHandleKey>() {
        let Some(track) = global.get::<TrackHandleKey>() else {
            return Err("faild to loop");
        };

        let Ok(track_info) = track.get_info().await else {
            return Err("im not play anything");
        };

        if track_info.loops == LoopState::Infinite {
            track.disable_loop().unwrap();
            drop(global);

            ctx.reply("stopd looping", msg).await;
        } else {
            track.enable_loop().unwrap();
            drop(global);

            ctx.reply("looping", msg).await;
        }
    } else {
        return Err("im not play anything");
    }

    Ok(())
}
