use std::time::Duration;

use serenity::all::{
    CommandOptionType, Context, CreateCommand, CreateCommandOption, InteractionContext,
};

use crate::utils::context::CtxExt;
use crate::utils::reply::Replyer;
use crate::utils::{ArgValue, Args};
use crate::TrackHandleKey;

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

    let global = ctx.data.try_read().unwrap();

    if global.contains_key::<TrackHandleKey>() {
        let Some(track) = global.get::<TrackHandleKey>() else {
            return Err("song ended..");
        };

        #[allow(clippy::cast_sign_loss)]
        let seek = track.seek_async(Duration::from_secs(*to_seek as u64)).await;
        drop(global);

        if seek.is_ok() {
            ctx.reply(format!("seekd to {to_seek} secs"), replyer).await;
        } else {
            ctx.reply("faild to seek", replyer).await;
        }
    } else {
        ctx.reply("im not play anything", replyer).await;
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
