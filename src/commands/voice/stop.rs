use serenity::all::{Context, CreateCommand, InteractionContext};

use crate::utils::context::CtxExt;
use crate::utils::reply::Replyer;
use crate::TrackHandleKey;

pub async fn stop(ctx: &Context, replyer: &Replyer<'_>) -> Result<(), &'static str> {
    let global = ctx.data.try_read().unwrap();
    if global.contains_key::<TrackHandleKey>() {
        let Some(track) = global.get::<TrackHandleKey>().cloned() else {
            return Err("faild to stop");
        };

        track.stop().unwrap();
        drop(global); // unlock the typemap

        ctx.data.write().await.remove::<TrackHandleKey>();
        ctx.reply("stopd", replyer).await;
    } else {
        ctx.reply("im not play anything", replyer).await;
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("stop")
        .add_context(InteractionContext::Guild)
        .description("stop bot playback")
}
