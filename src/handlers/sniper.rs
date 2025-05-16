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
        // ignore self
        if msg.author.id == ctx.cache.current_user().id {
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
