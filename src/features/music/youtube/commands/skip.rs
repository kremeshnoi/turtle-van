use super::shared::Errors;
use crate::features::music::youtube::commands::shared::get_user_voice_channel_id::get_user_voice_channel_id;
use crate::shared::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or(Errors::FAILED_TO_RETRIEVE_GUILD_ID)?;

    if get_user_voice_channel_id(&ctx).is_err() {
        ctx.say("You must be in a voice channel to use this command.")
            .await?;
        return Ok(());
    }

    let voice_client = songbird::get(ctx.serenity_context())
        .await
        .ok_or_else(|| Error::from(Errors::FAILED_TO_RETRIEVE_SONGBIRD_VOICE_CLIENT))?
        .clone();

    let call = match voice_client.get(guild_id) {
        Some(call) => call,
        None => {
            ctx.say("Not currently in a voice channel.").await?;
            return Ok(());
        }
    };

    let handler = call.lock().await;

    match handler.queue().current() {
        Some(track) => {
            track.stop()?;
            ctx.say("Skipped current track.").await?;
        }
        None => {
            ctx.say("No track is currently playing.").await?;
        }
    }

    Ok(())
}
