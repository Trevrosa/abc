use serenity::async_trait;
use songbird::{Event, EventContext};
use tracing::{error, info};

pub struct Voice;

#[async_trait]
impl songbird::EventHandler for Voice {
    async fn act(&self, event: &EventContext<'_>) -> Option<Event> {
        if let EventContext::SpeakingStateUpdate(speaking) = event {
            let Some(user) = speaking.user_id else {
                error!("speaking state received without user");
                return None;
            };
            info!("user {} -> ssrc {} ({:?})", user, speaking.ssrc, speaking);
        }

        None
    }
}
