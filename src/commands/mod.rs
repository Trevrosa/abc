use std::future::Future;

use serenity::all::{Context, CreateMessage, EditMessage, Message};

mod join;
pub use join::join;

mod test;
pub use test::test;

mod leave;
pub use leave::leave;

mod play;
pub use play::play;

mod resume;
pub use resume::resume;

mod pause;
pub use pause::pause;

mod set_loop;
pub use set_loop::set_loop;

mod status;
pub use status::status;

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

fn edit_message(content: &str) -> EditMessage {
    EditMessage::new().content(content)
}
