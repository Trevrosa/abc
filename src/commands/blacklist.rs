use serenity::all::{CommandOptionType, Context, CreateCommand, CreateCommandOption};

use crate::{
    utils::{context::CtxExt, reply::Replyer, ArgValue, Args},
    Blacklisted, OWNER,
};

pub async fn blacklist(
    ctx: &Context,
    replyer: &Replyer<'_>,
    args: Args<'_>,
) -> Result<(), &'static str> {
    let author_id = match replyer {
        Replyer::Prefix(msg) => msg.author.id,
        Replyer::Slash(int) => int.user.id,
    };

    if author_id != OWNER {
        return Err("u canot");
    }

    let mut global = ctx.data.write().await;
    let blacklisted = global.get_mut::<Blacklisted>().unwrap();

    if let Some(ArgValue::User(user, _)) = args.first_value() {
        let user = user.id.get();

        if let Some(seven) = blacklisted.iter().position(|x| x == &user) {
            blacklisted.remove(seven);
            drop(global);

            ctx.reply("unblackd", replyer).await;
        } else {
            blacklisted.push(user);
            drop(global);

            ctx.reply("blackd", replyer).await;
        }
    } else {
        let blacklisted: Vec<(&u64, String)> = blacklisted
            .iter()
            .map(|id| (id, ctx.cache.user(*id).unwrap().clone().name))
            .collect();
        let blacklisted = format!("```rust\n{blacklisted:#?}\n```");
        drop(global);

        ctx.reply(blacklisted, replyer).await;
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("black")
        .description("blacklist someone from using bot commands")
        .add_option(CreateCommandOption::new(
            CommandOptionType::User,
            "user",
            "the user to blacklist",
        ))
}
