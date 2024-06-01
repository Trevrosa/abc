use serde::Deserialize;
use serenity::all::{Context, Message};

use crate::utils::context::Ext;
use crate::utils::embed_message;
use crate::HttpClientKey;

#[allow(unused)]
#[derive(Deserialize)]
struct CatImage<'a> {
    id: &'a str,
    url: &'a str,
    width: u32,
    height: u32,
}

pub async fn cat(ctx: Context, msg: Message) {
    let global = ctx.data.try_read().unwrap();

    let Some(client) = global.get::<HttpClientKey>() else {
        drop(global);
        ctx.reply("failed to get http client", &msg).await;
        return;
    };

    let Ok(request) = client
        .get("https://api.thecatapi.com/v1/images/search")
        .header("x-api-key", include_str!("../../cat_apikey"))
        .build()
    else {
        ctx.reply("failed to create request", &msg).await;
        return;
    };

    let Ok(resp) = client.execute(request).await else {
        ctx.reply("failed to send request", &msg).await;
        return;
    };

    let resp = resp.text().await.unwrap();
    let resp: Vec<CatImage> = serde_json::from_str(&resp).unwrap();

    let new_msg = embed_message("car", resp[0].url);

    ctx.reply(new_msg, &msg).await;
}
