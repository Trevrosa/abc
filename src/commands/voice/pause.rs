use serenity::all::{Context, CreateCommand, InteractionContext};

use crate::utils::context::CtxExt;
use crate::utils::reply::Replyer;
use crate::TrackHandleKey;

pub async fn pause(ctx: &Context, replyer: &Replyer<'_>) -> Result<(), &'static str> {
    let global = ctx.data.try_read().unwrap();

    if global.contains_key::<TrackHandleKey>() {
        let Some(track) = global.get::<TrackHandleKey>() else {
            return Err("faild to pause");
        };

        track.pause().unwrap();
        drop(global);

        ctx.reply("pausd", replyer).await;
    } else {
        return Err("im not play anything");
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("pause")
        .add_context(InteractionContext::Guild)
        .description("pause bot playback")
}
