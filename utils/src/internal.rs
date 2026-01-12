use std::collections::hash_map::Iter;

use serenity::all::{
    Cache, ChannelId, ChannelType, Context, EditMessage, GuildChannel, Message, User,
};
use tracing::error;

use super::reply::{CreateReply, IntExt, Replyer};

/// # Panics
///
/// Will panic if message failed to send
pub(super) async fn reply(
    ctx: &Context,
    reply: impl Into<CreateReply>,
    replyer: &Replyer<'_>,
) -> Message {
    match replyer {
        Replyer::Prefix(reference) => {
            let new_msg = reply.into().into_msg(reference);
            reference
                .channel_id
                .send_message(&ctx, new_msg)
                .await
                .unwrap()
        }
        Replyer::Slash(int) => int.reply(ctx, reply).await.unwrap(),
    }
}

/// # Panics
/// will panic if message not sent
pub(super) async fn error_reply(
    ctx: &Context,
    content: impl Into<String>,
    replyer: &Replyer<'_>,
) -> Message {
    let content = content.into();
    error!("{content}");

    reply(ctx, content, replyer).await
}

/// Will do nothing on error.
pub(super) async fn edit(ctx: &Context, content: String, msg: &mut Message) {
    let _ = msg.edit(&ctx.http, edit_message(content)).await;
}

pub(super) fn edit_message(content: String) -> EditMessage {
    EditMessage::new().content(content)
}

pub(super) fn find_user_channel<'a>(
    cache: &Cache,
    user: &User,
    kind: ChannelType,
    channels: &'a mut Iter<ChannelId, GuildChannel>,
) -> Option<&'a GuildChannel> {
    channels.find_map(|c| {
        let c = c.1;

        if c.kind != kind {
            return None;
        }

        let Ok(members) = c.members(cache) else {
            return None;
        };

        if members.iter().any(|m| &m.user == user) {
            Some(c)
        } else {
            None
        }
    })
}
