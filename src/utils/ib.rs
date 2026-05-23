use scraper::{Html, Selector};
use serde::Deserialize;

use crate::CLIENT;

pub fn page_files(html: &Html) -> Vec<&str> {
    let mut files = Vec::new();

    let selector = Selector::parse(".path.cell-name").unwrap();
    let names = html.select(&selector);
    for name in names {
        // \n\t name \n
        let Some(name) = name.text().nth(1) else {
            continue;
        };
        files.push(name);
    }

    files
}

#[derive(Deserialize)]
struct Mirrors {
    mirrors: Vec<Mirror>,
}

#[derive(Debug, Deserialize)]
pub struct Mirror {
    pub name: String,
    pub url: String,
    pub status: String,
    // uptime: f32,
}

pub async fn get_mirrors() -> anyhow::Result<Vec<Mirror>> {
    const API: &str = "https://ibresources.cc/api/v3/mirrors";

    let mirrors = CLIENT.get(API).send().await?;

    Ok(mirrors.json::<Mirrors>().await?.mirrors)
}
