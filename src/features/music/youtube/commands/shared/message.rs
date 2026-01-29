pub struct MusicYoutubeMessage;

impl MusicYoutubeMessage {
    pub const QUEUE_CLEARED: &'static str = "Cleared track(s) from the queue.";
    pub const QUEUE_CLEARED_AND_LOADING_STOPPED: &'static str =
        "Stopped playlist loading and cleared track(s) from the queue.";
    pub const SKIPPED_TRACK: &'static str = "Skipped current track.";
    pub const PAUSED_TRACK: &'static str = "Paused the current track.";
    pub const RESUMED_TRACK: &'static str = "Resumed the current track.";
    pub const JOINED_CHANNEL: &'static str = "Joined the voice channel.";
    pub const LEFT_CHANNEL: &'static str = "Left the voice channel.";
    pub const QUEUE_SHUFFLED: &'static str = "Shuffled the queue.";
    pub const NO_METADATA_AVAILABLE: &'static str = "No metadata available for current track.";
    pub const UNKNOWN_TITLE: &'static str = "Unknown Title";
    pub const UNKNOWN_ARTIST: &'static str = "Unknown Artist";
    pub const UNKNOWN_DURATION: &'static str = "Unknown Duration";
    pub const STATE_PLAYING: &'static str = "Playing";
    pub const STATE_PAUSED: &'static str = "Paused";
    pub const STATE_STOPPED: &'static str = "Stopped";
}
