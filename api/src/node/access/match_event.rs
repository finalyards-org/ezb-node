
use super::{
    AccessColorDimmableLight,
    MatchError,
};

use crate::{
    ShortAddr
};

/**
* Event provided to the application on Zigbee match progress.
*/
pub enum MatchEvent {
    /// Bound with a matching node; use '.0' to communicate with it (copy it to keep after binding has finished).
    Bound(dyn AccessColorDimmableLight),
    /// No more events will come.
    Finished{ timeout: bool },
    /// Something went wrong. Can log; continue listening until 'Finished' is reached.
    Failed(MatchError)
}
