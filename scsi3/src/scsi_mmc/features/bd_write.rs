use arbitrary_int::u2;
use zerocopy::{BigEndian, FromBytes, Immutable, IntoBytes, KnownLayout, U16, Unaligned};

/// BD Write (0041h): the ability to write control structures and user data to certain BD discs.
///
/// Laid out as version 0.
///
/// MMC-5 and MMC-6r02 define bytes 8-23 as bitmaps of the versions the Drive can write, one per
/// disc type and class, where bit `n` stands for version `n`. By MMC-6r02g, they were replaced
/// with the explicit flags here, which keep their bitmap positions, and every other bit was made
/// obsolete. The whole bitmaps remain readable through [`Self::bd_re_write_support`] and
/// [`Self::bd_r_write_support`].
// NOTE: Currently don't have copies of MMC-6r02a through MMC-6r02f, so which revision first
// replaced the bitmaps is unknown.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct BdWrite {
    byte4: u8,
    _bytes5_7: [u8; 3],
    bd_re_write_support: [U16<BigEndian>; 4],
    bd_r_write_support: [U16<BigEndian>; 4],
}

impl BdWrite {
    super::bit!(#[doc(alias = "SVNR")] supports_verify_not_required: byte4[0]);

    /// Bit 2 of the Class 0 BD-RE bitmap.
    #[doc(alias = "RE2")]
    #[inline]
    pub const fn re2(&self) -> bool {
        self.bd_re_write_support[0].get() & 1 << 2 != 0
    }

    /// Bit 1 of the Class 0 BD-R bitmap.
    #[doc(alias = "R")]
    #[inline]
    pub const fn r(&self) -> bool {
        self.bd_r_write_support[0].get() & 1 << 1 != 0
    }

    /// The Class `class` BD-RE Write Support bitmap, which is obsolete except for the bit of
    /// [`Self::re2`].
    #[inline]
    pub const fn bd_re_write_support(&self, class: u2) -> u16 {
        self.bd_re_write_support[class.value() as usize].get()
    }

    /// The Class `class` BD-R Write Support bitmap, which is obsolete except for the bit of
    /// [`Self::r`].
    #[inline]
    pub const fn bd_r_write_support(&self, class: u2) -> u16 {
        self.bd_r_write_support[class.value() as usize].get()
    }
}

super::debug_fields!(BdWrite {
    supports_verify_not_required,
    re2,
    r
});
