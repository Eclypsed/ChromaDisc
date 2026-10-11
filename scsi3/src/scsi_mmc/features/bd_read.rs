use arbitrary_int::u2;
use zerocopy::{BigEndian, FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned, U16};

/// BD Read (0040h): the ability to read control structures and user data from a BD disc.
///
/// Laid out as version 1.
///
/// MMC-5 and MMC-6r02 define bytes 8-31 as bitmaps of the versions the Drive can read, one per
/// disc type and class, where bit `n` stands for version `n`. By MMC-6r02g, they were replaced
/// with the explicit flags here, which keep their bitmap positions, and every other bit was made
/// obsolete. The whole bitmaps remain readable through [`Self::bd_re_read_support`],
/// [`Self::bd_r_read_support`] and [`Self::bd_rom_read_support`].
// NOTE: Currently don't have copies of MMC-6r02a through MMC-6r02f, so which revision first
// replaced the bitmaps is unknown.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct BdRead {
    byte4: u8,
    _bytes5_7: [u8; 3],
    bd_re_read_support: [U16<BigEndian>; 4],
    bd_r_read_support: [U16<BigEndian>; 4],
    bd_rom_read_support: [U16<BigEndian>; 4],
}

impl BdRead {
    super::bit!(
        /// Since version 1.
        #[doc(alias = "BCA")]
        bca: byte4[0]
    );

    /// Bit 2 of the Class 0 BD-RE bitmap.
    #[doc(alias = "RE2")]
    #[inline]
    pub const fn re2(&self) -> bool {
        self.bd_re_read_support[0].get() & 1 << 2 != 0
    }

    /// Bit 1 of the Class 0 BD-RE bitmap.
    #[doc(alias = "RE1")]
    #[inline]
    pub const fn re1(&self) -> bool {
        self.bd_re_read_support[0].get() & 1 << 1 != 0
    }

    /// Bit 1 of the Class 0 BD-R bitmap.
    #[doc(alias = "R")]
    #[inline]
    pub const fn r(&self) -> bool {
        self.bd_r_read_support[0].get() & 1 << 1 != 0
    }

    /// Bit 1 of the Class 0 BD-ROM bitmap.
    #[doc(alias = "ROM")]
    #[inline]
    pub const fn rom(&self) -> bool {
        self.bd_rom_read_support[0].get() & 1 << 1 != 0
    }

    /// The Class `class` BD-RE Read Support bitmap, which is obsolete except for the bits of
    /// [`Self::re1`] and [`Self::re2`].
    #[inline]
    pub const fn bd_re_read_support(&self, class: u2) -> u16 {
        self.bd_re_read_support[class.value() as usize].get()
    }

    /// The Class `class` BD-R Read Support bitmap, which is obsolete except for the bit of
    /// [`Self::r`].
    #[inline]
    pub const fn bd_r_read_support(&self, class: u2) -> u16 {
        self.bd_r_read_support[class.value() as usize].get()
    }

    /// The Class `class` BD-ROM Read Support bitmap, which is obsolete except for the bit of
    /// [`Self::rom`].
    #[inline]
    pub const fn bd_rom_read_support(&self, class: u2) -> u16 {
        self.bd_rom_read_support[class.value() as usize].get()
    }
}

super::debug_fields!(BdRead {
    bca,
    re2,
    re1,
    r,
    rom
});
