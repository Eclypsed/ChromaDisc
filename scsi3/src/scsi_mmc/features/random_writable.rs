use zerocopy::{BigEndian, FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned, U16, U32};

/// Random Writable (0020h): write support for randomly addressed writes.
///
/// Laid out as version 1.
// NOTE: Version 0 is assumed to share this layout. MMC-2r09, a draft, gives version 0 a different
// 8-byte layout (Logical Block Address, Blocking, PP). Data in that layout is shorter than this
// one, so `data()` is `None` for it rather than misread. Need to find a better source to verify.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct RandomWritable {
    last_logical_block_address: U32<BigEndian>,
    logical_block_size: U32<BigEndian>,
    blocking: U16<BigEndian>,
    byte14: u8,
    _byte15: u8,
}

impl RandomWritable {
    #[inline]
    pub const fn last_logical_block_address(&self) -> u32 {
        self.last_logical_block_address.get()
    }

    #[inline]
    pub const fn logical_block_size(&self) -> u32 {
        self.logical_block_size.get()
    }

    #[inline]
    pub const fn blocking(&self) -> u16 {
        self.blocking.get()
    }

    super::bit!(#[doc(alias = "PP")] page_present: byte14[0]);
}

super::debug_fields!(RandomWritable {
    last_logical_block_address,
    logical_block_size,
    blocking,
    page_present
});
