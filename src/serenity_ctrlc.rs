use std::{
    future::Future,
    sync::{Arc, Weak},
};

use serenity::{gateway::ShardManager, Client};

// mostly taken from https://github.com/yehuthi/serenity_ctrlc/, changed to work with serenity 0.12.1
#[derive(Debug)]
#[repr(transparent)]
pub struct Disconnector {
    shard_manager: Arc<ShardManager>,
}

impl Disconnector {
    /// Creates a [`Disconnector`] [`Option`] from a [`Weak`] [`ShardManager`].
    ///
    /// Returns [`None`] if the [`ShardManager`] has been already dropped.
    fn from_weak_shard_manager(shard_manager: &Weak<ShardManager>) -> Option<Self> {
        Some(Self {
            shard_manager: shard_manager.upgrade()?,
        })
    }

    /// Disconnects the bot.
    pub async fn disconnect(self) {
        self.shard_manager.shutdown_all().await;
    }

    /// Disconnects the bot when there is [`Some`] [`Disconnector`].
    pub async fn disconnect_some(disconnector: Option<Self>) {
        if let Some(disconnector) = disconnector {
            disconnector.disconnect().await;
        }
    }
}

pub fn ctrlc_with<F: Future + Send>(
    client: &Client,
    mut f: impl (FnMut(Option<Disconnector>) -> F) + Send + 'static,
) -> Result<(), ctrlc::Error> {
    let rt = tokio::runtime::Handle::current();
    let shard_manager = Arc::downgrade(&client.shard_manager);
    ctrlc::set_handler(move || {
        let disconnect = Disconnector::from_weak_shard_manager(&shard_manager);
        rt.block_on(async {
            f(disconnect).await;
        });
    })
}

pub fn ctrlc(client: &Client) -> Result<(), ctrlc::Error> {
    ctrlc_with(client, Disconnector::disconnect_some)
}
