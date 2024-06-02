use serenity::all::{ChannelId, GuildId, Message, MessageId, Timestamp};
use songbird::typemap::TypeMapKey;
use std::collections::HashMap;

pub struct MostRecentDeletedMessage;

impl TypeMapKey for MostRecentDeletedMessage {
    type Value = HashMap<GuildId, DeletedMessage>;
}

pub struct MostRecentEditedMessage;

impl TypeMapKey for MostRecentEditedMessage {
    type Value = HashMap<GuildId, EditedMessage>;
}

pub struct DeletedMessage {
    pub id: MessageId,
    pub channel: ChannelId,
}

impl DeletedMessage {
    #[must_use]
    pub fn new(msg: MessageId, channel: ChannelId) -> Self {
        Self { id: msg, channel }
    }
}

pub struct EditedMessage {
    pub id: MessageId,
    pub timestamp: Timestamp,
    pub author: String,
    pub old_message: String,
    pub new_message: String,
}

impl EditedMessage {
    #[must_use]
    /// # Panics
    /// `new` must be an edited Message to not panic
    pub fn new(old: Message, new: Message) -> Self {
        Self {
            id: old.id,
            timestamp: new.edited_timestamp.unwrap(),
            author: old.author.name,
            old_message: old.content,
            new_message: new.content,
        }
    }
}
