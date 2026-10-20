use core::fmt;

use arbitrary_int::u3;
use bitfield::Bits;
use thiserror::Error;
use zerocopy::{
    BigEndian, CastError, FromBytes, Immutable, IntoBytes, KnownLayout, U16, Unaligned,
    transmute_ref,
};

use super::{ReadTocPmaAtip, ReadTocPmaAtipOpCode};
use crate::{
    core::{ReadCommand, TruncationError},
    mmc::device_models::{
        cd::{
            addressing::UnvalidatedMsf,
            atip::{CdRwSubtype, cdr, cdrw},
        },
        ddcd::atip as ddcd,
    },
};

/// Why an ATIP response couldn't be parsed.
#[derive(Debug, Error)]
pub enum AtipError {
    /// `buf` is shorter than the response's fixed length.
    #[error(transparent)]
    Truncated(#[from] TruncationError),
}

impl ReadCommand<ReadTocPmaAtipOpCode> for ReadTocPmaAtip<Atip> {
    type Len = u16;
    type Response<'a> = (&'a Atip, &'a [u8]);
    type Error = AtipError;

    fn response_len(&self) -> Self::Len {
        self.allocation_length
    }

    /// Reads the first 28 bytes of `buf` as the response, and returns the bytes after them.
    ///
    /// The response has a fixed length, so ATIP Data Length isn't used to find where it ends. The
    /// bytes of `buf` past byte 27 are returned, including the S4 Values when a Drive sends them.
    fn parse<'a>(&self, buf: &'a [u8]) -> Result<Self::Response<'a>, Self::Error> {
        match Atip::ref_from_prefix(buf) {
            Ok(res) => Ok(res),
            Err(CastError::Size(_)) => Err(TruncationError {
                expected: size_of::<Atip>(),
                received: buf.len(),
            }
            .into()),
            Err(CastError::Alignment(_)) => unreachable!("`Atip` should be `Unaligned`"),
        }
    }
}

/// The response to READ TOC/PMA/ATIP with Format 0100b: the ATIP of a recordable CD or DDCD, as
/// read from its Lead-in.
///
/// The accessors read the fields as MMC defines them. What most of them mean depends on the
/// medium, and [`Self::medium`] decodes them for it.
///
/// The response ends after the A3 Values. MMC-6 also places S4 Values in bytes 28-30, which no
/// revision of MMC explains. When a Drive returns them, they begin the bytes after the response.
// NOTE: MMC-6 lays out 32 bytes, but this layout stops at byte 27, so `size_of::<Atip>()` is 28.
// mmc3r02 added bytes 28-31 without saying what S4 holds or that the response grew. It's also the
// first revision to describe DDCD media in this response, and mmc4r05a says S4 is available for
// both DDCD-R and DDCD-RW. The ASUS DRW-24B1ST j (firmware 1.11) returns an ATIP Data Length of
// 26, ending the response at byte 27.
//
// NOTE: On the medium, the two start times are Special Information 2 and 3, in BCD, with bit 7 of
// each byte holding an ID bit instead. The Orange Book decodes those bits as 1, 0 and 0 in the
// Start Time of Lead-in, and as 0, 0 and 0 in the Last Possible Start Time of Lead-out. MMC-6
// doesn't say how the Drive returns the times. Earlier revisions say they're returned in "hex
// format", meaning binary, and mmc4r05a says that for DDCD media they're binary and not in the
// HMSF format. The ASUS DRW-24B1ST j (firmware 1.11) returns them in binary, with the ID bits
// decoded.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct Atip {
    atip_data_length: U16<BigEndian>,
    _bytes2_3: [u8; 2],
    special_information_1: [u8; 3],
    _byte7: u8,
    start_time_of_lead_in: UnvalidatedMsf,
    _byte11: u8,
    last_possible_start_time_of_lead_out: UnvalidatedMsf,
    _byte15: u8,
    a1_values: [u8; 3],
    _byte19: u8,
    a2_values: [u8; 3],
    _byte23: u8,
    a3_values: [u8; 3],
    _byte27: u8,
}

impl Atip {
    /// The length of the response that follows this field.
    #[inline]
    pub const fn atip_data_length(&self) -> u16 {
        self.atip_data_length.get()
    }

    /// The medium's recommended initial laser power setting, as a code whose meaning depends on
    /// the medium.
    // NOTE: For DDCD media, mmc4r05a says that unlike a CD's, this field "is contained within the
    // entire field", without saying which bits that covers.
    #[inline]
    pub const fn indicative_target_writing_power(&self) -> u3 {
        u3::extract_u8(self.special_information_1[0], 4)
    }

    /// Set when the medium is a DDCD-R or DDCD-RW.
    ///
    /// Obsolete. Revisions after MMC-4 drop DDCD, and MMC-6 reserves this bit.
    #[inline]
    pub const fn ddcd(&self) -> bool {
        self.special_information_1[0] & 1 << 3 != 0
    }

    /// The medium's recommended write speed, as a code whose meaning depends on the medium.
    #[inline]
    pub const fn reference_speed(&self) -> u3 {
        u3::extract_u8(self.special_information_1[0], 0)
    }

    /// Set when the disc is defined for unrestricted use.
    ///
    /// For a CD-R or CD-RW, this is the top bit of the Disc Application Code that
    /// [`Self::medium`] reads.
    // NOTE: MMC defines only bit 6 of byte 5, as Unrestricted Use Disc, and reserves bits 5-0. The
    // Orange Book defines bits 6-0 as the Disc Application Code.
    #[doc(alias = "URU")]
    #[inline]
    pub const fn unrestricted_use_disc(&self) -> bool {
        self.special_information_1[1] & 1 << 6 != 0
    }

    /// Clear for a CD-R or DDCD-R, and set for a CD-RW or DDCD-RW.
    #[inline]
    pub const fn disc_type(&self) -> bool {
        self.special_information_1[2] & 1 << 6 != 0
    }

    /// The kind of CD-R or CD-RW. Reserved for DDCD media.
    #[inline]
    pub const fn disc_sub_type(&self) -> u3 {
        u3::extract_u8(self.special_information_1[2], 3)
    }

    /// Set when [`Self::a1_values`] are valid.
    #[inline]
    pub const fn a1_valid(&self) -> bool {
        self.special_information_1[2] & 1 << 2 != 0
    }

    /// Set when [`Self::a2_values`] are valid.
    #[inline]
    pub const fn a2_valid(&self) -> bool {
        self.special_information_1[2] & 1 << 1 != 0
    }

    /// Set when [`Self::a3_values`] are valid.
    #[inline]
    pub const fn a3_valid(&self) -> bool {
        self.special_information_1[2] & 1 != 0
    }

    /// The ATIP Start Time of Lead-in.
    #[inline]
    pub const fn start_time_of_lead_in(&self) -> UnvalidatedMsf {
        self.start_time_of_lead_in
    }

    /// The ATIP Last Possible Start Time of Lead-out.
    #[inline]
    pub const fn last_possible_start_time_of_lead_out(&self) -> UnvalidatedMsf {
        self.last_possible_start_time_of_lead_out
    }

    /// The A1 Values, as their three raw bytes. They're valid only when [`Self::a1_valid`] is set.
    #[inline]
    pub const fn a1_values(&self) -> &[u8; 3] {
        &self.a1_values
    }

    /// The A2 Values, as their three raw bytes. They're valid only when [`Self::a2_valid`] is set.
    #[inline]
    pub const fn a2_values(&self) -> &[u8; 3] {
        &self.a2_values
    }

    /// The A3 Values, as their three raw bytes. They're valid only when [`Self::a3_valid`] is set.
    #[inline]
    pub const fn a3_values(&self) -> &[u8; 3] {
        &self.a3_values
    }

    /// Special Information 1 and the A1 to A3 Values, decoded for the medium that
    /// [`Self::ddcd`], [`Self::disc_type`] and [`Self::disc_sub_type`] select.
    #[inline]
    pub fn medium(&self) -> AtipMedium<'_> {
        let si1 = &self.special_information_1;
        let a1 = self.a1_valid().then_some(&self.a1_values);
        let a2 = self.a2_valid().then_some(&self.a2_values);
        let a3 = self.a3_valid().then_some(&self.a3_values);

        if self.ddcd() {
            return if self.disc_type() {
                AtipMedium::DdcdRw {
                    special_information_1: transmute_ref!(si1),
                    a1_values: a1,
                    a2_values: a2,
                    a3_values: a3,
                }
            } else {
                AtipMedium::DdcdR {
                    special_information_1: transmute_ref!(si1),
                    a1_values: a1,
                    a2_values: a2,
                    a3_values: a3,
                }
            };
        }

        if !self.disc_type() {
            return AtipMedium::CdR {
                special_information_1: transmute_ref!(si1),
                additional_information_1: a1.map(|a1| transmute_ref!(a1)),
                additional_information_2: a2.map(|a2| transmute_ref!(a2)),
                additional_information_3: a3.map(|a3| transmute_ref!(a3)),
            };
        }

        match Bits::<CdRwSubtype>::new(self.disc_sub_type()).get() {
            Ok(CdRwSubtype::Standard) => AtipMedium::CdRwStandard {
                special_information_1: transmute_ref!(si1),
                additional_information_1: a1.map(|a1| transmute_ref!(a1)),
                additional_information_2: a2.map(|a2| transmute_ref!(a2)),
            },
            Ok(CdRwSubtype::HighSpeed) => AtipMedium::CdRwHighSpeed {
                special_information_1: transmute_ref!(si1),
                additional_information_1: a1.map(|a1| transmute_ref!(a1)),
                additional_information_2: a2.map(|a2| transmute_ref!(a2)),
                additional_information_3: a3.map(|a3| transmute_ref!(a3)),
            },
            Ok(CdRwSubtype::UltraSpeed) => AtipMedium::CdRwUltraSpeed {
                special_information_1: transmute_ref!(si1),
                additional_information_1: a1.map(|a1| transmute_ref!(a1)),
                additional_information_2: a2.map(|a2| transmute_ref!(a2)),
                additional_information_3: a3.map(|a3| transmute_ref!(a3)),
            },
            Ok(CdRwSubtype::UltraSpeedPlus) => AtipMedium::CdRwUltraSpeedPlus {
                special_information_1: transmute_ref!(si1),
                additional_information_2: a2.map(|a2| transmute_ref!(a2)),
                additional_information_3: a3.map(|a3| transmute_ref!(a3)),
            },
            Err(_) => AtipMedium::CdRwReserved(self.disc_sub_type()),
        }
    }
}

/// Prints the A1 to A3 Values in hex, as the spec's tables write them, and what
/// [`Atip::medium`] decodes after them.
impl fmt::Debug for Atip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hex = |&[a, b, c]: &[u8; 3]| {
            fmt::from_fn(move |f| write!(f, "[{a:02X}h, {b:02X}h, {c:02X}h]"))
        };

        f.debug_struct("Atip")
            .field("atip_data_length", &self.atip_data_length())
            .field(
                "indicative_target_writing_power",
                &self.indicative_target_writing_power(),
            )
            .field("ddcd", &self.ddcd())
            .field("reference_speed", &self.reference_speed())
            .field("unrestricted_use_disc", &self.unrestricted_use_disc())
            .field("disc_type", &self.disc_type())
            .field("disc_sub_type", &self.disc_sub_type())
            .field("a1_valid", &self.a1_valid())
            .field("a2_valid", &self.a2_valid())
            .field("a3_valid", &self.a3_valid())
            .field("start_time_of_lead_in", &self.start_time_of_lead_in())
            .field(
                "last_possible_start_time_of_lead_out",
                &self.last_possible_start_time_of_lead_out(),
            )
            .field("a1_values", &hex(self.a1_values()))
            .field("a2_values", &hex(self.a2_values()))
            .field("a3_values", &hex(self.a3_values()))
            .field("medium", &self.medium())
            .finish()
    }
}

/// Special Information 1 and the A1 to A3 Values of an [`Atip`], decoded for the medium it
/// describes.
///
/// Each section is three bytes, M, S and F, and bit 7 of each byte holds the section's ID, which
/// the views don't read. A section the medium reserves has no field, and an Additional
/// Information field is `None` when its Valid bit is clear.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AtipMedium<'a> {
    /// A CD-R: Disc Type 0.
    CdR {
        special_information_1: &'a cdr::SpecialInformation1,
        additional_information_1: Option<&'a cdr::AdditionalInformation1>,
        additional_information_2: Option<&'a cdr::AdditionalInformation2>,
        additional_information_3: Option<&'a cdr::AdditionalInformation3>,
    },
    /// A Standard CD-RW: Disc Type 1, Disc Sub-Type 000b. It reserves Additional Information 3.
    CdRwStandard {
        special_information_1: &'a cdrw::standard::SpecialInformation1,
        additional_information_1: Option<&'a cdrw::standard::AdditionalInformation1>,
        additional_information_2: Option<&'a cdrw::standard::AdditionalInformation2>,
    },
    /// A High Speed CD-RW: Disc Type 1, Disc Sub-Type 001b.
    CdRwHighSpeed {
        special_information_1: &'a cdrw::high_speed::SpecialInformation1,
        additional_information_1: Option<&'a cdrw::high_speed::AdditionalInformation1>,
        additional_information_2: Option<&'a cdrw::high_speed::AdditionalInformation2>,
        additional_information_3: Option<&'a cdrw::high_speed::AdditionalInformation3>,
    },
    /// An Ultra Speed CD-RW: Disc Type 1, Disc Sub-Type 010b.
    CdRwUltraSpeed {
        special_information_1: &'a cdrw::ultra_speed::SpecialInformation1,
        additional_information_1: Option<&'a cdrw::ultra_speed::AdditionalInformation1>,
        additional_information_2: Option<&'a cdrw::ultra_speed::AdditionalInformation2>,
        additional_information_3: Option<&'a cdrw::ultra_speed::AdditionalInformation3>,
    },
    /// An Ultra Speed+ CD-RW: Disc Type 1, Disc Sub-Type 011b. It reserves Additional
    /// Information 1.
    CdRwUltraSpeedPlus {
        special_information_1: &'a cdrw::ultra_speed_plus::SpecialInformation1,
        additional_information_2: Option<&'a cdrw::ultra_speed_plus::AdditionalInformation2>,
        additional_information_3: Option<&'a cdrw::ultra_speed_plus::AdditionalInformation3>,
    },
    /// A CD-RW of a reserved Disc Sub-Type, from 100b to 111b, whose layout is unknown.
    CdRwReserved(u3),
    /// A DDCD-R: the obsolete [`Atip::ddcd`] set, Disc Type 0.
    ///
    /// mmc4r05a doesn't lay out the A1 to A3 Values for DDCD media, so they're left as bytes.
    DdcdR {
        special_information_1: &'a ddcd::SpecialInformation1,
        a1_values: Option<&'a [u8; 3]>,
        a2_values: Option<&'a [u8; 3]>,
        a3_values: Option<&'a [u8; 3]>,
    },
    /// A DDCD-RW: the obsolete [`Atip::ddcd`] set, Disc Type 1.
    ///
    /// mmc4r05a doesn't lay out the A1 to A3 Values for DDCD media, so they're left as bytes.
    DdcdRw {
        special_information_1: &'a ddcd::SpecialInformation1,
        a1_values: Option<&'a [u8; 3]>,
        a2_values: Option<&'a [u8; 3]>,
        a3_values: Option<&'a [u8; 3]>,
    },
}
