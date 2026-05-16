use serenity::all::{Context, CreateCommand, InteractionContext};

use crate::utils::context::CtxExt;
use crate::utils::reply::Replyer;

pub async fn resume(ctx: &Context, replyer: &Replyer<'_>) -> Result<(), &'static str> {
    let Some(guild_id) = replyer.guild() else {
        return Err("u not in a guild");
    };

    let Some(track) = ctx.current_track(guild_id).await else {
        return Err("im not play anything");
    };

    track.play().unwrap();

    ctx.reply("resumd", replyer).await;

    Ok(())
}

pub fn register() -> [CreateCommand; 2] {
    let resume = CreateCommand::new("resume")
        .add_context(InteractionContext::Guild)
        .description("resume bot playback");
    let unpause = CreateCommand::new("unpause")
        .add_context(InteractionContext::Guild)
        .description("unpause the current song");

    [resume, unpause]
}
