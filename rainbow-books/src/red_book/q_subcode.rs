// All 9-byte DATA-Q packets should first be classified by the ADR value into one of the following
// Q modes:
// - Mode 0 (ADR: 0b0000) [IEC 60908:1999 §17.5.4]
// - Mode 1 (ADR: 0b0001) [IEC 60908:1999 §17.5.1]
// - Mode 2 (ADR: 0b0010)
// - Mode 3 (ADR: 0b0011)
// - Mode 4 (ADR: 0b0100)
// - Mode 5 (ADR: 0b0101)
// - Mode 6 (ADR: 0b0110)
// - More ???
//
// Then determine based on the TNO value of DATA-Q, which area of the disc the subcode pertains to.
// After determining the area (or if area was N/A), if applicable, identify further with the
// POINT/INDEX value.no

use bitflags::bitflags;
use derive_more::Display;

bitflags! {
    /// The Control Field has 4 bits that define the type of information in the frame.
    ///
    /// See MMC-6 §4.2.3.4, Table 17.
    /// See IEC 60908 §17.5.
    #[repr(transparent)]
    #[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Control: u8 {
        /// If set and track is audio, the track has 4 channels, otherwise 2. Not set when
        /// track is data.
        ///
        /// NOTE: 4 channel never saw real world use and was eventually dropped from the CD
        /// stanard. The 4 channel bit was also called "Broadcasting use".
        ///
        /// # Examples
        ///
        /// - 00xx = 2 audio channels
        /// - 10xx = 4 audio channels
        const FOUR_CHANNELS = 1 << 3;
        /// If set, track is data, otherwise audio.
        ///
        /// # Examples
        ///
        /// - x0xx = Audio Track
        /// - 01xx = Data Track
        const IS_DATA = 1 << 2;
        /// If set digital copy is permitted, otherwise prohibited.
        ///
        /// # Examples
        ///
        /// - xx0x = Digital copy prohibited
        /// - xx1x = Digital copy permitted
        const COPY_PERMITTED = 1 << 1;
        /// If set and track is audio, pre-emphasis is enabled. If set and track is data, track
        /// is recorded incrementally, otherwise uninterrupted.
        ///
        /// # Examples
        ///
        /// - x0x0 = Audio without pre-emphasis
        /// - x0x1 = Audio with pre-emphasis
        /// - 01x0 = Data track recorded uninterrupted
        /// - 01x1 = Data track recorded incrementally
        const PREEMPHASIS_OR_INCREMENTAL = 1 << 0;
    }
}

impl<'a> DekuReader<'a> for Control {
    fn from_reader_with_ctx<R: deku::no_std_io::Read + deku::no_std_io::Seek>(
        reader: &mut Reader<R>,
        _: (),
    ) -> Result<Self, DekuError>
    where
        Self: Sized,
    {
        Ok(Self::from_bits_retain(u8::from_reader_with_ctx(
            reader,
            BitSize(4),
        )?))
    }
}

use bcd::{bcd, Bcd};
use deku::{ctx::BitSize, deku_error, reader::Reader, DekuError, DekuReader};
use thiserror::Error;

use crate::core::Msf;

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PaTrackNumber(Bcd<1>);

impl PaTrackNumber {
    pub const MIN: Self = Self(bcd!(1));
    pub const MAX: Self = Self(bcd!(99));
}

// impl TryFrom<u8> for PaTrackNumber {
//     type Error = PaTrackNumberRangeError;

//     fn try_from(value: u8) -> Result<Self, Self::Error> {
//         if !(Self::MIN.0..=Self::MAX.0).contains(&value) {
//             return Err(PaTrackNumberRangeError(value));
//         }

//         Ok(Self(value))
//     }
// }

// impl TryFrom<Bcd2> for PaTrackNumber {
//     type Error = PaTrackNumberRangeError;

//     fn try_from(value: Bcd2) -> Result<Self, Self::Error> {
//         Self::try_from(value.value())
//     }
// }

#[derive(Debug, Error)]
#[error(
    "Invalid PaTrackNumber: {0}. Expected {min:02}-{max:02}",
    min = PaTrackNumber::MIN,
    max = PaTrackNumber::MAX
)]
pub struct PaTrackNumberRangeError(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrackNumber {
    /// The Lead-In area (0x00)
    LeadIn,
    /// An audio track number (1-99)
    ProgramArea(PaTrackNumber),
    /// The Lead-Out area (0xAA)
    LeadOut,
}

impl TrackNumber {
    const LEAD_IN_RAW: u8 = 00;
    const LEAD_OUT_RAW: u8 = 0xAA;
}

// #[derive(Debug, Error)]
// #[error(
//     "Invalid TrackNumber: {0}. Expected {li:02}, {pa_min:02}-{pa_max:02}, or {lo:02X}",
//     li = TrackNumber::LEAD_IN_RAW,
//     pa_min = 0,
//     pa_max = crate::core::Bcd1::MAX,
//     lo = TrackNumber::LEAD_OUT_RAW
// )]
// pub struct TrackNumberRangeError(u8);

// impl TryFrom<u8> for TrackNumber {
//     type Error = TrackNumberRangeError;

//     fn try_from(value: u8) -> Result<Self, Self::Error> {
//         match value {
//             Self::LEAD_IN_RAW => Ok(Self::LeadIn),
//             Self::LEAD_OUT_RAW => Ok(Self::LeadOut),
//             tno => Bcd2::try_from_bcd_byte(tno)
//                 .ok()
//                 .and_then(|bcd| PaTrackNumber::try_from(bcd).ok())
//                 .map(Self::ProgramArea)
//                 .ok_or(TrackNumberRangeError(tno)),
//         }
//     }
// }

// impl<'a> DekuReader<'a> for TrackNumber {
//     fn from_reader_with_ctx<R: deku::no_std_io::Read + deku::no_std_io::Seek>(
//         reader: &mut Reader<R>,
//         _: (),
//     ) -> Result<Self, DekuError> {
//         let raw_tno = u8::from_reader_with_ctx(reader, ())?;
//         Self::try_from(raw_tno).map_err(|e| deku_error!(DekuError::Parse, e.to_string()))
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProgramAreaFormat {
    CddaOrCdrom = 0x00,
    Cdi = 0x10,
    CdromXa = 0x20,
}

mod private {
    pub trait Sealed {}
}

pub trait DataQ: private::Sealed {
    const ADR: u8;
}

macro_rules! impl_dataq {
    ($ty:ty, $adr:expr) => {
        impl private::Sealed for $ty {}
        impl DataQ for $ty {
            const ADR: u8 = $adr;
        }
    };
}

pub struct SubcodeQ<P: DataQ> {
    pub control: Control,
    pub payload: P,
}

// ADR=1 in Program Area
pub struct TrackPosition {
    pub tno: PaTrackNumber,
    pub index: Bcd<2>,
    pub relative_time: Msf,
    pub absolute_time: Msf,
}
impl_dataq!(TrackPosition, 1);

// NOTE: Once you have all the qpayloads modeled out, don't just go an put them all in a big enum.
// Make enums as needed like for RawToc. This lets you pick and choose which QPayloads are valid.

// Mode-1 Q
pub mod mode1_q {
    use super::ProgramAreaFormat;
    use crate::core::RawMsf;
    use bcd::Bcd;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum DataQ {
        TrackEntry(TrackEntry),
        FirstTrack(FirstTrack),
        LastTrack(LastTrack),
        LeadOutStart(LeadOutStart),
        Track(Track),
        LeadOut(LeadOut),
    }

    // TNO 0x00 - POINT 01-99bcd
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct TrackEntry {
        track_number: Bcd<2>,
        lead_in_running_time: RawMsf,
        track_start_time: RawMsf,
    }

    // TNO 0x00 - POINT A0 - Track number of first track on disc
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct FirstTrack {
        lead_in_running_time: RawMsf,
        first_track_number: Bcd<2>,
        program_area_format: ProgramAreaFormat,
    }

    // TNO 0x00 - POINT A1 - Track number of last track on disc
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct LastTrack {
        lead_in_running_time: RawMsf,
        last_track_number: Bcd<2>,
    }

    // TNO 0x00 - POINT A2 - Staring point of lead out track
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct LeadOutStart {
        lead_in_running_time: RawMsf,
        lead_out_start_time: RawMsf,
    }

    // TNO 01-99bcd - INDEX 00-99bcd
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Track {
        track_number: Bcd<2>,
        index: u8,
        relative_time: RawMsf,
        absolute_time: RawMsf,
    }

    // TNO 0xAA - INDEX 01bcd
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct LeadOut {
        relative_time: RawMsf,
        absolute_time: RawMsf,
    }
}

pub mod mode2_q {
    // Technically the AFRAME byte of Mode-2 and Mode-3 Q is defined, but
    // I can't imagine it being particullarly useful.

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct MediaCatalogNumber(String);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Isrc {
    country_code: [char; 2],
    owner_code: [char; 3],
    year_of_recording: u8,
    serial_number: u32,
}

pub mod mode5_q {
    use crate::core::RawMsf;
    use bcd::Bcd;

    // POINT 01h-40h (Audio only: This identifies a specific playback skip interval)
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct SkipInterval {
        stop_time: RawMsf,
        start_time: RawMsf,
    }

    // POINT B0h (multi-session disc)
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct NextSessionInfo {
        next_session_start_time: Option<RawMsf>,
        mode5_pointer_count: u8,
        outermost_lead_out_start_time: RawMsf,
    }

    // POINT B1h (Audio only: This identifies the presence of skip intervals)
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct SkipIntervalSummary {
        skip_interval_count: u8,
        skip_track_count: u8,
    }

    // POINT B2h, B3h, B4h (Audio only: This identifies tracks that should be skipped during playback)
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct SkipTrackAssignment {
        tracks: Vec<Bcd<2>>,
    }

    // POINT C0h (Together with POINT=B0h, this is used to identify a multi-session disc)
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct AtipInfo {
        atip_special_info: [u8; 3],
        first_lead_in_start_time: RawMsf,
    }

    // POINT C1h (Copy of information available in additional information 1 from the ATIP)
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct AdditionalInformation1([u8; 7]);
}
