use serenity::all::{ChannelId, ChannelType, Context, Message};

use crate::utils::context::Ext;

pub async fn join(ctx: Context, msg: Message) {
    let Some(guild) = msg.guild_id else {
        ctx.reply("faild to get guild", &msg).await;
        return;
    };

    let Ok(channels) = guild.channels(&ctx).await else {
        ctx.reply("faild to get channels", &msg).await;
        return;
    };

    let mut channels = channels.iter();

    let args = msg.content.trim().split(' ').collect::<Vec<&str>>();

    let channel = if args.len() == 2 {
        let id: Result<u64, std::num::ParseIntError> = if args[1].starts_with("<#") {
            args[1][2..args[1].len() - 1].parse::<u64>()
        } else if args[1].starts_with("https://discord.com/channels/") {
            args[1].split('/').collect::<Vec<&str>>()[5].parse::<u64>()
        } else {
            args[1].parse::<u64>()
        };

        let Ok(id) = id else {
            ctx.reply("not a vc", &msg).await;
            return;
        };

        let Ok(channel) = ctx.http.get_channel(ChannelId::new(id)).await else {
            ctx.reply("channel not exist", &msg).await;
            return;
        };

        let channel = channel.guild().unwrap();

        if channel.kind == ChannelType::Voice {
            Some(channel)
        } else {
            None
        }
    } else {
        ctx.find_user_channel(&msg.author, ChannelType::Voice, &mut channels)
            .cloned()
    };

    let Some(channel) = channel else {
        ctx.reply("u arent in a vc", &msg).await;
        return;
    };

    if let Some(manager) = songbird::get(&ctx).await.clone() {
        let Some(guild) = msg.guild_id else {
            ctx.reply("faild to get guild", &msg).await;
            return;
        };

        if manager.join(guild, channel.id).await.is_ok() {
            ctx.reply("joined u", &msg).await;
        } else {
            ctx.reply("faild to join", &msg).await;
        }
    } else {
        ctx.reply("voice manager failed", &msg).await;
    }
}
