use std::time::Duration;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serenity::all::Context;
use tokio::time::sleep;
use tracing::info;

use crate::{
    utils::{context::CtxExt, reply::Replyer},
    CLIENT,
};

use super::access_token::AccessToken;

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
            ("client_id", include_str!("../../../yt_clientid")),
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
            ("client_id", include_str!("../../../yt_clientid")),
            ("client_secret", include_str!("../../../yt_secret")),
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
