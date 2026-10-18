use crate::core::ascii::UnvalidatedSizedAsciiStr;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// Firmware Information (010Ch): the creation date of the Drive's firmware.
///
/// Laid out as version 0.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct FirmwareInformation {
    century: UnvalidatedSizedAsciiStr<2>,
    year: UnvalidatedSizedAsciiStr<2>,
    month: UnvalidatedSizedAsciiStr<2>,
    day: UnvalidatedSizedAsciiStr<2>,
    hour: UnvalidatedSizedAsciiStr<2>,
    minute: UnvalidatedSizedAsciiStr<2>,
    second: UnvalidatedSizedAsciiStr<2>,
    _bytes18_19: [u8; 2],
}

impl FirmwareInformation {
    #[inline]
    pub const fn century(&self) -> UnvalidatedSizedAsciiStr<2> {
        self.century
    }

    #[inline]
    pub const fn year(&self) -> UnvalidatedSizedAsciiStr<2> {
        self.year
    }

    #[inline]
    pub const fn month(&self) -> UnvalidatedSizedAsciiStr<2> {
        self.month
    }

    #[inline]
    pub const fn day(&self) -> UnvalidatedSizedAsciiStr<2> {
        self.day
    }

    #[inline]
    pub const fn hour(&self) -> UnvalidatedSizedAsciiStr<2> {
        self.hour
    }

    #[inline]
    pub const fn minute(&self) -> UnvalidatedSizedAsciiStr<2> {
        self.minute
    }

    #[inline]
    pub const fn second(&self) -> UnvalidatedSizedAsciiStr<2> {
        self.second
    }
}

super::debug_fields!(FirmwareInformation {
    century,
    year,
    month,
    day,
    hour,
    minute,
    second
});
