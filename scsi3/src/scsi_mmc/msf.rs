use std::fmt;

use derive_more::{Display, Into};
use thiserror::Error;

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Into, Ord, Hash)]
pub struct Minute(u8);

#[derive(Debug, Error)]
#[error("Invalid Minute {0}. Must be <= {max}", max = Minute::MAX)]
pub struct MinuteRangeError(u8);

impl Minute {
    pub const MIN: Self = Self(0);
    pub const MAX: Self = Self(99);
}

impl TryFrom<u8> for Minute {
    type Error = MinuteRangeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if u8::from(Self::MIN) <= value && value <= u8::from(Self::MAX) {
            Ok(Self(value))
        } else {
            Err(MinuteRangeError(value))
        }
    }
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Into, Ord, Hash)]
pub struct Second(u8);

#[derive(Debug, Error)]
#[error("Invalid Second {0}. Must be <= {max}", max = Second::MAX)]
pub struct SecondRangeError(u8);

impl Second {
    pub const MIN: Self = Self(0);
    pub const MAX: Self = Self(59);
}

impl TryFrom<u8> for Second {
    type Error = SecondRangeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if u8::from(Self::MIN) <= value && value <= u8::from(Self::MAX) {
            Ok(Self(value))
        } else {
            Err(SecondRangeError(value))
        }
    }
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Into, Ord, Hash)]
pub struct Frame(u8);

#[derive(Debug, Error)]
#[error("Invalid Frame {0}. Must be <= {max}", max = Frame::MAX)]
pub struct FrameRangeError(u8);

impl Frame {
    pub const MIN: Self = Self(0);
    pub const MAX: Self = Self(74);
}

impl TryFrom<u8> for Frame {
    type Error = FrameRangeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if u8::from(Self::MIN) <= value && value <= u8::from(Self::MAX) {
            Ok(Self(value))
        } else {
            Err(FrameRangeError(value))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Msf(Minute, Second, Frame);

impl Msf {
    pub const fn new(min: Minute, sec: Second, frame: Frame) -> Self {
        Self(min, sec, frame)
    }

    pub const fn minute(&self) -> &Minute {
        &self.0
    }

    pub const fn second(&self) -> &Second {
        &self.1
    }

    pub const fn frame(&self) -> &Frame {
        &self.2
    }

    pub const fn total_frames(&self) -> u32 {
        (self.minute().0 as u32 * 60 + self.second().0 as u32) * 75 + self.frame().0 as u32
    }
}

impl fmt::Display for Msf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.0, self.1, self.2)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnvalidatedMsf(u8, u8, u8);

impl UnvalidatedMsf {
    pub fn new(minute: u8, second: u8, frame: u8) -> Self {
        Self(minute, second, frame)
    }
}
