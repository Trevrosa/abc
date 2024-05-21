use std::{future::Future, sync::RwLockReadGuard};

use serenity::all::{Context, CreateMessage, EditMessage, Message};

mod join;
pub use join::join;

mod test;
use songbird::{tracks::TrackHandle, typemap::TypeMap};
pub use test::test;

mod leave;
pub use leave::leave;

use crate::TrackHandleKey;

pub mod voice;

/// # Panics
/// will panic if message not sent
async fn reply(content: &str, ctx: &Context, msg: &Message) -> Message {
    let new_msg = CreateMessage::new().content(content).reference_message(msg);
    msg.channel_id.send_message(&ctx, new_msg).await.unwrap()
}

pub trait Reply {
    fn reply(&self, content: &str, message: &Message) -> impl Future<Output = Message>;
}

impl Reply for Context {
    fn reply(&self, content: &str, msg: &Message) -> impl Future<Output = Message> {
        reply(content, self, msg)
    }
}

pub fn edit_message(content: &str) -> EditMessage {
    EditMessage::new().content(content)
}
