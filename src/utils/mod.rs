use serenity::all::{CreateEmbed, CreateMessage};

pub mod context;
mod internal;

pub fn message_with_image(url: impl Into<String>) -> CreateMessage {
    CreateMessage::new().embed(CreateEmbed::new().image(url))
}
