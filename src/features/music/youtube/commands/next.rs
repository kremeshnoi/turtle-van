use tracing::{debug, info};

use super::shared::{MusicYoutubeError, MusicYoutubeMessage, VoiceContext};
use crate::shared::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn next(ctx: Context<'_>) -> Result<(), Error> {
    let Some(call) = VoiceContext::validated_call(&ctx).await? else {
        return Ok(());
    };

    let handler = call.lock().await;
    debug!(queue_length = handler.queue().len(), "Queue state");

    match handler.queue().current() {
        Some(track) => {
            info!("Skipping current track");
            track.stop()?;
            ctx.say(MusicYoutubeMessage::NEXT_TRACK).await?;
        }
        None => {
            ctx.say(MusicYoutubeError::NoTrackPlaying.to_string())
                .await?;
        }
    }

    Ok(())
}
