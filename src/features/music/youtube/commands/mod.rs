pub(crate) mod join;
pub(crate) mod leave;
pub(crate) mod now;
pub(crate) mod pause;
pub(crate) mod play;
pub(crate) mod shared;
pub(crate) mod skip;

use crate::shared::{Data, Error};
use poise::Command;

pub fn commands() -> Vec<Command<Data, Error>> {
    vec![
        join::join(),
        leave::leave(),
        play::play(),
        skip::skip(),
        pause::pause(),
        now::now(),
    ]
}
