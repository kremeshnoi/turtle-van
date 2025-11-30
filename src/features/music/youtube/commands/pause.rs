use super::shared::Errors;
use crate::shared::{Context, Error};
use songbird::tracks::PlayMode;

#[poise::command(prefix_command, slash_command)]
pub async fn pause(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or(Errors::FAILED_TO_RETRIEVE_GUILD_ID)?;

    let voice_client = songbird::get(ctx.serenity_context())
        .await
        .ok_or_else(|| Error::from(Errors::FAILED_TO_RETRIEVE_SONGBIRD_VOICE_CLIENT))?
        .clone();

    if let Some(call) = voice_client.get(guild_id) {
        let handler = call.lock().await;

        if let Some(track) = handler.queue().current() {
            let track_info = track.get_info().await?;

            match track_info.playing {
                PlayMode::Play => {
                    track.pause()?;
                    ctx.say("Paused the current track").await?;
                }
                _ => {
                    track.play()?;
                    ctx.say("Resumed the current track").await?;
                }
            }
        } else {
            ctx.say("No track is currently playing").await?;
        }
    } else {
        ctx.say("Not currently in a voice channel").await?;
    }

    Ok(())
}
