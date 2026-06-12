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

use esp_idf_svc::sys::EspError;

#[derive(Debug)]
pub enum Error {
    AlreadyInUse,       // only one Controller/Router/EndDevice allowed
    InitializationFailed(EspError),
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

impl std::error::Error for Error {
    // 'anyhow' would be able to work without this, but this presumably enhances output details.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::AlreadyInUse => None,
            Self::InitializationFailed(e) => Some(e),
            Self::SpawnFailed(e) => Some(e),
        }
    }
}
