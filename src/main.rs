#![warn(clippy::pedantic)]
#![deny(clippy::disallowed_methods)]
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::too_many_lines
)]

mod commands;
mod handlers;
mod serenity_ctrlc;
mod utils;

use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;
use handlers::{CommandHandler, MessageSniper};
use serenity::{all::Settings, prelude::*};
use songbird::{tracks::TrackHandle, SerenityInit};

use utils::sniping::{MostRecentDeletedMessage, MostRecentEditedMessage};

pub struct TrackHandleKey;

impl TypeMapKey for TrackHandleKey {
    type Value = TrackHandle;
}

pub struct HttpClient;

impl TypeMapKey for HttpClient {
    type Value = reqwest::Client;
}

pub struct Blacklisted;

impl TypeMapKey for Blacklisted {
    type Value = Vec<u64>;
}

pub const SEVEN: u64 = 674143957755756545;

#[tokio::main]
async fn main() -> Result<()> {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::level_filters::LevelFilter::DEBUG)
    //     .without_time()
    //     .init();

    assert!(Path::new("/usr/bin/yt-dlp").exists());

    tracing_subscriber::fmt().without_time().init();

    let token: &str = include_str!("../token");
    let intents: GatewayIntents = GatewayIntents::all();

    // have to do this instead of a struct expression because Settings is non_exhaustive
    let mut cache_settings = Settings::default();
    cache_settings.max_messages = 100;

    let mut client: Client = Client::builder(token, intents)
        .event_handler(CommandHandler)
        .event_handler(MessageSniper)
        .type_map(TypeMap::new())
        .type_map_insert::<HttpClient>(reqwest::Client::new())
        .type_map_insert::<MostRecentDeletedMessage>(HashMap::new())
        .type_map_insert::<MostRecentEditedMessage>(HashMap::new())
        .type_map_insert::<Blacklisted>(Vec::new())
        .cache_settings(cache_settings)
        .register_songbird()
        .await?;

    serenity_ctrlc::ctrlc(&client)?;

    client.start().await?;

    Ok(())
}
