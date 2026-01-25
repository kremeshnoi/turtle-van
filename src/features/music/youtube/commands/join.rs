use crate::shared::{Context, Error};

use super::shared::MusicYoutubeError;
use super::shared::{
    get_user_voice_channel_id::get_user_voice_channel_id, join_voice_channel::join_voice_channel,
};

#[poise::command(prefix_command, slash_command)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    let serenity_ctx = ctx.serenity_context();

    let voice_client = songbird::get(serenity_ctx)
        .await
        .ok_or(MusicYoutubeError::SongbirdClientNotFound)?
        .clone();

    let guild_id = ctx.guild_id().ok_or(MusicYoutubeError::GuildIdNotFound)?;

    let channel_id = match get_user_voice_channel_id(&ctx) {
        Ok(id) => id,
        Err(_) => {
            return Err(Error::from(MusicYoutubeError::JoinChannelFailed));
        }
    };

    join_voice_channel(&voice_client, guild_id, channel_id).await?;

    Ok(())
}
