use std::time::{Duration, Instant};

use reqwest::Client;
use serde::Deserialize;
use serenity::prelude::TypeMapKey;
use tracing::{error, info};

#[derive(Deserialize, Debug, Clone)]
pub struct AccessToken {
    #[allow(clippy::struct_field_names)]
    access_token: String,
    token_type: String,
    expires_in: u64,

    #[serde(skip)]
    granted: Option<Instant>,
}

impl AsRef<str> for AccessToken {
    fn as_ref(&self) -> &str {
        &self.access_token
    }
}

impl AccessToken {
    pub fn expired(&self) -> bool {
        self.granted
            .is_some_and(|g| g.elapsed() > Duration::from_secs(self.expires_in))
    }
}

impl TypeMapKey for AccessToken {
    type Value = Option<AccessToken>;
}

pub async fn get_access_token(client: &Client) -> Option<AccessToken> {
    const AUTH_REQ: &str = "https://accounts.spotify.com/api/token";

    // see https://developer.spotify.com/documentation/web-api/tutorials/client-credentials-flow
    let resp = client
        .post(AUTH_REQ)
        .basic_auth(
            include_str!("../../../spotify_clientid"),
            Some(include_str!("../../../spotify_secret")),
        )
        .form(&[("grant_type", "client_credentials")])
        .send()
        .await;
    let Ok(resp) = resp else {
        return None;
    };

    if !resp.status().is_success() {
        error!(
            "failed to request access token: `{}`",
            resp.text()
                .await
                .unwrap_or("failed to read body".to_string())
        );
        return None;
    }

    let Ok(mut resp) = resp.json::<AccessToken>().await else {
        return None;
    };
    resp.granted = Some(Instant::now());

    info!(
        "got access token `{}`, expiring in {} secs",
        resp.token_type, resp.expires_in
    );

    Some(resp)
}
