use anyhow::Result;
use serenity::all::{Context, Message};

use crate::{error::General::CommandFailed, TrackHandleKey};

use super::Reply;

pub async fn pause(ctx: Context, msg: Message) -> Result<()> {
    let global_track = ctx.data.read().await;

    if global_track.is_empty() {
        ctx.reply("im not play anything", &msg).await;
        return Err(CommandFailed.into());
    } else {
        let Some(track) = global_track.get::<TrackHandleKey>() else {
            return Err(CommandFailed.into());
        };

        track.pause()?;
        Ok(())
    }
}
