use serenity::all::{ChannelId, ChannelType, Context, Message};

use crate::utils::context::Ext;

pub async fn join(ctx: &Context, msg: &Message) -> Result<(), &'static str> {
    let Some(guild) = msg.guild_id else {
        return Err("faild to get guild");
    };  

    let Ok(channels) = guild.channels(&ctx).await else {
        return Err("faild to get channels");
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
            return Err("not a vc");
        };

        let Ok(channel) = ctx.http.get_channel(ChannelId::new(id)).await else {
            return Err("channel not exist");
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
        return Err("u arent in a vc");
    };

    if let Some(manager) = songbird::get(&ctx).await.clone() {
        let Some(guild) = msg.guild_id else {
            return Err("faild to get guild");
        };

        if manager.join(guild, channel.id).await.is_ok() {
            ctx.reply("joined u", &msg).await;
        } else {
            ctx.reply("faild to join", &msg).await;
        }
    } else {
        return Err("voice manager failed");
    }

    Ok(())
}
