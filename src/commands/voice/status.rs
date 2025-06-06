use serenity::all::{Context, CreateCommand, InteractionContext};

use crate::utils::context::CtxExt;
use crate::utils::reply::Replyer;
use crate::TrackHandleKey;

pub async fn status(ctx: &Context, replyer: &Replyer<'_>) -> Result<(), &'static str> {
    let data = ctx.data.try_read().unwrap();

    if data.contains_key::<TrackHandleKey>() {
        let Some(track) = data.get::<TrackHandleKey>() else {
            return Err("song ended..");
        };

        let status = track.get_info().await;
        drop(data);

        ctx.reply(format!("```rust\n{status:#?}\n```",), replyer)
            .await;
    } else {
        return Err("im not play anything");
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("status")
        .add_context(InteractionContext::Guild)
        .description("get playback status")
}
