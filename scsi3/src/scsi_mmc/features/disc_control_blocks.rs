use derive_more::Debug;
use zerocopy::{BigEndian, FromBytes, Immutable, IntoBytes, KnownLayout, U32, Unaligned};

/// Disc Control Blocks (010Ah): the ability to read and/or write DCBs.
///
/// Laid out as version 0.
#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct DiscControlBlocks {
    supported_dcb_entries: [DcbContentDescriptor],
}

impl DiscControlBlocks {
    #[inline]
    pub const fn supported_dcb_entries(&self) -> &[DcbContentDescriptor] {
        &self.supported_dcb_entries
    }
}

super::debug_fields!(DiscControlBlocks {
    supported_dcb_entries
});

// TODO: DCBs are defined in §4.12 of MMC-6. Will probably want to move this newtype
// there if/when we implement those models
/// The Content Descriptor that identifies a kind of DCB.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(transparent)]
pub struct DcbContentDescriptor(#[debug("{_0:08X}h")] U32<BigEndian>);

impl DcbContentDescriptor {
    #[inline]
    pub const fn from_u32(number: u32) -> Self {
        Self(U32::new(number))
    }

    #[inline]
    pub const fn into_u32(self) -> u32 {
        self.0.get()
    }
}
