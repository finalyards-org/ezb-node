/*
*/
use core::fmt::{
    Display,
    Formatter,
};

use esp_idf_svc::sys::EspError;

/**
* An error that can arise either from the application (e.g. initialization), or some library (e.g. 'esp_zb').
*/
#[derive(Debug)]
pub enum AppError {
    Other(&'static str),
    ApiError(ezb_node::Error),
    SysError(EspError),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<ezb_node::Error> for AppError {
    fn from(err: ezb_node::Error) -> Self {
        Self::ApiError(err)
    }
}

impl From<EspError> for AppError {
    fn from(err: EspError) -> Self {
        Self::SysError(err)
    }
}
