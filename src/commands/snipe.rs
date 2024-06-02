use serenity::all::{Context, Message};

use crate::utils::{context::Ext, sniping::MostRecentDeletedMessage};

pub async fn snipe(ctx: Context, msg: Message) {
    let global = ctx.data.try_read().unwrap();

    let Some(deleted_msg) = global
        .get::<MostRecentDeletedMessage>()
        .unwrap() // should be safe since init in main
        .get(&msg.guild_id.unwrap())
    else {
        ctx.reply("no message to snipe", &msg).await;
        return;
    };

    let deleted_msg = ctx
        .cache
        .message(deleted_msg.channel, deleted_msg.id)
        .map(|x| x.clone());

    let Some(deleted_msg) = deleted_msg else {
        ctx.reply("msg not found", &msg).await;
        return;
    };

    let snipe = format!(
        "{} said: `{}` (<t:{}:R>)", // discord relative timestamp
        deleted_msg.author.name,
        deleted_msg.content,
        deleted_msg.timestamp.unix_timestamp()
    );

    ctx.reply(snipe, &msg).await;
}
