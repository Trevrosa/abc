use std::time::Instant;

use crate::utils::{context::CtxExt, ArgValue, Args, Replyer};
use serenity::all::{
    CommandOptionType, Context, CreateCommand, CreateCommandOption, RoleId, UserId,
};
use tokio_stream::StreamExt;

// non slashcmd syntax:
// <user> / <role> -- no keyword needed
// before: "" / after: "" / regex: "" -- keyword needed 
pub async fn delete(
    ctx: &Context,
    replyer: &Replyer<'_>,
    args: Args<'_>,
) -> Result<(), &'static str> {
    let Some(guild_id) = replyer.guild() else {
        return Err("faild to get guild");
    };

    let channel_id = replyer.channel();

    let args: Vec<_> = args.iter().collect();

    let mut filter = Filter::default();

    for arg in args {
        match &arg.value {
            ArgValue::User(user, _) => filter.user_id = Some(user.id),
            ArgValue::Role(role) => filter.role_id = Some(role.id),
            ArgValue::String(str) => {
                if arg.name
            }
        }
    }

    // let messages = channel_id
    //     .messages_iter(&ctx.http)
    //     .filter_map(|i| i.map(filter).ok());

    ctx.reply("", replyer).await;

    Ok(())
}

#[derive(Default)]
struct Filter {
    regex: Option<String>,
    user_id: Option<UserId>,
    role_id: Option<RoleId>,
    time: Option<Instant>,
    after: Option<bool>,
}

pub fn register() -> CreateCommand {
    CreateCommand::new("delete")
        .description("delete some msgs according to some criteria")
        .add_option(CreateCommandOption::new(
            CommandOptionType::String,
            "regex",
            "regex matcher to match in message contents",
        ))
        .add_option(CreateCommandOption::new(
            CommandOptionType::User,
            "from",
            "finds messages sent by user",
        ))
        .add_option(CreateCommandOption::new(
            CommandOptionType::Role,
            "from",
            "finds messages sent by role",
        ))
        .add_option(CreateCommandOption::new(
            CommandOptionType::String,
            "before",
            "finds messages sent before a date",
        ))
        .add_option(CreateCommandOption::new(
            CommandOptionType::String,
            "after",
            "finds messages sent after a date",
        ))
}
