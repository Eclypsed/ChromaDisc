use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

use crate::mmc::profiles::ProfileNumber;

/// OSSC (0142h): the TCG Optical Security Subsystem Class Feature.
///
/// Laid out as version 0.
#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Ossc {
    byte4: u8,
    number_of_profiles: u8,
    profile_numbers: [ProfileNumber],
}

impl Ossc {
    super::bit!(#[doc(alias = "PSAU")] psa_updates: byte4[7]);
    super::bit!(#[doc(alias = "LOSPB")] linked_ospb: byte4[6]);
    super::bit!(#[doc(alias = "ME")] mandatory_encryption: byte4[0]);

    #[inline]
    pub const fn number_of_profiles(&self) -> u8 {
        self.number_of_profiles
    }

    /// The first Number of Profiles entries of the Profile Number list.
    ///
    /// Any entries beyond Number of Profiles are ignored. Shorter than Number of Profiles only if
    /// fewer entries are present, which a drive conforming to the spec won't report.
    #[inline]
    pub const fn profile_numbers(&self) -> &[ProfileNumber] {
        match self
            .profile_numbers
            .split_at_checked(self.number_of_profiles as usize)
        {
            Some((profile_numbers, _)) => profile_numbers,
            None => &self.profile_numbers,
        }
    }
}

super::debug_fields!(Ossc {
    psa_updates,
    linked_ospb,
    mandatory_encryption,
    number_of_profiles,
    profile_numbers
});
