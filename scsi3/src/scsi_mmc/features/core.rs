use core::ops::RangeInclusive;

use derive_more::Debug;
use zerocopy::{BigEndian, FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned, U32};

use super::FeatureDescriptor;

/// Core (0001h): mandatory behavior for all devices.
///
/// Laid out as version 2. Version 0 data is only the Physical Interface Standard, which is too
/// short for this layout; read it with [`FeatureDescriptor::data_v0`].
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct Core {
    physical_interface_standard: PhysicalInterfaceStandard,
    byte8: u8,
    _bytes9_11: [u8; 3],
}

impl Core {
    #[inline]
    pub const fn physical_interface_standard(&self) -> PhysicalInterfaceStandard {
        self.physical_interface_standard
    }

    super::bit!(
        /// Since version 2.
        #[doc(alias = "INQ2")]
        inq2: byte8[1]
    );
    super::bit!(
        /// Since version 1.
        #[doc(alias = "DBE")]
        device_busy_event: byte8[0]
    );
}

super::debug_fields!(Core {
    physical_interface_standard,
    inq2,
    device_busy_event
});

/// The version 0 layout of [`Core`], which only holds the Physical Interface Standard.
///
/// Every version of Core begins with this layout, so it reads all of them.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct CoreV0 {
    physical_interface_standard: PhysicalInterfaceStandard,
}

impl CoreV0 {
    #[inline]
    pub const fn physical_interface_standard(&self) -> PhysicalInterfaceStandard {
        self.physical_interface_standard
    }
}

super::debug_fields!(CoreV0 {
    physical_interface_standard
});

impl FeatureDescriptor<Core> {
    /// The Feature Dependent Data as [`CoreV0`], which every version begins with.
    ///
    /// `None` only if the data is shorter than version 0's.
    #[inline]
    pub fn data_v0(&self) -> Option<&CoreV0> {
        CoreV0::ref_from_prefix(self.raw_data())
            .ok()
            .map(|(data, _)| data)
    }

    /// The Feature Dependent Data as [`CoreV0`], which every version begins with.
    ///
    /// `None` only if the data is shorter than version 0's.
    #[inline]
    pub fn data_v0_mut(&mut self) -> Option<&mut CoreV0> {
        CoreV0::mut_from_prefix(self.raw_data_mut())
            .ok()
            .map(|(data, _)| data)
    }
}

/// A 32-bit value representing what physical interface the drive is using.
///
/// See MMC-6 §5.3.2, Table 97.
#[derive(
    FromBytes,
    IntoBytes,
    KnownLayout,
    Immutable,
    Unaligned,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[repr(transparent)]
pub struct PhysicalInterfaceStandard(#[debug("{_0:08X}h")] U32<BigEndian>);

impl PhysicalInterfaceStandard {
    pub const UNSPECIFIED: Self = Self::from_u32(0x00000000);
    pub const SCSI_FAMILY: Self = Self::from_u32(0x00000001);
    pub const ATAPI: Self = Self::from_u32(0x00000002);
    pub const IEEE1394_1995: Self = Self::from_u32(0x00000003);
    pub const IEEE1394A: Self = Self::from_u32(0x00000004);
    pub const FIBRE_CHANNEL: Self = Self::from_u32(0x00000005);
    pub const IEEE1394B: Self = Self::from_u32(0x00000006);
    pub const SERIAL_ATAPI: Self = Self::from_u32(0x00000007);
    pub const USB: Self = Self::from_u32(0x00000008);
    pub const VENDOR_UNIQUE: Self = Self::from_u32(0x0000FFFF);
    pub const INCITS_DEFINED: RangeInclusive<Self> =
        Self::from_u32(0x00010000)..=Self::from_u32(0x0001FFFF);
    pub const SFF_DEFINED: RangeInclusive<Self> =
        Self::from_u32(0x00020000)..=Self::from_u32(0x0002FFFF);
    pub const IEEE_DEFINED: RangeInclusive<Self> =
        Self::from_u32(0x00030000)..=Self::from_u32(0x0003FFFF);

    #[inline]
    pub const fn from_u32(number: u32) -> Self {
        Self(U32::new(number))
    }

    #[inline]
    pub const fn into_u32(self) -> u32 {
        self.0.get()
    }
}
