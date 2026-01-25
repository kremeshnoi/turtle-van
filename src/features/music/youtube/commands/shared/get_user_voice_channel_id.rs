use super::MusicYoutubeError;
use crate::shared::Context;
use poise::serenity_prelude::ChannelId;

pub fn get_user_voice_channel_id(ctx: &Context<'_>) -> Result<ChannelId, MusicYoutubeError> {
    let guild = ctx.guild().ok_or(MusicYoutubeError::GuildNotFound)?;

    guild
        .voice_states
        .get(&ctx.author().id)
        .and_then(|state| state.channel_id)
        .ok_or(MusicYoutubeError::UserNotInVoiceChannel)
}
