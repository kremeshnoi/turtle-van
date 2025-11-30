use songbird::Songbird;
use std::sync::Arc;

use super::shared::Errors;
use crate::shared::{Context, Error};

async fn leave_channel(ctx: Context<'_>, voice_client: &Arc<Songbird>) -> Result<(), Error> {
    match voice_client
        .remove(ctx.guild_id().expect(Errors::FAILED_TO_RETRIEVE_GUILD_ID))
        .await
    {
        Ok(_) => Ok(()),
        Err(error) => {
            ctx.say(format!("{}: {}", Errors::FAILED_TO_LEAVE_CHANNEL, error))
                .await?;
            Err(Error::from(error))
        }
    }
}

#[poise::command(prefix_command, slash_command)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let voice_client = songbird::get(ctx.serenity_context())
        .await
        .expect(Errors::FAILED_TO_RETRIEVE_SONGBIRD_VOICE_CLIENT)
        .clone();

    let has_voice_client = voice_client
        .get(ctx.guild_id().expect(Errors::FAILED_TO_RETRIEVE_GUILD_ID))
        .is_some();

    if has_voice_client {
        leave_channel(ctx, &voice_client).await?;
    } else {
        ctx.say(Errors::FAILED_TO_LEAVE_CHANNEL).await?;
    }

    Ok(())
}
