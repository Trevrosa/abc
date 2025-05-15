// mostly taken from https://github.com/yehuthi/serenity_ctrlc/, changed to work with serenity 0.12.1 and for my own purposes

use std::{
    future::Future,
    sync::{Arc, Weak},
};

use serenity::{gateway::ShardManager, prelude::TypeMap, Client};
use tokio::sync::RwLock;

pub struct Disconnector {
    shard_manager: Arc<ShardManager>,
    pub data: Arc<RwLock<TypeMap>>,
}

impl Disconnector {
    /// Creates a [`Disconnector`] [`Option`] from a [`Weak`] [`ShardManager`].
    ///
    /// Returns [`None`] if the [`ShardManager`] has been already dropped.
    fn new(shard_manager: &Weak<ShardManager>, data: &Weak<RwLock<TypeMap>>) -> Option<Self> {
        Some(Self {
            shard_manager: shard_manager.upgrade()?,
            data: data.upgrade()?,
        })
    }

    /// Disconnects the bot.
    pub async fn disconnect(self) {
        self.shard_manager.shutdown_all().await;
    }

    // Disconnects the bot when there is [`Some`] [`Disconnector`].
    // pub async fn disconnect_some(disconnector: Option<Self>) {
    //     if let Some(disconnector) = disconnector {
    //         disconnector.disconnect().await;
    //     }
    // }
}

pub fn ctrlc_with<F: Future + Send>(
    client: &Client,
    mut f: impl (FnMut(Option<Disconnector>) -> F) + Send + 'static,
) -> Result<(), ctrlc::Error> {
    let rt = tokio::runtime::Handle::current();

    let shard_manager = Arc::downgrade(&client.shard_manager);
    let data = Arc::downgrade(&client.data);

    ctrlc::set_handler(move || {
        let disconnect = Disconnector::new(&shard_manager, &data);
        rt.block_on(async {
            f(disconnect).await;
        });
    })
}

// pub fn ctrlc(client: &Client) -> Result<(), ctrlc::Error> {
//     ctrlc_with(client, Disconnector::disconnect_some)
// }
