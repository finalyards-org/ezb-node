/*
*/
use core::fmt::{
    Display,
    Formatter,
};

use esp_idf_svc::sys::EspError;

#[derive(Debug, Clone)]
pub enum Error {
    AlreadyInUse,       // only one Controller/Router/EndDevice allowed
    InitializationFailed(EspError),
}

impl Display for Error {
    // tbd. more details in the messages
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}
