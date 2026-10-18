use derive_more::Debug;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// A Pack Type Indicator (ID1): what a CD-TEXT Pack's Text Data Field holds.
// NOTE: Taken from MMC-3 Annex J, the only description of CD-TEXT on hand.
#[doc(alias = "ID1")]
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(transparent)]
pub struct PackType(#[debug("{_0:02X}h")] u8);

impl PackType {
    /// Title of Album name (ID2=00h) or Track Titles (ID2=01h...63h)
    pub const TITLE: Self = Self::from_u8(0x80);
    /// Name(s) of the performer(s) (in ASCII)
    pub const PERFORMER: Self = Self::from_u8(0x81);
    /// Name(s) of the songwriter(s) (in ASCII)
    pub const SONGWRITER: Self = Self::from_u8(0x82);
    /// Name(s) of the composer(s) (in ASCII)
    pub const COMPOSER: Self = Self::from_u8(0x83);
    /// Name(s) of the arranger(s) (in ASCII)
    pub const ARRANGER: Self = Self::from_u8(0x84);
    /// Message(s) from content provider and/or artist (in ASCII)
    pub const MESSAGE: Self = Self::from_u8(0x85);
    /// Disc Identification information
    pub const DISC_IDENTIFICATION: Self = Self::from_u8(0x86);
    /// Genre Identification and Genre information
    pub const GENRE_IDENTIFICATION: Self = Self::from_u8(0x87);
    /// Table of Content information
    #[doc(alias = "TOC")]
    pub const TABLE_OF_CONTENT: Self = Self::from_u8(0x88);
    /// Second Table of Content information
    #[doc(alias = "TOC")]
    pub const SECOND_TABLE_OF_CONTENT: Self = Self::from_u8(0x89);

    // --- 8Ah - 8Ch Reserved ---

    /// Reserved for content provider only
    pub const RESERVED_FOR_CONTENT_PROVIDER: Self = Self::from_u8(0x8D);
    /// UPC/EAN code of the album and ISRC code of each track
    #[doc(alias = "UPC")]
    #[doc(alias = "EAN")]
    #[doc(alias = "ISRC")]
    pub const UPC_EAN_ISRC: Self = Self::from_u8(0x8E);
    /// Size information of the Block
    pub const SIZE_INFORMATION: Self = Self::from_u8(0x8F);

    #[inline]
    pub const fn from_u8(pack_type: u8) -> Self {
        Self(pack_type)
    }

    #[inline]
    pub const fn into_u8(self) -> u8 {
        self.0
    }
}

/// x^16 + x^12 + x^5 + 1, MSB first.
const POLY: u16 = 0x1021;

const TABLE: [u16; 256] = {
    let mut table = [0u16; 256];
    let mut i = 0usize;
    while i < 256 {
        let mut crc = (i as u16) << 8;
        let mut bit = 0;
        while bit < 8 {
            crc = if crc & 0x8000 != 0 {
                (crc << 1) ^ POLY
            } else {
                crc << 1
            };
            bit += 1;
        }
        table[i] = crc;
        i += 1;
    }
    table
};

/// The CD-TEXT CRC over `bytes`: x^16 + x^12 + x^5 + 1, with every bit of the result inverted.
// CONFIRM: MMC-3 Annex J gives only the polynomial and the inversion. This assumes a zero initial
// value and MSB-first bit order, which is CRC-16/GSM. Check it against the Packs of a real disc.
pub const fn crc(bytes: &[u8]) -> u16 {
    let mut crc = 0u16;
    let mut i = 0;
    while i < bytes.len() {
        crc = TABLE[((crc >> 8) ^ bytes[i] as u16) as usize] ^ (crc << 8);
        i += 1;
    }
    !crc
}
