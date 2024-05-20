use anyhow::Result;
use serenity::all::{ChannelType, Context, GuildChannel, Message};

use crate::error::{
    General::{CommandFailed, CommandRequirement, DiscordGet},
    Voice::Manager,
};

use super::Reply;

pub async fn join(ctx: Context, msg: Message) -> Result<()> {
    let channel = if let Some(guild) = msg.guild_id {
        if let Ok(channels) = guild.channels(&ctx).await {
            channels
        } else {
            ctx.reply("faild to get channels", &msg).await;
            return Err(DiscordGet.into());
        }
    } else {
        ctx.reply("faild to get guild", &msg).await;
        return Err(DiscordGet.into());
    };

    let channel = channel
        .iter()
        .filter_map(|c| {
            if let Ok(members) = c.1.members(&ctx.cache) {
                if c.1.kind == ChannelType::Voice
                    && members.iter().any(|m| m.user.id == msg.author.id)
                {
                    Some(c.1)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<&GuildChannel>>();

    if channel.is_empty() {
        ctx.reply("u arent in a vc", &msg).await;
        return Err(CommandRequirement.into());
    }

    let channel = channel[0];

    if let Some(manager) = songbird::get(&ctx).await {
        let Some(guild) = msg.guild_id else {
            ctx.reply("faild to get guild", &msg).await;
            return Err(DiscordGet.into());
        };

        if manager.join(guild, channel.id).await.is_ok() {
            ctx.reply("joined u", &msg).await;
            Ok(())
        } else {
            ctx.reply("faild to join", &msg).await;
            Err(CommandFailed.into())
        }
    } else {
        ctx.reply("voice manager failed", &msg).await;
        Err(Manager.into())
    }
}
