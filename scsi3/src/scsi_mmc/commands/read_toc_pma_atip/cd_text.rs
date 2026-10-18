use core::{fmt, mem::offset_of, num::NonZeroU16};

use arbitrary_int::{u3, u4, u7};
use thiserror::Error;
use zerocopy::{
    BigEndian, CastError, FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned, U16,
};

use super::{ReadTocPmaAtip, ReadTocPmaAtipOpCode};
use crate::{
    core::{
        util::{bit, debug_fields},
        ReadCommand, TruncationError,
    },
    mmc::device_models::cd::cd_text::{crc, PackType},
};

/// The length of the header that precedes the CD-TEXT Descriptors.
const CD_TEXT_HEADER_LEN: usize = 4;

/// Why a CD-TEXT response couldn't be parsed.
#[derive(Debug, Error)]
pub enum CdTextError {
    /// The response is shorter than its header.
    #[error(transparent)]
    Truncated(#[from] TruncationError<CD_TEXT_HEADER_LEN>),
}

impl ReadCommand<ReadTocPmaAtipOpCode> for ReadTocPmaAtip<CdText> {
    type Len = u16;
    type Response<'a> = (&'a CdText, &'a [u8]);
    type Error = CdTextError;

    fn response_len(&self) -> Self::Len {
        self.allocation_length
    }

    /// Reads the CD-TEXT Descriptors that fit in both CD-TEXT Data Length and `buf`, and returns
    /// the bytes after them.
    ///
    /// Only whole descriptors are read, so a descriptor cut short begins the returned bytes.
    /// `buf` should hold only the bytes the Drive transferred. Any bytes past the transfer that
    /// still fall within CD-TEXT Data Length, such as an allocated buffer's unused tail, are read
    /// as descriptors.
    fn parse<'a>(&self, buf: &'a [u8]) -> Result<Self::Response<'a>, Self::Error> {
        // CD-TEXT Data Length counts the bytes following its own 2-byte field. A `buf` too short
        // to hold it holds no descriptors either, so 0 stands in, and the cast reports the
        // truncation.
        let cd_text_data_length = buf.first_chunk().map_or(0, |&len| u16::from_be_bytes(len));
        let elems = usize::from(cd_text_data_length.saturating_sub(2))
            .min(buf.len().saturating_sub(CD_TEXT_HEADER_LEN))
            / size_of::<CdTextDescriptor>();

        match CdText::ref_from_prefix_with_elems(buf, elems) {
            Ok(res) => Ok(res),
            Err(CastError::Size(_)) => Err(TruncationError(buf.len()).into()),
            Err(CastError::Alignment(_)) => unreachable!("`CdText` should be `Unaligned`"),
        }
    }
}

/// The response to READ TOC/PMA/ATIP with Format 0101b: the CD-TEXT Packs of the Lead-in.
#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct CdText {
    cd_text_data_length: U16<BigEndian>,
    _reserved: [u8; 2],
    cd_text_descriptors: [CdTextDescriptor],
}

impl CdText {
    /// The length of the response that follows this field.
    ///
    /// Not cut by Allocation Length, so it can exceed the bytes the Drive transferred.
    #[inline]
    pub const fn cd_text_data_length(&self) -> u16 {
        self.cd_text_data_length.get()
    }

    /// The CD-TEXT Descriptors, in the order they appear in the response.
    #[inline]
    pub const fn cd_text_descriptors(&self) -> &[CdTextDescriptor] {
        &self.cd_text_descriptors
    }
}

debug_fields!(CdText {
    cd_text_data_length,
    cd_text_descriptors
});

/// One entry of a [`CdText`]: a CD-TEXT Pack.
// NOTE: Field names follow the text of MMC-3 Annex J, the only description of the Pack format on
// hand. Its table labels ID1 through ID3 all as Pack Type Indicators.
#[doc(alias = "Pack")]
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct CdTextDescriptor {
    id1: PackType,
    id2: u8,
    id3: u8,
    id4: u8,
    // NOTE: MMC-3 Annex J says Packs other than 88h, 89h and 8Fh hold character strings, but also
    // that 86h and 87h hold binary information. The docs follow the latter.
    text_data: [u8; 12],
    crc: U16<BigEndian>,
}

impl CdTextDescriptor {
    /// What the Text Data Field holds.
    #[doc(alias = "ID1")]
    #[inline]
    pub const fn pack_type_indicator(&self) -> PackType {
        self.id1
    }

    bit!(
        /// Set when the Pack is used for an extended application, beyond the scope of MMC-3
        /// Annex J. Normally clear.
        #[doc(alias = "ID2")]
        extension_flag: id2[7]
    );

    /// The track the Text Data Field belongs to, when it belongs to one.
    ///
    /// Reads the same bits as [`Self::pack_element_number`], which applies instead when the Pack
    /// is independent of the track.
    #[doc(alias = "ID2")]
    #[inline]
    pub const fn track_number(&self) -> u7 {
        u7::extract_u8(self.id2, 0)
    }

    /// The Pack Element Number, when the Pack is independent of the track. What it means depends
    /// on the pack type.
    ///
    /// Reads the same bits as [`Self::track_number`], which applies instead when the Text Data
    /// Field belongs to a track.
    #[doc(alias = "ID2")]
    #[inline]
    pub const fn pack_element_number(&self) -> u7 {
        u7::extract_u8(self.id2, 0)
    }

    /// Counts up by one from 00h at the first Pack of each Block, up to FFh.
    #[doc(alias = "ID3")]
    #[inline]
    pub const fn sequence_number_indicator(&self) -> u8 {
        self.id3
    }

    bit!(
        /// Whether the Text Data Field holds a Double Byte Character Code rather than a Single
        /// Byte Character Code.
        #[doc(alias = "DBCC")]
        #[doc(alias = "ID4")]
        double_byte_character_code: id4[7]
    );

    /// The Block the Pack belongs to. Each Block holds the text in one language, and up to 8 can
    /// be used at the same time.
    #[doc(alias = "ID4")]
    #[inline]
    pub const fn block_number(&self) -> u3 {
        u3::extract_u8(self.id4, 4)
    }

    /// How many characters of the first string in this Pack's Text Data Field were in the
    /// previous Pack, from 0 to 15. 15 means the string began before the previous Pack.
    ///
    /// A null counts as a character, and a double-byte character counts once. Packs with ID1 of
    /// 88h, 89h and 8Fh don't use this field and set it to 0.
    #[doc(alias = "ID4")]
    #[inline]
    pub const fn character_position(&self) -> u4 {
        u4::extract_u8(self.id4, 0)
    }

    /// The Text Data Field, exactly as recorded on the disc: character strings, except in Packs
    /// with ID1 of 86h, 87h, 88h, 89h and 8Fh, which hold binary information.
    ///
    /// Each string ends in a null code, or two for double-byte codes, and continues into the next
    /// Pack if it doesn't fit. Unused bytes are null. A tab code (09h, or two for double-byte
    /// codes) in place of a string means the same string as the previous track.
    #[inline]
    pub const fn text_data(&self) -> &[u8; 12] {
        &self.text_data
    }

    /// The CRC of the rest of the Pack, or `None` if the field is zero, as it is in a Pack without
    /// one. MMC-3 Annex J makes the CRC optional.
    ///
    /// A Pack whose CRC works out to 0000h also reads as `None`.
    #[inline]
    pub const fn crc(&self) -> Option<NonZeroU16> {
        NonZeroU16::new(self.crc.get())
    }

    /// The CRC of the bytes before [`Self::crc`], which that field should hold.
    #[inline]
    pub fn calculate_crc(&self) -> u16 {
        crc(&self.as_bytes()[..offset_of!(Self, crc)])
    }

    /// Whether [`Self::crc`] matches the Pack, or `None` if the Pack has no CRC.
    #[inline]
    pub fn crc_matches(&self) -> Option<bool> {
        Some(self.calculate_crc() == self.crc()?.get())
    }
}

impl fmt::Debug for CdTextDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CdTextDescriptor")
            .field("pack_type_indicator", &self.pack_type_indicator())
            .field("extension_flag", &self.extension_flag())
            .field("track_number", &self.track_number())
            .field("pack_element_number", &self.pack_element_number())
            .field(
                "sequence_number_indicator",
                &self.sequence_number_indicator(),
            )
            .field(
                "double_byte_character_code",
                &self.double_byte_character_code(),
            )
            .field("block_number", &self.block_number())
            .field("character_position", &self.character_position())
            .field("text_data", &self.text_data())
            .field(
                "crc",
                &self
                    .crc()
                    .map(|crc| fmt::from_fn(move |f| write!(f, "{crc:04X}h"))),
            )
            .finish()
    }
}
