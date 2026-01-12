use macro_cmd::command;
use serenity::all::{Context, CreateCommand, InteractionContext};

use glue::TrackHandleKey;
use utils::context::CtxExt;
use utils::reply::Replyer;

#[command]
pub async fn stop(ctx: &Context, replyer: &Replyer<'_>) -> Result<(), &'static str> {
    let data = ctx.data.try_read().unwrap();
    if data.contains_key::<TrackHandleKey>() {
        let Some(track) = data.get::<TrackHandleKey>().cloned() else {
            return Err("faild to stop");
        };

        track.stop().unwrap();
        drop(data); // unlock the typemap

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
