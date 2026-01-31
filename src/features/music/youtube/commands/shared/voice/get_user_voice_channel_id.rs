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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn make_voice_state(user_id: u64, channel_id: Option<u64>) -> VoiceState {
        serde_json::from_value(json!({
            "user_id": user_id.to_string(),
            "channel_id": channel_id.map(|id| id.to_string()),
            "session_id": "",
            "deaf": false,
            "mute": false,
            "self_deaf": false,
            "self_mute": false,
            "suppress": false,
            "self_video": false
        }))
        .unwrap()
    }

    #[test]
    fn test_get_user_voice_channel_id_returns_channel_when_user_in_voice() {
        let user_id = UserId::new(1);
        let expected_channel_id = ChannelId::new(100);
        let mut voice_states = HashMap::new();
        voice_states.insert(user_id, make_voice_state(1, Some(100)));

        let result = get_user_voice_channel_id(&voice_states, user_id);

        assert_eq!(result.unwrap(), expected_channel_id);
    }

    #[test]
    fn test_get_user_voice_channel_id_returns_error_when_user_not_in_voice() {
        let user_id = UserId::new(1);
        let voice_states = HashMap::new();

        let result = get_user_voice_channel_id(&voice_states, user_id);

        assert!(matches!(
            result,
            Err(MusicYoutubeError::UserNotInVoiceChannel)
        ));
    }
}
