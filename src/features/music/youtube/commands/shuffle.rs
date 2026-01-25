use rand::seq::SliceRandom;
use rand::thread_rng;

use super::shared::{MusicYoutubeError, MusicYoutubeMessage};
use crate::features::music::youtube::commands::shared::get_user_voice_channel_id::get_user_voice_channel_id;
use crate::shared::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn shuffle(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or(MusicYoutubeError::GuildIdNotFound)?;

    if let Err(e) = get_user_voice_channel_id(&ctx) {
        ctx.say(e.to_string()).await?;
        return Ok(());
    }

    let voice_client = songbird::get(ctx.serenity_context())
        .await
        .ok_or(MusicYoutubeError::SongbirdClientNotFound)?
        .clone();

    let call = match voice_client.get(guild_id) {
        Some(call) => call,
        None => {
            ctx.say(MusicYoutubeError::BotNotInVoiceChannel.to_string())
                .await?;
            return Ok(());
        }
    };

    let handler = call.lock().await;
    let queue = handler.queue();

    if queue.len() < 2 {
        ctx.say(MusicYoutubeError::QueueTooShortToShuffle.to_string())
            .await?;
        return Ok(());
    }

    queue.modify_queue(|q| {
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
