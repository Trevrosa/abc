use anyhow::Result;
use serenity::all::{Context, Message};

use super::Reply;

pub async fn leave(ctx: Context, msg: Message) {
    let Some(manager) = songbird::get(&ctx).await else {
        ctx.reply("voice client not init", &msg).await;
        return
    };

    let Some(guild_id) = msg.guild_id else {
        ctx.reply("faild to get guild", &msg).await;
        return
    };

    let Some(handler) = manager.get(guild_id) else {
        ctx.reply("faild to get voice handler", &msg).await;
        return
    };

    if manager.leave(guild_id).await.is_ok() {
        handler.lock().await.stop();
        ctx.reply("left u :(", &msg).await;
       
    } else {
        ctx.reply("faild to leave :)", &msg).await;
       
    }
}
