use macro_cmd::command;
use serenity::all::{Context, CreateCommand, InteractionContext};

use glue::TrackHandleKey;
use utils::context::CtxExt;
use utils::reply::Replyer;

#[command]
pub async fn pause(ctx: &Context, replyer: &Replyer<'_>) -> Result<(), &'static str> {
    let data = ctx.data.try_read().unwrap();

    if data.contains_key::<TrackHandleKey>() {
        let Some(track) = data.get::<TrackHandleKey>() else {
            return Err("faild to pause");
        };

        track.pause().unwrap();
        drop(data);

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
