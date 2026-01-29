use async_trait::async_trait;
use poise::serenity_prelude::{ChannelId, Http};
use serenity::prelude::TypeMapKey;
use songbird::events::{Event, EventContext, EventHandler};
use songbird::input::AuxMetadata;
use std::sync::Arc;
use tracing::{error, info};

use super::metadata::TrackDisplayInfo;

pub struct TrackMetaKey;

impl TypeMapKey for TrackMetaKey {
    type Value = AuxMetadata;
}

pub struct TrackPlayNotifier {
    pub channel_id: ChannelId,
    pub http: Arc<Http>,
}

#[async_trait]
impl EventHandler for TrackPlayNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(tracks) = ctx {
            for (_state, handle) in *tracks {
                let typemap = handle.typemap().read().await;
                if let Some(meta) = typemap.get::<TrackMetaKey>() {
                    let track_info = TrackDisplayInfo::from_metadata(meta).message();
                    info!(track = %track_info, "Now playing");

                    let message = format!("Now playing: {track_info}");
                    if let Err(e) = self.channel_id.say(&self.http, &message).await {
                        error!(error = ?e, "Failed to send track event message");
                    }
                }
            }
        }
        None
    }
}
