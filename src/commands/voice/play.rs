use std::{
    fs::{remove_file, File},
    io::{stdout, BufRead, BufReader, Read},
    path::Path,
    process::{Command, Stdio},
};

use bytes::Bytes;
use serenity::all::{ChannelType, Context, Message};
use tracing::info;

use super::Utils;
use crate::{HttpClientKey, TrackHandleKey};

// TODO: add progress incicator/bar

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
            .args(vec![args[1], "-o", "current_track", "-f", "ba*"]) // ba* = choose best quality audio format, maybe video
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
                        // should work since we put ``` already
                        format!(
                            "{}\n{}```",
                            &greet.content.strip_suffix("```").unwrap(),
                            chunk.unwrap().trim()
                        )
                    };
                    ctx.edit_msg(&new_msg, &mut greet).await;
                }
            }

            if !downloader.wait().unwrap().success() {
                return;
            }

            info!("downloaded {} with yt-dlp", args[1]);
            let mut bytes: Vec<u8> = Vec::new();

            if File::open("current_track")
                .unwrap()
                .read_to_end(&mut bytes)
                .is_err()
            {
                ctx.edit_msg("faild to read file", &mut greet).await;
                return;
            }

            bytes.into()
        } else {
            ctx.edit_msg("faild to start download", &mut greet).await;
            return;
        }
    } else if !msg.attachments.is_empty() {
        let global = ctx.data.try_read().unwrap();
        let client = global.get::<HttpClientKey>().unwrap();

        let Ok(request) = client.get(&msg.attachments[0].url).build() else {
            drop(global);
            return;
        };

        let Ok(response) = client.execute(request).await else {
            drop(global);
            return;
        };

        info!("downloaded {} with reqwest", args[1]);

        if let Ok(bytes) = response.bytes().await {
            bytes
        } else {
            ctx.edit_msg("faild to decode file", &mut greet).await;
            drop(global);
            return;
        }
    } else {
        ctx.edit_msg("u dont say wat i play", &mut greet).await;
        return;
    };

    let Ok(channels) = guild.channels(&ctx).await else {
        ctx.reply("faild to get channels", &msg).await;
        return;
    };

    // join vc if bot has never joined a vc on run
    if manager.get(guild).is_none() {
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

        if manager.join(guild, channel.id).await.is_err() {
            ctx.reply("faild to join u", &msg).await;
            return;
        };
    }

    if let Some(handler) = manager.get(guild) {
        let mut handler = handler.lock().await;

        // join vc if bot is not currently in a vc
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
        ctx.data.write().await.insert::<TrackHandleKey>(track);
        ctx.edit_msg("playing for u!", &mut greet).await;
    } else {
        ctx.edit_msg("faild to get voice handler", &mut greet).await;
    }
}
