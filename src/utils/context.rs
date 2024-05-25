use std::{collections::hash_map::Iter, future::Future};

use serenity::all::{ChannelId, ChannelType, Context, GuildChannel, Message, User};

pub struct CreateMessage(pub serenity::all::CreateMessage);

impl From<&str> for CreateMessage {
    fn from(value: &str) -> Self {
        CreateMessage(serenity::all::CreateMessage::new().content(value))
    }
}

impl From<String> for CreateMessage {
    fn from(value: String) -> Self {
        CreateMessage(serenity::all::CreateMessage::new().content(value))
    }
}

impl From<serenity::all::CreateMessage> for CreateMessage {
    fn from(value: serenity::all::CreateMessage) -> Self {
        CreateMessage(value)
    }
}

pub trait Ext {
    fn reply<T: Into<CreateMessage>>(
        &self,
        content: T,
        message: &Message,
    ) -> impl Future<Output = Message>;
    fn edit_msg<T: Into<String>>(&self, content: T, msg: &mut Message) -> impl Future<Output = ()>;
    fn find_user_channel<'a>(
        &self,
        user: &User,
        kind: ChannelType,
        channels: &'a mut Iter<ChannelId, GuildChannel>,
    ) -> Option<&'a GuildChannel>;
}

impl Ext for Context {
    fn reply<T: Into<CreateMessage>>(
        &self,
        content: T,
        msg: &Message,
    ) -> impl Future<Output = Message> {
        super::internal::reply(self, content, msg)
    }

    fn edit_msg<T: Into<String>>(&self, content: T, msg: &mut Message) -> impl Future<Output = ()> {
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
