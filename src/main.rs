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
use serenity::{all::Settings, prelude::*};
use serenity_ctrlc::Disconnector;
use songbird::{tracks::TrackHandle, SerenityInit};

use tracing::{error, info};
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

// discord id, so its ok to be unreadable
#[allow(clippy::unreadable_literal)]
pub const SEVEN: u64 = 674143957755756545;

#[allow(clippy::unreadable_literal)]
pub const OWNER: u64 = 758926553454870529;

// serialize blacklisted users to disk, then disconnect all shards
async fn end_handler(disconnector: Option<Disconnector>) {
    if let Some(disconnector) = disconnector {
        if let Ok(global) = disconnector.data.try_read() {
            let blacklisted = bincode::serialize(global.get::<Blacklisted>().unwrap()).unwrap();
            std::fs::write("blacklisted", blacklisted).unwrap();

            info!("saved blacklisted users");
        } else {
            error!("failed to save blacklisted users");
        }

        disconnector.disconnect().await;
    }
}

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

    // have to do this instead of a struct expression because Settings is non exhaustive
    let mut cache_settings = Settings::default();
    cache_settings.max_messages = 50;

    let blacklisted: Vec<u64> = if let Ok(serialized) = std::fs::read("blacklisted") {
        match bincode::deserialize(&serialized) {
            Ok(blacklisted) => {
                info!("loaded blacklisted users");
                blacklisted
            }
            Err(e) => {
                error!("failed to load blacklisted users ({e}); using empty");
                Vec::new()
            }
        }
    } else {
        Vec::new()
    };

    let mut client: Client = Client::builder(token, intents)
        .event_handler(handlers::Client)
        .event_handler(handlers::Command)
        .event_handler(handlers::Sniper)
        .type_map_insert::<HttpClient>(reqwest::Client::new())
        .type_map_insert::<MostRecentDeletedMessage>(HashMap::new())
        .type_map_insert::<MostRecentEditedMessage>(HashMap::new())
        .type_map_insert::<Blacklisted>(blacklisted)
        .cache_settings(cache_settings)
        .register_songbird()
        .await?;

    serenity_ctrlc::ctrlc_with(&client, end_handler)?;

    client.start().await?;

    Ok(())
}
