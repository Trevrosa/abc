use serenity::all::{Context, Message};

use crate::utils::context::Ext;

pub async fn leave(ctx: &Context, msg: &Message) -> Result<(), &'static str> {
    let Some(manager) = songbird::get(ctx).await.clone() else {
        return Err("voice client not init");
    };

    let Some(guild_id) = msg.guild_id else {
        return Err("faild to get guild");
    };

    let Some(handler) = manager.get(guild_id) else {
        return Err("faild to get voice handler");
    };

    if manager.leave(guild_id).await.is_ok() {
        handler.lock().await.stop();
        ctx.reply("left u :(", msg).await;
    } else {
        return Err("faild to leave :)");
    }

    Ok(())
}
