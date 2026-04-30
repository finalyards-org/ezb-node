/*
* Channel mask where each bit enables a certain channel:
*
*   - Full channel mask: channels 11..26 enabled
*   - Optimized mask: channels 11, 15, 20, 25 enabled
*/
use core::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChannelMask(pub(crate) u32);

impl ChannelMask {
    pub const PRIMARY_CHANNELS: Self = ChannelMask(1 << 11 | 1 << 15 | 1 << 20 | 1 << 25);
    pub const ALL_CHANNELS: Self = ChannelMask(0x07FFF800);    // channels 11..26 enabled
}

impl fmt::Debug for ChannelMask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ChannelMask(")?;

        // Zigbee 2.4 GHz channels are [11..26]
        write!(f, "[")?;

        let mut first = true;
        for i in 11..=26 {
            if (self.0 >> i) & 1 == 1 {
                write!(f, "{}{}", if first {""} else {", "}, i)?;
                first = false;
            }
        }
        //write!(f, "], bits: {:#010x})", self.0)
        write!(f, "])")
    }
}
