use anyhow::Result;
use serenity::all::{Context, Message};

use super::{edit_message, Reply};
use crate::{
    error::{
        General::{Argument, DiscordGet},
        PlayCommand::Download,
        Voice::{Handler, VoiceClientNotInit},
    },
    TrackHandleKey,
};

pub async fn play(ctx: Context, msg: Message) -> Result<()> {
    let Some(manager) = songbird::get(&ctx).await else {
        ctx.reply("voice client not init", &msg).await;
        return Err(VoiceClientNotInit.into());
    };

    let media = msg.content.trim().split(' ').collect::<Vec<&str>>();

    let Some(guild) = msg.guild_id else {
        ctx.reply("faild to get guild", &msg).await;
        return Err(DiscordGet.into());
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
                return Err(Download.into());
            }
        } else {
            greet
                .edit(ctx.http, edit_message("faild to download"))
                .await?;
            return Err(Download.into());
        }
    } else if !msg.attachments.is_empty() {
        if let Ok(input) = msg.attachments[0].download().await {
            input.into()
        } else {
            greet
                .edit(ctx.http, edit_message("faild to download"))
                .await?;
            return Err(Download.into());
        }
    } else {
        greet
            .edit(ctx.http, edit_message("u dont say wat i play"))
            .await?;
        return Err(Argument.into());
    };

    if let Some(handler) = manager.get(guild) {
        let mut handler = handler.lock().await;
        let track = handler.play_only_input(input.into());

        let mut global_track = ctx.data.write().await;
        global_track.clear();
        global_track.insert::<TrackHandleKey>(track);

        greet.edit(ctx.http, edit_message("playing for u!")).await?;
        Ok(())
    } else {
        greet
            .edit(ctx.http, edit_message("faild to get voice handler"))
            .await?;
        Err(Handler.into())
    }
}
