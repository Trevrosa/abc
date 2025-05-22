use serenity::all::{CreateEmbed, CreateMessage};

mod internal;

pub mod status;
pub mod context;
pub mod sniping;

pub mod spotify;
pub mod ytmusic;

mod yt_dlp;
pub fn embed_message(title: impl Into<String>, url: impl Into<String>) -> CreateMessage {
    CreateMessage::new().embed(CreateEmbed::new().title(title).image(url))
}
