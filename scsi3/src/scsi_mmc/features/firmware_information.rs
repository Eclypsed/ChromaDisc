use tinystr::UnvalidatedTinyAsciiStr;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

macro_rules! ascii_fields {
    ($($field:ident),+ $(,)?) => {$(
        #[inline]
        pub const fn $field(&self) -> UnvalidatedTinyAsciiStr<2> {
            UnvalidatedTinyAsciiStr::from_utf8_unchecked(self.$field)
        }
    )+};
}

/// Firmware Information (010Ch): the creation date of the Drive's firmware.
///
/// Laid out as version 0.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct FirmwareInformation {
    century: [u8; 2],
    year: [u8; 2],
    month: [u8; 2],
    day: [u8; 2],
    hour: [u8; 2],
    minute: [u8; 2],
    second: [u8; 2],
    _bytes18_19: [u8; 2],
}

impl FirmwareInformation {
    ascii_fields!(century, year, month, day, hour, minute, second);
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
