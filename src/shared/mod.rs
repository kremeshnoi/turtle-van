pub const CMD_PREFIX_SIGN: &str = "van!";

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Errors;

impl Errors {
    pub const FAILED_TO_START_CLIENT: &'static str = "Failed to start client";
    pub const FAILED_TO_CREATE_CLIENT: &'static str = "Failed to create client";
    pub const FAILED_TO_RETRIEVE_DISCORD_TOKEN: &'static str = "Failed to retrieve DISCORD_TOKEN";
}
