pub(crate) mod youtube;

use crate::shared::{Data, Error};
use poise::Command;

pub fn commands() -> Vec<Command<Data, Error>> {
    let mut cmds = Vec::new();
    cmds.extend(youtube::commands::commands());
    cmds
}
