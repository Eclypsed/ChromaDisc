use arbitrary_int::u3;
use thiserror::Error;

use crate::{
    core::{ReadCommand, TruncationError},
    mmc::msf::UnvalidatedMsf,
    rainbow_books::atip::{cdr, cdrw, CdRSubtype, DiscApplicationCode},
};

use super::{ReadTocPmaAtip, ReadTocPmaAtipOpCode};

const ATIP_MIN_BYTES: usize = 28;

#[derive(Debug, Error)]
pub enum AtipError {
    #[error("Unrecognized CD-RW subtype {0:03b}b")]
    UnknownCdRwSubtype(u3),
    #[error(transparent)]
    Truncated(#[from] TruncationError<ATIP_MIN_BYTES>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Atip {
    Cdr {
        write_power_ref_speed: cdr::WritePowerReferenceSpeed,
        reference_speed: cdr::ReferenceSpeed,
        disc_application_code: DiscApplicationCode,
        medium_type: CdRSubtype,

        lead_in_start_time: UnvalidatedMsf,
        lead_out_start_time: UnvalidatedMsf,

        additional_information_1: Option<cdr::AdditionalInformation1>,
        additional_information_2: Option<cdr::AdditionalInformation2>,
        additional_information_3: Option<cdr::AdditionalInformation3>,
    },
    CdrwStandard {
        write_power_ref_speed: cdrw::standard::WritePowerReferenceSpeed,
        reference_speed: cdrw::standard::ReferenceSpeed,
        disc_application_code: DiscApplicationCode,

        lead_in_start_time: UnvalidatedMsf,
        lead_out_start_time: UnvalidatedMsf,

        additional_information_1: Option<cdrw::standard::AdditionalInformation1>,
        additional_information_2: Option<cdrw::standard::AdditionalInformation2>,
    },
    CdrwHighSpeed {
        write_power_ref_speed: cdrw::high_speed::WritePower,
        reference_speed: cdrw::high_speed::ReferenceSpeed,
        disc_application_code: DiscApplicationCode,

        lead_in_start_time: UnvalidatedMsf,
        lead_out_start_time: UnvalidatedMsf,

        additional_information_1: Option<cdrw::high_speed::AdditionalInformation1>,
        additional_information_2: Option<cdrw::high_speed::AdditionalInformation2>,
        additional_information_3: Option<cdrw::high_speed::AdditionalInformation3>,
    },
    CdrwUltraSpeed {
        write_power_ref_speed: cdrw::ultra_speed::WritePower1tTestSpeed,
        reference_speed: cdrw::ultra_speed::TestSpeed1t,
        disc_application_code: DiscApplicationCode,

        lead_in_start_time: UnvalidatedMsf,
        lead_out_start_time: UnvalidatedMsf,

        additional_information_1: Option<cdrw::ultra_speed::AdditionalInformation1>,
        additional_information_2: Option<cdrw::ultra_speed::AdditionalInformation2>,
        additional_information_3: Option<cdrw::ultra_speed::AdditionalInformation3>,
    },
    CdrwUltraSpeedPlus {
        disc_application_code: DiscApplicationCode,

        lead_in_start_time: UnvalidatedMsf,
        lead_out_start_time: UnvalidatedMsf,

        // Ultra speed plus doesn't store any non-zero values
        // for Additional Information 1
        additional_information_2: Option<cdrw::ultra_speed_plus::AdditionalInformation2>,
        additional_information_3: Option<cdrw::ultra_speed_plus::AdditionalInformation3>,
    },
}

impl ReadCommand<ReadTocPmaAtipOpCode> for ReadTocPmaAtip<Atip> {
    type Len = u16;
    type Response<'a> = Atip;
    type Error = AtipError;

    fn response_len(&self) -> Self::Len {
        self.allocation_length
    }

    fn parse<'a>(&self, _buf: &'a [u8]) -> Result<Self::Response<'a>, Self::Error> {
        todo!()
    }
}
