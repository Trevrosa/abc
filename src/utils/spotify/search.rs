use anyhow::anyhow;
use reqwest::IntoUrl;
use serde::Deserialize;
use tracing::info;

use crate::CLIENT;

/// Only some of the fields.
///
/// <https://developer.spotify.com/documentation/web-api/reference/get-track>
#[derive(Deserialize, Debug)]
pub struct SpotifyTrack {
    pub name: String,
    pub artists: Vec<SpotifyArtist>,
}

/// I only want the name.
#[derive(Deserialize, Debug)]
pub struct SpotifyArtist {
    pub name: String,
}

const TRACK_API: &str = "https://api.spotify.com/v1/tracks";

/// Parse the track id from `url` and find its isrc.
pub async fn find_track_from_url<S: AsRef<str>>(
    url: impl IntoUrl,
    access_token: S,
) -> anyhow::Result<SpotifyTrack> {
    let url = url.into_url()?;

    // check if url is spotify track url
    if url.domain().is_none_or(|d| d != "spotify.com") && !url.as_str().contains("track") {
        return Err(anyhow!("{url} is not a spotify track url"));
    }

    let track_id = if let Some(query_string) = url.query() {
        url.path_segments()
            .iter_mut()
            .find_map(|p| p.next_back().map(|p| p.replace(query_string, "")))
    } else {
        url.path_segments()
            .iter_mut()
            .find_map(|p| p.next_back().map(str::to_string))
    };

    let Some(track_id) = track_id else {
        return Err(anyhow!("could not parse track id from url"));
    };

    find_track(track_id.as_ref(), access_token.as_ref()).await
}

// we can search youtube music by isrc by just using it as query.
pub async fn find_track<S: AsRef<str>>(
    track_id: S,
    access_token: S,
) -> anyhow::Result<SpotifyTrack> {
    let track_id = track_id.as_ref();
    let access_token = access_token.as_ref();

    info!("finding track id {track_id}");

    let resp = CLIENT
        .get(format!("{TRACK_API}/{track_id}"))
        .bearer_auth(access_token)
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(anyhow!("got {}: {:?}", resp.status(), resp.text().await));
    }

    let resp = resp.json::<SpotifyTrack>().await?;

    Ok(resp)
}
