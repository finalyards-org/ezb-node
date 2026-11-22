#![feature(adt_const_params)]
#![feature(unsized_const_params)]
    // enables use of '&'static str' in const generics
#![allow(incomplete_features)]
    // otherwise warns of the 'unsized_const_params' being enabled.

mod config_views;

mod node;
pub use node::*;

mod access;
#[cfg(feature = "_remote_any")]
pub use access::{
    //Accessor,
    Matcher,
    *   // device type specific accessors ('AccessColorDimmableLight' etc. ; they may grow to be many)
};

mod app_signal;
pub use app_signal::*;

mod error;
pub use error::*;

pub mod utils;
pub use utils::{
    IeeeAddr,
    ShortAddr,
};

mod types;
pub use types::*;

// Pass-through the config.
//
// Essential for the from-TOML-generated code.
pub use ezb_node_config as config;
use ezb_node_config::Config;    // just for own consumption, this way

mod zcl_event;
pub use zcl_event::*;
