use std::{
    fs::{remove_file, File},
    io::{stdout, BufRead, BufReader, Read},
    path::Path,
    process::{Command, Stdio},
};

use bytes::Bytes;
use serenity::all::{ChannelType, Context, Message};
use tracing::info;

use crate::utils::context::Ext;
use crate::{HttpClient, TrackHandleKey};

pub async fn play(ctx: &Context, msg: &Message) -> Result<(), &'static str> {
    let Some(manager) = songbird::get(ctx).await.clone() else {
        return Err("voice client not init");
    };

    let args = msg.content.trim().split(' ').collect::<Vec<&str>>();

    let Some(guild) = msg.guild_id else {
        return Err("faild to get guild");
    };

    let mut greet = ctx.reply("downloading for u", msg).await;

    let input: Bytes = if args.len() == 2 {
        if Path::new("current_track").exists() {
            remove_file("current_track").unwrap();
        }

        let downloader = Command::new("/usr/bin/yt-dlp")
            // ba* = choose best quality format with audio, which might be video
            // see: https://github.com/yt-dlp/yt-dlp?tab=readme-ov-file#format-selection
            .args([args[1], "-o", "current_track", "-f", "ba*"])
            .stdout(Stdio::piped())
            .stderr(stdout())
            .spawn();

        if let Ok(mut downloader) = downloader {
            {
                let output = downloader.stdout.as_mut().unwrap();
                let reader = BufReader::new(output);

                for (i, chunk) in reader.lines().enumerate() {
                    let new_msg = if i == 0 {
                        format!("```{}```", chunk.unwrap().trim())
                    } else {
                        // should work since we put ``` already at the start of msg
                        format!(
                            "{}\n{}```",
                            &greet.content.strip_suffix("```").unwrap(),
                            chunk.unwrap().trim()
                        )
                    };

                    ctx.edit_msg(new_msg, &mut greet).await;
                }
            }

            if !downloader.wait().unwrap().success() {
                ctx.edit_msg("download faild", &mut greet).await;
                return Err("");
            }

            info!("downloaded {} with yt-dlp", args[1]);
            let mut bytes: Vec<u8> = Vec::new();

            if File::open("current_track")
                .unwrap()
                .read_to_end(&mut bytes)
                .is_err()
            {
                ctx.edit_msg("faild to read file", &mut greet).await;
                return Err("");
            }

            bytes.into()
        } else {
            ctx.edit_msg("faild to start download", &mut greet).await;
            return Err("");
        }
    } else if !msg.attachments.is_empty() {
        let global = ctx.data.try_read().unwrap();
        let client = global.get::<HttpClient>().unwrap();

        let Ok(request) = client.get(&msg.attachments[0].url).build() else {
            drop(global);
            return Err("faild to create request");
        };

        let Ok(response) = client.execute(request).await else {
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
        };
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
            };
        }

        let track = handler.play_only_input(input.into());

        ctx.data.write().await.insert::<TrackHandleKey>(track);
        ctx.edit_msg("playing for u!", &mut greet).await;
    } else {
        ctx.edit_msg("faild to get voice handler", &mut greet).await;
    }

    Ok(())
}
