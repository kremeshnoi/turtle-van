use songbird::tracks::LoopState;
use tracing::info;

use super::shared::{MusicYoutubeError, MusicYoutubeMessage, VoiceContext};
use crate::shared::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn repeat(ctx: Context<'_>) -> Result<(), Error> {
    let Some(call) = VoiceContext::validated_call(&ctx).await? else {
        return Ok(());
    };

    let handler = call.lock().await;

    if let Some(track) = handler.queue().current() {
        match track.get_info().await?.loops {
            LoopState::Infinite => {
                info!("Disabling repeat");
                track.disable_loop()?;
                ctx.say(MusicYoutubeMessage::REPEAT_DISABLED).await?;
            }
            _ => {
                info!("Enabling repeat");
                track.enable_loop()?;
                ctx.say(MusicYoutubeMessage::REPEAT_ENABLED).await?;
            }
        }
    } else {
        ctx.say(MusicYoutubeError::NoTrackPlaying.to_string())
            .await?;
    }

    Ok(())
}
