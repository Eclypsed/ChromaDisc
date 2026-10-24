use core::fmt;

use zerocopy::{
    BigEndian, Immutable, IntoBytes, KnownLayout, TryFromBytes, U16, Unaligned, ValidityError,
    try_transmute_ref,
};

use crate::{
    core::util::debug_fields,
    mmc::device_models::{
        cd::{
            addressing::UnvalidatedMsf,
            atip::{cdr, cdrw},
        },
        ddcd::atip as ddcd,
    },
};

// TODO: Implement `ReadCommand` for the ATIP format, once the type `parse` returns is settled.
// `parse` would read the DDCD bit, Disc Type and Disc Sub-Type to pick the one layout below that
// can describe the response, then try-cast `buf` as it.

/// An accessor for one Additional Information section of a [`CdAtip`]: `None` when its Valid bit
/// is clear, or the section read as `$ty`.
// TODO: Settle the return type. These return a `Result` because the sections are bytes, checked
// only here. Once they can be typed fields (see the CONFIRM on `CdAtip`), the cast checks them,
// and these can return `Option<&$ty>`.
macro_rules! additional_information {
    ($name:ident: $ty:ty, $valid:ident, $n:literal) => {
        #[doc = concat!("Additional Information ", $n, ", or `None` if A", $n, " Valid is clear.")]
        ///
        #[doc = concat!(
            "An error if A", $n, " Valid is set, but the section's ID bits aren't those of ",
            "Additional Information ", $n, "."
        )]
        #[inline]
        pub fn $name(&self) -> Result<Option<&$ty>, ValidityError<&[u8; 3], $ty>> {
            if self.special_information_1.$valid() {
                try_transmute_ref!(&self.$name).map(Some)
            } else {
                Ok(None)
            }
        }
    };
}

/// Prints three bytes in hex, as the spec's tables write them.
fn hex(&[a, b, c]: &[u8; 3]) -> impl fmt::Debug {
    fmt::from_fn(move |f| write!(f, "[{a:02X}h, {b:02X}h, {c:02X}h]"))
}

/// The ATIP of a CD-R or CD-RW, with Special Information 1 read as `S1`.
///
/// Each medium's Special Information 1 accepts only its own ID bits, Disc Type and Disc Sub-Type,
/// with the DDCD bit clear, so `S1` decides which medium this is the ATIP of. Each alias, such as
/// [`CdRAtip`], reads the Additional Information for its medium.
///
/// The Additional Information is kept as bytes. Its accessors check a section's ID bits only when
/// its Valid bit is set.
///
/// The response ends after the A3 Values. MMC-6 also places S4 Values in bytes 28-30, which no
/// revision of MMC explains. When a Drive returns them, they begin the bytes after the response.
// CONFIRM: Whether a Drive sets the ID bits of an Additional Information section whose Valid bit
// is clear. If it does, the sections can be typed fields, which the cast checks.
//
// NOTE: MMC-6 lays out 32 bytes, but this layout stops at byte 27, so `size_of::<CdAtip<_>>()` is
// 28. mmc3r02 added bytes 28-31 without saying what S4 holds or that the response grew. It's also
// the first revision to describe DDCD media in this response, and mmc4r05a says S4 is available
// for both DDCD-R and DDCD-RW, so `DdcdAtip` reads them. The ASUS DRW-24B1ST j (firmware 1.11)
// returns an ATIP Data Length of 26, ending the response at byte 27.
//
// NOTE: On the medium, the two start times are Special Information 2 and 3, in BCD, with bit 7 of
// each byte holding an ID bit instead. The Orange Book decodes those bits as 1, 0 and 0 in the
// Start Time of Lead-in, and as 0, 0 and 0 in the Last Possible Start Time of Lead-out. MMC-6
// doesn't say how the Drive returns the times, and earlier revisions say they're returned in "hex
// format", meaning binary. The ASUS DRW-24B1ST j (firmware 1.11) returns them in binary, with the
// ID bits decoded.
#[derive(
    TryFromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct CdAtip<S1> {
    atip_data_length: U16<BigEndian>,
    _bytes2_3: [u8; 2],
    special_information_1: S1,
    _byte7: u8,
    start_time_of_lead_in: UnvalidatedMsf,
    _byte11: u8,
    last_possible_start_time_of_lead_out: UnvalidatedMsf,
    _byte15: u8,
    additional_information_1: [u8; 3],
    _byte19: u8,
    additional_information_2: [u8; 3],
    _byte23: u8,
    additional_information_3: [u8; 3],
    _byte27: u8,
}

impl<S1> CdAtip<S1> {
    /// The length of the response that follows this field.
    pub const fn atip_data_length(&self) -> u16 {
        self.atip_data_length.get()
    }

    pub const fn special_information_1(&self) -> &S1 {
        &self.special_information_1
    }

    /// The ATIP Start Time of Lead-in.
    pub const fn start_time_of_lead_in(&self) -> UnvalidatedMsf {
        self.start_time_of_lead_in
    }

    /// The ATIP Last Possible Start Time of Lead-out.
    pub const fn last_possible_start_time_of_lead_out(&self) -> UnvalidatedMsf {
        self.last_possible_start_time_of_lead_out
    }
}

/// The ATIP of a CD-R.
pub type CdRAtip = CdAtip<cdr::SpecialInformation1>;

impl CdRAtip {
    additional_information!(additional_information_1: cdr::AdditionalInformation1, a1_valid, 1);
    additional_information!(additional_information_2: cdr::AdditionalInformation2, a2_valid, 2);
    additional_information!(additional_information_3: cdr::AdditionalInformation3, a3_valid, 3);
}

debug_fields!(CdRAtip {
    atip_data_length,
    special_information_1,
    start_time_of_lead_in,
    last_possible_start_time_of_lead_out,
    additional_information_1,
    additional_information_2,
    additional_information_3,
});

/// The ATIP of a Standard Speed CD-RW. It reserves Additional Information 3.
pub type CdRwStandardAtip = CdAtip<cdrw::standard::SpecialInformation1>;

impl CdRwStandardAtip {
    additional_information!(
        additional_information_1: cdrw::standard::AdditionalInformation1,
        a1_valid,
        1
    );
    additional_information!(
        additional_information_2: cdrw::standard::AdditionalInformation2,
        a2_valid,
        2
    );
}

debug_fields!(CdRwStandardAtip {
    atip_data_length,
    special_information_1,
    start_time_of_lead_in,
    last_possible_start_time_of_lead_out,
    additional_information_1,
    additional_information_2,
});

/// The ATIP of a High Speed CD-RW.
pub type CdRwHighSpeedAtip = CdAtip<cdrw::high_speed::SpecialInformation1>;

impl CdRwHighSpeedAtip {
    additional_information!(
        additional_information_1: cdrw::high_speed::AdditionalInformation1,
        a1_valid,
        1
    );
    additional_information!(
        additional_information_2: cdrw::high_speed::AdditionalInformation2,
        a2_valid,
        2
    );
    additional_information!(
        additional_information_3: cdrw::high_speed::AdditionalInformation3,
        a3_valid,
        3
    );
}

debug_fields!(CdRwHighSpeedAtip {
    atip_data_length,
    special_information_1,
    start_time_of_lead_in,
    last_possible_start_time_of_lead_out,
    additional_information_1,
    additional_information_2,
    additional_information_3,
});

/// The ATIP of an Ultra Speed CD-RW.
pub type CdRwUltraSpeedAtip = CdAtip<cdrw::ultra_speed::SpecialInformation1>;

impl CdRwUltraSpeedAtip {
    additional_information!(
        additional_information_1: cdrw::ultra_speed::AdditionalInformation1,
        a1_valid,
        1
    );
    additional_information!(
        additional_information_2: cdrw::ultra_speed::AdditionalInformation2,
        a2_valid,
        2
    );
    additional_information!(
        additional_information_3: cdrw::ultra_speed::AdditionalInformation3,
        a3_valid,
        3
    );
}

debug_fields!(CdRwUltraSpeedAtip {
    atip_data_length,
    special_information_1,
    start_time_of_lead_in,
    last_possible_start_time_of_lead_out,
    additional_information_1,
    additional_information_2,
    additional_information_3,
});

/// The ATIP of an Ultra Speed+ CD-RW. It reserves Additional Information 1.
pub type CdRwUltraSpeedPlusAtip = CdAtip<cdrw::ultra_speed_plus::SpecialInformation1>;

impl CdRwUltraSpeedPlusAtip {
    additional_information!(
        additional_information_2: cdrw::ultra_speed_plus::AdditionalInformation2,
        a2_valid,
        2
    );
    additional_information!(
        additional_information_3: cdrw::ultra_speed_plus::AdditionalInformation3,
        a3_valid,
        3
    );
}

debug_fields!(CdRwUltraSpeedPlusAtip {
    atip_data_length,
    special_information_1,
    start_time_of_lead_in,
    last_possible_start_time_of_lead_out,
    additional_information_2,
    additional_information_3,
});

/// The ATIP of a CD-RW whose Disc Sub-Type is reserved, from 100b to 111b.
///
/// The Orange Book doesn't lay out its Additional Information, so the sections are read as bytes,
/// and their ID bits aren't checked.
pub type CdRwReservedAtip = CdAtip<cdrw::reserved::SpecialInformation1>;

impl CdRwReservedAtip {
    /// The A1 Values, or `None` if A1 Valid is clear.
    #[inline]
    pub const fn a1_values(&self) -> Option<&[u8; 3]> {
        if self.special_information_1.a1_valid() {
            Some(&self.additional_information_1)
        } else {
            None
        }
    }

    /// The A2 Values, or `None` if A2 Valid is clear.
    #[inline]
    pub const fn a2_values(&self) -> Option<&[u8; 3]> {
        if self.special_information_1.a2_valid() {
            Some(&self.additional_information_2)
        } else {
            None
        }
    }

    /// The A3 Values, or `None` if A3 Valid is clear.
    #[inline]
    pub const fn a3_values(&self) -> Option<&[u8; 3]> {
        if self.special_information_1.a3_valid() {
            Some(&self.additional_information_3)
        } else {
            None
        }
    }
}

/// Prints the A1 to A3 Values in hex, as the spec's tables write them.
impl fmt::Debug for CdRwReservedAtip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CdRwReservedAtip")
            .field("atip_data_length", &self.atip_data_length())
            .field("special_information_1", self.special_information_1())
            .field("start_time_of_lead_in", &self.start_time_of_lead_in())
            .field(
                "last_possible_start_time_of_lead_out",
                &self.last_possible_start_time_of_lead_out(),
            )
            .field("a1_values", &self.a1_values().map(hex))
            .field("a2_values", &self.a2_values().map(hex))
            .field("a3_values", &self.a3_values().map(hex))
            .finish()
    }
}

/// The ATIP of a DDCD-R or DDCD-RW, as mmc4r05a describes it.
///
/// mmc4r05a doesn't lay out the A1 to A3 Values or the S4 Values for DDCD media, so they're read
/// as bytes, and their ID bits aren't checked.
// NOTE: mmc4r05a says that for DDCD media, the start times are returned in binary, not in the HMSF
// format.
#[derive(
    TryFromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct DdcdAtip {
    atip_data_length: U16<BigEndian>,
    _bytes2_3: [u8; 2],
    special_information_1: ddcd::SpecialInformation1,
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
    s4_values: [u8; 3],
    _byte31: u8,
}

impl DdcdAtip {
    /// The length of the response that follows this field.
    #[inline]
    pub const fn atip_data_length(&self) -> u16 {
        self.atip_data_length.get()
    }

    #[inline]
    pub const fn special_information_1(&self) -> &ddcd::SpecialInformation1 {
        &self.special_information_1
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

    /// The A1 Values, or `None` if A1 Valid is clear.
    #[inline]
    pub const fn a1_values(&self) -> Option<&[u8; 3]> {
        if self.special_information_1.a1_valid() {
            Some(&self.a1_values)
        } else {
            None
        }
    }

    /// The A2 Values, or `None` if A2 Valid is clear.
    #[inline]
    pub const fn a2_values(&self) -> Option<&[u8; 3]> {
        if self.special_information_1.a2_valid() {
            Some(&self.a2_values)
        } else {
            None
        }
    }

    /// The A3 Values, or `None` if A3 Valid is clear.
    #[inline]
    pub const fn a3_values(&self) -> Option<&[u8; 3]> {
        if self.special_information_1.a3_valid() {
            Some(&self.a3_values)
        } else {
            None
        }
    }

    #[inline]
    pub const fn s4_values(&self) -> &[u8; 3] {
        &self.s4_values
    }
}

/// Prints the A1 to A3 and S4 Values in hex, as the spec's tables write them.
impl fmt::Debug for DdcdAtip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DdcdAtip")
            .field("atip_data_length", &self.atip_data_length())
            .field("special_information_1", self.special_information_1())
            .field("start_time_of_lead_in", &self.start_time_of_lead_in())
            .field(
                "last_possible_start_time_of_lead_out",
                &self.last_possible_start_time_of_lead_out(),
            )
            .field("a1_values", &self.a1_values().map(hex))
            .field("a2_values", &self.a2_values().map(hex))
            .field("a3_values", &self.a3_values().map(hex))
            .field("s4_values", &hex(self.s4_values()))
            .finish()
    }
}
