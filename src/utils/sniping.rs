use serenity::all::{GuildId, Message, Timestamp};
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
    pub author: String,
    pub content: String,
    pub timestamp: Timestamp,
}

impl From<Message> for DeletedMessage {
    fn from(value: Message) -> Self {
        Self {
            timestamp: value.timestamp,
            author: value.author.name,
            content: value.content,
        }
    }
}

pub struct EditedMessage {
    pub timestamp: Option<Timestamp>,
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
            timestamp: new.edited_timestamp,
            author: old.author.name,
            old_message: old.content,
            new_message: new.content,
        }
    }
}
