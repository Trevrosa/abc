use serenity::all::{Context, Message};

use super::Reply;

pub async fn start(ctx: Context, msg: Message) {
    let Some(manager) = songbird::get(&ctx).await else {
        ctx.reply("voice client not init", &msg).await;
        return;
    };

    let Some(guild) = msg.guild_id else {
        ctx.reply("faild to get guild", &msg).await;
        return;
    };
}
