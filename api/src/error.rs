/*
*/
use core::fmt::{
    Display,
    Formatter,
};

#[derive(Debug, Clone)]
pub enum Error {
    AlreadyInUse,     // only one Controller/Router/EndDevice allowed
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}
