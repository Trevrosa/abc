use std::{io::ErrorKind, path::Path};

use tracing::{error, info};

/// Like [`std::ops::Index`], but we don't have to return a `ref` of the `Output`.
pub trait Get<Idx, Output> {
    fn get(&self, index: Idx) -> Option<Output>;
    fn get_or<Idx2>(&self, index: Idx, index2: Idx2) -> Option<Output>
    where
        Self: Get<Idx2, Output>,
    {
        let a = <Self as Get<Idx, Output>>::get(self, index);
        let b = <Self as Get<Idx2, Output>>::get(self, index2);

        a.or(b)
    }
}

/// A guard struct that removes `self.path` after `self` is [`drop()`]ped.
pub struct DeleteWhenDone<'a> {
    path: &'a Path,
}

impl<'a> DeleteWhenDone<'a> {
    #[must_use = "this should be set to a variable or else the path will instantly be deleted"]
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
