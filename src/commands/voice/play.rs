use std::path::Path;

use bytes::Bytes;
use serenity::all::{ChannelType, Context, Message};
use tokio::{
    fs::{remove_file, File},
    io::AsyncReadExt,
};
use tracing::info;

use crate::TrackHandleKey;
use crate::{utils::context::Ext, CLIENT};

pub async fn play(ctx: &Context, msg: &Message) -> Result<(), &'static str> {
    let Some(manager) = songbird::get(ctx).await else {
        return Err("voice client not init");
    };

    let args = msg.content.trim().split(' ').collect::<Vec<&str>>();

    let Some(guild) = msg.guild_id else {
        return Err("faild to get guild");
    };

    let mut greet = ctx.reply("downloading for u", msg).await;

    let input: Bytes = if args.len() == 2 {
        if Path::new("current_track").exists() {
            remove_file("current_track").await.unwrap();
        }

        // FIXME: change to current_track{GUILD} so it works for multiple servers at the same time
        ctx.yt_dlp(args[1], Some("current_track"), "ba*", None, &mut greet)
            .await?;

        let mut bytes = Vec::new();
        if File::open("current_track")
            .await
            .unwrap()
            .read_to_end(&mut bytes)
            .await
            .is_err()
        {
            ctx.edit_msg("faild to read file", &mut greet).await;
            return Err("");
        }

        bytes.into()
    } else if !msg.attachments.is_empty() {
        let global = ctx.data.try_read().unwrap();

        let Ok(request) = CLIENT.get(&msg.attachments[0].url).build() else {
            drop(global);
            return Err("faild to create request");
        };

        let Ok(response) = CLIENT.execute(request).await else {
            drop(global);
            return Err("faild to download");
        };

        info!("downloaded {} with reqwest", &msg.attachments[0].url);

        let Ok(bytes) = response.bytes().await else {
            ctx.edit_msg("faild to decode file", &mut greet).await;
            drop(global);
            return Err("");
        };

        bytes
    } else {
        ctx.edit_msg("u dont say wat i play", &mut greet).await;
        return Err("");
    };

    let Ok(channels) = guild.channels(&ctx).await else {
        ctx.edit_msg("faild to get channels", &mut greet).await;
        return Err("");
    };

    let mut channels = channels.iter();

    // join vc if bot has never joined a vc
    if manager.get(guild).is_none() {
        let Some(channel) = ctx.find_user_channel(&msg.author, ChannelType::Voice, &mut channels)
        else {
            ctx.edit_msg("u arent in a vc", &mut greet).await;
            return Err("");
        };

        if manager.join(guild, channel.id).await.is_err() {
            ctx.edit_msg("faild to join u", &mut greet).await;
            return Err("");
        }
    }

    if let Some(handler) = manager.get(guild) {
        let mut handler = handler.lock().await;

        // join vc if bot is not currently in a vc
        if handler.current_connection().is_none() {
            let Some(channel) =
                ctx.find_user_channel(&msg.author, ChannelType::Voice, &mut channels)
            else {
                ctx.edit_msg("u arent in a vc", &mut greet).await;
                return Err("");
            };

            if handler.join(channel.id).await.is_err() {
                ctx.edit_msg("faild to join u", &mut greet).await;
                return Err("");
            }
        }

        let track = handler.play_only_input(input.into());
        drop(handler);

        ctx.data.write().await.insert::<TrackHandleKey>(track);
        ctx.edit_msg("playing for u!", &mut greet).await;
    } else {
        ctx.edit_msg("faild to get voice handler", &mut greet).await;
    }

    Ok(())
}
