use std::{fmt, ops::RangeInclusive};

use deku::{DekuError, DekuRead};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("RawMsf {0} out of valid range: {min}-{max}", min = Msf::MIN, max = Msf::MAX)]
pub struct MsfRangeError(RawMsf);

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

/// A raw, bounds unchecked container for three BCD values.
///
/// This is intended to be used for MSF values that are read from hardware at runtime.
///
/// > While MSF addresses are usually encoded using BCD, this crate deviates in favor of binary
/// > encoding and provides methods for automatic conversions while reading/writing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, DekuRead)]
#[deku(ctx = "bcd: bool", ctx_default = "false")]
pub struct RawMsf(
    #[deku(map = "|f| map_bcd(f, bcd)")] u8,
    #[deku(map = "|f| map_bcd(f, bcd)")] u8,
    #[deku(map = "|f| map_bcd(f, bcd)")] u8,
);

impl RawMsf {
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

    pub const fn minute(&self) -> u8 {
        self.0
    }

    pub const fn second(&self) -> u8 {
        self.1
    }

    pub const fn frame(&self) -> u8 {
        self.2
    }
}

impl fmt::Display for RawMsf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.0, self.1, self.2)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Msf(u8, u8, u8);

/// Minute, Second, Frame format
///
/// A time-based index represented as MM:SS:FF. There are 75 frames per second.
///
/// > While MSF addresses are usually encoded using BCD, this crate deviates in favor of binary
/// > encoding and provides methods for automatic conversions while reading/writing.
impl Msf {
    pub const FRAME_MIN: u8 = 0;
    pub const FRAME_MAX: u8 = 74;
    pub const SECOND_MIN: u8 = 0;
    pub const SECOND_MAX: u8 = 59;
    pub const MINUTE_MIN: u8 = 0;
    pub const MINUTE_MAX: u8 = 99;

    pub const MIN: Msf = Msf(Self::MINUTE_MIN, Self::SECOND_MIN, Self::FRAME_MIN);
    pub const MAX: Msf = Msf(Self::MINUTE_MAX, Self::SECOND_MAX, Self::FRAME_MAX);
}

impl Msf {
    pub const fn minute(&self) -> u8 {
        self.0
    }

    pub const fn second(&self) -> u8 {
        self.1
    }

    pub const fn frame(&self) -> u8 {
        self.2
    }
}

impl TryFrom<RawMsf> for Msf {
    type Error = MsfRangeError;

    fn try_from(value: RawMsf) -> Result<Self, Self::Error> {
        const M_RANGE: RangeInclusive<u8> = Msf::MINUTE_MIN..=Msf::MINUTE_MAX;
        const S_RANGE: RangeInclusive<u8> = Msf::SECOND_MIN..=Msf::SECOND_MAX;
        const F_RANGE: RangeInclusive<u8> = Msf::FRAME_MIN..=Msf::FRAME_MAX;

        let m = value.minute();
        let s = value.second();
        let f = value.frame();

        if !M_RANGE.contains(&m) || !S_RANGE.contains(&s) || !F_RANGE.contains(&f) {
            return Err(MsfRangeError(value));
        }

        Ok(Self(m, s, f))
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

    use crate::core::RawMsf;

    #[test]
    fn parse_msf_bcd() {
        let data: &[u8] = &[0b1001_0111, 0b0100_1001, 0b0000_0000];
        let mut reader = Reader::new(Cursor::new(data));

        let msf = RawMsf::from_reader_with_ctx(&mut reader, true).unwrap();

        assert_eq!(RawMsf(97, 49, 00), msf);
    }
}
