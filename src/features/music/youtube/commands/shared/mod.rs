pub(crate) mod get_user_voice_channel_id;
pub(crate) mod join_voice_channel;

use serenity::prelude::TypeMapKey;
use songbird::input::AuxMetadata;
use thiserror::Error;

pub struct TrackMetaKey;

impl TypeMapKey for TrackMetaKey {
    type Value = AuxMetadata;
}

#[derive(Error, Debug)]
pub enum MusicYoutubeError {
    #[error("Failed to retrieve Guild ID")]
    GuildIdNotFound,

    #[error("Unable to access guild information")]
    GuildNotFound,

    #[error("You must be in a voice channel to use this command.")]
    UserNotInVoiceChannel,

    #[error("Not currently in a voice channel.")]
    BotNotInVoiceChannel,

    #[error("Failed to join channel")]
    JoinChannelFailed,

    #[error("Failed to leave channel")]
    LeaveChannelFailed,

    #[error("Failed to retrieve Songbird voice client")]
    SongbirdClientNotFound,

    #[error("Failed to retrieve HTTP client")]
    HttpClientNotFound,

    #[error("Failed to execute yt-dlp: {0}")]
    YtDlpExecutionFailed(String),

    #[error("yt-dlp failed: {0}")]
    YtDlpFailed(String),

    #[error("No track is currently playing.")]
    NoTrackPlaying,

    #[error("No videos found in playlist.")]
    NoVideosInPlaylist,

    #[error("The queue is already empty.")]
    QueueEmpty,

    #[error("Need at least 2 tracks in queue to shuffle.")]
    QueueTooShortToShuffle,
}

pub struct MusicYoutubeMessage;

impl MusicYoutubeMessage {
    pub const QUEUE_CLEARED: &'static str = "Cleared track(s) from the queue.";
    pub const QUEUE_CLEARED_AND_LOADING_STOPPED: &'static str =
        "Stopped playlist loading and cleared track(s) from the queue.";
    pub const SKIPPED_TRACK: &'static str = "Skipped current track.";
    pub const PAUSED_TRACK: &'static str = "Paused the current track.";
    pub const RESUMED_TRACK: &'static str = "Resumed the current track.";
    pub const QUEUE_SHUFFLED: &'static str = "Shuffled the queue.";
    pub const NO_METADATA_AVAILABLE: &'static str = "No metadata available for current track.";
    pub const UNKNOWN_TITLE: &'static str = "Unknown Title";
    pub const UNKNOWN_ARTIST: &'static str = "Unknown Artist";
    pub const UNKNOWN_DURATION: &'static str = "Unknown Duration";
    pub const STATE_PLAYING: &'static str = "Playing";
    pub const STATE_PAUSED: &'static str = "Paused";
    pub const STATE_STOPPED: &'static str = "Stopped";
}
