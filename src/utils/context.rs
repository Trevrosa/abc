use std::{collections::hash_map::Iter, path::Path};

use serenity::all::{ChannelId, ChannelType, Context, GuildChannel, Message, User};

use super::{
    reply::{CreateReply, Replyer},
    yt_dlp,
};

/// Only impl for Context
pub trait CtxExt {
    /// Reply to the `replyer` with message `reply`.
    async fn reply(&self, reply: impl Into<CreateReply>, replyer: &Replyer) -> Message;
    /// Reply t
    async fn error_reply(&self, content: impl Into<String>, message: &Replyer) -> Message;
    /// Edit message `msg` to new content `content`.
    async fn edit_msg(&self, content: impl Into<String>, msg: &mut Message);
    /// Add a new line `line` to `msg`.
    async fn msg_new_line(&self, line: impl Into<String>, msg: &mut Message);
    /// Find what channel user `user` is in, filtering by `kind`.
    fn find_user_channel<'a>(
        &self,
        user: &User,
        kind: ChannelType,
        channels: &'a mut Iter<ChannelId, GuildChannel>,
    ) -> Option<&'a GuildChannel>;
    /// Download url `url` with `yt-dlp`, allowing output and download format selection, editing message `status_msg` with its status.
    async fn yt_dlp<P: AsRef<Path>, S: AsRef<str>>(
        &self,
        url: S,
        output: Option<P>,
        download_format: S,
        extra_args: Option<&[&str]>,
        status_msg: &mut Message,
    ) -> Result<(), &'static str>;
}

impl CtxExt for Context {
    async fn reply(&self, reply: impl Into<CreateReply>, replyer: &Replyer<'_>) -> Message {
        super::internal::reply(self, reply, replyer).await
    }

    async fn error_reply(&self, content: impl Into<String>, msg: &Replyer<'_>) -> Message {
        super::internal::error_reply(self, content, msg).await
    }

    async fn edit_msg(&self, content: impl Into<String>, msg: &mut Message) {
        super::internal::edit(self, content.into(), msg).await;
    }

    async fn msg_new_line(&self, line: impl Into<String>, msg: &mut Message) {
        super::internal::edit(self, format!("{}\n{}", msg.content, line.into()), msg).await;
    }

    fn find_user_channel<'a>(
        &self,
        user: &User,
        kind: ChannelType,
        channels: &'a mut Iter<ChannelId, GuildChannel>,
    ) -> Option<&'a GuildChannel> {
        super::internal::find_user_channel(&self.cache, user, kind, channels)
    }

    async fn yt_dlp<P: AsRef<Path>, S: AsRef<str>>(
        &self,
        url: S,
        output: Option<P>,
        download_format: S,
        extra_args: Option<&[&str]>,
        status_msg: &mut Message,
    ) -> Result<(), &'static str> {
        yt_dlp::download(self, url, output, download_format, extra_args, status_msg).await
    }
}
