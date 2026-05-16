use serenity::all::{Context, CreateCommand, InteractionContext};

use crate::utils::context::CtxExt;
use crate::utils::reply::Replyer;

pub async fn stop(ctx: &Context, replyer: &Replyer<'_>) -> Result<(), &'static str> {
    let Some(guild_id) = replyer.guild() else {
        return Err("u not in a guild");
    };

    let Some(track) = ctx.current_track(guild_id).await else {
        return Err("im not play anything");
    };

    track.stop().unwrap();

    ctx.reply("stopd", replyer).await;

    Ok(())
}

pub fn register() -> [CreateCommand; 2] {
    let stop = CreateCommand::new("stop")
        .add_context(InteractionContext::Guild)
        .description("stop bot playback");
    let skip = CreateCommand::new("skip")
        .add_context(InteractionContext::Guild)
        .description("skip the current song");

    [stop, skip]
}
