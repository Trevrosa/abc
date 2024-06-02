use serenity::all::{Context, Message};

use crate::utils::context::Ext;

pub async fn test(ctx: Context, msg: Message) {
    ctx.reply("im brown", &msg).await;
}
