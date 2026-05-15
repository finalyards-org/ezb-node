#![cfg_attr(not(feature = "toml"), no_std)]
#[cfg(not(feature = "toml"))]
extern crate alloc;

#[cfg(feature = "toml")]
mod convert;
#[cfg(feature = "toml")]
pub use convert::*;

mod channel_mask;
pub use channel_mask::ChannelMask;

mod config;
pub use config::Config;
