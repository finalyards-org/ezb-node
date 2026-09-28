// If targeting embedded, add ability to use 'alloc'
#![cfg_attr(not(feature = "toml"), no_std)]

// IDE: don't suggest 'assert_eq!'
#![allow(clippy::assert_eq_with_expected_predicate)]

mod convert;
#[cfg(feature = "toml")]
pub use convert::*;

// Allowing 'common' for also the 'convert' use (TOML -> code) is needed for two things:
//  - validity checking, parsing etc. functions
//  - code completion within the 'quote!' macros
mod common;
pub use common::*;
