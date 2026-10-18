use zerocopy::{BigEndian, FromBytes, Immutable, IntoBytes, KnownLayout, U16, Unaligned};

/// Timeout (0105h): the ability to respond to all commands within a specific time.
///
/// Laid out as version 1. Version 0 has no Feature Dependent Data.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct Timeout {
    byte4: u8,
    _byte5: u8,
    unit_length: U16<BigEndian>,
}

impl Timeout {
    super::bit!(
        /// Since version 1.
        #[doc(alias = "Group 3")]
        group3: byte4[0]
    );

    /// Since version 1.
    #[inline]
    pub const fn unit_length(&self) -> u16 {
        self.unit_length.get()
    }
}

super::debug_fields!(Timeout {
    group3,
    unit_length
});
