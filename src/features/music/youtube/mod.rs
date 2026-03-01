pub(crate) mod commands;

use std::collections::HashMap;
use std::sync::Arc;

use poise::serenity_prelude::GuildId;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

pub struct MusicYoutubeState {
    pub playlist_cancel_tokens: Arc<RwLock<HashMap<GuildId, CancellationToken>>>,
    pub pre_shuffle_order: Arc<RwLock<HashMap<GuildId, Vec<Uuid>>>>,
}

impl MusicYoutubeState {
    pub fn new() -> Self {
        Self {
            playlist_cancel_tokens: Arc::new(RwLock::new(HashMap::new())),
            pre_shuffle_order: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for MusicYoutubeState {
    fn default() -> Self {
        Self::new()
    }
}
