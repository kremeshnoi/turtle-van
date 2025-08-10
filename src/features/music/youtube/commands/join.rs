use crate::shared::{Context, Error};

use super::shared::Errors;
use super::shared::{get_user_voice_channel::get_user_voice_channel, join_voice_channel::join_voice_channel};

#[poise::command(prefix_command, slash_command)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    let serenity_ctx = ctx.serenity_context();

    let voice_client = songbird::get(serenity_ctx)
        .await
        .ok_or(Errors::FAILED_TO_RETRIEVE_SONGBIRD_VOICE_CLIENT)?
        .clone();

    let guild_id = ctx.guild_id().ok_or(Errors::FAILED_TO_RETRIEVE_GUILD_ID)?;

    let channel_id = match get_user_voice_channel(&ctx) {
        Ok(id) => {
            // ctx.say("Retrieved voice channel").await?;
            id
        }
        Err(_) => {
            // ctx.say(Errors::FAILED_TO_JOIN_CHANNEL).await?;
            return Ok(());
        }
    };

    // ctx.say("Connecting to your voice channel...").await?;
    join_voice_channel(&voice_client, guild_id, channel_id).await?;

    Ok(())
}
