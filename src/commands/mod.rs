use std::future::Future;

use serenity::all::{Context, CreateMessage, EditMessage, Message};

mod join;
pub use join::join;

mod test;
pub use test::test;

mod leave;
pub use leave::leave;

pub mod voice;

/// # Panics
/// will panic if message not sent
async fn reply(ctx: &Context, content: &str, msg: &Message) -> Message {
    let new_msg = CreateMessage::new().content(content).reference_message(msg);
    msg.channel_id.send_message(&ctx, new_msg).await.unwrap()
}

pub trait Utils {
    fn reply(&self, content: &str, message: &Message) -> impl Future<Output = Message>;
    fn edit_msg(&self, content: &str, msg: &mut Message) -> impl Future<Output = ()>;
}

impl Utils for Context {
    fn reply(&self, content: &str, msg: &Message) -> impl Future<Output = Message> {
        reply(self, content, msg)
    }

    fn edit_msg(&self, content: &str, msg: &mut Message) -> impl Future<Output = ()> {
        edit(self, content, msg)
    }
}

/// will do nothing if errored
async fn edit(ctx: &Context, content: &str, msg: &mut Message) {
    let _ = msg.edit(&ctx.http, edit_message(content)).await;
}

fn edit_message(content: &str) -> EditMessage {
    EditMessage::new().content(content)
}
