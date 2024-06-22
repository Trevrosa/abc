use serenity::all::{Context, Message};

use crate::utils::context::Ext;
use crate::TrackHandleKey;

pub async fn status(ctx: &Context, msg: &Message) -> Result<(), &'static str> {
    let global = ctx.data.try_read().unwrap();

    if global.contains_key::<TrackHandleKey>() {
        let Some(track) = global.get::<TrackHandleKey>() else {
            return Err("song ended..");
        };

        let status = track.get_info().await;
        drop(global);

        ctx.reply(format!("```rust\n{status:#?}\n```",), msg).await;
    } else {
        return Err("im not play anything");
    }

    Ok(())
}
