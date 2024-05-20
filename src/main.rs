#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

pub mod commands;
pub mod error;
mod listener;

use anyhow::Result;
use listener::Listener;
use serenity::prelude::*;
use songbird::{tracks::TrackHandle, SerenityInit};

pub struct TrackHandleKey;

impl TypeMapKey for TrackHandleKey {
    type Value = TrackHandle;
}

#[tokio::main]
async fn main() -> Result<()> {
    // tracing_subscriber::fmt().with_max_level(LevelFilter::DEBUG).init();
    tracing_subscriber::fmt().without_time().init();

    let token: &str = include_str!("../token");
    let intents: GatewayIntents = GatewayIntents::all();

    let mut client: Client = Client::builder(token, intents)
        .event_handler(Listener)
        .type_map(TypeMap::custom())
        .register_songbird()
        .await?;

    client.start().await?;

    Ok(())
}
