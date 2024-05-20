use anyhow::Result;
use serenity::all::{Context, Message};

use super::Reply;
use crate::error::{
    General::{CommandFailed, DiscordGet},
    Voice::{Handler, VoiceClientNotInit},
};

pub async fn leave(ctx: Context, msg: Message) -> Result<()> {
    let Some(manager) = songbird::get(&ctx).await else {
        ctx.reply("voice client not init", &msg).await;
        return Err(VoiceClientNotInit.into());
    };

    let Some(guild_id) = msg.guild_id else {
        ctx.reply("faild to get guild", &msg).await;
        return Err(DiscordGet.into());
    };

    let Some(handler) = manager.get(guild_id) else {
        ctx.reply("faild to get voice handler", &msg).await;
        return Err(Handler.into());
    };

    if manager.leave(guild_id).await.is_ok() {
        handler.lock().await.stop();
        ctx.reply("left u :(", &msg).await;
        Ok(())
    } else {
        ctx.reply("faild to leave :)", &msg).await;
        Err(CommandFailed.into())
    }
}
