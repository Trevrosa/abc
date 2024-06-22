use serenity::all::{Context, Guild, Message, UserId};

use crate::{utils::context::Ext, Blacklisted, OWNER};

pub async fn blacklist(ctx: &Context, msg: &Message) -> Result<(), &'static str> {
    if msg.author.id != OWNER {
        return Err("u canot");
    }

    let mut global = ctx.data.write().await;
    let blacklisted = global.get_mut::<Blacklisted>().unwrap();

    let args: Vec<&str> = msg.content.split("`black ").collect();

    if args.len() == 2 {
        let guild: Guild = msg.guild(&ctx.cache).unwrap().clone();
        let members = guild.members;

        let user: u64 = if let Ok(user) = args[1].parse::<u64>() {
            if !members.contains_key(&UserId::new(user)) {
                return Err("that not real");
            }

            user
        } else if args[1].starts_with("<@") {
            let Ok(user) = args[1][2..args[1].len() - 1].parse::<u64>() else {
                return Err("that not real");
            };

            if !members.contains_key(&UserId::new(user)) {
                return Err("that not real");
            }

            user
        } else {
            return Err("that not real");
        };

        if let Some(seven) = blacklisted.iter().position(|x| x == &user) {
            blacklisted.remove(seven);
            ctx.reply("unblackd", msg).await;
        } else {
            blacklisted.push(user);
            ctx.reply("blackd", msg).await;
        }
    } else {
        let blacklisted: Vec<(&u64, String)> = blacklisted
            .iter()
            .map(|id| (id, ctx.cache.user(*id).unwrap().clone().name))
            .collect();
        let blacklisted = format!("```rust\n{blacklisted:#?}\n```");
        ctx.reply(blacklisted, msg).await;
    }

    Ok(())
}
