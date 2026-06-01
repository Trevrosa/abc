use std::path::Path;

use abc::code_fmt;
use scraper::Html;
use serenity::all::{CommandOptionType, Context, CreateCommand, CreateCommandOption, ReactionType};

use crate::utils::context::CtxExt;
use crate::utils::ib::parsing::{Query, QueryKind};
use crate::utils::ib::{self, Mirror};
use crate::utils::reply::Replyer;
use crate::utils::{Args, flaresolverr};

pub async fn ib(ctx: &Context, replyer: &Replyer<'_>, args: Args<'_>) -> Result<(), &'static str> {
    let query = args.full_string();

    if query.is_empty() {
        return Err("no query");
    }

    let Ok(mirrors) = ib::get_mirrors().await else {
        return Err("could not get mirrors");
    };

    let Ok(base) = flaresolverr::get(&mirrors[0].url).await else {
        return Err("could not get mirror");
    };

    let (query, warn) = Query::parse(&query);
    if let Some(warn) = warn {
        ctx.reply(format!("{warn} cus u didnt specify"), replyer)
            .await;
    }
    let Some(query) = query else {
        return Err("not understand");
    };

    handle(ctx, replyer, &mirrors, query).await;

    let page = Html::parse_document(&base);
    let files = ib::page_files(&page);

    if files.is_empty() {
        return Err("no files");
    }

    ctx.reply(code_fmt!(files), replyer).await;

    Ok(())
}

async fn handle(ctx: &Context, replyer: &Replyer<'_>, mirrors: &[Mirror], query: Query) {
    if matches!(query.kind, QueryKind::All) {
        for kind in QueryKind::BASES {
            let Ok(page) = get_html(kind).await else {
                continue;
            };
            let files = ib::page_files(&page);

            for file in files {
                let msg = ctx.reply(Path::new(&mirrors[0]).join(kind).join(file).to_string_lossy(), replyer).await;
            }

            ctx.reply("change mirror", replyer)
                .await
                .react(&ctx.http, ReactionType::Unicode("🔃".to_string()))
                .await;
        }
        return;
    }

    let base = QueryKind::BASES[query.kind as usize];
}

async fn get_html(url: &str) -> anyhow::Result<Html> {
    let page = flaresolverr::get(url).await?;
    Ok(Html::parse_document(&page))
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
