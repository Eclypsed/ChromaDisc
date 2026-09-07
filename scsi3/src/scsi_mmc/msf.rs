use std::fmt;

use derive_more::{Display, Into};
use thiserror::Error;

macro_rules! bounded_u8 {
    ($name:ident, $err:ident, $max:literal) => {
        #[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Into)]
        pub struct $name(u8);

        #[derive(Debug, Error)]
        #[error("Invalid {name} {0}. Must be <= {max}", name = stringify!($name), max = $max)]
        pub struct $err(u8);

        impl $name {
            pub const MIN: Self = Self(0);
            pub const MAX: Self = Self($max);

            pub const fn new(value: u8) -> Result<Self, $err> {
                if value <= $max {
                    Ok(Self(value))
                } else {
                    Err($err(value))
                }
            }

            pub const fn get(self) -> u8 {
                self.0
            }
        }

        impl TryFrom<u8> for $name {
            type Error = $err;
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

bounded_u8!(Minute, MinuteRangeError, 99);
bounded_u8!(Second, SecondRangeError, 59);
bounded_u8!(Frame, FrameRangeError, 74);

#[derive(Debug, Error)]
pub enum MsfError {
    #[error(transparent)]
    Minute(#[from] MinuteRangeError),
    #[error(transparent)]
    Second(#[from] SecondRangeError),
    #[error(transparent)]
    Frame(#[from] FrameRangeError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Msf(Minute, Second, Frame);

impl Msf {
    pub const fn new(min: Minute, sec: Second, frame: Frame) -> Self {
        Self(min, sec, frame)
    }

    pub const fn try_new(m: u8, s: u8, f: u8) -> Result<Self, MsfError> {
        let min = match Minute::new(m) {
            Ok(v) => v,
            Err(e) => return Err(MsfError::Minute(e)),
        };
        let sec = match Second::new(s) {
            Ok(v) => v,
            Err(e) => return Err(MsfError::Second(e)),
        };
        let frame = match Frame::new(f) {
            Ok(v) => v,
            Err(e) => return Err(MsfError::Frame(e)),
        };
        Ok(Self(min, sec, frame))
    }

    pub const fn minute(&self) -> Minute {
        self.0
    }

    pub const fn second(&self) -> Second {
        self.1
    }

    pub const fn frame(&self) -> Frame {
        self.2
    }

    pub const fn total_frames(&self) -> u32 {
        (self.minute().0 as u32 * 60 + self.second().0 as u32) * 75 + self.frame().0 as u32
    }
}

// TODO: MSF macro, e.g. msf!(07:49:32)

impl TryFrom<(u8, u8, u8)> for Msf {
    type Error = MsfError;
    fn try_from((m, s, f): (u8, u8, u8)) -> Result<Self, Self::Error> {
        Self::try_new(m, s, f)
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
