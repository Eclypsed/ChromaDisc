use std::fmt;

use derive_more::{Add, AddAssign, Div, DivAssign, Into, Mul, MulAssign, Neg, Sub, SubAssign};
use thiserror::Error;

use crate::constants::{FRAMES_PER_MINUTE, FRAMES_PER_SECOND, PREGAP_OFFSET};

#[derive(Error, Debug)]
pub enum BlockAddressError {
    #[error("Block address out of range")]
    OutOfRange,
}

trait SectorIndex {
    fn raw(self) -> i32;
}

/// Newtype representing a Logical Block Address (LBA)
///
/// An LBA is a block index that includes the disc pregap. This means that LBA index 0 corresponds
/// to the block at 00:00:00 in the potentially unreadable region depending on whether or not your
/// drive can read hidden track one audio (HTOA).
#[repr(transparent)]
#[derive(
    Clone,
    Copy,
    Debug,
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

impl Lba {
    /// The Maximum Logical Block Address (LBA)
    // TBH I've kinda made up this number because I don't understand why these constansts are the
    // values they are in libcdio. My logic here is 100m * 60s * 75fps = 450,000. It's more
    // conservative than libcdio's and nothing actually gets recorded up this high anyway so it should
    // be fine hopefully.
    pub const MAX: Lba = Lba(450_000);

    /// The Minimum Logical Block Address (LBA)
    pub const MIN: Lba = Lba(-450_000);

    /// The logical block at addres 0.
    pub const ZERO: Lba = Lba(0);
}

impl SectorIndex for Lba {
    fn raw(self) -> i32 {
        self.0
    }
}

impl TryFrom<i32> for Lba {
    type Error = BlockAddressError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let lba = Self(value);

        if !(Self::MIN..=Self::MAX).contains(&lba) {
            return Err(BlockAddressError::OutOfRange);
        }

        Ok(lba)
    }
}

impl From<Lsn> for Lba {
    fn from(value: Lsn) -> Self {
        Self(value.raw() + i32::from(PREGAP_OFFSET))
    }
}

impl TryFrom<Msf> for Lba {
    type Error = BlockAddressError;

    fn try_from(value: Msf) -> Result<Self, Self::Error> {
        let Msf(m, s, f) = value;

        let m: i32 = i32::from(m) * i32::from(FRAMES_PER_MINUTE);
        let s: i32 = i32::from(s) * i32::from(FRAMES_PER_SECOND);
        let f: i32 = f.into();

        Self::try_from(m + s + f)
    }
}

/// Newtype representing a Logical Sector Number (LSN)
///
/// An LSN is a block index that does not include the disc pregap. This means that LSN index 0
/// corresponds to the first playable frame of audio in a CD-DA at 00:02:00.
#[repr(transparent)]
#[derive(
    Clone,
    Copy,
    Debug,
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
pub struct Lsn(i32);

impl Lsn {
    /// The Maximum Logical Sector Number (LSN)
    pub const MAX: Lsn = Lsn(Lba::MAX.0 - PREGAP_OFFSET as i32);

    /// The Minimum Logical Sector Number (LSN)
    pub const MIN: Lsn = Lsn(Lba::MIN.0 - PREGAP_OFFSET as i32);

    /// The logical sector at index 0.
    pub const ZERO: Lsn = Lsn(0);
}

impl SectorIndex for Lsn {
    fn raw(self) -> i32 {
        self.0
    }
}

impl TryFrom<i32> for Lsn {
    type Error = BlockAddressError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let lsn = Self(value);

        if !(Self::MIN..=Self::MAX).contains(&lsn) {
            return Err(BlockAddressError::OutOfRange);
        }

        Ok(lsn)
    }
}

impl From<Lba> for Lsn {
    fn from(value: Lba) -> Self {
        Self(value.raw() - i32::from(PREGAP_OFFSET))
    }
}

impl TryFrom<Msf> for Lsn {
    type Error = BlockAddressError;

    fn try_from(value: Msf) -> Result<Self, Self::Error> {
        let lba = Lba::try_from(value)?;
        Ok(Self::from(lba))
    }
}

/// Minute, Second, Frame format
///
/// A time based indexer represented as MM:SS:FF. Indexing is done using the 75 frames per second
/// conversion from time to LBA.
///
/// NOTE: libcdio stores MSF values in BCD notation. To quote its own GNU manual documents,
/// "Perhaps this is a libcdio design flaw. It was originally done I guess because it was
/// convenient for VCDs." Currently, I see little reason to keep the BCD so we'll just use binary.
#[derive(Clone, Copy, Debug)]
pub struct Msf(u8, u8, u8);

impl fmt::Display for Msf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.0, self.1, self.2)
    }
}

impl From<Lba> for Msf {
    /* The below is adapted from libcdio which itself is adapted from cdparanoia code which claims
     * to be straight from the MMC3 spec. */
    fn from(value: Lba) -> Self {
        let mut value = if value >= Lba::ZERO {
            value.raw()
        } else {
            (value + Lba::MAX).raw()
        };

        let m = value / (i32::from(FRAMES_PER_MINUTE));
        value -= m * i32::from(FRAMES_PER_MINUTE);
        let s = value / i32::from(FRAMES_PER_SECOND);
        value -= s * i32::from(FRAMES_PER_SECOND);
        let f = value;

        // Given a valid LBA, we should be mathematically garunteed safe truncation here
        Msf(m as u8, s as u8, f as u8)
    }
}

impl From<Lsn> for Msf {
    fn from(value: Lsn) -> Self {
        Msf::from(Lba::from(value))
    }
}
