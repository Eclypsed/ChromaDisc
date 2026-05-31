use std::io::{Cursor, Seek, SeekFrom};

use deku::{deku_derive, reader::Reader, DekuError, DekuReader};
use rainbow_books::{
    core::RawMsf,
    orange_book::atip::{
        cdr, cdrw, DiscApplicationCode, DiscSpeed, SpecialInformation1, SpecialInformation2,
        SpecialInformation3,
    },
};

use crate::scsi::mmc::commands::Response;

#[deku_derive(DekuRead)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Atip {
    #[deku(bytes = "2", temp, endian = "big", pad_bytes_after = "2")]
    _atip_data_length: usize,

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

        lead_in_start_time: RawMsf,
        lead_out_start_time: RawMsf,

        additional_information_1: Option<cdr::AdditionalInformation1>,
        additional_information_2: Option<cdr::AdditionalInformation2>,
        additional_information_3: Option<cdr::AdditionalInformation3>,
    },
    CdrwStandard {
        write_power_ref_speed: cdrw::standard::WritePower,
        reference_speed: DiscSpeed,
        disc_application_code: DiscApplicationCode,

        lead_in_start_time: RawMsf,
        lead_out_start_time: RawMsf,

        additional_information_1: Option<cdrw::standard::AdditionalInformation1>,
        additional_information_2: Option<cdrw::standard::AdditionalInformation2>,
    },
    CdrwHighSpeed {
        write_power_ref_speed: cdrw::high_speed::WritePower,
        reference_speed: DiscSpeed,
        disc_application_code: DiscApplicationCode,

        lead_in_start_time: RawMsf,
        lead_out_start_time: RawMsf,

        additional_information_1: Option<cdrw::high_speed::AdditionalInformation1>,
        additional_information_2: Option<cdrw::high_speed::AdditionalInformation2>,
        additional_information_3: Option<cdrw::high_speed::AdditionalInformation3>,
    },
    CdrwUltraSpeed {
        write_power_ref_speed: cdrw::ultra_speed::WritePower,
        reference_speed: DiscSpeed,
        disc_application_code: DiscApplicationCode,

        lead_in_start_time: RawMsf,
        lead_out_start_time: RawMsf,

        additional_information_1: Option<cdrw::ultra_speed::AdditionalInformation1>,
        additional_information_2: Option<cdrw::ultra_speed::AdditionalInformation2>,
        additional_information_3: Option<cdrw::ultra_speed::AdditionalInformation3>,
    },
    CdrwUltraSpeedPlus {
        disc_application_code: DiscApplicationCode,
        lead_in_start_time: RawMsf,
        lead_out_start_time: RawMsf,

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
        let si2: SpecialInformation2 = read_info(reader)?;
        let si3: SpecialInformation3 = read_info(reader)?;

        match si1 {
            SpecialInformation1::Cdr(si1) => {
                let ai1: Option<cdr::AdditionalInformation1> = read_optional(reader, si1.a1_valid)?;
                let ai2: Option<cdr::AdditionalInformation2> = read_optional(reader, si1.a2_valid)?;
                let ai3: Option<cdr::AdditionalInformation3> = read_optional(reader, si1.a3_valid)?;

                Ok(AtipDescriptor::Cdr {
                    write_power_ref_speed: si1.target_writing_power,
                    reference_speed: si1.reference_speed,
                    disc_application_code: si1.disc_application_code,
                    lead_in_start_time: si2.lead_in_start_time,
                    lead_out_start_time: si3.lead_out_start_time,
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
                    lead_in_start_time: si2.lead_in_start_time,
                    lead_out_start_time: si3.lead_out_start_time,
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
                    lead_in_start_time: si2.lead_in_start_time,
                    lead_out_start_time: si3.lead_out_start_time,
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
                    lead_in_start_time: si2.lead_in_start_time,
                    lead_out_start_time: si3.lead_out_start_time,
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
                    lead_in_start_time: si2.lead_in_start_time,
                    lead_out_start_time: si3.lead_out_start_time,
                    additional_information_2: ai2,
                    additional_information_3: ai3,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use deku::{reader::Reader, DekuReader};
    use rainbow_books::orange_book::atip::MediaIdentificationCode;

    use super::*;

    #[test]
    fn cdr_atip() {
        let data: &[u8] = &[
            0b0000_0000,
            0b0000_0110,
            0b0000_0000,
            0b0000_0000,
            // SI1
            0b1000_0000,
            0b0000_0000,
            0b1000_0111,
            0b0000_0000,
            // SI2
            0b1001_0111,
            0b1100_1001,
            0b0000_0000,
            0b0000_0000,
            // SI3
            0b1111_0000,
            0b1100_0101,
            0b1001_0101,
            0b0000_0000,
            // AI1
            0b0010_1010,
            0b0000_0010,
            0b1011_0000,
            0b0000_0000,
            // AI2
            0b0010_1001,
            0b1100_0100,
            0b0000_0000,
            0b0000_0000,
            // AI3
            0b0000_1100,
            0b1100_1010,
            0b1001_1100,
            0b0000_0000,
        ];

        let mut reader = Reader::new(Cursor::new(data));
        let val = Atip::from_reader_with_ctx(&mut reader, ()).unwrap();

        assert_eq!(
            Atip {
                atip_descriptor: AtipDescriptor::Cdr {
                    write_power_ref_speed: cdr::WritePowerRefSpeed::W4_0,
                    reference_speed: DiscSpeed::X1,
                    disc_application_code: DiscApplicationCode::GeneralPurposeDisc,
                    lead_in_start_time: RawMsf::from_bcd(0b1001_0111, 0b0100_1001, 0b0000_0000),
                    lead_out_start_time: RawMsf::from_bcd(0b0111_0000, 0b0100_0101, 0b0001_0101),
                    additional_information_1: Some(cdr::AdditionalInformation1 {
                        lowest_test_speed: DiscSpeed::X4,
                        highest_test_speed: DiscSpeed::X40,
                        high_speed_subtype: cdr::HighSpeedSubtype::CdrMultiSpeed,
                        optimum_beta_range: cdr::OptimumBetaRange::TargetP4,
                        optimum_pulse_length: cdr::OptimumPulseLength::ThetaP0_75,
                        additional_capacity_len: cdr::AdditionalCapacityLength::Minutes2
                    }),
                    additional_information_2: Some(cdr::AdditionalInformation2 {
                        writing_power_lowest_speed: cdr::WritePowerMinSpeed::W9_0,
                        writing_power_highest_speed: cdr::WritePowerMaxSpeed::W34_0,
                        power_boost_i3_pits: cdr::PowerBoostI3::Percent8,
                        pulse_length_correction_i3_lands: cdr::PulseLengthCorrectionI3::T1_16,
                    }),
                    additional_information_3: Some(cdr::AdditionalInformation3 {
                        media_technology_type: cdr::MediaTechnologyType::Cyanine,
                        media_identification_code: MediaIdentificationCode(0b0110_0100_1010_0011),
                        product_revision_number: 4
                    })
                }
            },
            val
        );

        // println!("{val:#?}");
    }
}
