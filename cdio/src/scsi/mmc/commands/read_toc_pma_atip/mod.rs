use std::marker::PhantomData;

use crate::core::{addressing::Lba, msf::Msf};

use super::{Command, Control, OpCode, OpCodeDef, Response};

pub mod atip;
pub mod cd_text;
pub mod formatted_toc;
pub mod multi_session_info;
pub mod pma;
pub mod raw_toc;

mod private {
    pub trait AddressingModeSeal {
        const MSF: bool;
    }
    pub trait ReadTocPmaAtipFormat {
        const MSF: bool;
        const FORMAT: u8;
    }
}

pub trait AddressingMode: private::AddressingModeSeal {}

impl private::AddressingModeSeal for Msf {
    const MSF: bool = true;
}
impl AddressingMode for Msf {}

impl private::AddressingModeSeal for Lba {
    const MSF: bool = false;
}
impl AddressingMode for Lba {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReadTocPmaAtip<R: ReadTocPmaAtipResponse> {
    _response_marker: PhantomData<R>,
    track_session_number: u8,
    allocation_length: u16,
    control: Control,
}

pub trait ReadTocPmaAtipResponse: private::ReadTocPmaAtipFormat + Response {}

// Formatted TOC
impl<A: formatted_toc::TrackStartAddress> private::ReadTocPmaAtipFormat
    for formatted_toc::FormattedToc<A>
{
    const MSF: bool = A::MSF;
    const FORMAT: u8 = 0b0000;
}
impl<A: formatted_toc::TrackStartAddress> ReadTocPmaAtipResponse
    for formatted_toc::FormattedToc<A>
{
}

// Multi-session Information
impl<A: multi_session_info::TrackStartAddress> private::ReadTocPmaAtipFormat
    for multi_session_info::MultiSessionInformation<A>
{
    const MSF: bool = A::MSF;
    const FORMAT: u8 = 0b0001;
}
impl<A: multi_session_info::TrackStartAddress> ReadTocPmaAtipResponse
    for multi_session_info::MultiSessionInformation<A>
{
}

// Raw TOC
impl private::ReadTocPmaAtipFormat for raw_toc::RawToc {
    const MSF: bool = <Msf as private::AddressingModeSeal>::MSF;
    const FORMAT: u8 = 0b0010;
}
impl ReadTocPmaAtipResponse for raw_toc::RawToc {}

// PMA
impl private::ReadTocPmaAtipFormat for pma::Pma {
    const MSF: bool = <Msf as private::AddressingModeSeal>::MSF;
    const FORMAT: u8 = 0b0011;
}
impl ReadTocPmaAtipResponse for pma::Pma {}

// ATIP
impl private::ReadTocPmaAtipFormat for atip::Atip {
    const MSF: bool = <Msf as private::AddressingModeSeal>::MSF;
    const FORMAT: u8 = 0b0100;
}
impl ReadTocPmaAtipResponse for atip::Atip {}

// CD-TEXT
// ? [MMC-6] doesn't actually say that the MSF bit for CD-TEXT can't be zero, but all the other
// ? formats for which the MSF Field is "Ignored by Drive", describe the MSF bit to be set to one.
impl private::ReadTocPmaAtipFormat for cd_text::CdText {
    const MSF: bool = <Msf as private::AddressingModeSeal>::MSF;
    const FORMAT: u8 = 0b0101;
}
impl ReadTocPmaAtipResponse for cd_text::CdText {}

// Distinct impls for each to enable specificity like in the constructors for example
impl<A: formatted_toc::TrackStartAddress> ReadTocPmaAtip<formatted_toc::FormattedToc<A>> {
    pub fn new(track_number: u8, allocation_length: u16, control: Control) -> Self {
        Self {
            _response_marker: PhantomData,
            track_session_number: track_number,
            allocation_length,
            control,
        }
    }
}

impl<A: multi_session_info::TrackStartAddress>
    ReadTocPmaAtip<multi_session_info::MultiSessionInformation<A>>
{
    pub fn new(allocation_length: u16, control: Control) -> Self {
        Self {
            _response_marker: PhantomData,
            track_session_number: 0,
            allocation_length,
            control,
        }
    }
}

impl ReadTocPmaAtip<raw_toc::RawToc> {
    pub fn new(session_number: u8, allocation_length: u16, control: Control) -> Self {
        Self {
            _response_marker: PhantomData,
            track_session_number: session_number,
            allocation_length,
            control,
        }
    }
}

impl ReadTocPmaAtip<pma::Pma> {
    pub fn new(allocation_length: u16, control: Control) -> Self {
        Self {
            _response_marker: PhantomData,
            track_session_number: 0,
            allocation_length,
            control,
        }
    }
}

impl ReadTocPmaAtip<atip::Atip> {
    pub fn new(allocation_length: u16, control: Control) -> Self {
        Self {
            _response_marker: PhantomData,
            track_session_number: 0,
            allocation_length,
            control,
        }
    }
}

impl ReadTocPmaAtip<cd_text::CdText> {
    pub fn new(allocation_length: u16, control: Control) -> Self {
        Self {
            _response_marker: PhantomData,
            track_session_number: 0,
            allocation_length,
            control,
        }
    }
}

type ReadTocPmaAtipOpCode = OpCode<0x43>;

impl<R: ReadTocPmaAtipResponse> Command<ReadTocPmaAtipOpCode> for ReadTocPmaAtip<R> {
    type Response = R;

    fn as_cdb(&self) -> <ReadTocPmaAtipOpCode as OpCodeDef>::Cdb {
        [
            ReadTocPmaAtipOpCode::OP_CODE,
            (u8::from(R::MSF) << 1),
            (R::FORMAT & 0x0F),
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
