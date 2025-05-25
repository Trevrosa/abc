use reply::CreateReply;
use serenity::all::CreateEmbed;

mod internal;

pub mod reply;

mod arg;
pub use arg::Arg;
pub use arg::ArgValue;
pub use arg::Args;
pub use arg::Get;
pub use arg::Is;

pub mod context;
pub mod sniping;
pub mod status;

pub mod spotify;
pub mod ytmusic;

mod yt_dlp;

pub fn embed_message(title: impl Into<String>, url: impl Into<String>) -> CreateReply {
    CreateReply::new().embed(CreateEmbed::new().title(title).image(url))
}
