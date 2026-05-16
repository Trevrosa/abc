use serenity::all::{Context, CreateCommand, InteractionContext};
use songbird::tracks::LoopState;

use crate::utils::context::CtxExt;
use crate::utils::reply::Replyer;

pub async fn set_loop(ctx: &Context, replyer: &Replyer<'_>) -> Result<(), &'static str> {
    let Some(guild_id) = replyer.guild() else {
        return Err("u not in a guild");
    };

    let Some(track) = ctx.current_track(guild_id).await else {
        return Err("im not play anything");
    };

    let Ok(track_info) = track.get_info().await else {
        return Err("faild to loop");
    };

    if track_info.loops == LoopState::Infinite {
        track.disable_loop().unwrap();
        ctx.reply("stopd looping", replyer).await;
    } else {
        track.enable_loop().unwrap();
        ctx.reply("looping", replyer).await;
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("loop")
        .add_context(InteractionContext::Guild)
        .description("toggle whether the current song should be looped")
}
