#![cfg(feature = "_remote_any")]

use crate::{
    ZdpError
};

use super::{
    Accessor,
};

/**
* Event provided to the application on Zigbee match progress.
*/
pub enum MatchEvent<T : Accessor> {
    /// Found a match. The application gets a typed accessor to communicate with the remote node.
    Found(u8, T),

    /// Something went wrong in the other node. Can log these, or ignore.
    Error(ZdpError)
}
