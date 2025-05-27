use serenity::all::{Context, CreateCommand, InteractionContext, MessageBuilder};

use crate::utils::{context::CtxExt, reply::Replyer, sniping::MostRecentDeletedMessage};

pub async fn snipe(ctx: &Context, replyer: &Replyer<'_>) -> Result<(), &'static str> {
    let data = ctx.data.try_read().unwrap();

    let guild_id = match replyer {
        Replyer::Prefix(msg) => msg.guild_id,
        Replyer::Slash(int) => int.guild_id,
    };

    let Some(guild_id) = guild_id else {
        return Err("faild to get guild");
    };

    let Some(deleted_msg) = data
        .get::<MostRecentDeletedMessage>()
        .unwrap() // should be safe since init in main
        .get(&guild_id)
    else {
        return Err("no message to snipe");
    };

    let snipe = MessageBuilder::new()
        .push(&deleted_msg.author)
        .push(" deleted their message: ")
        .push_mono_safe(&deleted_msg.content)
        .push(format!(
            " (<t:{}:R>)",
            deleted_msg.timestamp.unix_timestamp()
        ))
        .build();

    ctx.reply(snipe, replyer).await;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("snipe")
        .add_context(InteractionContext::Guild)
        .description("snipe a deleted message")
}
