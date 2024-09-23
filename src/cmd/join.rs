use crate::types::{Error, Context};

use crate::consts::{
    FAILED_TO_JOIN_CHANNEL,
    FAILED_TO_RETRIEVE_GUILD_ID,
    FAILED_TO_RETRIEVE_SONGBIRD_VOICE_CLIENT
};

use crate::cmd::shared::join_voice_channel::join_voice_channel;
use crate::cmd::shared::get_user_voice_channel::get_user_voice_channel;

#[poise::command(prefix_command, slash_command)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    let voice_client = songbird::get(ctx.serenity_context())
        .await
        .ok_or(FAILED_TO_RETRIEVE_SONGBIRD_VOICE_CLIENT)?
        .clone();
    
    let guild_id = ctx.guild_id().ok_or(FAILED_TO_RETRIEVE_GUILD_ID)?;
    
    match get_user_voice_channel(&ctx) {
        Some(channel) => {
            join_voice_channel(&voice_client, guild_id, channel).await?;
        }
        None => {
            ctx.say(FAILED_TO_JOIN_CHANNEL).await?;
        }
    }

    Ok(())
}