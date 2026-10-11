use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// Layer Jump Rigid Restricted Overwrite (0034h): the ability to perform Layer Jump recording on
/// Rigid Restricted Overwritable media.
///
/// Laid out as version 0.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct LayerJumpRigidRestrictedOverwrite {
    byte4: u8,
    _bytes5_6: [u8; 2],
    buffer_block_size: u8,
}

impl LayerJumpRigidRestrictedOverwrite {
    super::bit!(#[doc(alias = "CLJB")] cljb: byte4[0]);

    #[inline]
    pub const fn buffer_block_size(&self) -> u8 {
        self.buffer_block_size
    }
}

super::debug_fields!(LayerJumpRigidRestrictedOverwrite {
    cljb,
    buffer_block_size
});
