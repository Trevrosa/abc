use serenity::all::{Context, CreateCommand, InteractionContext};

use crate::utils::context::CtxExt;
use crate::utils::reply::Replyer;

pub async fn pause(ctx: &Context, replyer: &Replyer<'_>) -> Result<(), &'static str> {
    let Some(guild_id) = replyer.guild() else {
        return Err("u not in a guild");
    };

    let Some(track) = ctx.current_track(guild_id).await else {
        return Err("im not play anything");
    };

    track.pause().unwrap();
    ctx.reply("pausd", replyer).await;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("pause")
        .add_context(InteractionContext::Guild)
        .description("pause bot playback")
}
