use serenity::all::{CreateEmbed, CreateMessage};

pub mod context;
mod internal;

pub fn embed_message(title: impl Into<String>, url: impl Into<String>) -> CreateMessage {
    CreateMessage::new().embed(CreateEmbed::new().title(title).image(url))
}
