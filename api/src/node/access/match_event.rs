#![cfg(feature = "_match_any")]

use super::{
    Accessor,
};

use crate::{
    ZdpError
};

/**
* Event provided to the application on Zigbee match progress.
*/
pub enum MatchEvent<T : Accessor + Copy + Clone> {
    /// Bound with a matching node; use '.0' to communicate with it.
    Bound(T),
    /// Something went wrong in the other node. Can log these, or ignore.
    Error(ZdpError)
}
