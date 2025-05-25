use serenity::all::{Context, CreateCommand, InteractionContext, MessageBuilder};

use crate::utils::{context::CtxExt, reply::Replyer, sniping::MostRecentEditedMessage};

#[allow(clippy::significant_drop_tightening)]
pub async fn edit_snipe(ctx: &Context, replyer: &Replyer<'_>) -> Result<(), &'static str> {
    let global = ctx.data.try_read().unwrap();

    let guild_id = match replyer {
        Replyer::Prefix(msg) => msg.guild_id,
        Replyer::Slash(int) => int.guild_id,
    };

    let Some(guild_id) = guild_id else {
        return Err("faild to get guild");
    };

    let Some(edited_msg) = global
        .get::<MostRecentEditedMessage>()
        .unwrap() // should be safe since init in main
        .get(&guild_id)
    else {
        return Err("no message to snipe");
    };

    let timestamp = if let Some(timestamp) = edited_msg.timestamp {
        format!(" (<t:{}:R>)", timestamp.unix_timestamp())
    } else {
        " (unknown time)".to_string()
    };

    let snipe = MessageBuilder::new()
        .push(&edited_msg.author)
        .push(" edited their message ")
        .push_mono_safe(&edited_msg.old_message)
        .push(" to: ")
        .push_mono_safe(&edited_msg.new_message)
        .push(timestamp)
        .build();

    ctx.reply(snipe, replyer).await;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("editsnipe")
        .add_context(InteractionContext::Guild)
        .description("snipe an edited message")
}
