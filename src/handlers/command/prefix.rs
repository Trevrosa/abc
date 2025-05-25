use std::time::Instant;

use serenity::{
    all::{Context, EventHandler, Message},
    async_trait,
};
use tracing::info;

use crate::{
    handlers::command::handle_cmd,
    utils::{reply::Replyer, Arg, ArgValue, Args},
    OWNER,
};
use crate::{utils::context::CtxExt, Blacklisted, SEVEN};

pub struct PrefixCommands;

#[async_trait]
impl EventHandler for PrefixCommands {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id == ctx.cache.current_user().id {
            return;
        }

        let replyer = Replyer::Prefix(&msg);

        if msg.guild_id.is_none() && msg.author.id != OWNER {
            if msg.author.id == SEVEN {
                ctx.reply("wasup boss", &replyer).await;
            } else {
                ctx.reply("im busy", &replyer).await;
            }
            return;
        }

        // TODO: make dm cmd work
        // let msg = if msg.guild_id.is_none() {
        //     let mut msg = msg;
        //     msg.guild_id = Some(DEFAULT_GUILD.into());

        //     msg
        // } else {
        //     msg
        // };

        // info!("c {} g {:?}", msg.channel_id, msg.guild_id);

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

        let mut words = msg.content.split(' ');

        // ignore the first char "`", the cmd prefix
        let cmd = &words.next().unwrap()[1..];

        let mut args: Vec<Arg> = Vec::new();

        let parse_start = Instant::now();
        // put attachments at the front of args
        if !msg.attachments.is_empty() {
            for attachment in &msg.attachments {
                args.push(Arg::unnamed(ArgValue::Attachment(attachment.clone())));
            }
        }

        // parse the arguments and add them
        args.extend(words.map(|w| Arg::unnamed(ArgValue::from_str(&ctx.cache, msg.guild_id, w))));

        info!(
            "took {:?} to parse {} args from string",
            parse_start.elapsed(),
            args.len()
        );

        let result: Result<(), &str> = handle_cmd(cmd, &ctx, &replyer, Args::new(&args)).await;

        if let Err(error) = result {
            // if error == "", no response
            if !error.is_empty() {
                ctx.reply(error, &replyer).await;
            }
        }

        typing.stop();
    }
}
