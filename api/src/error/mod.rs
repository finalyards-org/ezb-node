/*
* Zigbee errors.
*
* Design intent:
*   The 'Error' struct is intended to be easily usable by 'anyhow' library, on the application level.
*/
use core::fmt::{
    Display,
    Formatter,
};

mod esp_error;
pub(super) use esp_error::EspError;

mod ezb_error;
pub(super) use ezb_error::EzbError;

#[derive(Debug)]
pub enum Error {
    AlreadyInUse,       // only one Controller/Router/EndDevice allowed
    InitializationFailed(EzbEspError),
    SpawnFailed(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyInUse => {
                write!(f, "Zigbee node cannot be initialized twice.")
            }
            Self::InitializationFailed(e) => {
                write!(f, "Initialization failed: {e}")
            }
            Self::SpawnFailed(e) => {
                write!(f, "Failed to spawn Zigbee task: {e}")
            }
        }
    }
}

impl core::error::Error for Error {
    // 'anyhow' would be able to work without this, but this presumably enhances output details.
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::AlreadyInUse => None,
            Self::InitializationFailed(e) => Some(e),
            Self::SpawnFailed(e) => Some(e),
        }
    }
}

/// Carrier for either 'EspError' (esp_idf_...) or 'EzbError' (esp_zigbee_lib) errors (but not
/// others).
///
#[derive(Debug)]
pub enum EzbEspError {
    Esp(EspError),
    Ezb(EzbError),
}

impl From<EspError> for EzbEspError {
    fn from(e: EspError) -> Self {
        Self::Esp(e)
    }
}

impl From<EzbError> for EzbEspError {
    fn from(e: EzbError) -> Self {
        Self::Ezb(e)
    }
}

impl Display for EzbEspError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Esp(e) => {
                write!(f, "ESP-IDF error: {e}")
            }
            Self::Ezb(e) => {
                write!(f, "Zigbee error: {e}")
            }
        }
    }
}

impl core::error::Error for EzbEspError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Esp(e) => Some(e),
            Self::Ezb(e) => Some(e),
        }
    }
}
