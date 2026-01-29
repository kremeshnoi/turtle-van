use std::collections::HashMap;

use poise::serenity_prelude::{ChannelId, UserId, VoiceState};

use super::super::error::MusicYoutubeError;

pub fn get_user_voice_channel_id(
    voice_states: &HashMap<UserId, VoiceState>,
    user_id: UserId,
) -> Result<ChannelId, MusicYoutubeError> {
    voice_states
        .get(&user_id)
        .and_then(|state| state.channel_id)
        .ok_or(MusicYoutubeError::UserNotInVoiceChannel)
}
