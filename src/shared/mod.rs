use std::collections::HashMap;
use std::sync::Arc;

use poise::serenity_prelude::GuildId;
use thiserror::Error;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

pub const CMD_PREFIX_SIGN: &str = "van!";

pub struct Data {
    pub playlist_cancel_tokens: Arc<RwLock<HashMap<GuildId, CancellationToken>>>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            playlist_cancel_tokens: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed to start client: {0}")]
    ClientStartFailed(String),

    #[error("Failed to create client")]
    ClientCreateFailed,

    #[error("Failed to retrieve DISCORD_TOKEN")]
    DiscordTokenNotFound,
}
