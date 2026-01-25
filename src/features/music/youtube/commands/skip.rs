use super::shared::{MusicYoutubeError, MusicYoutubeMessage};
use crate::features::music::youtube::commands::shared::get_user_voice_channel_id::get_user_voice_channel_id;
use crate::shared::{Context, Error};

#[poise::command(prefix_command, slash_command, subcommands("all"))]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
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

    match handler.queue().current() {
        Some(track) => {
            track.stop()?;
            ctx.say(MusicYoutubeMessage::SKIPPED_TRACK).await?;
        }
        None => {
            ctx.say(MusicYoutubeError::NoTrackPlaying.to_string())
                .await?;
        }
    }

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn all(ctx: Context<'_>) -> Result<(), Error> {
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

    let mut loading_cancelled = false;
    {
        let tokens = ctx.data().playlist_cancel_tokens.read().await;
        if let Some(token) = tokens.get(&guild_id) {
            token.cancel();
            loading_cancelled = true;
        }
    }

    let handler = call.lock().await;
    let queue = handler.queue();

    let count = queue.len();
    queue.stop();

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
