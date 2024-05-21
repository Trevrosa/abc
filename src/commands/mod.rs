use std::future::Future;

use bytes::Bytes;
use serenity::all::{Context, CreateMessage, EditMessage, Message};

mod join;
pub use join::join;

mod test;
pub use test::test;

mod leave;
pub use leave::leave;

use crate::HttpClientKey;

pub mod voice;

/// # Panics
/// will panic if message not sent
async fn reply(ctx: &Context, content: &str, msg: &Message) -> Message {
    let new_msg = CreateMessage::new().content(content).reference_message(msg);
    msg.channel_id.send_message(&ctx, new_msg).await.unwrap()
}

pub trait Reply {
    fn reply(&self, content: &str, message: &Message) -> impl Future<Output = Message>;
}

impl Reply for Context {
    fn reply(&self, content: &str, msg: &Message) -> impl Future<Output = Message> {
        reply(self, content, msg)
    }
}

pub fn edit_message(content: &str) -> EditMessage {
    EditMessage::new().content(content)
}

/// `.unwrap()` here should never panic, since the `Client` should have been initialized in `main`
async fn get(ctx: &Context, url: &str) -> Result<Bytes, reqwest::Error> {
    let global = ctx.data.read().await;
    let client = global.get::<HttpClientKey>().unwrap();

    let request = client.get(url).build()?;
    client.execute(request).await?.bytes().await
}

pub trait Get {
    fn get(&self, url: &str) -> impl Future<Output = Result<Bytes, reqwest::Error>>;
}

impl Get for Context {
    fn get(&self, url: &str) -> impl Future<Output = Result<Bytes, reqwest::Error>> {
        get(self, url)
    }
}
