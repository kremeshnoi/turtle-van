pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub const CMD_PREFIX_SIGN: &str = "van!";

pub struct Errors;

impl Errors {
    pub const FAILED: &'static str = "Failed";

    // Discord related
    pub const FAILED_TO_START_CLIENT: &'static str = "Failed to start client";
    pub const FAILED_TO_CREATE_CLIENT: &'static str = "Failed to create client";
    pub const FAILED_TO_RETRIEVE_GUILD_ID: &'static str = "Failed to retrieve Guild ID";
    pub const FAILED_TO_RETRIEVE_DISCORD_TOKEN: &'static str = "Failed to retrieve DISCORD_TOKEN";

    // Voice related
    pub const FAILED_TO_JOIN_CHANNEL: &'static str = "Failed to join channel";
    pub const FAILED_TO_LEAVE_CHANNEL: &'static str = "Failed to leave channel";
    pub const FAILED_TO_RETRIEVE_SONGBIRD_VOICE_CLIENT: &'static str =
        "Failed to retrieve the Songbird voice client";

    // Media related
    pub const FAILED_TO_PROVIDE_URL_TO_SONG_QUERY: &'static str =
        "Failed to provide a URL or song query";

    // HTTP related
    pub const FAILED_TO_RETRIEVE_HTTP: &'static str = "Failed to retrieve HTTP client";
}
