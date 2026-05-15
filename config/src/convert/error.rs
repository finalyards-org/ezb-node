#![cfg(feature = "toml")]

use std::{
    borrow::Cow,
    fmt,
    string::String
};

#[derive(Debug)]
pub enum ConfigError {
    ContentError(Cow<'static, str>),    // msg in '.0'
    FeatureConflict(&'static str),      // feature name in '.0'
    //
    ParseError(toml::de::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::ContentError(msg) => {
                write!(f, "{}", msg)
            },
            ConfigError::FeatureConflict(feat) => {
                write!(f, "'node.type' \"{}\" but that feature is not enabled.", feat)
            },
            //
            ConfigError::ParseError(err) => write!(f, "TOML parsing: {err}"),
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
