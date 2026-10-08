#![cfg(feature = "toml")]

use std::{
    borrow::Cow,
    fmt,
    string::String
};

#[derive(Debug)]
pub enum ConfigError {
    /// Error in the inner content of the TOML.
    /// For example:
    ///     - string content not in expected syntax
    ///     - values out of expected range
    ContentError(Cow<'static, str>),    // msg in '.0'

    //r /// Error if TOML contains a node type that's not supported by the features.
    //r FeatureConflict(&'static str),      // feature name in '.0'

    ParseError(toml::de::Error),
    InternalError(syn::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::ContentError(msg) => {
                write!(f, "{}", msg)
            },
            //r ConfigError::FeatureConflict(feat) => {
            //r     write!(f, "'node.type' \"{}\" but that feature is not enabled.", feat)
            //r },
            ConfigError::ParseError(err) => write!(f, "TOML parsing: {err}"),
            ConfigError::InternalError(err) => {
                write!(f, "Internal error (syntax error in generated code): {err}")
            }
        }
    }
}

// Allows diving to the core error. #Good_manners (also, required by e.g. 'anyhow')
impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            ConfigError::ParseError(err) => Some(err),
            _ => None,
        }
    }
}

// Allow creation from mere string constant.
impl From<&'static str> for ConfigError {
    fn from(msg: &'static str) -> Self {
        ConfigError::ContentError(Cow::Borrowed(msg))
    }
}

// Allow creation from 'format!()' output.
impl From<String> for ConfigError {
    fn from(msg: String) -> Self {
        ConfigError::ContentError(Cow::Owned(msg))
    }
}

// Enable '?' operator with 'toml' library call
impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        ConfigError::ParseError(err)
    }
}

// Wrap 'syn' errors
impl From<syn::Error> for ConfigError {
    fn from(err: syn::Error) -> Self {
        ConfigError::InternalError(err)
    }
}
