use songbird::Songbird;
use std::sync::Arc;

use crate::consts::{
    FAILED, FAILED_TO_LEAVE_CHANNEL, FAILED_TO_RETRIEVE_GUILD_ID,
    FAILED_TO_RETRIEVE_SONGBIRD_VOICE_CLIENT,
};

use crate::types::{Context, Error};

async fn leave_channel(ctx: Context<'_>, voice_client: &Arc<Songbird>) -> Result<(), Error> {
    if let Err(error) = voice_client
        .remove(ctx.guild_id().expect(FAILED_TO_RETRIEVE_GUILD_ID))
        .await
    {
        ctx.say(format!("{}: {}", FAILED, error)).await?;
    }

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let voice_client = songbird::get(ctx.serenity_context())
        .await
        .expect(FAILED_TO_RETRIEVE_SONGBIRD_VOICE_CLIENT)
        .clone();

    let has_voice_client = voice_client
        .get(ctx.guild_id().expect(FAILED_TO_RETRIEVE_GUILD_ID))
        .is_some();

    if has_voice_client {
        leave_channel(ctx, &voice_client).await?;
    } else {
        ctx.say(FAILED_TO_LEAVE_CHANNEL).await?;
    }

    Ok(())
}
