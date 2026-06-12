use std::fmt;

use deku::{reader::Reader, DekuError, DekuRead, DekuReader};
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

impl<'a> DekuReader<'a> for Minute {
    fn from_reader_with_ctx<R: deku::no_std_io::Read + deku::no_std_io::Seek>(
        reader: &mut Reader<R>,
        _: (),
    ) -> Result<Self, DekuError> {
        Self::try_from(u8::from_reader_with_ctx(reader, ())?)
            .map_err(|e| DekuError::Parse(e.to_string().into()))
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

impl<'a> DekuReader<'a> for Second {
    fn from_reader_with_ctx<R: deku::no_std_io::Read + deku::no_std_io::Seek>(
        reader: &mut Reader<R>,
        _: (),
    ) -> Result<Self, DekuError> {
        Self::try_from(u8::from_reader_with_ctx(reader, ())?)
            .map_err(|e| DekuError::Parse(e.to_string().into()))
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

impl<'a> DekuReader<'a> for Frame {
    fn from_reader_with_ctx<R: deku::no_std_io::Read + deku::no_std_io::Seek>(
        reader: &mut Reader<R>,
        _: (),
    ) -> Result<Self, DekuError> {
        Self::try_from(u8::from_reader_with_ctx(reader, ())?)
            .map_err(|e| DekuError::Parse(e.to_string().into()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, DekuRead)]
pub struct Msf(Minute, Second, Frame);

impl Msf {
    pub const fn new(min: Minute, sec: Second, frame: Frame) -> Self {
        Self(min, sec, frame)
    }

    pub const fn min(&self) -> Minute {
        self.0
    }

    pub const fn sec(&self) -> Second {
        self.1
    }

    pub const fn frame(&self) -> Frame {
        self.2
    }
}

impl fmt::Display for Msf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.0, self.1, self.2)
    }
}
