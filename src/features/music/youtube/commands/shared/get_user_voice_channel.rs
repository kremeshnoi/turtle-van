use poise::serenity_prelude::ChannelId;
use thiserror::Error;
use crate::shared::{Context};

#[derive(Error, Debug)]
pub enum VoiceChannelError {
    #[error("You must be in a voice channel to use this command")]
    UserNotInVoiceChannel,
    #[error("Unable to access guild information")]
    GuildNotFound,
}

pub fn get_user_voice_channel(ctx: &Context<'_>) -> Result<ChannelId, VoiceChannelError> {
    let guild = ctx.guild().ok_or(VoiceChannelError::GuildNotFound)?;

    guild.voice_states
        .get(&ctx.author().id)
        .and_then(|state| state.channel_id)
        .ok_or(VoiceChannelError::UserNotInVoiceChannel)
}