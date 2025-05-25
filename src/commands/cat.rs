use serde::Deserialize;
use serenity::all::{Context, CreateCommand};

use crate::utils::context::CtxExt;
use crate::utils::embed_message;
use crate::utils::reply::Replyer;
use crate::CLIENT;

#[allow(unused)]
#[derive(Deserialize)]
struct CatImage {
    id: String,
    url: String,
    width: u32,
    height: u32,
}

pub async fn cat(ctx: &Context, replyer: &Replyer<'_>) -> Result<(), &'static str> {
    let Ok(request) = CLIENT
        .get("https://api.thecatapi.com/v1/images/search")
        .header("x-api-key", include_str!("../../cat_apikey"))
        .build()
    else {
        return Err("failed to create request");
    };

    let Ok(resp) = CLIENT.execute(request).await else {
        return Err("failed to send request");
    };

    let resp = resp.json::<Vec<CatImage>>().await.unwrap();
    let new_msg = embed_message("car", &resp[0].url);

    ctx.reply(new_msg, replyer).await;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("cat").description("car :3")
}
