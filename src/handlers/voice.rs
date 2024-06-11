use serenity::async_trait;
use songbird::{Event, EventContext};
use tracing::{error, info};

pub struct VoiceHandler;

#[async_trait]
impl songbird::EventHandler for VoiceHandler {
    async fn act(&self, event: &EventContext<'_>) -> Option<Event> {
        match event {
            EventContext::SpeakingStateUpdate(speaking) => {
                let Some(user) = speaking.user_id else {
                    error!("speaking state received without user");
                    return None;
                };
                info!("user {} -> ssrc {} ({:?})", user, speaking.ssrc, speaking);
            }
            &_ => (),
        }

        None
    }
}
