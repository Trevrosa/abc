pub mod access_token;
pub use access_token::get_access_token;
pub use access_token::AccessToken;
pub use search::find_track_from_url;

pub mod search;

use std::time::Duration;

use serenity::{
    all::{
        Context, CreateButton,
        CreateInteractionResponse::{Acknowledge, UpdateMessage},
        CreateInteractionResponseMessage, ReactionType,
    },
    futures::StreamExt,
};
use tracing::{info, warn};

use crate::utils::context::CtxExt;
use crate::CLIENT;

use super::reply::CreateReply;
use super::reply::Replyer;
use super::ytmusic::{self, search::parsing::SearchResult};

async fn spotify_request_token_and_save(
    data: Option<&mut Option<AccessToken>>,
) -> Result<AccessToken, &'static str> {
    info!("requesting new spotify access token");
    let Some(access_token) = get_access_token(&CLIENT).await else {
        return Err("could not get access token");
    };

    if let Some(data) = data {
        data.replace(access_token.clone());
        info!("saved new access token to cache");
    } else {
        warn!("failed to save new access token to cache");
    }

    Ok(access_token)
}

/// From a Spotify url, search on youtube music for an equivalent song.
///
/// Give the user an interactive pager for the returned search results.
///
/// Return the user's choice.
pub async fn extract_spotify(
    ctx: &Context,
    replyer: &Replyer<'_>,
    spotify_url: &str,
) -> Result<String, &'static str> {
    let Ok(mut data) = ctx.data.try_write() else {
        return Err("failed to write global data");
    };

    let token = data.get::<AccessToken>().expect("should at least be None");

    let token = if let Some(token) = token {
        info!("got spotify token from cache");
        token
    } else {
        &spotify_request_token_and_save(data.get_mut::<AccessToken>()).await?
    };

    let token = if token.expired() {
        &spotify_request_token_and_save(data.get_mut::<AccessToken>()).await?
    } else {
        token
    };

    ctx.reply("got spotify token", replyer).await;

    let Ok(spotify_track) = find_track_from_url(spotify_url, token).await else {
        return Err("failed to find metadata");
    };

    let spotify_artists = spotify_track
        .artists
        .iter()
        .map(|a| a.name.as_str())
        .collect::<Vec<_>>();

    ctx.reply(
        format!("extracted metadata ```rs\n{spotify_track:#?}```"),
        replyer,
    )
    .await;

    let stored = data.get::<ytmusic::AccessToken>().expect("");

    #[allow(clippy::single_match_else)]
    let access_token: ytmusic::AccessToken = match stored {
        Some(token) => {
            if token.expired() {
                ctx.reply("cached token found, but expired. refreshing it..", replyer)
                    .await;
                info!("cached yt token expired, refreshing it");
                let mut token = token.clone();
                if let Err(err) = token.refresh().await {
                    let log = format!("failed to refresh: {err:#?}");
                    ctx.error_reply(log, replyer).await;
                    return Err("");
                }
                data.insert::<ytmusic::AccessToken>(Some(token.clone()));
                token
            } else {
                ctx.reply("using valid cached token", replyer).await;
                info!("cached yt token not expired, using it");
                token.clone()
            }
        }
        None => {
            let auth = ytmusic::oauth(ctx, replyer).await;
            let Ok(token) = auth else {
                let log = format!("failed to auth: {:#?}", auth.unwrap_err());
                ctx.error_reply(log, replyer).await;
                return Err("");
            };
            data.insert::<ytmusic::AccessToken>(Some(token.clone()));
            token
        }
    };

    ctx.reply(
        format!(
            "authed! yt token was granted at <t:{}:t>, expires <t:{}:R>",
            access_token.granted.timestamp(),
            access_token.expires_at().timestamp()
        ),
        replyer,
    )
    .await;

    let query = format!("{} {}", spotify_track.name, spotify_artists.join(" "));
    let searched = ytmusic::search(query.as_str(), access_token.as_ref()).await;
    let Ok(searched) = searched else {
        let err = format!("```rs\n{:#?}```", searched.unwrap_err());
        ctx.reply(err, replyer).await;
        return Err("");
    };

    if !searched.status().is_success() {
        warn!("ytm api search endpoint failed with {}", searched.status());
        let err = format!(
            "ytm api gave {}\n```rs\n{:#?}```",
            searched.status(),
            searched.text().await
        );
        ctx.reply(err, replyer).await;
        return Err("");
    }

    let results = match searched.json().await {
        Ok(res) => res,
        Err(err) => {
            let err = format!("failed to deserialize json: {err}");
            ctx.error_reply(err, replyer).await;
            return Err("");
        }
    };

    let Some(results) = ytmusic::parse_results(&results) else {
        warn!("failed to parse search result json");
        return Err("couldnt parse search results");
    };

    if results.is_empty() {
        return Err("search results was empty");
    }

    ctx.reply(format!("yt said yes! ({} results)", results.len()), replyer)
        .await;

    let as_emoji = |emoji: &str| emoji.parse::<ReactionType>().unwrap();

    let left_button = CreateButton::new("left_button").emoji(as_emoji("⬅️"));
    let right_button = CreateButton::new("right_button").emoji(as_emoji("➡️"));
    let ok_button = CreateButton::new("ok_button").emoji(as_emoji("✅"));
    let cancel_button = CreateButton::new("cancel").emoji(as_emoji("❌"));

    let get_url = |result: &SearchResult| {
        if let Some(id) = &result.video_id {
            Some(format!("https://youtube.com/watch?v={id}"))
        } else {
            result
                .playlist_id
                .as_ref()
                .map(|id| format!("https://youtube.com/playlist?list={id}"))
        }
    };

    let fmt_result = |page: usize| {
        let result = &results[page];
        let page = page + 1;

        if let Some(url) = get_url(result) {
            format!("{page}. {url}")
        } else {
            format!("{page}. {}: (no video/playlist id?)", result.title)
        }
    };

    let mut current_page: usize = 0;

    // show the first result by default
    let paginated = CreateReply::new()
        .content(fmt_result(current_page).to_string())
        .button(left_button)
        .button(right_button)
        .button(ok_button)
        .button(cancel_button);

    let paginated = ctx.reply(paginated, replyer).await;

    let mut interactions = paginated
        .await_component_interaction(&ctx.shard)
        .timeout(Duration::from_secs(10 * 60))
        .stream();

    let mut choice = None;

    while let Some(interaction) = interactions.next().await {
        match interaction.data.custom_id.as_str() {
            "left_button" => {
                // we are at the left-most page
                if current_page == 0 {
                    interaction
                        .create_response(&ctx, Acknowledge)
                        .await
                        .unwrap();
                    continue;
                }

                current_page -= 1;

                let response = UpdateMessage(
                    CreateInteractionResponseMessage::new().content(fmt_result(current_page)),
                );
                interaction.create_response(&ctx, response).await.unwrap();
            }
            "right_button" => {
                // we are at the right-most page
                if current_page == results.len() {
                    interaction
                        .create_response(&ctx, Acknowledge)
                        .await
                        .unwrap();
                    continue;
                }

                current_page += 1;

                let response = UpdateMessage(
                    CreateInteractionResponseMessage::new().content(fmt_result(current_page)),
                );
                interaction.create_response(&ctx, response).await.unwrap();
            }
            "ok_button" => {
                choice = Some(current_page);
                interaction
                    .create_response(&ctx, Acknowledge)
                    .await
                    .unwrap();
                break;
            }
            "cancel" => {
                return Err("ok, cancelled");
            }
            id => {
                warn!("received unexpected interaction custom_id {id}");
            }
        }
    }

    let Some(choice) = choice else {
        return Err("there was no choice made");
    };

    let Some(url) = get_url(&results[choice]) else {
        return Err("ur choice had no url ?");
    };

    ctx.reply(format!("chose <{url}>!"), replyer).await;

    Ok(url)
}
