use zerocopy::{BigEndian, FromBytes, Immutable, IntoBytes, KnownLayout, U16, U32, Unaligned};

/// Random Readable (0010h): the ability to read sectors with random addressing.
///
/// Laid out as version 0.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct RandomReadable {
    logical_block_size: U32<BigEndian>,
    blocking: U16<BigEndian>,
    byte10: u8,
    _byte11: u8,
}

impl RandomReadable {
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

super::debug_fields!(RandomReadable {
    logical_block_size,
    blocking,
    page_present
});
