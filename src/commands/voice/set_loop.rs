use serenity::all::{Context, CreateCommand, InteractionContext};
use songbird::tracks::LoopState;

use crate::utils::context::CtxExt;
use crate::utils::reply::Replyer;
use crate::TrackHandleKey;

pub async fn set_loop(ctx: &Context, replyer: &Replyer<'_>) -> Result<(), &'static str> {
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

            ctx.reply("stopd looping", replyer).await;
        } else {
            track.enable_loop().unwrap();
            drop(global);

            ctx.reply("looping", replyer).await;
        }
    } else {
        return Err("im not play anything");
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("loop")
        .add_context(InteractionContext::Guild)
        .description("toggle whether the current song should be looped")
}
