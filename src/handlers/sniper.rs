use serenity::{
    all::{ChannelId, Context, EventHandler, GuildId, Message, MessageId, MessageUpdateEvent},
    async_trait,
};
use tracing::{error, info};

use crate::utils::sniping::{EditedMessage, MostRecentDeletedMessage, MostRecentEditedMessage};

pub struct Sniper;

#[async_trait]
impl EventHandler for Sniper {
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
        // ignore self
        if new.author.id == ctx.cache.current_user().id {
            return;
        }
        // only log if content changes
        if new.content == old.content {
            return;
        }
        // ignore non-guilds
        let Some(guild) = new.guild_id else {
            return;
        };

        ctx.data
            .write()
            .await
            .get_mut::<MostRecentEditedMessage>()
            .unwrap()
            .insert(guild, EditedMessage::new(old, new));

        info!("new edited message stored");
    }

    async fn message_delete(
        &self,
        ctx: Context,
        channel: ChannelId,
        msg: MessageId,
        guild: Option<GuildId>,
    ) {
        let msg = match ctx.cache.message(channel, msg).map(|x| x.clone()) {
            msg @ Some(_) => msg,
            None => ctx.http.get_message(channel, msg).await.ok(),
        };

        let Some(msg) = msg else {
            error!("message didnt exist in cache or at all.");
            return;
        };

        // ignore bots
        if msg.author.bot {
            return;
        }
        // ignore self
        if msg.author.id == ctx.cache.current_user().id {
            return;
        }
        // ignore non-guilds
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
