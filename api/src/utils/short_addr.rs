use core::fmt;

/**
* Wrapper for 16-bit addresses; C represents them as u16, and has some consts.
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShortAddr(pub(crate) u16);

impl ShortAddr {
    pub(crate) const GROUPCAST: Self = ShortAddr(0xFFFD);     // broadcast to all devices with 'RxOnWhenIdle' = true
}

impl From<u16> for ShortAddr {
    fn from(v: u16) -> Self {
        Self(v)
    }
}

impl fmt::UpperHex for ShortAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Delegate to inner 'u16'
        fmt::UpperHex::fmt(&self.0, f)
    }
}
