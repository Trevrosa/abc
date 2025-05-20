use std::collections::hash_map::Iter;

use serenity::all::{ChannelId, ChannelType, Context, EditMessage, GuildChannel, Message, User};
use tracing::error;

use super::context::CreateMessage;

/// # Panics
/// will panic if message not sent
pub(super) async fn reply(
    ctx: &Context,
    content: impl Into<CreateMessage> + Send,
    msg: &Message,
) -> Message {
    let new_msg = content.into().0.reference_message(msg);
    msg.channel_id.send_message(&ctx, new_msg).await.unwrap()
}

/// # Panics
/// will panic if message not sent
pub(super) async fn error_reply(
    ctx: &Context,
    content: impl Into<String>,
    msg: &Message,
) -> Message {
    let content = content.into();
    error!("{content}");

    let new_msg = serenity::all::CreateMessage::new().content(content);
    reply(ctx, new_msg, msg).await
}

/// Will do nothing on error.
pub(super) async fn edit(ctx: &Context, content: String, msg: &mut Message) {
    let _ = msg.edit(&ctx.http, edit_message(content)).await;
}

pub(super) fn edit_message(content: String) -> EditMessage {
    EditMessage::new().content(content)
}

pub(super) fn find_user_channel<'a>(
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
