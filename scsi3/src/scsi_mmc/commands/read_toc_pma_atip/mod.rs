use core::marker::PhantomData;

use arbitrary_int::u4;
use derive_where::derive_where;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

use crate::{
    core::{addressing::Lba, Command, Control, OpCode, OpCodeDef},
    mmc::device_models::ddcd::addressing::Hmsf,
};

pub mod atip;
pub mod cd_text;
pub mod formatted_toc;
pub mod multi_session_info;
pub mod pma;
pub mod raw_toc;

mod private {
    pub trait ReadTocPmaAtipFormat {
        const MSF: bool;
        const FORMAT: super::u4;
    }

    pub trait AddressingMode {
        const MSF: bool;
    }
}

/// READ TOC/PMA/ATIP (43h): requests the TOC, PMA, ATIP or CD-TEXT of a CD, in the response
/// format `R`.
///
/// `R` sets the Format and MSF fields of the CDB, and the layout the response is read as. A
/// response longer than Allocation Length is cut short.
// #[derive_where] because it means the trait bounds don't get applied to R
#[derive_where(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReadTocPmaAtip<R: ReadTocPmaAtipResponse + ?Sized> {
    _response_marker: PhantomData<R>,
    track_session_number: u8,
    allocation_length: u16,
    control: Control,
}

/// A response format of READ TOC/PMA/ATIP.
pub trait ReadTocPmaAtipResponse: private::ReadTocPmaAtipFormat {}

/// The form the Formatted TOC and Multi-session Information formats report a track's start
/// address in: an [`Lba`] when the MSF bit is zero, or an [`Hmsf`] when it's one.
pub trait TrackStartAddress:
    private::AddressingMode
    + FromBytes
    + IntoBytes
    + KnownLayout
    + Immutable
    + Unaligned
    + Copy
    + 'static
{
}

impl private::AddressingMode for Lba {
    const MSF: bool = false;
}
impl TrackStartAddress for Lba {}

// NOTE: MMC-6 says the Formatted TOC descriptor's values are binary, and says nothing about
// Multi-session Information's. The ASUS DRW-24B1ST j (firmware 1.11) returns binary for both.
impl private::AddressingMode for Hmsf {
    const MSF: bool = true;
}
impl TrackStartAddress for Hmsf {}

/// The accessors that Raw TOC and PMA descriptors share, for the Q sub-channel fields from ADR
/// through POINT.
macro_rules! q_sub_channel_fields {
    () => {
        /// The mode of this Q sub-channel entry, which decides what POINT and the fields after it
        /// hold.
        #[inline]
        pub const fn adr(&self) -> u4 {
            u4::extract_u8(self.adr_control, 4)
        }

        /// The attributes of the track, as its Q sub-channel reports them.
        #[inline]
        pub const fn control(&self) -> q_subcode::Control {
            q_subcode::Control::from_bits_truncate(self.adr_control)
        }

        /// The track number of the area this entry was recorded in: 00h in the Lead-in and the
        /// PMA.
        #[inline]
        pub const fn tno(&self) -> u8 {
            self.tno
        }

        /// What this entry describes, which depends on ADR: a track number, for example, or a
        /// pointer such as A0h in the Lead-in.
        #[inline]
        pub const fn point(&self) -> u8 {
            self.point
        }

        /// Adds the Q sub-channel fields, in hex as the spec's tables write them, including those
        /// without accessors.
        fn debug_q_sub_channel(&self, s: &mut fmt::DebugStruct<'_, '_>) {
            s.field("adr", &self.adr())
                .field("control", &self.control())
                .field("tno", &format_args!("{:02X}h", self.tno()))
                .field("point", &format_args!("{:02X}h", self.point()))
                .field("min", &format_args!("{:02X}h", self.min))
                .field("sec", &format_args!("{:02X}h", self.sec))
                .field("frame", &format_args!("{:02X}h", self.frame))
                .field("zero", &format_args!("{:02X}h", self.zero))
                .field("pmin", &format_args!("{:02X}h", self.pmin))
                .field("psec", &format_args!("{:02X}h", self.psec))
                .field("pframe", &format_args!("{:02X}h", self.pframe));
        }
    };
}

use q_sub_channel_fields;

// Formatted TOC
impl<A: TrackStartAddress> private::ReadTocPmaAtipFormat for formatted_toc::FormattedToc<A> {
    const MSF: bool = A::MSF;
    const FORMAT: u4 = u4::new(0b0000);
}
impl<A: TrackStartAddress> ReadTocPmaAtipResponse for formatted_toc::FormattedToc<A> {}

// Multi-session Information
impl<A: TrackStartAddress> private::ReadTocPmaAtipFormat
    for multi_session_info::MultiSessionInformation<A>
{
    const MSF: bool = A::MSF;
    const FORMAT: u4 = u4::new(0b0001);
}
impl<A: TrackStartAddress> ReadTocPmaAtipResponse
    for multi_session_info::MultiSessionInformation<A>
{
}

// Raw TOC
impl private::ReadTocPmaAtipFormat for raw_toc::RawToc {
    const MSF: bool = true;
    const FORMAT: u4 = u4::new(0b0010);
}
impl ReadTocPmaAtipResponse for raw_toc::RawToc {}

// PMA
impl private::ReadTocPmaAtipFormat for pma::Pma {
    const MSF: bool = true;
    const FORMAT: u4 = u4::new(0b0011);
}
impl ReadTocPmaAtipResponse for pma::Pma {}

// ATIP
impl private::ReadTocPmaAtipFormat for atip::Atip {
    const MSF: bool = true;
    const FORMAT: u4 = u4::new(0b0100);
}
impl ReadTocPmaAtipResponse for atip::Atip {}

// CD-TEXT
// NOTE: [MMC-6] doesn't actually say that the MSF bit for CD-TEXT can't be zero, but all the
// other formats for which the MSF Field is "Ignored by Drive", describe the MSF bit to be set to
// one.
impl private::ReadTocPmaAtipFormat for cd_text::CdText {
    const MSF: bool = true;
    const FORMAT: u4 = u4::new(0b0101);
}
impl ReadTocPmaAtipResponse for cd_text::CdText {}

// Distinct impls for each to enable specificity like in the constructors for example
impl<A: TrackStartAddress> ReadTocPmaAtip<formatted_toc::FormattedToc<A>> {
    /// Requests the TOC Track Descriptors of the tracks from `starting_track_number` on, then
    /// the Lead-out's.
    ///
    /// Zero starts at the first track, and AAh requests the Lead-out's descriptor alone.
    pub const fn formatted_toc(
        starting_track_number: u8,
        allocation_length: u16,
        control: Control,
    ) -> Self {
        Self {
            _response_marker: PhantomData,
            track_session_number: starting_track_number,
            allocation_length,
            control,
        }
    }
}

impl<A: TrackStartAddress> ReadTocPmaAtip<multi_session_info::MultiSessionInformation<A>> {
    /// Requests the first and last complete session numbers, and the TOC Track Descriptor of the
    /// first track in the last complete session.
    pub const fn multi_session_information(allocation_length: u16, control: Control) -> Self {
        Self {
            _response_marker: PhantomData,
            track_session_number: 0,
            allocation_length,
            control,
        }
    }
}

impl ReadTocPmaAtip<raw_toc::RawToc> {
    /// Requests the Q sub-channel data of the Lead-in TOC, from session `starting_session_number`
    /// on.
    pub const fn raw_toc(
        starting_session_number: u8,
        allocation_length: u16,
        control: Control,
    ) -> Self {
        Self {
            _response_marker: PhantomData,
            track_session_number: starting_session_number,
            allocation_length,
            control,
        }
    }
}

impl ReadTocPmaAtip<pma::Pma> {
    /// Requests the Q sub-channel data of the PMA.
    pub const fn pma(allocation_length: u16, control: Control) -> Self {
        Self {
            _response_marker: PhantomData,
            track_session_number: 0,
            allocation_length,
            control,
        }
    }
}

impl ReadTocPmaAtip<atip::Atip> {
    /// Requests the ATIP of a CD-R or CD-RW.
    pub const fn atip(allocation_length: u16, control: Control) -> Self {
        Self {
            _response_marker: PhantomData,
            track_session_number: 0,
            allocation_length,
            control,
        }
    }
}

impl ReadTocPmaAtip<cd_text::CdText> {
    /// Requests the CD-TEXT packs from the Lead-in.
    pub const fn cd_text(allocation_length: u16, control: Control) -> Self {
        Self {
            _response_marker: PhantomData,
            track_session_number: 0,
            allocation_length,
            control,
        }
    }
}

type ReadTocPmaAtipOpCode = OpCode<0x43>;

impl<R: ReadTocPmaAtipResponse + ?Sized> Command<ReadTocPmaAtipOpCode> for ReadTocPmaAtip<R> {
    fn as_cdb(&self) -> <ReadTocPmaAtipOpCode as OpCodeDef>::Cdb {
        [
            ReadTocPmaAtipOpCode::OP_CODE,
            (u8::from(R::MSF) << 1),
            R::FORMAT.value(),
            0,
            0,
            0,
            self.track_session_number,
            (self.allocation_length >> 8) as u8,
            self.allocation_length as u8,
            self.control.into(),
        ]
    }
}
