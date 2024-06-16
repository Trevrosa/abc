use serenity::all::{Context, Message};

use crate::utils::{context::Ext, sniping::MostRecentEditedMessage};

pub async fn edit_snipe(ctx: &Context, msg: &Message) -> Result<(), &'static str> {
    let global = ctx.data.try_read().unwrap();

    let Some(deleted_msg) = global
        .get::<MostRecentEditedMessage>()
        .unwrap() // should be safe since init in main
        .get(&msg.guild_id.unwrap())
    else {
        return Err("no message to snipe");
    };

    let snipe = if deleted_msg.timestamp.is_some() {
        format!(
            "{} edited their message `{}` to: `{}` (<t:{}:R>)", // discord relative timestamp
            deleted_msg.author,
            deleted_msg.old_message.replace('`', ""),
            deleted_msg.new_message.replace('`', ""),
            deleted_msg.timestamp.unwrap().unix_timestamp()
        )
    } else {
        format!(
            "{} edited their message `{}` to: `{}`", // discord relative timestamp
            deleted_msg.author, deleted_msg.old_message, deleted_msg.new_message,
        )
    };

    ctx.reply(snipe, msg).await;

    Ok(())
}
