use anyhow::Result;
use serenity::all::{Context, Message};

use super::Reply;

pub async fn test(ctx: Context, msg: Message) -> Result<()> {
    ctx.reply("im brown", &msg).await;
    Ok(())
}
