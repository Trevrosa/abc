use serenity::all::{Context, Message};

use super::Utils;

pub async fn test(ctx: Context, msg: Message) {
    ctx.reply("im brown", &msg).await;
}
