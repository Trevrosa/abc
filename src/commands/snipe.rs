use serenity::all::{Context, Message, MessageBuilder};

use crate::utils::{context::Ext, sniping::MostRecentDeletedMessage};

#[allow(clippy::significant_drop_tightening)]
pub async fn snipe(ctx: &Context, msg: &Message) -> Result<(), &'static str> {
    let global = ctx.data.try_read().unwrap();

    let Some(deleted_msg) = global
        .get::<MostRecentDeletedMessage>()
        .unwrap() // should be safe since init in main
        .get(&msg.guild_id.unwrap())
    else {
        return Err("no message to snipe");
    };

    let snipe = MessageBuilder::new()
        .push(&deleted_msg.author)
        .push(" deleted their message: ")
        .push_mono_safe(&deleted_msg.content)
        .push(format!(" (<t:{}:R>)", deleted_msg.timestamp.unix_timestamp()))
        .build();

    ctx.reply(snipe, msg).await;

    Ok(())
}
