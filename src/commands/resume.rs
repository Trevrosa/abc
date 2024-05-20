use anyhow::Result;
use serenity::all::{Context, Message};

use super::Reply;
use crate::{error::General::CommandFailed, TrackHandleKey};

pub async fn resume(ctx: Context, msg: Message) -> Result<()> {
    let global_track = ctx.data.read().await;

    if global_track.is_empty() {
        ctx.reply("im not play anything", &msg).await;
        return Err(CommandFailed.into());
    }
    
    let Some(track) = global_track.get::<TrackHandleKey>() else {
        return Err(CommandFailed.into());
    };

    track.play()?;
    Ok(())
}
