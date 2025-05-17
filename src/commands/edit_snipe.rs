use serenity::all::{Context, Message, MessageBuilder};

use crate::utils::{context::Ext, sniping::MostRecentEditedMessage};

#[allow(clippy::significant_drop_tightening)]
pub async fn edit_snipe(ctx: &Context, msg: &Message) -> Result<(), &'static str> {
    let global = ctx.data.try_read().unwrap();

    let Some(edited_msg) = global
        .get::<MostRecentEditedMessage>()
        .unwrap() // should be safe since init in main
        .get(&msg.guild_id.unwrap())
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

    ctx.reply(snipe, msg).await;

    Ok(())
}
