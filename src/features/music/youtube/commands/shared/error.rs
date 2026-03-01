use thiserror::Error;

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

    #[error("No shuffle history available. Use /shuffle first.")]
    NoShuffleHistory,
}
