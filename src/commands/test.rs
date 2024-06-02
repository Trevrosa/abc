use serenity::all::{Context, Message};

use crate::{utils::context::Ext, Blacklisted, SEVEN};

pub async fn test(ctx: Context, msg: Message) {
    ctx.reply("im brown", &msg).await;

    if msg.author.name == "trevorerer" {
        let mut global = ctx.data.write().await;
        let blacklisted = global.get_mut::<Blacklisted>().unwrap();

        if let Some(seven) = blacklisted.iter().position(|x| x == &SEVEN) {
            blacklisted.remove(seven);
        } else {
            blacklisted.push(SEVEN);
        }

        ctx.reply(format!("```rust\n{blacklisted:#?}\n```"), &msg)
            .await;
    }
}
