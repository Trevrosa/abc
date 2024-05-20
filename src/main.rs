#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

pub mod commands;
mod handlers;

use anyhow::Result;
use handlers::Handler;
use serenity::prelude::*;
use songbird::{tracks::TrackHandle, SerenityInit};

pub struct TrackHandleKey;

impl TypeMapKey for TrackHandleKey {
    type Value = TrackHandle;
}

#[tokio::main]
async fn main() -> Result<()> {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::level_filters::LevelFilter::DEBUG)
    //     .without_time()
    //     .init();
    tracing_subscriber::fmt().without_time().init();

    let token: &str = include_str!("../token");
    let intents: GatewayIntents = GatewayIntents::all();

    let mut client: Client = Client::builder(token, intents)
        .event_handler(Handler)
        .type_map(TypeMap::custom())
        .register_songbird()
        .await?;

    client.start().await?;

    Ok(())
}