use std::path::Path;

use bytes::Bytes;
use serenity::all::{
    ChannelType, CommandOptionType, Context, CreateCommand, CreateCommandOption, InteractionContext,
};
use tokio::{
    fs::{remove_file, File},
    io::AsyncReadExt,
};
use tracing::{info, warn};

use crate::{
    utils::{context::CtxExt, ArgValue, Args, DeleteWhenDone},
    CLIENT,
};
use crate::{
    utils::{reply::Replyer, spotify::extract_spotify},
    TrackHandleKey,
};

// TODO: queuing
pub async fn play(
    ctx: &Context,
    replyer: &Replyer<'_>,
    args: Args<'_>,
) -> Result<(), &'static str> {
    let Some(manager) = songbird::get(ctx).await else {
        return Err("voice client not init");
    };

    let guild = match replyer {
        Replyer::Prefix(msg) => msg.guild_id,
        Replyer::Slash(int) => int.guild_id,
    };

    let Some(guild) = guild else {
        return Err("faild to get guild");
    };

    let mut greet = ctx.reply("ok..", replyer).await;

    if args.is_empty() {
        ctx.edit_msg("u dont say wat i play", &mut greet).await;
        return Err("");
    }

    let track_path = format!("current_track{}", guild.get());
    let track_path = Path::new(&track_path);

    // its ok to delete the file because we read it to memory after anyway
    let _cleanup = DeleteWhenDone::new(track_path);

    let mut is_spotify = false;
    let input: Bytes = if let Some(ArgValue::String(url)) = args.first_value() {
        if Path::new(&track_path).exists() {
            remove_file(&track_path).await.unwrap();
        }

        let url = if url.contains("spotify.com") {
            ctx.reply(
                "this is a spotify url, we need to do some stuff first.",
                replyer,
            )
            .await;
            is_spotify = true;
            extract_spotify(ctx, replyer, url).await?
        } else {
            (*url).to_string()
        };

        let mut greet = ctx.reply("now im downloading..", replyer).await;

        ctx.yt_dlp(url.as_str(), Some(&track_path), "ba*", None, &mut greet)
            .await?;

        let mut bytes = Vec::new();
        if File::open(&track_path)
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
    } else if let Some(ArgValue::Attachment(attachment)) = args.first_value() {
        let data = ctx.data.try_read().unwrap();

        let Ok(request) = CLIENT.get(&attachment.url).build() else {
            drop(data);
            return Err("faild to create request");
        };

        ctx.edit_msg("downloading now", &mut greet).await;

        let Ok(response) = CLIENT.execute(request).await else {
            drop(data);
            return Err("faild to download");
        };

        info!("downloaded {} with reqwest", &attachment.url);

        let Ok(bytes) = response.bytes().await else {
            ctx.edit_msg("faild to decode file", &mut greet).await;
            drop(data);
            return Err("");
        };

        bytes
    } else {
        warn!("unexpected args {args:?}");
        ctx.edit_msg("u dont say wat i play", &mut greet).await;
        return Err("");
    };

    let Ok(channels) = guild.channels(&ctx).await else {
        ctx.edit_msg("faild to get channels", &mut greet).await;
        return Err("");
    };

    let mut channels = channels.iter();
    let user = match replyer {
        Replyer::Prefix(msg) => &msg.author,
        Replyer::Slash(int) => &int.user,
    };

    // join vc if bot has never joined a vc
    if manager.get(guild).is_none() {
        let Some(channel) = ctx.find_user_channel(user, ChannelType::Voice, &mut channels) else {
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
            let Some(channel) = ctx.find_user_channel(user, ChannelType::Voice, &mut channels)
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
        if is_spotify {
            ctx.reply("playing for u!", replyer).await;
        } else {
            ctx.edit_msg("playing for u!", &mut greet).await;
        }
    } else if is_spotify {
        ctx.reply("faild to get voice handler", replyer).await;
    } else {
        ctx.edit_msg("faild to get voice handler", &mut greet).await;
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("play")
        .description("play a song")
        .add_context(InteractionContext::Guild)
        .add_option(CreateCommandOption::new(
            CommandOptionType::Attachment,
            "songfile",
            "the song to play",
        ))
        .add_option(CreateCommandOption::new(
            CommandOptionType::String,
            "songurl",
            "the url of the song to play",
        ))
}
