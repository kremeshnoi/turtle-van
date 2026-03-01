use songbird::tracks::PlayMode;
use tracing::info;

use super::shared::{MusicYoutubeError, MusicYoutubeMessage, VoiceContext};
use crate::shared::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn unpause(ctx: Context<'_>) -> Result<(), Error> {
    let Some(call) = VoiceContext::validated_call(&ctx).await? else {
        return Ok(());
    };

    let handler = call.lock().await;

    if let Some(track) = handler.queue().current() {
        match track.get_info().await?.playing {
            PlayMode::Play => {
                ctx.say(MusicYoutubeMessage::TRACK_ALREADY_PLAYING).await?;
            }
            _ => {
                info!("Resuming track");
                track.play()?;
                ctx.say(MusicYoutubeMessage::RESUMED_TRACK).await?;
            }
        }
    } else {
        ctx.say(MusicYoutubeError::NoTrackPlaying.to_string())
            .await?;
    }

    Ok(())
}
