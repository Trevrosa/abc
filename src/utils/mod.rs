use std::{io::ErrorKind, path::Path};

use reply::CreateReply;
use serenity::all::CreateEmbed;

mod internal;

pub mod reply;
pub use reply::Replyer;

mod arg;
pub use arg::{Arg, ArgValue, Args, Get, Is};
use tracing::{error, info};

pub mod context;
pub mod sniping;
pub mod status;

pub mod spotify;
pub mod ytmusic;

mod yt_dlp;

pub fn embed_message<S: Into<String>>(title: S, image: S, desc: Option<S>) -> CreateReply {
    let mut embed = CreateEmbed::new().title(title).image(image);
    if let Some(desc) = desc {
        embed = embed.description(desc);
    }
    CreateReply::new().embed(embed)
}

/// A guard struct that removes `self.path` after `self` is [`drop()`]ped.
pub struct DeleteWhenDone<'a> {
    path: &'a Path,
}

impl<'a> DeleteWhenDone<'a> {
    pub fn new(path: &'a Path) -> Self {
        Self { path }
    }
}

impl Drop for DeleteWhenDone<'_> {
    fn drop(&mut self) {
        let path = self.path.to_owned();
        tokio::task::spawn_blocking(move || {
            if path.is_dir() {
                if let Err(err) = std::fs::remove_dir_all(&path) {
                    // we don't care if `path` wasn't found.
                    if err.kind() != ErrorKind::NotFound {
                        error!("failed to clean {path:?}: {err:#?}");
                    }
                } else {
                    info!("cleaned path {path:?}");
                }
            } else if let Err(err) = std::fs::remove_file(&path) {
                // we don't care if `path` wasn't found.
                if err.kind() == ErrorKind::NotFound {
                    info!("cleaned {path:?}");
                } else {
                    error!("failed to delete {path:?}: {err:#?}");
                }
            }
        });
    }
}
