/*
*/
use core::fmt::{
    Display,
    Formatter,
};

/**
* An error that can arise either from the application (e.g. initialization), or some library (e.g. 'esp_zb').
*/
#[derive(Debug, Clone)]
pub enum AppError {
    Other(&'static str),
    ApiError(esp_zb::Error),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<esp_zb::Error> for AppError {
    fn from(err: esp_zb::Error) -> Self {
        Self::ApiError(err)
    }
}
