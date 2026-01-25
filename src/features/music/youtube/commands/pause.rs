use super::shared::{MusicYoutubeError, MusicYoutubeMessage};
use crate::shared::{Context, Error};
use songbird::tracks::PlayMode;

#[poise::command(prefix_command, slash_command)]
pub async fn pause(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or(MusicYoutubeError::GuildIdNotFound)?;

    let voice_client = songbird::get(ctx.serenity_context())
        .await
        .ok_or(MusicYoutubeError::SongbirdClientNotFound)?
        .clone();

    if let Some(call) = voice_client.get(guild_id) {
        let handler = call.lock().await;

        if let Some(track) = handler.queue().current() {
            let track_info = track.get_info().await?;

            match track_info.playing {
                PlayMode::Play => {
                    track.pause()?;
                    ctx.say(MusicYoutubeMessage::PAUSED_TRACK).await?;
                }
                _ => {
                    track.play()?;
                    ctx.say(MusicYoutubeMessage::RESUMED_TRACK).await?;
                }
            }
        } else {
            ctx.say(MusicYoutubeError::NoTrackPlaying.to_string())
                .await?;
        }
    } else {
        ctx.say(MusicYoutubeError::BotNotInVoiceChannel.to_string())
            .await?;
    }

    Ok(())
}
