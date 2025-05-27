use std::fmt::Debug;

use serenity::all::{
    Attachment, Cache, ChannelId, ChannelType, Guild, GuildId, PartialMember, Permissions,
    ResolvedOption, Role, ThreadMetadata, User,
};
use tracing::warn;

pub struct Args<'a>(&'a [Arg<'a>]);

impl Debug for Args<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a> Args<'a> {
    pub fn new(args: &'a [Arg]) -> Self {
        Self(args)
    }

    /// Returns true if the number of arguments is 0.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the number of arguments.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns the first argument, or `None` if it is empty.
    pub fn first(&self) -> Option<&Arg> {
        self.0.first()
    }

    /// Returns the first argument's value, or `None` if it is empty.
    pub fn first_value(&self) -> Option<&ArgValue> {
        self.0.first().map(|a| &a.value)
    }
}

/// Like [`Index`], but we don't have to return a `ref` of [`Get::Output`].
pub trait Get<Idx> {
    type Output;
    fn get(&self, index: Idx) -> Option<Self::Output>;
}

impl<'a> Get<usize> for Args<'a> {
    type Output = &'a Arg<'a>;
    fn get(&self, index: usize) -> Option<Self::Output> {
        self.0.get(index)
    }
}

impl<'a> Get<&'a str> for Args<'a> {
    type Output = &'a Arg<'a>;
    fn get(&self, index: &'a str) -> Option<Self::Output> {
        self.0
            .iter()
            .find(|a| a.name.as_ref().is_some_and(|n| n == &index))
    }
}

/// Taken mostly from [`serenity::all::ResolvedOption`]
#[derive(Debug)]
pub struct Arg<'a> {
    pub name: Option<&'a str>,
    pub value: ArgValue<'a>,
}

/// See if an [`Arg`]'s value can be coerced to `T`, and is equal to `cmp`.
pub trait Is<T> {
    fn is(&self, cmp: T) -> bool;
}

impl Is<bool> for Arg<'_> {
    fn is(&self, cmp: bool) -> bool {
        let ArgValue::Boolean(val) = self.value else {
            return false;
        };
        val == cmp
    }
}

impl Is<&str> for Arg<'_> {
    fn is(&self, cmp: &str) -> bool {
        let ArgValue::String(val) = self.value else {
            return false;
        };
        val == cmp
    }
}

impl<'a> Arg<'a> {
    /// Create an unnamed argument.
    pub fn unnamed(value: ArgValue<'a>) -> Self {
        Self { name: None, value }
    }

    /// Create the argument from a [`ResolvedOption`], usually from a [`serenity::all::CommandInteraction`]
    pub fn from_resolved(option: ResolvedOption<'a>) -> Option<Self> {
        Some(Self {
            name: Some(option.name),
            value: ArgValue::from_resolved(option)?,
        })
    }
}

/// The possible arguments that a command can receive.
///
/// Taken mostly from [`serenity::all::ResolvedValue`]
#[derive(Debug)]
#[allow(unused)]
pub enum ArgValue<'a> {
    Boolean(bool),
    /// 64-bit signed integer.
    Integer(i64),
    /// 64-bit float.
    Number(f64),
    /// [`Self::from_str`] only parses to this after trying to parse every other type.
    String(&'a str),
    // SubCommand(Vec<ResolvedOption<'a>>),
    // SubCommandGroup(Vec<ResolvedOption<'a>>),
    Channel(PartialChannel),
    Role(Role),
    User(User, Box<Option<PartialMember>>),
    Attachment(&'a Attachment),
    NotResolved,
}

/// See [`serenity::all::PartialChannel`].
///
/// Copied here so I can construct it
#[derive(Debug)]
#[allow(unused)]
pub struct PartialChannel {
    /// The channel Id.
    pub id: ChannelId,
    /// The channel name.
    pub name: Option<String>,
    /// The channel type.
    pub kind: ChannelType,
    /// The channel permissions.
    pub permissions: Option<Permissions>,
    /// The thread metadata.
    ///
    /// **Note**: This is only available on thread channels.
    pub thread_metadata: Option<ThreadMetadata>,
    /// The Id of the parent category for a channel, or of the parent text channel for a thread.
    ///
    /// **Note**: This is only available on thread channels.
    pub parent_id: Option<ChannelId>,
}

impl From<&serenity::all::PartialChannel> for PartialChannel {
    fn from(value: &serenity::all::PartialChannel) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            kind: value.kind,
            permissions: value.permissions,
            thread_metadata: value.thread_metadata,
            parent_id: value.parent_id,
        }
    }
}

/// Helper to parse ids from string
fn parse_id(str: &str, prefix: &str, suffix: &str) -> Option<u64> {
    if !str.starts_with(prefix) && !str.ends_with(suffix) {
        return None;
    }

    str[prefix.len()..str.len() - suffix.len()].parse().ok()
}

/// Find a channel from a string and the channel's guild.
fn parse_channel(guild: Option<&Guild>, str: &str) -> Option<PartialChannel> {
    fn parse_link(str: &str) -> Option<u64> {
        str.split('/').nth(5)?.parse().ok()
    }

    let id = parse_id(str, "<#", ">").or_else(|| parse_link(str))?.into();
    let channel = guild?.channels.get(&id)?;

    let channel = PartialChannel {
        id: channel.id,
        kind: channel.kind,
        name: Some(channel.name.clone()),
        parent_id: channel.parent_id,
        permissions: channel.permissions,
        thread_metadata: channel.thread_metadata,
    };

    Some(channel)
}

fn parse_user(
    cache: &Cache,
    guild: Option<&Guild>,
    str: &str,
) -> Option<(User, Option<PartialMember>)> {
    let id = parse_id(str, "<@", ">")?.into();
    let member = guild?
        .members
        .get(&id)
        .cloned()
        .map(std::convert::Into::into);

    Some((cache.user(id)?.clone(), member))
}

fn parse_role(guild: Option<&Guild>, str: &str) -> Option<Role> {
    let id = parse_id(str, "<@&", ">")?.into();
    guild?.roles.get(&id).cloned()
}

impl<'a> ArgValue<'a> {
    /// Is guaranteed to return a non [`Arg::NotResolved`] value;
    /// if all other [`Arg`] types can't be parsed, it fallbacks to [`Arg::String`].
    ///
    /// Cannot parse to [`Arg::Attachment`].
    #[inline]
    pub fn from_str(cache: &Cache, guild_id: Option<GuildId>, str: &'a str) -> Self {
        // TODO: test this

        let parsed = if let Some(guild_id) = guild_id {
            let guild = cache.guild(guild_id);

            if let Some(role) = parse_role(guild.as_deref(), str) {
                Some(Self::Role(role))
            } else if let Some(channel) = parse_channel(guild.as_deref(), str) {
                Some(Self::Channel(channel))
            } else if let Some((user, member)) = parse_user(cache, guild.as_deref(), str) {
                Some(Self::User(user, Box::new(member)))
            } else {
                None
            }
        } else {
            None
        };

        if let Some(parsed) = parsed {
            return parsed;
        }

        #[allow(clippy::same_functions_in_if_condition)]
        if let Ok(bool) = str.parse() {
            Self::Boolean(bool)
        } else if let Ok(int) = str.parse() {
            Self::Integer(int)
        } else if let Ok(num) = str.parse() {
            Self::Number(num)
        } else {
            Self::String(str)
        }
    }

    /// Will return [`None`] if `option.value` matches [`serenity::all::ResolvedValue::Autocomplete`],
    /// [`serenity::all::ResolvedValue::SubCommand`], or [`serenity::all::ResolvedValue::SubCommandGroup`]
    #[inline]
    pub fn from_resolved(option: ResolvedOption<'a>) -> Option<Self> {
        use serenity::all::ResolvedValue::{
            Attachment, Autocomplete, Boolean, Channel, Integer, Number, Role, String, SubCommand,
            SubCommandGroup, Unresolved, User,
        };

        let arg = match option.value {
            Attachment(a) => Self::Attachment(a),
            Boolean(b) => Self::Boolean(b),
            Channel(c) => Self::Channel(c.into()),
            Integer(i) => Self::Integer(i),
            Number(n) => Self::Number(n),
            Role(r) => Self::Role(r.clone()),
            String(s) => Self::String(s),
            User(u, m) => Self::User(u.clone(), Box::new(m.cloned())),
            Unresolved(_) => Self::NotResolved,
            Autocomplete { .. } | SubCommand(_) | SubCommandGroup(_) => return None,
            val => {
                warn!("did not handle {val:?} while trying to convert to an Arg");
                return None;
            }
        };

        Some(arg)
    }
}
