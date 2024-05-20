use serenity::all::{Context, Message};

use super::{edit_message, Reply};
use crate::TrackHandleKey;

pub async fn play(ctx: Context, msg: Message) {
    let global_track = ctx.data.read().await;
    let global_track = global_track.get::<TrackHandleKey>();

    if global_track.is_some() {
        let mut global_track = ctx.data.write().await;
        global_track.clear();
    }

    let Some(manager) = songbird::get(&ctx).await else {
        ctx.reply("voice client not init", &msg).await;
        return;
    };

    let media = msg.content.trim().split(' ').collect::<Vec<&str>>();

    let Some(guild) = msg.guild_id else {
        ctx.reply("faild to get guild", &msg).await;
        return;
    };

    let mut greet = ctx.reply("downloading for u", &msg).await;

    let input = if media.len() == 2 {
        if let Ok(resp) = reqwest::get(media[1]).await {
            if let Ok(bytes) = resp.bytes().await {
                bytes
            } else {
                greet
                    .edit(ctx.http, edit_message("faild to download"))
                    .await
                    .unwrap();
                return;
            }
        } else {
            greet
                .edit(ctx.http, edit_message("faild to download"))
                .await
                .unwrap();
            return;
        }
    } else if !msg.attachments.is_empty() {
        if let Ok(input) = msg.attachments[0].download().await {
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

    if let Some(handler) = manager.get(guild) {
        let mut handler = handler.lock().await;
        let track = handler.play_only_input(input.into());

        greet
            .edit(ctx.http, edit_message("playing for u!"))
            .await
            .unwrap();
        // track.add_event(Event::Track(TrackEvent::End), VoiceHandler);

        let mut global_track = ctx.data.write().await;
        global_track.clear();
        global_track.insert::<TrackHandleKey>(track);
    } else {
        greet
            .edit(ctx.http, edit_message("faild to get voice handler"))
            .await
            .unwrap();
    }
}
