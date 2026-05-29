use anyhow::anyhow;
use reqwest::Response;
use serde::Deserialize;
use serde_json::{Value, json};
use tokio::task;
use tracing::{info, warn};

use crate::{CLIENT, utils::ib};

#[derive(Debug, Deserialize)]
struct SolverResponse {
    solution: Solution,
}

#[derive(Debug, Deserialize)]
struct Solution {
    status: u16,
    response: String,
}

pub async fn get(url: impl AsRef<str>) -> anyhow::Result<String> {
    let response = request(
        "request.get",
        json!({"session": "abc", "url": url.as_ref()}),
    )
    .await?;

    let response: SolverResponse = response.json().await?;

    if response.solution.status != 200 {
        return Err(anyhow!("not OK"));
    }

    Ok(response.solution.response)
}

pub async fn setup() -> reqwest::Result<()> {
    if !CLIENT
        .get("http://localhost:8191")
        .send()
        .await?
        .status()
        .is_success()
    {
        panic!("flaresolverr not started!");
    }

    request("sessions.create", json!({"session": "abc"})).await?;

    task::spawn(async {
        if let Ok(mirrors) = ib::get_mirrors().await
            && !mirrors.is_empty()
            && get(&mirrors[0].url).await.is_ok()
        {
            info!("prepared flaresolverr");
        } else {
            warn!("failed to prepare flaresolverr");
        }
    });

    Ok(())
}

pub async fn destroy_session() -> reqwest::Result<Response> {
    request("sessions.destroy", json!({"session": "abc"})).await
}

async fn request(
    cmd: impl Into<String>,
    mut params: serde_json::Value,
) -> reqwest::Result<Response> {
    params
        .as_object_mut()
        .expect("should be object")
        .insert("cmd".to_string(), Value::String(cmd.into()));
    CLIENT
        .post("http://localhost:8191/v1")
        .json(&params)
        .send()
        .await
}
