use tracing::info;

use super::shared::{MusicYoutubeError, MusicYoutubeMessage, VoiceContext};
use crate::shared::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn unshuffle(ctx: Context<'_>) -> Result<(), Error> {
    let Some(call) = VoiceContext::validated_call(&ctx).await? else {
        return Ok(());
    };

    let vc = VoiceContext::from_ctx(&ctx).await?;

    let saved_order = ctx
        .data()
        .music_youtube
        .pre_shuffle_order
        .write()
        .await
        .remove(&vc.guild_id);

    let Some(saved_order) = saved_order else {
        ctx.say(MusicYoutubeError::NoShuffleHistory.to_string())
            .await?;
        return Ok(());
    };

    let handler = call.lock().await;

    info!(
        count = handler.queue().len(),
        "Restoring original queue order"
    );
    handler.queue().modify_queue(|q| {
        if let Some(current) = q.pop_front() {
            let mut rest: Vec<_> = q.drain(..).collect();
            rest.sort_by_key(|track| {
                saved_order
                    .iter()
                    .position(|id| *id == track.uuid())
                    .unwrap_or(usize::MAX)
            });
            q.push_back(current);
            q.extend(rest);
        }
    });

    ctx.say(MusicYoutubeMessage::QUEUE_UNSHUFFLED).await?;

    Ok(())
}
