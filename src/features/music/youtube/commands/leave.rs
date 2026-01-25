use songbird::Songbird;
use std::sync::Arc;

use super::shared::MusicYoutubeError;
use crate::shared::{Context, Error};

async fn leave_channel(ctx: Context<'_>, voice_client: &Arc<Songbird>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or(MusicYoutubeError::GuildIdNotFound)?;

    voice_client
        .remove(guild_id)
        .await
        .map_err(|e| Error::from(format!("{}: {e}", MusicYoutubeError::LeaveChannelFailed)))
}

#[poise::command(prefix_command, slash_command)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let voice_client = songbird::get(ctx.serenity_context())
        .await
        .ok_or(MusicYoutubeError::SongbirdClientNotFound)?
        .clone();

    let guild_id = ctx.guild_id().ok_or(MusicYoutubeError::GuildIdNotFound)?;

    let has_voice_client = voice_client.get(guild_id).is_some();

    if has_voice_client {
        leave_channel(ctx, &voice_client).await?;
    } else {
        return Err(Error::from(MusicYoutubeError::LeaveChannelFailed));
    }

    Ok(())
}
