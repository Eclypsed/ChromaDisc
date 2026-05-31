use std::marker::PhantomData;

use arbitrary_int::u4;
use rainbow_books::core::RawMsf;

use crate::core::addressing::Lba;

use super::{Command, Control, OpCode, OpCodeDef, Response};

pub mod atip;
pub mod cd_text;
pub mod formatted_toc;
pub mod multi_session_info;
pub mod pma;
pub mod raw_toc;

mod private {
    pub trait AddressingModeSeal {}
    pub trait FormatSeal {}
    pub trait ReadTocPmaAtipSeal {}
}

pub trait AddressingMode: private::AddressingModeSeal {
    const MSF: bool;
}

impl private::AddressingModeSeal for RawMsf {}
impl AddressingMode for RawMsf {
    const MSF: bool = true;
}

impl private::AddressingModeSeal for Lba {}
impl AddressingMode for Lba {
    const MSF: bool = false;
}

pub trait Format: private::FormatSeal {
    const FORMAT: u4; // 4-bit format field
}

pub mod format {
    use arbitrary_int::u4;

    macro_rules! impl_format_field {
        ($($name:ident = $value:literal),+ $(,)?) => {
            $(
                pub struct $name;
                impl super::private::FormatSeal for $name {}
                impl super::Format for $name {
                    const FORMAT: u4 = u4::new($value);
                }
            )+
        };
    }

    impl_format_field!(
        FormattedToc = 0b0000,
        MultiSessionInformation = 0b0001,
        RawToc = 0b0010,
        Pma = 0b0011,
        Atip = 0b0100,
        CdText = 0b0101,
    );
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReadTocPmaAtip<A: AddressingMode, F: Format> {
    _msf_marker: PhantomData<A>,
    _format_marker: PhantomData<F>,
    track_session_number: u8,
    allocation_length: u16,
    control: Control,
}

pub trait ReadTocPmaAtipDef: private::ReadTocPmaAtipSeal {
    type Response: Response;
}

macro_rules! impl_read_toc_pma_atip_def {
    ($a:ty, $f:ty, $response:ty) => {
        impl private::ReadTocPmaAtipSeal for ReadTocPmaAtip<$a, $f> {}
        impl ReadTocPmaAtipDef for ReadTocPmaAtip<$a, $f> {
            type Response = $response;
        }
    };
}

impl_read_toc_pma_atip_def!(Lba, format::FormattedToc, formatted_toc::FormattedToc<Lba>);
impl_read_toc_pma_atip_def!(
    RawMsf,
    format::FormattedToc,
    formatted_toc::FormattedToc<RawMsf>
);
impl_read_toc_pma_atip_def!(
    Lba,
    format::MultiSessionInformation,
    multi_session_info::MultiSessionInformation<Lba>
);
impl_read_toc_pma_atip_def!(
    RawMsf,
    format::MultiSessionInformation,
    multi_session_info::MultiSessionInformation<RawMsf>
);
impl_read_toc_pma_atip_def!(RawMsf, format::RawToc, raw_toc::RawToc);
impl_read_toc_pma_atip_def!(RawMsf, format::Pma, pma::Pma);
impl_read_toc_pma_atip_def!(RawMsf, format::Atip, atip::Atip);
// ? [MMC-6] doesn't actually say that the MSF bit for CD-TEXT can't be zero, but all the other
// ? formats for which the MSF Field is "Ignored by Drive", describe the MSF bit to be set to one.
impl_read_toc_pma_atip_def!(RawMsf, format::CdText, cd_text::CdText);

impl<A: AddressingMode, F: Format> ReadTocPmaAtip<A, F>
where
    ReadTocPmaAtip<A, F>: ReadTocPmaAtipDef,
{
    pub fn new(track_session_number: u8, allocation_length: u16, control: Control) -> Self {
        Self {
            _msf_marker: PhantomData,
            _format_marker: PhantomData,
            track_session_number,
            allocation_length,
            control,
        }
    }
}

type ReadTocPmaAtipOpCode = OpCode<0x43>;

impl<A: AddressingMode, F: Format> Command<ReadTocPmaAtipOpCode> for ReadTocPmaAtip<A, F>
where
    ReadTocPmaAtip<A, F>: ReadTocPmaAtipDef,
{
    type Response = <Self as ReadTocPmaAtipDef>::Response;

    fn as_cdb(&self) -> <ReadTocPmaAtipOpCode as OpCodeDef>::Cdb {
        [
            ReadTocPmaAtipOpCode::OP_CODE,
            (u8::from(A::MSF) << 1),
            F::FORMAT.value(),
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
