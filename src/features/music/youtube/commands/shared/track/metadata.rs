use songbird::input::AuxMetadata;
use std::time::Duration;

use super::super::message::MusicYoutubeMessage;

pub fn format_duration(d: Duration) -> String {
    format!("{}:{:02}", d.as_secs() / 60, d.as_secs() % 60)
}

pub struct TrackDisplayInfo {
    pub title: String,
    pub artist: String,
    pub duration: String,
    pub source_url: String,
}

impl TrackDisplayInfo {
    pub fn from_metadata(meta: &AuxMetadata) -> Self {
        Self {
            title: meta
                .title
                .clone()
                .unwrap_or_else(|| MusicYoutubeMessage::UNKNOWN_TITLE.to_string()),
            artist: meta
                .artist
                .clone()
                .unwrap_or_else(|| MusicYoutubeMessage::UNKNOWN_ARTIST.to_string()),
            duration: meta.duration.map_or(
                MusicYoutubeMessage::UNKNOWN_DURATION.to_string(),
                format_duration,
            ),
            source_url: meta.source_url.clone().unwrap_or_default(),
        }
    }

    pub fn summary(&self) -> String {
        format!("{} - {} ({})", self.title, self.artist, self.duration)
    }

    pub fn message(&self) -> String {
        if self.source_url.is_empty() {
            format!("`{}`", self.summary())
        } else {
            format!("`{}`\n{}", self.summary(), self.source_url)
        }
    }
}
