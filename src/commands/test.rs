use serenity::all::{Context, Message};

use crate::utils::context::Ext;

pub async fn test(ctx: &Context, msg: &Message) -> Result<(), &'static str> {
    ctx.reply("im brown", msg).await;

    let args = msg.content.split(' ').collect::<Vec<&str>>();

    if args.len() == 1 {
        Ok(())
    } else {
        let cmd = args[1];

        crate::handlers::handle_cmd(cmd, ctx, msg).await?;

        Ok(())
    }
}
