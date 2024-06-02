use serenity::{
    all::{
        ActivityData, ChannelId, Context, EventHandler, GuildId, Message, MessageId,
        MessageUpdateEvent, OnlineStatus, Ready,
    },
    async_trait,
};
use tracing::{error, info};

use crate::{
    commands,
    utils::sniping::{EditedMessage, MostRecentDeletedMessage, MostRecentEditedMessage},
};
use crate::{utils::context::Ext, Blacklisted, SEVEN};

#[derive(Debug)]
pub struct CommandHandler;

#[async_trait]
impl EventHandler for CommandHandler {
    async fn ready(&self, ctx: Context, _: Ready) {
        ctx.set_presence(
            Some(ActivityData::custom("Disrupting the Social Democrats")),
            OnlineStatus::DoNotDisturb,
        );

        info!("successfully set status");
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.is_private() && !msg.is_own(&ctx.cache) {
            if msg.author.id.get() == SEVEN {
                ctx.reply("wasup boss", &msg).await;
            } else {
                ctx.reply("im busy", &msg).await;
            }
            return;
        }

        let global = ctx.data.try_read().unwrap();
        let blacklisted = global.get::<Blacklisted>().unwrap();

        if !msg.content.starts_with('`') || blacklisted.contains(&msg.author.id.get()) {
            drop(global);
            return;
        }

        drop(global);

        info!("received cmd in guild {}", msg.guild_id.unwrap());

        let typing = msg.channel_id.start_typing(&ctx.http);

        match &msg.content.split(' ').collect::<Vec<&str>>()[0][1..] {
            // misc commands
            "test" => commands::test(ctx, msg).await,
            "join" => commands::join(ctx, msg).await,
            "leave" => commands::leave(ctx, msg).await,
            "cat" => commands::cat(ctx, msg).await,

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

pub struct MessageSniper;

#[async_trait]
impl EventHandler for MessageSniper {
    async fn message_update(
        &self,
        ctx: Context,
        old: Option<Message>,
        new: Option<Message>,
        _: MessageUpdateEvent,
    ) {
        let Some(old) = old else {
            return;
        };

        let Some(new) = new else {
            return;
        };

        // ignore bots
        if new.author.bot {
            return;
        }
        // ignore commands
        if new.content.starts_with('`') {
            return;
        }

        ctx.data
            .write()
            .await
            .get_mut::<MostRecentEditedMessage>()
            .unwrap()
            .insert(new.guild_id.unwrap(), EditedMessage::new(old, new));

        info!("new edited message stored");
    }

    async fn message_delete(
        &self,
        ctx: Context,
        channel: ChannelId,
        msg: MessageId,
        guild: Option<GuildId>,
    ) {
        let msg = ctx.cache.message(channel, msg).map(|x| x.clone());

        let Some(msg) = msg else {
            error!("tried to get message that didnt exist in cache");
            return;
        };

        // ignore bots
        if msg.author.bot {
            return;
        }
        // ignore commands
        if msg.content.starts_with('`') {
            return;
        }

        let Some(guild) = guild else {
            return;
        };

        ctx.data
            .write()
            .await
            .get_mut::<MostRecentDeletedMessage>()
            .unwrap() // safe to unwrap since hashmap initialized in main
            .insert(guild, msg.into());

        info!("new deleted msg stored");
    }
}
