
use super::{
    AccessColorDimmableLight,
    MatchError,
};

use crate::{
    ShortAddr
};

pub enum MatchEvent {
    Bound(AccessColorDimmableLight, ShortAddr, u8),
    Finished,
    Timeout,
    Failed(MatchError)
}
