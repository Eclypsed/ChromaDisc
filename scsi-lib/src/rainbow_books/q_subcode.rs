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
use deku::{ctx::BitSize, reader::Reader, DekuError, DekuReader};
use thiserror::Error;

use crate::core::msf::Msf;

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TrackNumber(Bcd<1>);

impl TrackNumber {
    pub const MIN: Self = Self(bcd!(1));
    pub const MAX: Self = Self(bcd!(99));
}

#[derive(Debug, Error)]
#[error(
    "Invalid TrackNumber: {0}. Expected {min:02}-{max:02}",
    min = TrackNumber::MIN,
    max = TrackNumber::MAX
)]
pub struct TrackNumberRangeError(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrackAddress {
    /// The Lead-In area (0x00)
    LeadIn,
    /// An audio track number (1-99)
    ProgramArea(TrackNumber),
    /// The Lead-Out area (0xAA)
    LeadOut,
}

impl TrackAddress {
    const LEAD_IN_RAW: u8 = 00;
    const LEAD_OUT_RAW: u8 = 0xAA;
}

#[derive(Debug, Error)]
#[error(
    "Invalid TrackNumber: {0}. Expected {li:02}, {pa_min:02}-{pa_max:02}, or {lo:02X}",
    li = TrackAddress::LEAD_IN_RAW,
    pa_min = TrackNumber::MIN,
    pa_max = TrackNumber::MAX,
    lo = TrackAddress::LEAD_OUT_RAW
)]
pub struct TrackAddressRangeError(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProgramAreaFormat {
    CddaOrCdrom = 0x00,
    Cdi = 0x10,
    CdromXa = 0x20,
}

pub enum TrackStopTime {
    Complete(Msf),
    Incomplete,
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
    pub tno: TrackNumber,
    pub index: Bcd<1>,
    pub relative_time: Msf,
    pub absolute_time: Msf,
}
impl_dataq!(TrackPosition, 1);

pub struct MediaCatalogNumber {
    pub mcn: String,
    pub aframe: Bcd<1>,
}

// NOTE: Once you have all the qpayloads modeled out, don't just go an put them all in a big enum.
// Make enums as needed like for RawToc. This lets you pick and choose which QPayloads are valid.
