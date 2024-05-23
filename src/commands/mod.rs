use std::{collections::hash_map::Iter, future::Future};

use serenity::all::{
    ChannelId, ChannelType, Context, CreateMessage, EditMessage, GuildChannel, Message, User,
};

mod join;
pub use join::join;

mod test;
pub use test::test;

mod leave;
pub use leave::leave;

pub mod voice;

pub trait Utils {
    fn reply(&self, content: &str, message: &Message) -> impl Future<Output = Message>;
    fn edit_msg(&self, content: &str, msg: &mut Message) -> impl Future<Output = ()>;
    fn find_user_channel<'a>(
        &self,
        user: &User,
        kind: ChannelType,
        channels: &'a mut Iter<ChannelId, GuildChannel>,
    ) -> Option<&'a GuildChannel>;
}

impl Utils for Context {
    fn reply(&self, content: &str, msg: &Message) -> impl Future<Output = Message> {
        reply(self, content, msg)
    }

    fn edit_msg(&self, content: &str, msg: &mut Message) -> impl Future<Output = ()> {
        edit(self, content, msg)
    }

    fn find_user_channel<'a>(
        &self,
        user: &User,
        kind: ChannelType,
        channels: &'a mut Iter<ChannelId, GuildChannel>,
    ) -> Option<&'a GuildChannel> {
        find_user_channel(self, user, kind, channels)
    }
}

/// # Panics
/// will panic if message not sent
async fn reply(ctx: &Context, content: &str, msg: &Message) -> Message {
    let new_msg = CreateMessage::new().content(content).reference_message(msg);
    msg.channel_id.send_message(&ctx, new_msg).await.unwrap()
}

/// will do nothing if errored
async fn edit(ctx: &Context, content: &str, msg: &mut Message) {
    let _ = msg.edit(&ctx.http, edit_message(content)).await;
}

fn edit_message(content: &str) -> EditMessage {
    EditMessage::new().content(content)
}

fn find_user_channel<'a>(
    ctx: &Context,
    user: &User,
    kind: ChannelType,
    channels: &'a mut Iter<ChannelId, GuildChannel>,
) -> Option<&'a GuildChannel> {
    channels.find_map(|c| {
        let c = c.1;

        if c.kind != kind {
            return None;
        }

        let Ok(members) = c.members(&ctx.cache) else {
            return None;
        };

        if members.iter().any(|m| &m.user == user) {
            Some(c)
        } else {
            None
        }
    })
}
