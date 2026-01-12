use std::{
    cmp::Ordering,
    fmt::{self, Display},
};

use derive_more::{
    Add, AddAssign, Display, Div, DivAssign, Into, Mul, MulAssign, Neg, Sub, SubAssign,
};
use thiserror::Error;

use crate::constants::PREGAP_OFFSET;

mod sealed {
    use std::fmt::Debug;

    pub trait Sealed {
        type Raw: Debug;
    }
}

pub trait Address: sealed::Sealed + Copy + Display {
    const MIN: Self;
    const MAX: Self;
}

#[derive(Error, Debug)]
pub enum AddressError<Addr: Address> {
    #[error("Address input {0} out of range for: {min}..={max}", min = Addr::MIN, max = Addr::MAX)]
    OutOfRange(Addr::Raw),
}

/// Newtype representing a Logical Block Address (LBA)
///
/// The LBA is the number that a Host uses to reference Logical Blocks on a block storage device.
#[repr(transparent)]
#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Into,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
    Neg,
)]
pub struct Lba(i32);

/// Creates an LBA from a constant expression. Will result in a compile error if the expression is
/// outside the valid range for LBAs.
macro_rules! lba {
    ($e:expr) => {
        const {
            match $crate::Lba::try_from_i32($e) {
                Ok(v) => v,
                Err(_) => panic!("LBA must be in range -451150..=404849"),
            }
        }
    };
}
pub(crate) use lba;

impl Lba {
    const MAX_RAW: i32 = 404849;
    const MIN_RAW: i32 = -451150;

    const fn in_range(value: &i32) -> bool {
        Self::MIN_RAW <= *value && *value <= Self::MAX_RAW
    }

    const fn new_unchecked(value: i32) -> Self {
        assert!(
            Self::in_range(&value),
            "LBA must be in range -451150..=404849"
        );

        Self(value)
    }

    pub const fn try_from_i32(value: i32) -> Result<Self, AddressError<Self>> {
        if Self::in_range(&value) {
            return Ok(Self::new_unchecked(value));
        }

        Err(AddressError::OutOfRange(value))
    }
}

impl sealed::Sealed for Lba {
    type Raw = i32;
}

impl Address for Lba {
    const MIN: Self = lba!(Self::MIN_RAW);
    const MAX: Self = lba!(Self::MAX_RAW);
}

impl TryFrom<i32> for Lba {
    type Error = AddressError<Lba>;

    #[inline]
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::try_from_i32(value)
    }
}

impl From<Msf> for Lba {
    /* The following comes strainght from the MMC-6 spec (Table 677) */
    fn from(value: Msf) -> Self {
        let offset_lba = (value.0 as i32 * 60 + value.1 as i32) * 75 + value.2 as i32;

        // Range from MMC-6: 00:00:00 <= MSF <= 89:59:74
        if value <= Msf::new_unchecked(89, 59, 74) {
            Self::new_unchecked(offset_lba - PREGAP_OFFSET as i32)
        }
        // Range from MMC-6: 90:00:00 <= MSF <= 99:59:74
        else {
            Self::new_unchecked(offset_lba - 450150)
        }
    }
}

/// Minute, Second, Frame format
///
/// A time based indexer represented as MM:SS:FF. Indexing is done using the 75 frames per second
/// conversion from time to LBA.
///
/// NOTE: libcdio stores MSF values in BCD notation. To quote its own GNU manual documents,
/// "Perhaps this is a libcdio design flaw. It was originally done I guess because it was
/// convenient for VCDs." Currently, I see little reason to use BCD so we'll just use binary.
#[derive(Clone, Copy, Debug)]
pub struct Msf(u8, u8, u8);

impl sealed::Sealed for Msf {
    type Raw = (u8, u8, u8);
}

impl Address for Msf {
    const MIN: Self = Self::new_unchecked(0, 0, 0);
    const MAX: Self = Self::new_unchecked(Self::MAX_MIN, Self::MAX_SEC, Self::MAX_FRAME);
}

impl Msf {
    const MAX_MIN: u8 = 99;
    const MAX_SEC: u8 = 59;
    const MAX_FRAME: u8 = 74;

    pub fn new(min: u8, sec: u8, frame: u8) -> Result<Self, AddressError<Msf>> {
        if min > Self::MAX_MIN || sec > Self::MAX_SEC || frame > Self::MAX_FRAME {
            return Err(AddressError::OutOfRange((min, sec, frame)));
        }

        Ok(Self::new_unchecked(min, sec, frame))
    }

    const fn new_unchecked(min: u8, sec: u8, frame: u8) -> Self {
        assert!(min <= Self::MAX_MIN, "minutes out of range");
        assert!(sec <= Self::MAX_SEC, "seconds out of range");
        assert!(frame <= Self::MAX_FRAME, "frames out of range");

        Self(min, sec, frame)
    }
}

impl From<Lba> for Msf {
    /* The following comes strainght from the MMC-6 spec (Table 677) */
    fn from(value: Lba) -> Self {
        let raw_lba: i32 = value.into();

        if (-150..=404849).contains(&raw_lba) {
            let m = (raw_lba + PREGAP_OFFSET as i32) / (60 * 75);
            let s = (raw_lba + PREGAP_OFFSET as i32 - m * 60 * 75) / 75;
            let f = raw_lba + PREGAP_OFFSET as i32 - m * 60 * 75 - s * 75;

            // We should be mathmatecially garunteed safe truncation here
            Msf::new_unchecked(m as u8, s as u8, f as u8)
        } else {
            let m = (raw_lba + 450150) / (60 * 75);
            let s = (raw_lba + 450150 - m * 60 * 75) / 75;
            let f = raw_lba + 450150 - m * 60 * 75 - s * 75;

            // We should be mathmatecially garunteed safe truncation here
            Msf::new_unchecked(m as u8, s as u8, f as u8)
        }
    }
}

impl fmt::Display for Msf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.0, self.1, self.2)
    }
}

impl PartialEq for Msf {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl Eq for Msf {}

impl PartialOrd for Msf {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Msf {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let m = self.0.cmp(&other.0);
        match m {
            Ordering::Equal => {
                let s = self.1.cmp(&other.1);
                match s {
                    Ordering::Equal => self.2.cmp(&other.2),
                    _ => s,
                }
            }
            _ => m,
        }
    }
}
