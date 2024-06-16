use serenity::all::{Context, Message};

use crate::utils::context::Ext;
use crate::TrackHandleKey;

pub async fn stop(ctx: &Context, msg: &Message) -> Result<(), &'static str> {
    let global = ctx.data.try_read().unwrap();
    if global.contains_key::<TrackHandleKey>() {
        let Some(track) = global.get::<TrackHandleKey>().cloned() else {
            return Err("faild to stop");
        };

        track.stop().unwrap();
        drop(global); // unlock the typemap

        ctx.data.write().await.remove::<TrackHandleKey>();
        ctx.reply("stopd", msg).await;
    } else {
        ctx.reply("im not play anything", msg).await;
    }

    Ok(())
}
