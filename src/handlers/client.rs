use serenity::{
    all::{ActivityData, Context, EventHandler, OnlineStatus, Ready},
    async_trait,
};
use tracing::info;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        ctx.set_presence(
            Some(ActivityData::custom("Disrupting the Social Democrats")),
            OnlineStatus::DoNotDisturb,
        );

        info!("successfully set status");
    }
}
