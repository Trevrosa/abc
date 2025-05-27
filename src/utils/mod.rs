use reply::CreateReply;
use serenity::all::CreateEmbed;

mod internal;

pub mod reply;
pub use reply::Replyer;

mod arg;
pub use arg::{Arg, ArgValue, Args, Get, Is};

pub mod context;
pub mod sniping;
pub mod status;

pub mod spotify;
pub mod ytmusic;

mod yt_dlp;

pub fn embed_message<S: Into<String>>(title: S, image: S, desc: Option<S>) -> CreateReply {
    let mut embed = CreateEmbed::new().title(title).image(image);
    if let Some(desc) = desc {
        embed = embed.description(desc);
    }
    CreateReply::new().embed(embed)
}
