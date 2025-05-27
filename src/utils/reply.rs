//! Stolen mostly from [poise::reply](https://github.com/serenity-rs/poise/blob/518ff0564865bca2abf01ae8995b77340f439ef9/src/reply/mod.rs)

use serenity::all::{
    create_poll, CommandInteraction, Context, CreateActionRow, CreateAllowedMentions,
    CreateAttachment, CreateButton, CreateEmbed, CreateInteractionResponseFollowup, CreateMessage,
    CreatePoll, Message,
};

/// This enum tells us *how* we are going to be replying.
///
/// To be used in a command context.
///
/// If the command was run by prefix (".cmd"):
/// - We are replying to a [`Message`], contained in the [`Replyer::Prefix`] variant.
///
/// If the command was run by slash command ("/cmd"):
/// - We need to reply through a [`CommandInteraction`], contained in the [`Replyer::Slash`] variant.
#[derive(Debug)]
pub enum Replyer<'a> {
    /// We are replying to a normal [`Message`].
    Prefix(&'a Message),
    /// We are going to be creating a followup response to a [`CommandInteraction`].
    Slash(&'a CommandInteraction),
}

#[allow(unused)]
impl Replyer<'_> {
    pub fn as_prefix(&self) -> &Message {
        match self {
            Self::Prefix(msg) => msg,
            Self::Slash(_) => panic!("Expected prefix variant, but got slash."),
        }
    }

    pub fn as_slash(&self) -> &CommandInteraction {
        match self {
            Self::Slash(int) => int,
            Self::Prefix(_) => panic!("Expected slash variant, but got prefix."),
        }
    }
}

#[derive(Default, Clone)]
pub struct CreateReply {
    /// Message content.
    pub(super) content: Option<String>,
    /// Embeds, if present.
    pub(super) embeds: Vec<CreateEmbed>,
    /// Message attachments.
    pub(super) attachments: Vec<CreateAttachment>,
    /// Whether the message is ephemeral (only has an effect in application commands)
    pub(super) ephemeral: Option<bool>,
    /// Message components, that is, buttons and select menus.
    pub(super) components: Option<Vec<CreateActionRow>>,
    /// The allowed mentions for the message.
    pub(super) allowed_mentions: Option<CreateAllowedMentions>,
    /// Message poll, if present.
    pub(super) poll: Option<CreatePoll<create_poll::Ready>>,
}

impl From<String> for CreateReply {
    fn from(value: String) -> Self {
        Self {
            content: Some(value),
            ..Default::default()
        }
    }
}

impl From<&str> for CreateReply {
    fn from(value: &str) -> Self {
        Self {
            content: Some(value.to_string()),
            ..Default::default()
        }
    }
}

#[allow(unused)]
impl CreateReply {
    /// [`Self::default`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the content of the message.
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Adds an embed to the message.
    ///
    /// Existing embeds are kept.
    pub fn embed(mut self, embed: CreateEmbed) -> Self {
        self.embeds.push(embed);
        self
    }

    /// Set components (buttons and select menus) for this message.
    ///
    /// Any previously set components will be overwritten.
    pub fn components(mut self, components: Vec<CreateActionRow>) -> Self {
        self.components = Some(components);
        self
    }

    /// Adds a clickable button to this message.
    ///
    /// Convenience method that wraps [`Self::components`]. Arranges buttons in action rows automatically.
    ///
    /// (Taken from [`serenity::all::CreateMessage::button`].)
    pub fn button(mut self, button: CreateButton) -> Self {
        let rows = self.components.get_or_insert_with(Vec::new);
        let row_with_space_left = rows.last_mut().and_then(|row| match row {
            CreateActionRow::Buttons(buttons) if buttons.len() < 5 => Some(buttons),
            _ => None,
        });

        match row_with_space_left {
            Some(row) => row.push(button),
            None => rows.push(CreateActionRow::Buttons(<[_]>::into_vec(Box::new([
                button,
            ])))),
        }
        self
    }

    /// Add an attachment.
    pub fn add_file(mut self, attachment: CreateAttachment) -> Self {
        self.attachments.push(attachment);
        self
    }

    /// Toggles whether the message is an ephemeral response (only invoking user can see it).
    ///
    /// This only has an effect in slash commands!
    pub fn ephemeral(mut self, ephemeral: bool) -> Self {
        self.ephemeral = Some(ephemeral);
        self
    }

    /// Set the allowed mentions for the message.
    ///
    /// See [`serenity::CreateAllowedMentions`] for more information.
    pub fn allowed_mentions(mut self, allowed_mentions: CreateAllowedMentions) -> Self {
        self.allowed_mentions = Some(allowed_mentions);
        self
    }

    /// Adds a poll to the message. Only one poll can be added per message.
    ///
    /// See [`serenity::CreatePoll`] for more information on creating and configuring a poll.
    pub fn poll(mut self, poll: CreatePoll<create_poll::Ready>) -> Self {
        self.poll = Some(poll);
        self
    }
}

impl CreateReply {
    pub fn into_followup(
        self,
        mut builder: CreateInteractionResponseFollowup,
    ) -> CreateInteractionResponseFollowup {
        let CreateReply {
            allowed_mentions,
            attachments,
            components,
            content,
            embeds,
            ephemeral,
            poll,
        } = self;

        if let Some(content) = content {
            builder = builder.content(content);
        }
        builder = builder.embeds(embeds);
        if let Some(components) = components {
            builder = builder.components(components);
        }
        if let Some(allowed_mentions) = allowed_mentions {
            builder = builder.allowed_mentions(allowed_mentions);
        }
        if let Some(ephemeral) = ephemeral {
            builder = builder.ephemeral(ephemeral);
        }
        if let Some(poll) = poll {
            builder = builder.poll(poll);
        }

        builder.add_files(attachments)
    }

    pub fn into_msg(self, reference: &Message) -> CreateMessage {
        let CreateReply {
            allowed_mentions,
            attachments,
            components,
            content,
            embeds,
            ephemeral: _, // normal msg cant be ephemeral.
            poll,
        } = self;

        let mut builder = CreateMessage::new();

        if let Some(content) = content {
            builder = builder.content(content);
        }
        builder = builder.embeds(embeds);
        if let Some(components) = components {
            builder = builder.components(components);
        }
        if let Some(allowed_mentions) = allowed_mentions {
            builder = builder.allowed_mentions(allowed_mentions);
        }
        if let Some(poll) = poll {
            builder = builder.poll(poll);
        }

        builder.add_files(attachments).reference_message(reference)
    }
}

pub trait IntExt {
    async fn reply(
        &self,
        ctx: &Context,
        reply: impl Into<CreateReply>,
    ) -> Result<Message, serenity::Error>;
}

impl IntExt for CommandInteraction {
    async fn reply(
        &self,
        ctx: &Context,
        reply: impl Into<CreateReply>,
    ) -> Result<Message, serenity::Error> {
        let followup = reply
            .into()
            .into_followup(CreateInteractionResponseFollowup::new());
        self.create_followup(&ctx.http, followup).await
    }
}
