use serenity::all::{Context, CreateCommand, InteractionContext};

use crate::utils::context::CtxExt;
use crate::utils::reply::Replyer;

pub async fn status(ctx: &Context, replyer: &Replyer<'_>) -> Result<(), &'static str> {
    let Some(guild_id) = replyer.guild() else {
        return Err("u not in a guild");
    };

    let manager = songbird::get(ctx).await.unwrap();

    if let Some(handler) = manager.get(guild_id) {
        let handler = handler.lock().await;
        let queue = handler.queue();

        if queue.is_empty() {
            return Err("im not play anything");
        }

        let plural = if queue.len() > 1 { "s" } else { "" };

        let msg = format!("{} queued song{plural}", queue.len());
        ctx.reply(msg, replyer).await;

        let status = queue
            .current()
            .expect("queue is not empty")
            .get_info()
            .await;

        ctx.reply(format!("current song: ```rust\n{status:#?}\n```"), replyer)
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
