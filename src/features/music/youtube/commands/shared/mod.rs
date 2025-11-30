pub(crate) mod get_user_voice_channel_id;
pub(crate) mod join_voice_channel;

pub struct Errors;

impl Errors {
    pub const FAILED_TO_RETRIEVE_GUILD_ID: &'static str = "Failed to retrieve Guild ID";
    pub const FAILED_TO_JOIN_CHANNEL: &'static str = "Failed to join channel";
    pub const FAILED_TO_LEAVE_CHANNEL: &'static str = "Failed to leave channel";
    pub const FAILED_TO_RETRIEVE_SONGBIRD_VOICE_CLIENT: &'static str =
        "Failed to retrieve the Songbird voice client";
    pub const FAILED_TO_RETRIEVE_HTTP: &'static str = "Failed to retrieve HTTP client";
}
