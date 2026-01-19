use std::time::Duration;

use anyhow::anyhow;
use chrono::{DateTime, TimeDelta, Utc};
use serde::{Deserialize, Serialize};
use serenity::all::Context;
use serenity::prelude::TypeMapKey;
use tokio::time::sleep;
use tracing::info;

use std::fmt::Display;

use crate::{
    utils::{context::CtxExt, reply::Replyer, ytmusic::Authentication},
    CLIENT,
};

const CLIENT_ID: &str = include_str!("../../../../yt_clientid");
const CLIENT_SECRET: &str = include_str!("../../../../yt_secret");

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessToken {
    #[allow(clippy::struct_field_names)]
    pub access_token: String,
    token_type: String,
    refresh_token: String,
    expires_in: i64,
    pub granted: DateTime<Utc>,
}

impl TypeMapKey for AccessToken {
    type Value = Option<AccessToken>;
}

impl AsRef<str> for AccessToken {
    fn as_ref(&self) -> &str {
        &self.access_token
    }
}

impl Display for AccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.token_type, self.access_token)
    }
}

impl Authentication for AccessToken {
    fn value(&self) -> String {
        self.to_string()
    }
}

/// Response of a token refresh request.
#[derive(Deserialize)]
struct RefreshResponse {
    access_token: String,
    expires_in: i64,
    // ignore the other fields
}

impl AccessToken {
    /// Create an [`AccessToken`] from a [`AccessTokenResponse`], setting granted to [`Utc::now`].
    pub(super) fn from_resp(resp: AccessTokenResponse) -> Self {
        Self {
            access_token: resp.access_token,
            token_type: resp.token_type,
            refresh_token: resp.refresh_token,
            expires_in: resp.expires_in,
            granted: Utc::now(),
        }
    }

    /// Check if this access token has expired.
    #[inline]
    pub fn expired(&self) -> bool {
        (Utc::now() - self.granted) > TimeDelta::seconds(self.expires_in)
    }

    /// The [`DateTime<Utc>`] this access token expires at.
    #[inline]
    pub fn expires_at(&self) -> DateTime<Utc> {
        self.granted + TimeDelta::seconds(self.expires_in)
    }

    /// Refresh the access token using its refresh token, updating `self`.
    ///
    /// <https://developers.google.com/youtube/v3/guides/auth/devices#offline>
    pub async fn refresh(&mut self) -> anyhow::Result<()> {
        let resp = CLIENT
            .post("https://oauth2.googleapis.com/token")
            .form(&[
                ("client_id", CLIENT_ID),
                ("client_secret", CLIENT_SECRET),
                ("grant_type", "refresh_token"),
                ("refresh_token", &self.refresh_token),
            ])
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(anyhow!(
                "failed to refresh {}: {:?}",
                resp.status(),
                resp.text().await
            ));
        }

        let refreshed: RefreshResponse = resp.json().await?;

        self.access_token = refreshed.access_token;
        self.expires_in = refreshed.expires_in;
        self.granted = Utc::now();

        Ok(())
    }
}

#[derive(Deserialize, Debug)]
struct AuthResponse {
    device_code: String,
    user_code: String,
    expires_in: u64,
    interval: u64,
    verification_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct AccessTokenResponse {
    #[allow(clippy::struct_field_names)]
    pub access_token: String,
    pub token_type: String,
    pub refresh_token: String,
    pub expires_in: i64,
    // ignore fields refresh_token_expires_in, scope
}

/// Go through the [oauth flow](https://developers.google.com/youtube/v3/guides/auth/devices) to get the access token.
///
/// - `context` is the bot's [`Context`]
/// - `msg` is the user's message requesting this oauth.
pub async fn oauth(ctx: &Context, replyer: &Replyer<'_>) -> anyhow::Result<AccessToken> {
    let auth_resp = CLIENT
        .post("https://oauth2.googleapis.com/device/code")
        .form(&[
            ("client_id", CLIENT_ID),
            ("scope", "https://www.googleapis.com/auth/youtube"),
        ])
        .send()
        .await?;

    if !auth_resp.status().is_success() {
        return Err(anyhow!(
            "initial req {}: `{:?}`",
            auth_resp.status(),
            auth_resp.text().await
        ));
    }

    let AuthResponse {
        device_code,
        user_code,
        expires_in,
        interval,
        verification_url,
    } = auth_resp.json::<AuthResponse>().await?;

    info!("successfully got initial oauth response: polling at {interval} sec intervals");

    ctx
        .reply(
            format!("we need to authenticate with google.\ngo to <{verification_url}>\nand enter this code: `{user_code}` (u have {expires_in} secs)"),
            replyer,
        )
        .await;

    let poll = CLIENT
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("client_id", CLIENT_ID),
            ("client_secret", CLIENT_SECRET),
            ("device_code", &device_code),
            ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
        ])
        .build()?;

    loop {
        let poll_resp = CLIENT
            .execute(poll.try_clone().expect("body not stream"))
            .await?;

        match poll_resp.status() {
            reqwest::StatusCode::OK => {
                let resp: AccessTokenResponse = poll_resp.json().await?;
                info!("success! finished polling");
                return Ok(AccessToken::from_resp(resp));
            }

            reqwest::StatusCode::PRECONDITION_REQUIRED => {
                info!("still polling oauth");
            }

            // other statuses should be error statuses
            err => {
                return Err(anyhow!("polled {err}: `{:?}`", poll_resp.text().await));
            }
        }

        sleep(Duration::from_secs(interval)).await;
    }
}
