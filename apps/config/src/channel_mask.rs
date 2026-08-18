use core::fmt;

const VALID_CHANNELS: core::ops::RangeInclusive<u8> = 11..=26;

/**
* Channel mask where each bit enables a certain Zigbee channel.
*/
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChannelMask(u32);

impl ChannelMask {
    // Special cases (for secondary scan)
    pub const ALL: Self = ChannelMask(0x07FFF800_u32);    // channels 11..=26 enabled
    pub const PREFERRED: Self = ChannelMask( 1 << 11 | 1 << 15 | 1 << 20 | 1 << 25 );
        // note: generating from an array would not be 'const fn', because of the for loop

    // Generated snippets use this (from application crate; possibly as 'const')
    pub const fn new(v: u32) -> Self {
        Self(v)
    }

    pub fn bits(&self) -> u32 {
        self.0
    }
}

impl fmt::Debug for ChannelMask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ChannelMask(")?;

        write!(f, "[")?;
        {
            let mut first = true;
            for ch in VALID_CHANNELS {
                if (self.0 >> ch) & 1 == 1 {
                    if !first { write!(f, ", ")?; }
                    write!(f, "{ch}")?;
                    first = false;
                }
            }
        }
        write!(f, "])")
    }
}

#[allow(dead_code)]
fn is_valid_channel(ch: u8) -> bool { VALID_CHANNELS.contains(&ch) }
