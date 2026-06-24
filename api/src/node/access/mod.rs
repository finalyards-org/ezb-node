#![cfg(feature = "_match_any")]

mod access_color_dimmable_light;
#[cfg(feature = "match_color_dimmable_light")]
pub use access_color_dimmable_light::*;

mod match_color_dimmable_light;
#[cfg(feature = "match_color_dimmable_light")]
pub use match_color_dimmable_light::*;

mod matching_context;
pub(self) use matching_context::*;

mod match_event;
pub use match_event::MatchEvent;

mod accessor;
use accessor::{AccessorCtx};
pub use accessor::Accessor; // needed by applications, for using '.get()'
