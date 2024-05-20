use anyhow::Result;
use serenity::all::{Context, Message};

use crate::error::{GeneralError::*, VoiceError::*};

use super::Reply;

pub async fn stop(ctx: Context, msg: Message) -> Result<()> {
    let Some(manager) = songbird::get(&ctx).await else {
        ctx.reply("voice client not init", &msg).await;
        return Err(VoiceClientNotInit.into());
    };

    let Some(guild) = msg.guild_id else {
        ctx.reply("failed to get guild", &msg).await;
        return Err(DiscordGetError.into());
    };

    if let Some(handler) = manager.get(guild) {
        let mut handler = handler.lock().await;
        handler.stop();
        ctx.reply("stopped", &msg).await;
        Ok(())
    } else {
        ctx.reply("faild to stop", &msg).await;
        Err(CommandFailed.into())
    }
}
