pub(crate) mod join;
pub(crate) mod leave;
pub(crate) mod play;
pub(crate) mod shared;
pub(crate) mod skip;
pub(crate) mod pause;
pub(crate) mod now;

use crate::shared::{Data, Error};
use poise::Command;

pub fn commands() -> Vec<Command<Data, Error>> {
    vec![join::join(), leave::leave(), play::play(), skip::skip(), pause::pause(), now::now()]
}
