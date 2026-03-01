use crate::features::music::youtube::MusicYoutubeState;

pub struct Data {
    pub http_client: reqwest::Client,
    pub music_youtube: MusicYoutubeState,
}

impl Data {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
            music_youtube: MusicYoutubeState::new(),
        }
    }
}

impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}

pub type Error = anyhow::Error;
pub type Context<'a> = poise::Context<'a, Data, Error>;
