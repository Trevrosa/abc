use std::fmt::Display;

use anyhow::anyhow;
use chrono::{DateTime, TimeDelta, Utc};
use serde::{Deserialize, Serialize};
use serenity::prelude::TypeMapKey;

use crate::CLIENT;

use super::auth::AccessTokenResponse;
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

/// Response of a token refresh request.
#[derive(Deserialize)]
struct RefreshResponse {
    access_token: String,
    expires_in: i64,
    // ignore the other fields
}

impl AccessToken {
    /// Create an [`AccessToken`] from a [`AccessTokenResponse`] by setting granted to [`Utc::now`].
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
                ("client_id", include_str!("../../../yt_clientid")),
                ("client_secret", include_str!("../../../yt_secret")),
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
