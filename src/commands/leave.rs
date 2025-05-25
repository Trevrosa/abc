use serenity::all::{Context, CreateCommand, InteractionContext};

use crate::utils::{context::CtxExt, reply::Replyer};

pub async fn leave(ctx: &Context, replyer: &Replyer<'_>) -> Result<(), &'static str> {
    let Some(manager) = songbird::get(ctx).await else {
        return Err("voice client not init");
    };

    let guild_id = match replyer {
        Replyer::Prefix(msg) => msg.guild_id,
        Replyer::Slash(int) => int.guild_id,
    };

    let Some(guild_id) = guild_id else {
        return Err("faild to get guild");
    };

    let Some(handler) = manager.get(guild_id) else {
        return Err("faild to get voice handler");
    };

    if manager.leave(guild_id).await.is_ok() {
        handler.lock().await.stop();
        ctx.reply("left u :(", replyer).await;
    } else {
        return Err("faild to leave :)");
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("leave")
        .add_context(InteractionContext::Guild)
        .description("make the bot leave the vc its currently in")
}
