use serenity::all::{Context, CreateCommand, CreateCommandOption, InteractionContext};

use crate::utils::context::CtxExt;
use crate::utils::reply::Replyer;
use crate::utils::{ArgValue, Args};

pub async fn dequeue(
    ctx: &Context,
    replyer: &Replyer<'_>,
    args: Args<'_>,
) -> Result<(), &'static str> {
    let Some(guild_id) = replyer.guild() else {
        return Err("u not in a guild");
    };

    let Some(ArgValue::Integer(index)) = args.first_value() else {
        return Err("u no say which track to remove");
    };

    if *index == 1 {
        return Err("cant dequeue the track im playing now, use /skip");
    }

    let manager = songbird::get(ctx).await.unwrap();

    if let Some(handler) = manager.get(guild_id) {
        let handler = handler.lock().await;
        let queue = handler.queue();

        if *index as usize - 1 > queue.len() {
            return Err("that track no exist in queue ...");
        }
        queue.dequeue(*index as usize - 1).unwrap();
    } else {
        return Err("im not play anything");
    }

    ctx.reply(format!("removed track {index}"), replyer).await;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("dequeue")
        .add_context(InteractionContext::Guild)
        .description("remove a track from the queue")
        .add_option(CreateCommandOption::new(
            serenity::all::CommandOptionType::Integer,
            "tracknumber",
            "the number of the track in queue u want to remove",
        ))
}
