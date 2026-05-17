use std::path::Path;

use abc::DeleteWhenDone;
use bytes::Bytes;
use reqwest::Url;
use serenity::all::{
    ChannelType, CommandOptionType, Context, CreateCommand, CreateCommandOption, InteractionContext,
};
use tokio::fs::remove_file;
use tracing::{info, warn};

use crate::Volume;
use crate::utils::spotify::search;
use crate::utils::{reply::Replyer, spotify::extract_spotify};
use crate::{
    CLIENT,
    utils::{ArgValue, Args, context::CtxExt},
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

    let Some(guild_id) = replyer.guild() else {
        return Err("faild to get guild");
    };

    let mut greet = ctx.reply("ok..", replyer).await;

    if args.is_empty() {
        ctx.edit_msg("u dont say wat i play", &mut greet).await;
        return Err("");
    }

    let track_path = format!("current_track{}", guild_id.get());
    let track_path = Path::new(&track_path);

    // its ok to delete the file because we read it to memory after anyway
    let _cleanup = DeleteWhenDone::new(track_path);

    let mut is_spotify = false;
    let input: Bytes = if let Some(ArgValue::String(input)) = args.first_value() {
        if Path::new(&track_path).exists() {
            remove_file(&track_path).await.unwrap();
        }

        let url = if Url::parse(input).is_ok() {
            if input.contains("spotify.com") {
                is_spotify = true;
                extract_spotify(ctx, replyer, input).await?
            } else {
                input.to_string()
            }
        } else {
            search(ctx, replyer, args.full_string()).await?
        };

        let mut greet = ctx.reply("now im downloading..", replyer).await;

        ctx.yt_dlp(url.as_str(), Some(&track_path), "ba*", None, &mut greet)
            .await?;

        let Ok(bytes) = tokio::fs::read(&track_path).await else {
            ctx.edit_msg("faild to read file", &mut greet).await;
            return Err("");
        };

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

    let Ok(channels) = guild_id.channels(&ctx).await else {
        ctx.edit_msg("faild to get channels", &mut greet).await;
        return Err("");
    };

    let mut channels = channels.iter();
    let user = replyer.user();

    // join vc if bot has never joined a vc
    if manager.get(guild_id).is_none() {
        let Some(channel) = ctx.find_user_channel(user, ChannelType::Voice, &mut channels) else {
            ctx.edit_msg("u arent in a vc", &mut greet).await;
            return Err("");
        };

        if manager.join(guild_id, channel.id).await.is_err() {
            ctx.edit_msg("faild to join u", &mut greet).await;
            return Err("");
        }
    }

    if let Some(handler) = manager.get(guild_id) {
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

        let track = handler.enqueue(input.into()).await;

        {
            if let Ok(data) = ctx.data.try_read()
                && let Some(volumes) = data.get::<Volume>()
                && let Some(volume) = volumes.get(&guild_id)
            {
                track.set_volume(*volume).unwrap();
            }
        }

        let queue_len = handler.queue().len();
        drop(handler);

        let msg = if queue_len == 1 {
            "playing for u!"
        } else {
            let queued = queue_len - 1;
            let plural = if queued > 1 { "s" } else { "" };
            &format!("added to queue ({queued} song{plural} till urs)")
        };

        if is_spotify {
            ctx.reply(msg, replyer).await;
        } else {
            ctx.edit_msg(msg, &mut greet).await;
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
            "song",
            "the url of the song to play, or a query to search yt with",
        ))
}
