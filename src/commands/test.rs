use serenity::all::{Context, Message};

use super::Reply;

pub async fn test(ctx: Context, msg: Message) {
    ctx.reply("im brown", &msg).await;
}
