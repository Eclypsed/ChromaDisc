use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

use crate::mmc::profiles::ProfileNumber;

/// Profile List (0000h): a list of all Profiles supported by the Drive.
///
/// Laid out as version 0.
#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct ProfileList {
    profiles: [ProfileDescriptor],
}

impl ProfileList {
    #[inline]
    pub const fn profiles(&self) -> &[ProfileDescriptor] {
        &self.profiles
    }
}

super::debug_fields!(ProfileList { profiles });

/// One entry of the Profile List: a Profile the Drive supports.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct ProfileDescriptor {
    profile_number: ProfileNumber,
    byte2: u8,
    _byte3: u8,
}

impl ProfileDescriptor {
    #[inline]
    pub const fn profile_number(&self) -> ProfileNumber {
        self.profile_number
    }

    super::bit!(current_profile: byte2[0]);
}

super::debug_fields!(ProfileDescriptor {
    profile_number,
    current_profile
});
