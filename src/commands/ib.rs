use scraper::Html;
use serenity::all::{CommandOptionType, Context, CreateCommand, CreateCommandOption};

use crate::utils::context::CtxExt;
use crate::utils::ib;
use crate::utils::reply::Replyer;
use crate::utils::{Args, flaresolverr};

pub async fn ib(ctx: &Context, replyer: &Replyer<'_>, args: Args<'_>) -> Result<(), &'static str> {
    // let query = args.full_string();

    // if query.is_empty() {
    //     return Err("no query");
    // };

    let Ok(mirrors) = ib::get_mirrors().await else {
        return Err("could not get mirrors");
    };

    let Ok(mirror) = flaresolverr::get(&mirrors[0].url).await else {
        return Err("could not get mirror");
    };

    let mirror = Html::parse_document(&mirror);

    let files = ib::page_files(&mirror);

    if files.is_empty() {
        return Err("no files");
    }

    ctx.reply(format!("```rust\n{files:#?}\n```"), replyer)
        .await;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ib")
        .description("find an ib resource")
        .add_option(CreateCommandOption::new(
            CommandOptionType::String,
            "query",
            "what you want to find",
        ))
}
