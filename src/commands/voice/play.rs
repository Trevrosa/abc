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
        if Path::new("current_track").exists() {
            remove_file("current_track").unwrap();
        }

        let downloader = Command::new("/usr/bin/yt-dlp")
            // ba* = choose best quality format with audio, which might be video
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

                    // ignore error since command will still work if msg not edited
                    let _ = ctx.edit_msg(new_msg, &mut greet).await;
                }
            }

            if !downloader.wait().unwrap().success() {
                ctx.edit_msg("download faild", &mut greet).await.unwrap();
                return;
            }

            info!("downloaded {} with yt-dlp", args[1]);
            let mut bytes: Vec<u8> = Vec::new();

            if File::open("current_track")
                .unwrap()
                .read_to_end(&mut bytes)
                .is_err()
            {
                ctx.edit_msg("faild to read file", &mut greet)
                    .await
                    .unwrap();
                return;
            }

            bytes.into()
        } else {
            ctx.edit_msg("faild to start download", &mut greet)
                .await
                .unwrap();
            return;
        }
    } else if !msg.attachments.is_empty() {
        let global = ctx.data.try_read().unwrap();
        let client = global.get::<HttpClient>().unwrap();

        let Ok(request) = client.get(&msg.attachments[0].url).build() else {
            drop(global);
            return;
        };

        let Ok(response) = client.execute(request).await else {
            drop(global);
            return;
        };

        info!("downloaded {} with reqwest", &msg.attachments[0].url);

        let Ok(bytes) = response.bytes().await else {
            ctx.edit_msg("faild to decode file", &mut greet)
                .await
                .unwrap();
            drop(global);
            return;
        };

        bytes
    } else {
        ctx.edit_msg("u dont say wat i play", &mut greet)
            .await
            .unwrap();
        return;
    };

    let Ok(channels) = guild.channels(&ctx).await else {
        ctx.reply("faild to get channels", &msg).await;
        return;
    };

    let mut channels = channels.iter();

    // join vc if bot has never joined a vc
    if manager.get(guild).is_none() {
        let Some(channel) = ctx.find_user_channel(&msg.author, ChannelType::Voice, &mut channels)
        else {
            ctx.reply("u arent in a vc", &msg).await;
            return;
        };

        if manager.join(guild, channel.id).await.is_err() {
            ctx.reply("faild to join u", &msg).await;
            return;
        };
    }

    if let Some(handler) = manager.get(guild) {
        let mut handler = handler.lock().await;

        // join vc if bot is not currently in a vc
        if handler.current_connection().is_none() {
            let Some(channel) =
                ctx.find_user_channel(&msg.author, ChannelType::Voice, &mut channels)
            else {
                ctx.reply("u arent in a vc", &msg).await;
                return;
            };

            if handler.join(channel.id).await.is_err() {
                ctx.reply("faild to join u", &msg).await;
                return;
            };
        }

        let track = handler.play_only_input(input.into());

        ctx.data.write().await.insert::<TrackHandleKey>(track);
        ctx.edit_msg("playing for u!", &mut greet).await.unwrap();
    } else {
        ctx.edit_msg("faild to get voice handler", &mut greet)
            .await
            .unwrap();
    }
}
