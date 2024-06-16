use serenity::all::{Context, Message};

use crate::utils::{context::Ext, sniping::MostRecentDeletedMessage};

pub async fn snipe(ctx: &Context, msg: &Message) -> Result<(), &'static str> {
    let global = ctx.data.try_read().unwrap();

    let Some(deleted_msg) = global
        .get::<MostRecentDeletedMessage>()
        .unwrap() // should be safe since init in main
        .get(&msg.guild_id.unwrap())
    else {
        return Err("no message to snipe");
    };

    let snipe = format!(
        "{} deleted their message: `{}` (<t:{}:R>)", // discord relative timestamp
        deleted_msg.author,
        deleted_msg.content.replace('`', ""),
        deleted_msg.timestamp.unix_timestamp()
    );

    ctx.reply(snipe, &msg).await;

    Ok(())
}
