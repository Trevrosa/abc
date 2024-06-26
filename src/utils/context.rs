use std::{collections::hash_map::Iter, future::Future};

use serenity::all::{ChannelId, ChannelType, Context, GuildChannel, Message, User};

pub struct CreateMessage(pub serenity::all::CreateMessage);

impl From<&str> for CreateMessage {
    fn from(value: &str) -> Self {
        Self(serenity::all::CreateMessage::new().content(value))
    }
}

impl From<String> for CreateMessage {
    fn from(value: String) -> Self {
        Self(serenity::all::CreateMessage::new().content(value))
    }
}

impl From<serenity::all::CreateMessage> for CreateMessage {
    fn from(value: serenity::all::CreateMessage) -> Self {
        Self(value)
    }
}

/// Only impl for Context
pub trait Ext {
    fn reply(
        &self,
        content: impl Into<CreateMessage> + Send,
        message: &Message,
    ) -> impl Future<Output = Message> + Send;
    fn edit_msg(&self, content: impl Into<String>, msg: &mut Message) -> impl Future<Output = ()>;
    fn find_user_channel<'a>(
        &self,
        user: &User,
        kind: ChannelType,
        channels: &'a mut Iter<ChannelId, GuildChannel>,
    ) -> Option<&'a GuildChannel>;
}

impl Ext for Context {
    fn reply(
        &self,
        content: impl Into<CreateMessage> + Send,
        msg: &Message,
    ) -> impl Future<Output = Message> + Send {
        super::internal::reply(self, content, msg)
    }

    fn edit_msg(&self, content: impl Into<String>, msg: &mut Message) -> impl Future<Output = ()> {
        super::internal::edit(self, content.into(), msg)
    }

    fn find_user_channel<'a>(
        &self,
        user: &User,
        kind: ChannelType,
        channels: &'a mut Iter<ChannelId, GuildChannel>,
    ) -> Option<&'a GuildChannel> {
        super::internal::find_user_channel(self, user, kind, channels)
    }
}
