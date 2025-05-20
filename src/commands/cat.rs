use serde::Deserialize;
use serenity::all::{Context, Message};

use crate::utils::context::Ext;
use crate::utils::embed_message;
use crate::CLIENT;

#[allow(unused)]
#[derive(Deserialize)]
struct CatImage {
    id: String,
    url: String,
    width: u32,
    height: u32,
}

pub async fn cat(ctx: &Context, msg: &Message) -> Result<(), &'static str> {
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

    ctx.reply(new_msg, msg).await;

    Ok(())
}
