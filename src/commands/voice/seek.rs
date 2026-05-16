use std::time::Duration;

use serenity::all::{
    CommandOptionType, Context, CreateCommand, CreateCommandOption, InteractionContext,
};

use crate::utils::context::CtxExt;
use crate::utils::reply::Replyer;
use crate::utils::{ArgValue, Args};

pub async fn seek(
    ctx: &Context,
    replyer: &Replyer<'_>,
    args: Args<'_>,
) -> Result<(), &'static str> {
    if args.len() != 1 {
        return Err("u dont say wat i seek to");
    }

    let Some(ArgValue::Integer(to_seek)) = args.first_value() else {
        return Err("not number");
    };

    let Some(guild_id) = replyer.guild() else {
        return Err("u not in a guild");
    };

    let Some(track) = ctx.current_track(guild_id).await else {
        return Err("im not play anything");
    };

    #[allow(clippy::cast_sign_loss)]
    let seek = track.seek_async(Duration::from_secs(*to_seek as u64)).await;

    if seek.is_ok() {
        ctx.reply(format!("seekd to {to_seek} secs"), replyer).await;
    } else {
        ctx.reply("faild to seek", replyer).await;
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("seek")
        .add_context(InteractionContext::Guild)
        .description("seek the current song to the given amount of seconds")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "seektime",
                "the amount of seconds to seek the current song to",
            )
            .required(true),
        )
}
