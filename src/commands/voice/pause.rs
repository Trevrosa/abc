use serenity::all::{Context, Message};

use crate::utils::context::Ext;
use crate::TrackHandleKey;

pub async fn pause(ctx: &Context, msg: &Message) -> Result<(), &'static str> {
    let global = ctx.data.try_read().unwrap();

    if global.contains_key::<TrackHandleKey>() {
        let Some(track) = global.get::<TrackHandleKey>() else {
            return Err("faild to pause");
        };

        track.pause().unwrap();
        ctx.reply("pausd", &msg).await;
    } else {
        return Err("im not play anything");
    }

    Ok(())
}
