use serenity::{
    all::{ActivityData, Context, EventHandler, Message, OnlineStatus, Ready},
    async_trait,
};

use crate::commands;

#[derive(Debug)]
pub struct Listener;

#[async_trait]
impl EventHandler for Listener {
    async fn ready(&self, ctx: Context, _: Ready) {
        ctx.set_presence(
            Some(ActivityData::custom("Disrupting the Social Democrats")),
            OnlineStatus::DoNotDisturb,
        );
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.content.starts_with('`') {
            return;
        }

        let _ = match &msg.content.split(' ').collect::<Vec<&str>>()[0][1..] {
            "test" => commands::test(ctx, msg).await,
            "join" => commands::join(ctx, msg).await,
            "leave" => commands::leave(ctx, msg).await,
            "play" => commands::play(ctx, msg).await,
            "pause" => commands::pause(ctx, msg).await,
            "resume" => commands::resume(ctx, msg).await,
            "status" => commands::status(ctx, msg).await,
            "loop" => commands::set_loop(ctx, msg).await,
            &_ => Ok(()),
        };
    }
}
