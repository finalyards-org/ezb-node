use core::fmt;
use core::num::NonZeroI32;

/// @background There is an 'esp_idf_sys::EspError'. This serves the same purpose, but:
///     - allows '::wrap' for more ergonomic creation
///         (the author thinks 'from()' is best resrved only for 'impl From<...>')
///     - allows joining with 'EzbError'
///
#[derive(Debug)]
pub struct EspError(NonZeroI32);

impl EspError {
    pub(crate) fn wrap(v: i32 /*esp_err_t*/) -> Result<(), Self> {
        match NonZeroI32::new(v) {
            None => Ok(()),
            Some(x) => Err(EspError(x)),
        }
    }
}

impl fmt::Display for EspError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let ee = esp_idf_svc::sys::EspError::from(self.0.get()).unwrap();
        write!(f, "{}", ee)
    }
}

impl core::error::Error for EspError {}
