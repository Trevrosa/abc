use serenity::{
    all::{ActivityData, Context, EventHandler, Message, OnlineStatus, Ready},
    async_trait,
};

use crate::commands::{self, Reply};

#[derive(Debug)]
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        ctx.set_presence(
            Some(ActivityData::custom("Disrupting the Social Democrats")),
            OnlineStatus::DoNotDisturb,
        );
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.is_private() && !msg.is_own(&ctx.cache) {
            if msg.author.name == "devon03747" {
                ctx.reply("wasup boss", &msg).await;
            } else {
                ctx.reply("im busy working", &msg).await;
            }
            return;
        }

        if !msg.content.starts_with('`') {
            return;
        }

        match &msg.content.split(' ').collect::<Vec<&str>>()[0][1..] {
            "test" => commands::test(ctx, msg).await,
            "join" => commands::join(ctx, msg).await,
            "leave" => commands::leave(ctx, msg).await,
            "play" => commands::voice::play(ctx, msg).await,
            "pause" => commands::voice::pause(ctx, msg).await,
            "resume" | "unpause" => commands::voice::resume(ctx, msg).await,
            "status" => commands::voice::status(ctx, msg).await,
            "loop" => commands::voice::set_loop(ctx, msg).await,
            "stop" => commands::voice::stop(ctx, msg).await,
            &_ => (),
        };
    }
}
