pub(crate) mod clear;
pub(crate) mod join;
pub(crate) mod leave;
pub(crate) mod next;
pub(crate) mod now;
pub(crate) mod pause;
pub(crate) mod play;
pub(crate) mod repeat;
pub(crate) mod shared;
pub(crate) mod shuffle;
pub(crate) mod unpause;
pub(crate) mod unshuffle;

use crate::shared::{Data, Error};
use poise::Command;

pub fn commands() -> Vec<Command<Data, Error>> {
    vec![
        join::join(),
        leave::leave(),
        play::play(),
        next::next(),
        clear::clear(),
        pause::pause(),
        unpause::unpause(),
        now::now(),
        shuffle::shuffle(),
        unshuffle::unshuffle(),
        repeat::repeat(),
    ]
}
