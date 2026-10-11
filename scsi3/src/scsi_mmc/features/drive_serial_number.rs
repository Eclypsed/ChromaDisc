use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

use crate::core::ascii::UnvalidatedAsciiStr;

/// Drive Serial Number (0108h): the Drive has a unique identifier.
///
/// Laid out as version 0.
#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct DriveSerialNumber {
    serial_number: UnvalidatedAsciiStr,
}

impl DriveSerialNumber {
    #[inline]
    pub const fn serial_number(&self) -> &UnvalidatedAsciiStr {
        &self.serial_number
    }
}

super::debug_fields!(DriveSerialNumber { serial_number });
