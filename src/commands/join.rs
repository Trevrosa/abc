use serenity::all::{ChannelId, ChannelType, Context, Message};

use super::Reply;

pub async fn join(ctx: Context, msg: Message) {
    let Some(guild) = msg.guild_id else {
        ctx.reply("faild to get guild", &msg).await;
        return;
    };

    let Ok(channels) = guild.channels(&ctx).await else {
        ctx.reply("faild to get channels", &msg).await;
        return;
    };

    let args = msg.content.trim().split(' ').collect::<Vec<&str>>();

    let channel = if args.len() == 2 {
        let id: u64 = if args[1].starts_with("<#") {
            args[1][2..args[1].len() - 1].parse().unwrap()
        } else if args[1].starts_with("https://discord.com/channels/") {
            args[1].split('/').collect::<Vec<&str>>()[5]
                .parse()
                .unwrap()
        } else {
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
        channels
            .iter()
            .find_map(|c| {
                let c = c.1;

                if c.kind != ChannelType::Voice {
                    return None;
                }

                let Ok(members) = c.members(&ctx.cache) else {
                    return None;
                };

                if members.iter().any(|m| m.user == msg.author) {
                    Some(c)
                } else {
                    None
                }
            })
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
