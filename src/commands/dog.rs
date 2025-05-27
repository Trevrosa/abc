use std::time::Duration;

use crate::{
    utils::{context::CtxExt, embed_message, Replyer},
    CLIENT,
};
use serenity::all::{Context, CreateCommand};

use super::cat::Image;

pub async fn dog(ctx: &Context, replyer: &Replyer<'_>) -> Result<(), &'static str> {
    let Ok(request) = CLIENT
        .get("https://api.thedogapi.com/v1/images/search")
        .header("x-api-key", include_str!("../../dog_apikey"))
        .timeout(Duration::from_secs(5))
        .build()
    else {
        return Err("failed to create request");
    };

    let Ok(resp) = CLIENT.execute(request).await else {
        return Err("failed to send request");
    };

    let resp = resp.json::<Vec<Image>>().await.unwrap();
    let new_msg = embed_message("dog :o", &resp[0].url, None);

    ctx.reply(new_msg, replyer).await;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("dog").description("૮˶• ﻌ •˶ა")
}
