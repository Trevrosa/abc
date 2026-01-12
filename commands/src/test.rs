use macro_cmd::command;
use serenity::all::{Context, CreateCommand};

use utils::{context::CtxExt, reply::Replyer};

#[command]
pub async fn test(ctx: &Context, replyer: &Replyer<'_>) -> Result<(), &'static str> {
    ctx.reply("im brown", replyer).await;
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("test").description("test")
}
