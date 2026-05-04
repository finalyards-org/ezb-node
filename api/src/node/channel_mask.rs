/*
* Channel mask where each bit enables a certain channel:
*
*   - Full channel mask: channels 11..26 enabled
*   - Optimized mask: channels 11, 15, 20, 25 enabled
*/
use core::fmt;

const VALID_CHANNELS: core::ops::RangeInclusive<u8> = 11..=26;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChannelMask(pub(crate) u32);

impl ChannelMask {
    pub const PREFERRED: Self = ChannelMask::from([11,15,20,25]);
    pub const ALL: Self = ChannelMask(0x07FFF800);    // channels 11..26 enabled
}

impl From<[u8;_]> for ChannelMask {
    fn from(channels: &[u8;_]) -> Self {
        let mut mask: u32 = 0;

        for ch in channels {
            if VALID_CHANNELS.contains(ch) {
                mask |= 1 << ch;
            } else {
                panic!("Invalid Zigbee channel (not within {}): {}", &VALID_CHANNELS, ch);
            }
        }
        Self(mask)
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
                    write!(f, "{}{}", if first {""} else {", "}, ch)?;
                    first = false;
                }
            }
        }
        write!(f, "])")
    }
}

fn is_valid_channel(ch: u8) -> bool {
    VALID_CHANNELS.contains(&ch)
}

