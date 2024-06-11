use serenity::{
    all::{Context, EventHandler, Message},
    async_trait,
};
use tracing::info;

use crate::{commands, OWNER};
use crate::{utils::context::Ext, Blacklisted, SEVEN};

#[derive(Debug)]
pub struct CommandHandler;

#[async_trait]
impl EventHandler for CommandHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.guild_id.is_none()
            && msg.author != **ctx.cache.current_user()
            && msg.author.id != OWNER
        {
            if msg.author.id == SEVEN {
                ctx.reply("wasup boss", &msg).await;
            } else {
                ctx.reply("im busy", &msg).await;
            }
            return;
        }

        let typing = msg.channel_id.start_typing(&ctx.http);

        // here, we want to wait instead of panicking.
        #[allow(clippy::disallowed_methods)]
        let global = ctx.data.read().await;
        let blacklisted = global.get::<Blacklisted>().unwrap();

        if !msg.content.starts_with('`') || blacklisted.contains(&msg.author.id.get()) {
            drop(global);
            return;
        }

        drop(global);

        info!("received cmd '{}'", &msg.content);

        match &msg.content.split(' ').collect::<Vec<&str>>()[0][1..] {
            // misc commands
            "test" => commands::test(ctx, msg).await,
            "cat" => commands::cat(ctx, msg).await,
            "black" => commands::blacklist(ctx, msg).await,

            "join" => commands::join(ctx, msg).await,
            "leave" => commands::leave(ctx, msg).await,

            "snipe" => commands::snipe(ctx, msg).await,
            "editsnipe" => commands::edit_snipe(ctx, msg).await,

            // voice commands
            "play" => commands::voice::play(ctx, msg).await,
            "pause" => commands::voice::pause(ctx, msg).await,
            "resume" | "unpause" => commands::voice::resume(ctx, msg).await,
            "status" => commands::voice::status(ctx, msg).await,
            "loop" => commands::voice::set_loop(ctx, msg).await,
            "stop" => commands::voice::stop(ctx, msg).await,
            "seek" => commands::voice::seek(ctx, msg).await,

            // do nothing if not matched
            &_ => (),
        };

        typing.stop();
    }
}
