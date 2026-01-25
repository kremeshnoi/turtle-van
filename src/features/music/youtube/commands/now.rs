use super::shared::{MusicYoutubeError, MusicYoutubeMessage, TrackMetaKey};
use crate::shared::{Context, Error};
use songbird::tracks::PlayMode;

#[poise::command(prefix_command, slash_command)]
pub async fn now(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or(MusicYoutubeError::GuildIdNotFound)?;

    let voice_client = songbird::get(ctx.serenity_context())
        .await
        .ok_or(MusicYoutubeError::SongbirdClientNotFound)?
        .clone();

    if let Some(call) = voice_client.get(guild_id) {
        let handler = call.lock().await;

        if let Some(track) = handler.queue().current() {
            let track_info = track.get_info().await?;
            let metadata = track.typemap().read().await.get::<TrackMetaKey>().cloned();

            let state = match track_info.playing {
                PlayMode::Play => MusicYoutubeMessage::STATE_PLAYING,
                PlayMode::Pause => MusicYoutubeMessage::STATE_PAUSED,
                _ => MusicYoutubeMessage::STATE_STOPPED,
            };

            if let Some(metadata) = metadata {
                let title = metadata
                    .title
                    .unwrap_or_else(|| MusicYoutubeMessage::UNKNOWN_TITLE.to_string());
                let artist = metadata
                    .artist
                    .unwrap_or_else(|| MusicYoutubeMessage::UNKNOWN_ARTIST.to_string());
                let duration = metadata
                    .duration
                    .map_or(MusicYoutubeMessage::UNKNOWN_DURATION.to_string(), |d| {
                        format!("{}:{:02}", d.as_secs() / 60, d.as_secs() % 60)
                    });

                ctx.say(format!(
                    "{state}\n **{title}**\n {artist}\n Duration: {duration}"
                ))
                .await?;
            } else {
                ctx.say(format!(
                    "{state}\n{}",
                    MusicYoutubeMessage::NO_METADATA_AVAILABLE
                ))
                .await?;
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
