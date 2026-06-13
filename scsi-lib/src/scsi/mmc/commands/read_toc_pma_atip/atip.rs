use std::io::{Cursor, Seek, SeekFrom};

use crate::core::msf::Msf;
use crate::rainbow_books::atip::{
    cdr, cdrw, CdrSubtype, CdrwSubtype, DiscApplicationCode, DiscSpeed, DiscType,
};
use deku::{ctx::Order, deku_derive, reader::Reader, DekuError, DekuReader};

use crate::scsi::mmc::commands::Response;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SpecialInformation1 {
    Cdr(cdr::SpecialInformation1),
    CdrwStandard(cdrw::standard::SpecialInformation1),
    CdrwHighSpeed(cdrw::high_speed::SpecialInformation1),
    CdrwUltraSpeed(cdrw::ultra_speed::SpecialInformation1),
    CdrwUltraSpeedPlus(cdrw::ultra_speed_plus::SpecialInformation1),
}

impl<'a> DekuReader<'a> for SpecialInformation1 {
    fn from_reader_with_ctx<R: deku::no_std_io::Read + deku::no_std_io::Seek>(
        reader: &mut Reader<R>,
        _: (),
    ) -> Result<Self, DekuError> {
        reader.seek(SeekFrom::Current(2))?;
        reader.skip_bits(1, Order::Msb0)?;
        let disc_type = DiscType::from_reader_with_ctx(reader, ())?;
        reader.skip_bits(3, Order::Msb0)?;
        reader.seek(SeekFrom::Current(-3))?;

        Ok(match disc_type {
            DiscType::Cdr(_) => {
                Self::Cdr(cdr::SpecialInformation1::from_reader_with_ctx(reader, ())?)
            }
            DiscType::Cdrw(CdrwSubtype::Standard) => Self::CdrwStandard(
                cdrw::standard::SpecialInformation1::from_reader_with_ctx(reader, ())?,
            ),
            DiscType::Cdrw(CdrwSubtype::HighSpeed) => Self::CdrwHighSpeed(
                cdrw::high_speed::SpecialInformation1::from_reader_with_ctx(reader, ())?,
            ),
            DiscType::Cdrw(CdrwSubtype::UltraSpeed) => Self::CdrwUltraSpeed(
                cdrw::ultra_speed::SpecialInformation1::from_reader_with_ctx(reader, ())?,
            ),
            DiscType::Cdrw(CdrwSubtype::UltraSpeedPlus) => Self::CdrwUltraSpeedPlus(
                cdrw::ultra_speed_plus::SpecialInformation1::from_reader_with_ctx(reader, ())?,
            ),
            _ => return Err(DekuError::IdVariantNotFound),
        })
    }
}

#[deku_derive(DekuRead)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Atip {
    #[deku(bytes = "2", temp, endian = "big", pad_bytes_after = "2")]
    _atip_data_length: usize,

    // #[deku(read_all)]
    // pub atip_descriptor: Vec<u8>,
    pub atip_descriptor: AtipDescriptor,
}

impl Response for Atip {
    type Error = DekuError;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
        Self::from_reader_with_ctx(&mut Reader::new(Cursor::new(bytes)), ())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AtipDescriptor {
    Cdr {
        write_power_ref_speed: cdr::WritePowerRefSpeed,
        reference_speed: DiscSpeed,
        disc_application_code: DiscApplicationCode,
        medium_type: CdrSubtype,

        lead_in_start_time: Msf,
        lead_out_start_time: Msf,

        additional_information_1: Option<cdr::AdditionalInformation1>,
        additional_information_2: Option<cdr::AdditionalInformation2>,
        additional_information_3: Option<cdr::AdditionalInformation3>,
    },
    CdrwStandard {
        write_power_ref_speed: cdrw::standard::WritePower,
        reference_speed: DiscSpeed,
        disc_application_code: DiscApplicationCode,

        lead_in_start_time: Msf,
        lead_out_start_time: Msf,

        additional_information_1: Option<cdrw::standard::AdditionalInformation1>,
        additional_information_2: Option<cdrw::standard::AdditionalInformation2>,
    },
    CdrwHighSpeed {
        write_power_ref_speed: cdrw::high_speed::WritePower,
        reference_speed: DiscSpeed,
        disc_application_code: DiscApplicationCode,

        lead_in_start_time: Msf,
        lead_out_start_time: Msf,

        additional_information_1: Option<cdrw::high_speed::AdditionalInformation1>,
        additional_information_2: Option<cdrw::high_speed::AdditionalInformation2>,
        additional_information_3: Option<cdrw::high_speed::AdditionalInformation3>,
    },
    CdrwUltraSpeed {
        write_power_ref_speed: cdrw::ultra_speed::WritePower,
        reference_speed: DiscSpeed,
        disc_application_code: DiscApplicationCode,

        lead_in_start_time: Msf,
        lead_out_start_time: Msf,

        additional_information_1: Option<cdrw::ultra_speed::AdditionalInformation1>,
        additional_information_2: Option<cdrw::ultra_speed::AdditionalInformation2>,
        additional_information_3: Option<cdrw::ultra_speed::AdditionalInformation3>,
    },
    CdrwUltraSpeedPlus {
        disc_application_code: DiscApplicationCode,

        lead_in_start_time: Msf,
        lead_out_start_time: Msf,

        // Ultra speed plus doesn't store any non-zero values
        // for Additional Information 1
        additional_information_2: Option<cdrw::ultra_speed_plus::AdditionalInformation2>,
        additional_information_3: Option<cdrw::ultra_speed_plus::AdditionalInformation3>,
    },
}

impl<'a> DekuReader<'a> for AtipDescriptor {
    fn from_reader_with_ctx<R: deku::no_std_io::Read + deku::no_std_io::Seek>(
        reader: &mut Reader<R>,
        _: (),
    ) -> Result<Self, deku::DekuError> {
        fn read_info<'a, I, R>(reader: &mut Reader<R>) -> Result<I, DekuError>
        where
            I: DekuReader<'a>,
            R: deku::no_std_io::Read + deku::no_std_io::Seek,
        {
            let info = I::from_reader_with_ctx(reader, ())?;
            reader.seek(SeekFrom::Current(1))?;
            Ok(info)
        }

        fn read_optional<'a, I, R>(
            reader: &mut Reader<R>,
            valid: bool,
        ) -> Result<Option<I>, DekuError>
        where
            I: DekuReader<'a>,
            R: deku::no_std_io::Read + deku::no_std_io::Seek,
        {
            if valid {
                read_info(reader).map(Some)
            } else {
                reader.seek(SeekFrom::Current(4))?;
                Ok(None)
            }
        }

        let si1: SpecialInformation1 = read_info(reader)?;
        let lead_in_start_time = Msf::from_reader_with_ctx(reader, ())?;
        reader.seek(SeekFrom::Current(1))?;
        let lead_out_start_time = Msf::from_reader_with_ctx(reader, ())?;
        reader.seek(SeekFrom::Current(1))?;

        match si1 {
            SpecialInformation1::Cdr(si1) => {
                let ai1: Option<cdr::AdditionalInformation1> = read_optional(reader, si1.a1_valid)?;
                let ai2: Option<cdr::AdditionalInformation2> = read_optional(reader, si1.a2_valid)?;
                let ai3: Option<cdr::AdditionalInformation3> = read_optional(reader, si1.a3_valid)?;

                Ok(AtipDescriptor::Cdr {
                    write_power_ref_speed: si1.target_writing_power,
                    reference_speed: si1.reference_speed,
                    disc_application_code: si1.disc_application_code,
                    medium_type: si1.medium_type,
                    lead_in_start_time,
                    lead_out_start_time,
                    additional_information_1: ai1,
                    additional_information_2: ai2,
                    additional_information_3: ai3,
                })
            }
            SpecialInformation1::CdrwStandard(si1) => {
                let ai1: Option<cdrw::standard::AdditionalInformation1> =
                    read_optional(reader, si1.a1_valid)?;
                let ai2: Option<cdrw::standard::AdditionalInformation2> =
                    read_optional(reader, si1.a2_valid)?;
                reader.seek(SeekFrom::Current(4))?;

                Ok(AtipDescriptor::CdrwStandard {
                    write_power_ref_speed: si1.target_writing_power,
                    reference_speed: si1.reference_speed,
                    disc_application_code: si1.disc_application_code,
                    lead_in_start_time,
                    lead_out_start_time,
                    additional_information_1: ai1,
                    additional_information_2: ai2,
                })
            }
            SpecialInformation1::CdrwHighSpeed(si1) => {
                let ai1: Option<cdrw::high_speed::AdditionalInformation1> =
                    read_optional(reader, si1.a1_valid)?;
                let ai2: Option<cdrw::high_speed::AdditionalInformation2> =
                    read_optional(reader, si1.a2_valid)?;
                let ai3: Option<cdrw::high_speed::AdditionalInformation3> =
                    read_optional(reader, si1.a3_valid)?;

                Ok(AtipDescriptor::CdrwHighSpeed {
                    write_power_ref_speed: si1.target_writing_power,
                    reference_speed: si1.reference_speed,
                    disc_application_code: si1.disc_application_code,
                    lead_in_start_time,
                    lead_out_start_time,
                    additional_information_1: ai1,
                    additional_information_2: ai2,
                    additional_information_3: ai3,
                })
            }
            SpecialInformation1::CdrwUltraSpeed(si1) => {
                let ai1: Option<cdrw::ultra_speed::AdditionalInformation1> =
                    read_optional(reader, si1.a1_valid)?;
                let ai2: Option<cdrw::ultra_speed::AdditionalInformation2> =
                    read_optional(reader, si1.a2_valid)?;
                let ai3: Option<cdrw::ultra_speed::AdditionalInformation3> =
                    read_optional(reader, si1.a3_valid)?;

                Ok(AtipDescriptor::CdrwUltraSpeed {
                    write_power_ref_speed: si1.target_writing_power,
                    reference_speed: si1.reference_speed,
                    disc_application_code: si1.disc_application_code,
                    lead_in_start_time,
                    lead_out_start_time,
                    additional_information_1: ai1,
                    additional_information_2: ai2,
                    additional_information_3: ai3,
                })
            }
            SpecialInformation1::CdrwUltraSpeedPlus(si1) => {
                reader.seek(SeekFrom::Current(4))?;
                let ai2: Option<cdrw::ultra_speed_plus::AdditionalInformation2> =
                    read_optional(reader, si1.a2_valid)?;
                let ai3: Option<cdrw::ultra_speed_plus::AdditionalInformation3> =
                    read_optional(reader, si1.a3_valid)?;

                Ok(AtipDescriptor::CdrwUltraSpeedPlus {
                    disc_application_code: si1.disc_application_code,
                    lead_in_start_time,
                    lead_out_start_time,
                    additional_information_2: ai2,
                    additional_information_3: ai3,
                })
            }
        }
    }
}
