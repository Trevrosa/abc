use bytes::Bytes;
use serenity::all::{ChannelType, Context, Message};
use songbird::tracks::LoopState;

use super::{edit_message, Reply};
use crate::{commands::Get, TrackHandleKey};

pub async fn play(ctx: Context, msg: Message) {
    let Some(manager) = songbird::get(&ctx).await.clone() else {
        ctx.reply("voice client not init", &msg).await;
        return;
    };

    let args = msg.content.trim().split(' ').collect::<Vec<&str>>();

    let Some(guild) = msg.guild_id else {
        ctx.reply("faild to get guild", &msg).await;
        return;
    };

    let mut greet = ctx.reply("downloading for u", &msg).await;

    let input: Bytes = if args.len() == 2 {
        // songbird::input::YoutubeDl::new(ctx.data.read().await.get::<HttpClientKey>().cloned().unwrap(), args[1].to_string()).into()
        if let Ok(resp) = ctx.get(args[1]).await {
            resp
        } else {
            greet
                .edit(ctx.http, edit_message("faild to download"))
                .await
                .unwrap();
            return;
        }
    } else if !msg.attachments.is_empty() {
        if let Ok(input) = ctx.get(&msg.attachments[0].url).await {
            input.into()
        } else {
            greet
                .edit(ctx.http, edit_message("faild to download"))
                .await
                .unwrap();
            return;
        }
    } else {
        greet
            .edit(ctx.http, edit_message("u dont say wat i play"))
            .await
            .unwrap();
        return;
    };

    let Ok(channels) = guild.channels(&ctx).await else {
        ctx.reply("faild to get channels", &msg).await;
        return;
    };

    if let Some(handler) = manager.get(guild) {
        let mut handler = handler.lock().await;
        // join vc if not already in one
        if handler.current_connection().is_none() {
            let channel = channels.iter().find_map(|c| {
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
            });

            let Some(channel) = channel else {
                ctx.reply("u arent in a vc", &msg).await;
                return;
            };

            if handler.join(channel.id).await.is_err() {
                ctx.reply("faild to join u", &msg).await;
                return;
            };
        }
        let track = handler.play_only_input(input.into());

        let global = ctx.data.read().await;

        if let Some(old_track) = global.get::<TrackHandleKey>() {
            if old_track.get_info().await.unwrap().loops == LoopState::Infinite {
                track.enable_loop().unwrap();
            }
        }

        ctx.data.write().await.insert::<TrackHandleKey>(track);

        greet
            .edit(ctx.http, edit_message("playing for u!"))
            .await
            .unwrap();
    } else {
        greet
            .edit(ctx.http, edit_message("faild to get voice handler"))
            .await
            .unwrap();
    }
}
