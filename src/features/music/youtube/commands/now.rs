use super::shared::Errors;
use crate::shared::{Context, Error};
use serenity::prelude::TypeMapKey;
use songbird::input::AuxMetadata;
use songbird::tracks::PlayMode;

struct TrackMetaKey;

impl TypeMapKey for TrackMetaKey {
    type Value = AuxMetadata;
}

#[poise::command(prefix_command, slash_command)]
pub async fn now(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or(Errors::FAILED_TO_RETRIEVE_GUILD_ID)?;

    let voice_client = songbird::get(ctx.serenity_context())
        .await
        .ok_or_else(|| Error::from(Errors::FAILED_TO_RETRIEVE_SONGBIRD_VOICE_CLIENT))?
        .clone();

    if let Some(call) = voice_client.get(guild_id) {
        let handler = call.lock().await;

        if let Some(track) = handler.queue().current() {
            let track_info = track.get_info().await?;
            let metadata = track.typemap().read().await.get::<TrackMetaKey>().cloned();

            let state = match track_info.playing {
                PlayMode::Play => "Playing",
                PlayMode::Pause => "Paused",
                _ => "Stopped",
            };

            if let Some(metadata) = metadata {
                let title = metadata
                    .title
                    .unwrap_or_else(|| "Unknown Title".to_string());
                let artist = metadata
                    .artist
                    .unwrap_or_else(|| "Unknown Artist".to_string());
                let duration = metadata
                    .duration
                    .map_or("Unknown Duration".to_string(), |d| {
                        format!("{}:{:02}", d.as_secs() / 60, d.as_secs() % 60)
                    });

                ctx.say(format!(
                    "{state}\n🎵 **{title}**\n👤 {artist}\n⏱️ Duration: {duration}"
                ))
                .await?;
            } else {
                ctx.say(format!(
                    "{state}\nℹ️ No metadata available for current track"
                ))
                .await?;
            }
        } else {
            ctx.say("ℹ️ No track is currently playing").await?;
        }
    } else {
        ctx.say("❌ Not currently in a voice channel").await?;
    }

    Ok(())
}
