#![warn(clippy::pedantic)]
#![deny(clippy::disallowed_methods)]
#![allow(clippy::too_many_lines)]

mod commands;
mod handlers;
mod serenity_ctrlc;
mod utils;

use std::fs;
use std::path::Path;
use std::{collections::HashMap, sync::LazyLock};

use anyhow::Result;
use bincode::{config, serde};
use serenity::{all::Settings, prelude::*};
use serenity_ctrlc::Disconnector;
use songbird::{tracks::TrackHandle, SerenityInit};

use tracing::{error, info, warn};
use utils::sniping::{MostRecentDeletedMessage, MostRecentEditedMessage};
use utils::spotify;
use utils::ytmusic::{self, AccessToken};

pub struct TrackHandleKey;

impl TypeMapKey for TrackHandleKey {
    type Value = TrackHandle;
}

pub struct Blacklisted;

impl TypeMapKey for Blacklisted {
    type Value = Vec<u64>;
}

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[cfg(target_env = "msvc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

// discord id, so its ok to be unreadable
#[allow(clippy::unreadable_literal)]
pub const SEVEN: u64 = 674143957755756545;

#[allow(clippy::unreadable_literal)]
pub const OWNER: u64 = 758926553454870529;

const YT_TOKEN_PATH: &str = "yt_token";
const BLACKLISTED_PATH: &str = "blacklisted";

// serialize blacklisted users to disk, then disconnect all shards
async fn end_handler(disconnector: Option<Disconnector>) {
    if let Some(disconnector) = disconnector {
        if let Ok(global) = disconnector.data.try_read() {
            let blacklisted =
                serde::encode_to_vec(global.get::<Blacklisted>().unwrap(), config::standard())
                    .unwrap();
            fs::write(BLACKLISTED_PATH, blacklisted).unwrap();

            info!("saved blacklisted users");

            if let Some(yt_token) = global.get::<AccessToken>().unwrap() {
                let yt_token = serde::encode_to_vec(yt_token, config::standard()).unwrap();
                fs::write(YT_TOKEN_PATH, yt_token).unwrap();
            }

            info!("saved yt token");
        } else {
            error!("failed to read from global data, can't save");
        }

        disconnector.disconnect().await;
    }
}

pub static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);

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

    let blacklisted: Vec<u64> = fs::read(BLACKLISTED_PATH).map_or_else(
        |_err| {
            warn!("no {BLACKLISTED_PATH} to load from");
            Vec::new()
        },
        |stored| match serde::decode_from_slice(&stored, config::standard()) {
            Ok((blacklisted, _len)) => {
                info!("loaded blacklisted users");
                blacklisted
            }
            Err(e) => {
                error!("failed to load blacklisted users ({e}); using empty");
                Vec::new()
            }
        },
    );

    let access_token = fs::read(YT_TOKEN_PATH).map_or_else(
        |_err| {
            warn!("no {YT_TOKEN_PATH} to load from");
            None
        },
        |stored| match serde::decode_from_slice(&stored, config::standard()) {
            Ok((access_token, _len)) => {
                info!("loaded yt access token");
                Some(access_token)
            }
            Err(e) => {
                error!("failed to load yt access token({e})");
                None
            }
        },
    );

    let mut client: Client = Client::builder(token, intents)
        .event_handler(handlers::Client)
        .event_handler(handlers::Command)
        .event_handler(handlers::Sniper)
        .type_map_insert::<MostRecentDeletedMessage>(HashMap::new())
        .type_map_insert::<MostRecentEditedMessage>(HashMap::new())
        .type_map_insert::<Blacklisted>(blacklisted)
        .type_map_insert::<spotify::AccessToken>(None)
        .type_map_insert::<ytmusic::AccessToken>(access_token)
        .cache_settings(cache_settings)
        .register_songbird()
        .await?;

    serenity_ctrlc::ctrlc_with(&client, end_handler)?;

    client.start().await?;

    Ok(())
}
