#![cfg(feature = "_remote_any")]

/**
* 'access' is about finding, and binding with, remote counterparts over Zigbee.
*
* Such binding happens _between end points_ and thus, this code does not have anything to do
* with the 'Node' struct. We *could* join them (and initially that was the intention) but it only
* adds complexity.
*/

mod matcher;
pub(self) use matcher::Accessor;

mod matching_context;
pub(self) use matching_context::*;

mod match_event;
pub use match_event::MatchEvent;

mod access_dt_color_dimmable_light;
#[cfg(feature = "remote_dt_color_dimmable_light")]
pub use access_dt_color_dimmable_light::*;
