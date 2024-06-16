use serenity::{
    all::{Context, EventHandler, Message},
    async_trait,
};
use tracing::info;

use crate::{commands, OWNER};
use crate::{utils::context::Ext, Blacklisted, SEVEN};

#[derive(Debug)]
pub struct CommandHandler;

#[inline]
pub async fn handle_cmd(cmd: &str, ctx: &Context, msg: &Message) -> Result<(), &'static str> {
    match cmd {
        // misc commands
        "cat" => commands::cat(&ctx, &msg).await,
        "black" => commands::blacklist(&ctx, &msg).await,

        "join" => commands::join(&ctx, &msg).await,
        "leave" => commands::leave(&ctx, &msg).await,

        "snipe" => commands::snipe(&ctx, &msg).await,
        "editsnipe" => commands::edit_snipe(&ctx, &msg).await,

        // voice commands
        "play" => commands::voice::play(&ctx, &msg).await,
        "pause" => commands::voice::pause(&ctx, &msg).await,
        "resume" | "unpause" => commands::voice::resume(&ctx, &msg).await,
        "status" => commands::voice::status(&ctx, &msg).await,
        "loop" => commands::voice::set_loop(&ctx, &msg).await,
        "stop" => commands::voice::stop(&ctx, &msg).await,
        "seek" => commands::voice::seek(&ctx, &msg).await,

        // do nothing if not matched
        &_ => Ok(()),
    }
}

#[async_trait]
impl EventHandler for CommandHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author == **ctx.cache.current_user() {
            return;
        }

        if msg.guild_id.is_none() && msg.author.id != OWNER {
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

        let cmd = &msg.content.split(' ').collect::<Vec<&str>>()[0][1..];
        
        // test is a special command since it can run other commands,
        // and needs to be handled separate from `handle_cmd` since 
        // `async fn`s cannot be recursive.
        let result: Result<(), &str> = if cmd == "test" {
            commands::test(&ctx, &msg).await
        }
        else {
            handle_cmd(cmd, &ctx, &msg).await
        };

        // if error == "", no response
        if let Err(error) = result {
            ctx.reply(error, &msg).await;
        }

        typing.stop();
    }
}
