use songbird::tracks::PlayMode;
use tracing::debug;

use super::shared::{
    MusicYoutubeError, MusicYoutubeMessage, TrackDisplayInfo, TrackMetaKey, VoiceContext,
};
use crate::shared::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn now(ctx: Context<'_>) -> Result<(), Error> {
    let Some(call) = VoiceContext::validated_call(&ctx).await? else {
        return Ok(());
    };
    let handler = call.lock().await;
    debug!(
        queue_length = handler.queue().len(),
        has_current = handler.queue().current().is_some(),
        "Queue state",
    );

    let Some(track) = handler.queue().current() else {
        ctx.say(MusicYoutubeError::NoTrackPlaying.to_string())
            .await?;
        return Ok(());
    };

    let track_info = track.get_info().await?;
    debug!(state = ?track_info.playing, "Track state");

    let state = match track_info.playing {
        PlayMode::Play => MusicYoutubeMessage::STATE_PLAYING,
        PlayMode::Pause => MusicYoutubeMessage::STATE_PAUSED,
        _ => MusicYoutubeMessage::STATE_STOPPED,
    };

    let message = match track.typemap().read().await.get::<TrackMetaKey>() {
        Some(meta) => format!(
            "{state}: {}",
            TrackDisplayInfo::from_metadata(meta).message()
        ),
        None => format!("{state}\n{}", MusicYoutubeMessage::NO_METADATA_AVAILABLE),
    };

    ctx.say(message).await?;

    Ok(())
}
