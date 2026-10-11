use arbitrary_int::u3;
use bitfield::{Bits, BitsEnum};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// Removable Medium (0003h): the medium may be removed from the device.
///
/// Laid out as version 2.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct RemovableMedium {
    byte4: u8,
    _bytes5_7: [u8; 3],
}

impl RemovableMedium {
    #[inline]
    pub const fn loading_mechanism_type(&self) -> Bits<LoadingMechanism> {
        Bits::new(u3::extract_u8(self.byte4, 5))
    }

    super::bit!(
        /// Since version 1.
        load: byte4[4]
    );
    super::bit!(eject: byte4[3]);
    super::bit!(#[doc(alias = "Pvnt Jmpr")] prevent_jumper: byte4[2]);
    super::bit!(
        /// Since version 2.
        #[doc(alias = "DBML")]
        device_busy_medium_loading: byte4[1]
    );
    super::bit!(lock: byte4[0]);
}

super::debug_fields!(RemovableMedium {
    loading_mechanism_type,
    load,
    eject,
    prevent_jumper,
    device_busy_medium_loading,
    lock
});

/// A 3-bit value representing the phyical Loading Mechanism Type used by drives that support
/// Removable Mediums.
///
/// See MMC-6 §5.3.4, Table 102.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3, reserved = 0b011 | 0b110..=0b111)]
pub enum LoadingMechanism {
    /// Caddy/Slot type loading mechanism
    CaddySlot = 0b000,
    /// Tray type loading mechanism
    Tray = 0b001,
    /// Pop-up type loading mechanism
    PopUp = 0b010,
    /// Embedded changer with individually changeable discs
    EmbeddedIndividuallyChangeable = 0b100,
    /// Embedded changer using a magazine mechanism
    EmbeddedMagazine = 0b101,
}
