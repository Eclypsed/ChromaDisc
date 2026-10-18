use zerocopy::{BigEndian, FromBytes, Immutable, IntoBytes, KnownLayout, U16, U32, Unaligned};

/// Write Once (0025h): write support for write-once media that is writable in random order.
///
/// Laid out as version 0.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct WriteOnce {
    logical_block_size: U32<BigEndian>,
    blocking: U16<BigEndian>,
    byte10: u8,
    _byte11: u8,
}

impl WriteOnce {
    #[inline]
    pub const fn logical_block_size(&self) -> u32 {
        self.logical_block_size.get()
    }

    #[inline]
    pub const fn blocking(&self) -> u16 {
        self.blocking.get()
    }

    super::bit!(#[doc(alias = "PP")] page_present: byte10[0]);
}

super::debug_fields!(WriteOnce {
    logical_block_size,
    blocking,
    page_present
});
