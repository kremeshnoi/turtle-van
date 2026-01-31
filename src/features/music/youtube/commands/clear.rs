use tracing::info;

use super::shared::{MusicYoutubeError, MusicYoutubeMessage, VoiceContext};
use crate::shared::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn clear(ctx: Context<'_>) -> Result<(), Error> {
    let Some(call) = VoiceContext::validated_call(&ctx).await? else {
        return Ok(());
    };

    let vc = VoiceContext::from_ctx(&ctx).await?;
    let loading_cancelled = {
        let tokens = ctx.data().playlist_cancel_tokens.read().await;
        match tokens.get(&vc.guild_id) {
            Some(token) => {
                token.cancel();
                true
            }
            None => false,
        }
    };

    let handler = call.lock().await;
    let count = handler.queue().len();
    info!(count, "Clearing queue");
    handler.queue().stop();

    let message = if count == 0 && !loading_cancelled {
        MusicYoutubeError::QueueEmpty.to_string()
    } else if loading_cancelled {
        MusicYoutubeMessage::QUEUE_CLEARED_AND_LOADING_STOPPED.to_string()
    } else {
        MusicYoutubeMessage::QUEUE_CLEARED.to_string()
    };

    ctx.say(message).await?;

    Ok(())
}
