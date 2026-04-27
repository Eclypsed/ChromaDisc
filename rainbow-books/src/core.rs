use std::fmt;

use deku::{DekuError, DekuRead};

const fn bcd_to_decimal(num: u8) -> u8 {
    ((num >> 4) & 0x0F) * 10 + (num & 0x0F)
}

const fn map_bcd(field: u8, bcd: bool) -> Result<u8, DekuError> {
    if bcd {
        Ok(bcd_to_decimal(field))
    } else {
        Ok(field)
    }
}

/// Minute, Second, Frame format
///
/// A time-based index represented as MM:SS:FF. There are 75 frames per second.
///
/// > While MSF addresses are usually encoded using BCD, this crate deviates in favor of binary
/// > encoding and provides methods for automatic conversions while reading/writing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, DekuRead)]
#[deku(ctx = "bcd: bool", ctx_default = "true")]
pub struct Msf(
    #[deku(map = "|f| map_bcd(f, bcd)")] u8,
    #[deku(map = "|f| map_bcd(f, bcd)")] u8,
    #[deku(map = "|f| map_bcd(f, bcd)")] u8,
);

impl Msf {
    pub const fn new(min: u8, sec: u8, frame: u8) -> Self {
        Self(min, sec, frame)
    }

    pub const fn from_bcd(min: u8, sec: u8, frame: u8) -> Self {
        Self(
            bcd_to_decimal(min),
            bcd_to_decimal(sec),
            bcd_to_decimal(frame),
        )
    }
}

impl fmt::Display for Msf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.0, self.1, self.2)
    }
}

#[cfg(test)]
mod feature_deku_tests {
    use std::io::Cursor;

    use deku::{reader::Reader, DekuReader};

    use crate::core::Msf;

    #[test]
    fn parse_msf_bcd() {
        let data: &[u8] = &[0b1001_0111, 0b0100_1001, 0b0000_0000];
        let mut reader = Reader::new(Cursor::new(data));

        let msf = Msf::from_reader_with_ctx(&mut reader, ()).unwrap();

        assert_eq!(Msf::new(97, 49, 00), msf);
    }
}
