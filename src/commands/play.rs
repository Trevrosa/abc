use anyhow::Result;
use serenity::all::{Context, Message};
// use songbird::{tracks::TrackHandle, SongbirdKey};

use super::{edit_message, Reply};
use crate::error::{GeneralError::*, PlayError::*, VoiceError::*};

pub async fn play(ctx: Context, msg: Message) -> Result<()> {
    let Some(manager) = songbird::get(&ctx).await else {
        ctx.reply("voice client not init", &msg).await;
        return Err(VoiceClientNotInit.into());
    };

    let media = msg.content.trim().split(' ').collect::<Vec<&str>>();

    let Some(guild) = msg.guild_id else {
        ctx.reply("faild to get guild", &msg).await;
        return Err(VoiceClientNotInit.into());
    };

    let mut greet = ctx.reply("downloading for u", &msg).await;

    let input = if media.len() == 2 {
        if let Ok(resp) = reqwest::get(media[1]).await {
            if let Ok(bytes) = resp.bytes().await {
                bytes
            } else {
                greet
                    .edit(ctx.http, edit_message("faild to download"))
                    .await?;
                return Err(DownloadError.into());
            }
        } else {
            greet
                .edit(ctx.http, edit_message("faild to download"))
                .await?;
            return Err(DownloadError.into());
        }
    } else if !msg.attachments.is_empty() {
        if let Ok(input) = msg.attachments[0].download().await {
            input.into()
        } else {
            greet
                .edit(ctx.http, edit_message("faild to download"))
                .await?;
            return Err(DownloadError.into());
        }
    } else {
        greet
            .edit(ctx.http, edit_message("u dont say wat i play"))
            .await?;
        return Err(ArgumentError.into());
    };

    if let Some(handler) = manager.get(guild) {
        let mut handler = handler.lock().await;
        handler.play_only_input(input.into());

        // let global_track = ctx.data.write().await;
        // global_track.clear();
        // global_track.insert::<TrackHandle>(track);

        ctx.reply("playing for u!", &msg).await;
        Ok(())
    } else {
        ctx.reply("faild to play", &msg).await;
        return Err(CommandFailed.into());
    }
}
