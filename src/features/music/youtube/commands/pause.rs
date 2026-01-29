use songbird::tracks::PlayMode;
use tracing::{debug, info};

use super::shared::{MusicYoutubeError, MusicYoutubeMessage, VoiceContext};
use crate::shared::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn pause(ctx: Context<'_>) -> Result<(), Error> {
    let Some(call) = VoiceContext::validated_call(&ctx).await? else {
        return Ok(());
    };

    let handler = call.lock().await;

    debug!(
        queue_length = handler.queue().len(),
        has_current = handler.queue().current().is_some(),
        "Queue state",
    );

    if let Some(track) = handler.queue().current() {
        match track.get_info().await?.playing {
            PlayMode::Play => {
                info!("Pausing track");
                track.pause()?;
                ctx.say(MusicYoutubeMessage::PAUSED_TRACK).await?;
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
