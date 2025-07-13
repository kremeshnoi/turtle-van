pub(crate) mod join;
pub(crate) mod leave;
pub(crate) mod play;
mod shared;
mod skip;

use crate::shared::{Data, Error};
use poise::Command;

pub fn commands() -> Vec<Command<Data, Error>> {
    vec![join::join(), leave::leave(), play::play(), skip::skip()]
}
