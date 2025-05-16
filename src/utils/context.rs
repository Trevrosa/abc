use std::{collections::hash_map::Iter, future::Future, path::Path};

use serenity::all::{ChannelId, ChannelType, Context, GuildChannel, Message, User};

use super::yt_dlp;

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
    /// Reply to message `message` with message `content`.
    fn reply(
        &self,
        content: impl Into<CreateMessage> + Send,
        message: &Message,
    ) -> impl Future<Output = Message> + Send;
    /// Edit message `msg` to new content `content`.
    fn edit_msg(&self, content: impl Into<String>, msg: &mut Message) -> impl Future<Output = ()>;
    /// Find what channel user `user` is in, filtering by `kind`.
    fn find_user_channel<'a>(
        &self,
        user: &User,
        kind: ChannelType,
        channels: &'a mut Iter<ChannelId, GuildChannel>,
    ) -> Option<&'a GuildChannel>;
    /// Download url `url` with `yt-dlp`, allowing output and download format selection, editing message `status_msg` with its status.
    fn yt_dlp<P: AsRef<Path>, S: AsRef<str>>(
        &self,
        url: S,
        output: Option<P>,
        download_format: S,
        extra_args: Option<&[&str]>,
        status_msg: &mut Message,
    ) -> impl Future<Output = Result<(), &'static str>>;
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

    fn yt_dlp<P: AsRef<Path>, S: AsRef<str>>(
        &self,
        url: S,
        output: Option<P>,
        download_format: S,
        extra_args: Option<&[&str]>,
        status_msg: &mut Message,
    ) -> impl Future<Output = Result<(), &'static str>> {
        yt_dlp::download(self, url, output, download_format, extra_args, status_msg)
    }
}
