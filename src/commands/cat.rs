use std::time::Duration;

use serde::Deserialize;
use serenity::all::{Context, CreateCommand};

use crate::{
    utils::{context::CtxExt, embed_message, reply::Replyer},
    CLIENT,
};

#[allow(unused)]
#[derive(Deserialize)]
pub struct Image {
    pub url: String,
    pub breeds: Option<Vec<Breed>>,
}

// TODO: use this
#[allow(unused)]
#[derive(Deserialize)]
pub struct Breed {
    name: String,
    temperament: String,
}

pub async fn cat(ctx: &Context, replyer: &Replyer<'_>) -> Result<(), &'static str> {
    let Ok(request) = CLIENT
        .get("https://api.thecatapi.com/v1/images/search")
        .header("x-api-key", include_str!("../../cat_apikey"))
        .timeout(Duration::from_secs(5))
        .build()
    else {
        return Err("failed to create request");
    };

    let Ok(resp) = CLIENT.execute(request).await else {
        return Err("failed to send request");
    };

    let resp = resp.json::<Vec<Image>>().await.unwrap();
    let new_msg = embed_message("car", &resp[0].url, None);

    ctx.reply(new_msg, replyer).await;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("cat").description("car :3")
}
