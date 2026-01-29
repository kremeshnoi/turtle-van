use rand::seq::SliceRandom;
use rand::thread_rng;
use tracing::info;

use super::shared::{MusicYoutubeError, MusicYoutubeMessage, VoiceContext};
use crate::shared::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn shuffle(ctx: Context<'_>) -> Result<(), Error> {
    let Some(call) = VoiceContext::validated_call(&ctx).await? else {
        return Ok(());
    };
    let handler = call.lock().await;

    if handler.queue().len() < 2 {
        ctx.say(MusicYoutubeError::QueueTooShortToShuffle.to_string())
            .await?;
        return Ok(());
    }

    info!(count = handler.queue().len(), "Shuffling queue");
    handler.queue().modify_queue(|q| {
        if let Some(current) = q.pop_front() {
            let mut rest: Vec<_> = q.drain(..).collect();
            rest.shuffle(&mut thread_rng());
            q.push_back(current);
            q.extend(rest);
        }
    });

    ctx.say(MusicYoutubeMessage::QUEUE_SHUFFLED).await?;

    Ok(())
}
