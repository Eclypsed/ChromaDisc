use std::{
    error::Error,
    io::{Seek, SeekFrom},
};

use bcd::Bcd;
use deku::{
    ctx::{BitSize, Order},
    deku_derive,
    reader::Reader,
    DekuError, DekuRead, DekuReader,
};

use crate::msf::{Frame, Minute, Msf, Second};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiscType {
    Cdr,
    Cdrw(CdrwSubtype),
}

impl<'a> DekuReader<'a> for DiscType {
    fn from_reader_with_ctx<R: std::io::Read + std::io::Seek>(
        reader: &mut Reader<R>,
        _ctx: (),
    ) -> Result<Self, DekuError>
    where
        Self: Sized,
    {
        let id = u8::from_reader_with_ctx(reader, BitSize(1))?;
        if id == 0 {
            reader.skip_bits(3, Order::Msb0)?;
            Ok(Self::Cdr)
        } else {
            let subtype = CdrwSubtype::from_reader_with_ctx(reader, ())?;
            Ok(Self::Cdrw(subtype))
        }
    }
}

/// A 3-bit value representing the different CD-RW sub-types
///
/// See CD-RW System Description (Orange Book Part III Volume 3) §I.2, Table 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
#[deku(id_type = "u8", bits = 3)]
#[repr(u8)]
pub enum CdrwSubtype {
    Standard = 0b000,
    HighSpeed = 0b001,
    UltraSpeed = 0b010,
    UltraSpeedPlus = 0b011,
    #[deku(id_pat = "_")]
    Reserved(u8),
}

/// A 7-bit value distiguishing between discs used for different applications
///
/// See CD-R System Description (Orange Book Part II Volume 2) §4.4.1.3
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
#[deku(id_type = "u8", bits = 7)]
#[repr(u8)]
pub enum DiscApplicationCode {
    GeneralPurposeDisc = 0b0000000,
    #[deku(id_pat = "0b0000001..=0b0111111")]
    SpecialPurposeDisc(u8),
    UnrestrictedUse = 0b1000000,
    #[deku(id_pat = "_")]
    Reserved(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
#[deku(id_type = "u8", ctx = "bitsize: usize", bits = "bitsize")]
#[repr(u8)]
pub enum DiscSpeed {
    X1 = 0b000,
    X2 = 0b001,
    X4 = 0b010,
    X8 = 0b011,
    X10 = 0b100,
    X12 = 0b101,
    X16 = 0b110,
    X20 = 0b111,
    X24 = 0b1000,
    X32 = 0b1001,
    X40 = 0b1010,
    X48 = 0b1011,
    X52 = 0b1100,
    #[deku(id_pat = "_")]
    Reserved(u8),
}

/// A 16-bit value that contains a unique identifying code for the Disc Manufacturer and the type
/// of disc.
///
/// See CD-R System Description (Orange Book Part II Volume 2) §4.4.6.2
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
pub struct MediaIdentificationCode(#[deku(endian = "big")] pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpecialInformation1 {
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
            DiscType::Cdr => Self::Cdr(cdr::SpecialInformation1::from_reader_with_ctx(reader, ())?),
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpecialInformation2 {
    #[deku(bits = 1, temp, assert_eq = "1", pad_bits_after = "7")]
    _m1: u8,

    #[deku(bits = 1, temp, assert_eq = "1", pad_bits_after = "7")]
    _s1: u8,

    #[deku(bits = 1, temp, assert_eq = "0", pad_bits_after = "7")]
    _f1: u8,

    #[deku(
        seek_from_current = "-3",
        reader = "SpecialInformation2::read_lead_in_start_time(deku::reader)"
    )]
    pub lead_in_start_time: Msf,
}

impl SpecialInformation2 {
    fn read_lead_in_start_time<R: deku::no_std_io::Read + deku::no_std_io::Seek>(
        reader: &mut Reader<R>,
    ) -> Result<Msf, DekuError> {
        fn parse_err(e: impl Error) -> DekuError {
            DekuError::Parse(e.to_string().into())
        }

        let mut bytes = <[u8; 3]>::from_reader_with_ctx(reader, ())?;
        bytes[0] |= 0x80;
        bytes[1] &= 0x7F;
        bytes[2] &= 0x7f;

        for byte in bytes.iter_mut() {
            *byte = Bcd::<1>::from_bcd_bytes([*byte])
                .map(|bcd| bcd.into_u8())
                .map_err(parse_err)?;
        }

        Ok(Msf::new(
            Minute::try_from(bytes[0]).map_err(parse_err)?,
            Second::try_from(bytes[1]).map_err(parse_err)?,
            Frame::try_from(bytes[2]).map_err(parse_err)?,
        ))
    }
}

#[deku_derive(DekuRead)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpecialInformation3 {
    #[deku(bits = 1, temp, assert_eq = "1", pad_bits_after = "7")]
    _m1: u8,

    #[deku(bits = 1, temp, assert_eq = "1", pad_bits_after = "7")]
    _s1: u8,

    #[deku(bits = 1, temp, assert_eq = "0", pad_bits_after = "7")]
    _f1: u8,

    #[deku(
        seek_from_current = "-3",
        reader = "SpecialInformation3::read_lead_out_start_time(deku::reader)"
    )]
    pub lead_out_start_time: Msf,
}

impl SpecialInformation3 {
    fn read_lead_out_start_time<R: deku::no_std_io::Read + deku::no_std_io::Seek>(
        reader: &mut Reader<R>,
    ) -> Result<Msf, DekuError> {
        fn parse_err(e: impl Error) -> DekuError {
            DekuError::Parse(e.to_string().into())
        }

        let mut bytes = <[u8; 3]>::from_reader_with_ctx(reader, ())?.map(|b| b & 0x7f);

        for byte in bytes.iter_mut() {
            *byte = Bcd::<1>::from_bcd_bytes([*byte])
                .map(|bcd| bcd.into_u8())
                .map_err(parse_err)?;
        }

        Ok(Msf::new(
            Minute::try_from(bytes[0]).map_err(parse_err)?,
            Second::try_from(bytes[1]).map_err(parse_err)?,
            Frame::try_from(bytes[2]).map_err(parse_err)?,
        ))
    }
}

pub mod cdr {
    use std::ops::Range;

    use deku::{deku_derive, DekuRead};

    use crate::orange_book::atip::{
        DiscApplicationCode, DiscSpeed, DiscType, MediaIdentificationCode,
    };

    /// A 3-bit value representing the optimum recording power in mW for CD-R and CD-RW discs.
    ///
    /// See CD-R/WO System Description (Orange Book Part II Volume 1) §4.4
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
    #[deku(id_type = "u8", bits = 3)]
    #[repr(u8)]
    pub enum WritePowerRefSpeed {
        W4_0 = 0b000,
        W4_4 = 0b001,
        W4_9 = 0b010,
        W5_4 = 0b011,
        W5_9 = 0b100,
        W6_6 = 0b101,
        W7_2 = 0b110,
        W8_0 = 0b111,
    }

    impl WritePowerRefSpeed {
        pub const fn milliwatt(self) -> f32 {
            match self {
                Self::W4_0 => 4.0,
                Self::W4_4 => 4.4,
                Self::W4_9 => 4.9,
                Self::W5_4 => 5.4,
                Self::W5_9 => 5.9,
                Self::W6_6 => 6.6,
                Self::W7_2 => 7.2,
                Self::W8_0 => 8.0,
            }
        }
    }

    /// A 3-bit value representing a sub-class withing the Multi-Speed Recordable disc types
    ///
    /// See CD-R System Description (Orange Book Part II Volume 2) §4.4.4.3
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
    #[deku(id_type = "u8", bits = 3)]
    #[repr(u8)]
    pub enum HighSpeedSubtype {
        CdrMultiSpeed = 0b000,
        #[deku(id_pat = "_")]
        Reserved(u8),
    }

    /// A 2-bit value representing a sub-class withing the Multi-Speed Recordable disc types
    ///
    /// See CD-R System Description (Orange Book Part II Volume 2) §4.4.4.4
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
    #[deku(id_type = "u8", bits = 2)]
    #[repr(u8)]
    pub enum OptimumBetaRange {
        TargetN4 = 0b00,
        Target0 = 0b01,
        TargetP4 = 0b10,
        TargetP8 = 0b11,
    }

    impl OptimumBetaRange {
        pub const fn beta_range(self) -> Range<i8> {
            match self {
                Self::TargetN4 => -8..0,
                Self::Target0 => -4..4,
                Self::TargetP4 => 0..8,
                Self::TargetP8 => 4..12,
            }
        }

        pub const fn target_beta(self) -> i8 {
            match self {
                Self::TargetN4 => -4,
                Self::Target0 => 0,
                Self::TargetP4 => 4,
                Self::TargetP8 => 8,
            }
        }
    }

    /// A 3-bit value representing the optimum Write Pulse length for the medium for recording at the
    /// High Test Speed
    ///
    /// See CD-R System Description (Orange Book Part II Volume 2) §4.4.4.5
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
    #[deku(id_type = "u8", bits = 3)]
    #[repr(u8)]
    pub enum OptimumPulseLength {
        Theta0 = 0b000,
        ThetaP0_25 = 0b001,
        ThetaP0_50 = 0b010,
        ThetaP0_75 = 0b011,
        ThetaN0_25 = 0b100,
        ThetaN0_50 = 0b101,
        ThetaN0_75 = 0b110,
        ThetaN1_00 = 0b111,
    }

    impl OptimumPulseLength {
        pub const fn theta(self) -> f32 {
            match self {
                Self::Theta0 => 0.0,
                Self::ThetaP0_25 => 0.25,
                Self::ThetaP0_50 => 0.5,
                Self::ThetaP0_75 => 0.75,
                Self::ThetaN0_25 => -0.25,
                Self::ThetaN0_50 => -0.5,
                Self::ThetaN0_75 => -0.75,
                Self::ThetaN1_00 => -1.0,
            }
        }
    }

    /// A 4-bit value specifying the Additional Capacity & Lead-out area length and the location of
    /// PCA2 by means of an offset relative to the Start Time of the Additional Capacity & Lead-out
    /// Area.
    ///
    /// See CD-R System Description (Orange Book Part II Volume 2) §4.4.4.6
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
    #[deku(id_type = "u8", bits = 4)]
    #[repr(u8)]
    pub enum AdditionalCapacityLength {
        Minutes2 = 0b0000,
        #[deku(id_pat = "_")]
        Reserved(u8),
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
    #[deku(id_type = "u8", bits = 3)]
    #[repr(u8)]
    pub enum WritePowerMinSpeed {
        W7_0 = 0b000,
        W8_0 = 0b001,
        W9_0 = 0b010,
        W10_0 = 0b011,
        W11_0 = 0b100,
        W12_0 = 0b101,
        W13_0 = 0b110,
        W14_0 = 0b111,
    }

    impl WritePowerMinSpeed {
        pub const fn milliwatt(self) -> f32 {
            match self {
                Self::W7_0 => 7.0,
                Self::W8_0 => 8.0,
                Self::W9_0 => 9.0,
                Self::W10_0 => 10.0,
                Self::W11_0 => 11.0,
                Self::W12_0 => 12.0,
                Self::W13_0 => 13.0,
                Self::W14_0 => 14.0,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
    #[deku(id_type = "u8", bits = 4)]
    #[repr(u8)]
    pub enum WritePowerMaxSpeed {
        W16_0 = 0b0000,
        W18_0 = 0b0001,
        W20_0 = 0b0010,
        W22_0 = 0b0011,
        W24_0 = 0b0100,
        W26_0 = 0b0101,
        W28_0 = 0b0110,
        W30_0 = 0b0111,
        W32_0 = 0b1000,
        W34_0 = 0b1001,
        W36_0 = 0b1010,
        W38_0 = 0b1011,
        W41_0 = 0b1100,
        W44_0 = 0b1101,
        W47_0 = 0b1110,
        W50_0 = 0b1111,
    }

    impl WritePowerMaxSpeed {
        pub const fn milliwatt(self) -> f32 {
            match self {
                Self::W16_0 => 16.0,
                Self::W18_0 => 16.0,
                Self::W20_0 => 20.0,
                Self::W22_0 => 22.0,
                Self::W24_0 => 24.0,
                Self::W26_0 => 26.0,
                Self::W28_0 => 28.0,
                Self::W30_0 => 30.0,
                Self::W32_0 => 32.0,
                Self::W34_0 => 34.0,
                Self::W36_0 => 36.0,
                Self::W38_0 => 38.0,
                Self::W41_0 => 41.0,
                Self::W44_0 => 44.0,
                Self::W47_0 => 47.0,
                Self::W50_0 => 50.0,
            }
        }
    }

    /// A 3-bit value that specifies the optimum Delta-P for the I3 Write Pulse for recording at the
    /// Highest Test Speed.
    ///
    /// See CD-R System Description (Orange Book Part II Volume 2) §4.4.5.3
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
    #[deku(id_type = "u8", bits = 3)]
    #[repr(u8)]
    pub enum PowerBoostI3 {
        Percent0 = 0b000,
        Percent2 = 0b001,
        Percent4 = 0b010,
        Percent6 = 0b011,
        Percent8 = 0b100,
        Percent10 = 0b101,
        Percent12 = 0b110,
        Percent14 = 0b111,
    }

    impl PowerBoostI3 {
        pub const fn delta_p(self) -> u8 {
            match self {
                Self::Percent0 => 0,
                Self::Percent2 => 2,
                Self::Percent4 => 4,
                Self::Percent6 => 6,
                Self::Percent8 => 8,
                Self::Percent10 => 10,
                Self::Percent12 => 12,
                Self::Percent14 => 14,
            }
        }
    }

    /// A 2-bit value that specifies the optimum Delta-T for each Write Pulse after an I3 Land for
    ///  recording at the Highest Test Speed.
    ///
    /// See CD-R System Description (Orange Book Part II Volume 2) §4.4.5.4
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
    #[deku(id_type = "u8", bits = 2)]
    #[repr(u8)]
    pub enum PulseLengthCorrectionI3 {
        T0 = 0b00,
        T1_16 = 0b01,
        T2_16 = 0b10,
        T3_16 = 0b11,
    }

    impl PulseLengthCorrectionI3 {
        pub const fn delta_t(self) -> f32 {
            match self {
                Self::T0 => 0.0,
                Self::T1_16 => 1.0 / 16.0,
                Self::T2_16 => 2.0 / 16.0,
                Self::T3_16 => 3.0 / 16.0,
            }
        }
    }

    /// A 2-bit value specifying the type of technology of the recordable layer on the disc.
    ///
    /// See CD-R System Description (Orange Book Part II Volume 2) §4.4.6.1
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
    #[deku(id_type = "u8", bits = 2)]
    #[repr(u8)]
    pub enum MediaTechnologyType {
        Cyanine = 0b00,
        PhtaloCyanine = 0b01,
        Reserved = 0b10,
        Other = 0b11,
    }

    #[deku_derive(DekuRead)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct SpecialInformation1 {
        #[deku(bits = 1, temp, assert_eq = "1")]
        _m1: u8,
        #[deku(pad_bits_after = "1")]
        pub target_writing_power: WritePowerRefSpeed,
        #[deku(ctx = "3")]
        pub reference_speed: DiscSpeed,

        #[deku(bits = 1, temp, assert_eq = "0")]
        _s1: u8,
        pub disc_application_code: DiscApplicationCode,

        #[deku(bits = 1, temp, assert_eq = "1")]
        _f1: u8,
        #[deku(temp, assert_eq = "DiscType::Cdr")]
        _disc_type: DiscType,
        #[deku(bits = 1)]
        pub a1_valid: bool,
        #[deku(bits = 1)]
        pub a2_valid: bool,
        #[deku(bits = 1)]
        pub a3_valid: bool,
    }

    #[deku_derive(DekuRead)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct AdditionalInformation1 {
        #[deku(bits = 1, temp, assert_eq = "0")]
        _m1: u8,
        #[deku(ctx = "3")]
        pub lowest_test_speed: DiscSpeed,
        #[deku(ctx = "4")]
        pub highest_test_speed: DiscSpeed,

        #[deku(bits = 1, temp, assert_eq = "0")]
        _s1: u8,
        #[deku(pad_bits_after = "2")]
        pub high_speed_subtype: HighSpeedSubtype,
        pub optimum_beta_range: OptimumBetaRange,

        #[deku(bits = 1, temp, assert_eq = "1")]
        _f1: u8,
        pub optimum_pulse_length: OptimumPulseLength,
        pub additional_capacity_len: AdditionalCapacityLength,
    }

    #[deku_derive(DekuRead)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct AdditionalInformation2 {
        #[deku(bits = 1, temp, assert_eq = "0")]
        _m1: u8,
        pub writing_power_lowest_speed: WritePowerMinSpeed,
        pub writing_power_highest_speed: WritePowerMaxSpeed,

        #[deku(bits = 1, temp, assert_eq = "1")]
        _s1: u8,
        pub power_boost_i3_pits: PowerBoostI3,
        #[deku(pad_bits_after = "2")]
        pub pulse_length_correction_i3_lands: PulseLengthCorrectionI3,

        #[deku(bits = 1, temp, assert_eq = "0", pad_bits_after = "7")]
        _f1: u8,
    }

    #[deku_derive(DekuRead)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct AdditionalInformation3 {
        #[deku(bits = 1, temp, assert_eq = "0")]
        _m1: u8,
        pub media_technology_type: MediaTechnologyType,
        #[deku(bits = 5, temp)]
        _q1q5: u16,

        #[deku(bits = 1, temp, assert_eq = "1")]
        _s1: u8,
        #[deku(bits = 7, temp)]
        _q6q12: u16,

        #[deku(bits = 1, temp, assert_eq = "1")]
        _f1: u8,
        #[deku(bits = 4, temp)]
        _q13q16: u16,

        #[deku(
            skip,
            default = "MediaIdentificationCode(*_q1q5 << 11 | *_q6q12 << 4 | *_q13q16)"
        )]
        pub media_identification_code: MediaIdentificationCode,
        #[deku(bits = 3)]
        pub product_revision_number: u8,
    }

    #[cfg(test)]
    mod tests {
        use std::io::Cursor;

        use deku::{reader::Reader, DekuReader};

        use super::*;

        #[test]
        fn parse_additional_information_1() {
            let data: &[u8] = &[0b0010_1000, 0b0000_0001, 0b1010_0000];
            let mut reader = Reader::new(Cursor::new(data));

            let val = AdditionalInformation1::from_reader_with_ctx(&mut reader, ()).unwrap();

            assert_eq!(
                AdditionalInformation1 {
                    lowest_test_speed: DiscSpeed::X4,
                    highest_test_speed: DiscSpeed::X24,
                    high_speed_subtype: HighSpeedSubtype::CdrMultiSpeed,
                    optimum_beta_range: OptimumBetaRange::Target0,
                    optimum_pulse_length: OptimumPulseLength::ThetaP0_50,
                    additional_capacity_len: AdditionalCapacityLength::Minutes2,
                },
                val
            );
        }

        #[test]
        fn parse_additional_information_2() {
            let data: &[u8] = &[0b0001_1010, 0b1100_0101, 0b0010_0000];
            let mut reader = Reader::new(Cursor::new(data));

            let val = AdditionalInformation2::from_reader_with_ctx(&mut reader, ()).unwrap();

            assert_eq!(
                AdditionalInformation2 {
                    writing_power_lowest_speed: WritePowerMinSpeed::W8_0,
                    writing_power_highest_speed: WritePowerMaxSpeed::W36_0,
                    power_boost_i3_pits: PowerBoostI3::Percent8,
                    pulse_length_correction_i3_lands: PulseLengthCorrectionI3::T1_16,
                },
                val
            );
        }

        #[test]
        fn parse_additional_information_3() {
            let data: &[u8] = &[0b0010_1110, 0b1101_0101, 0b1000_0010];
            let mut reader = Reader::new(Cursor::new(data));

            let val = AdditionalInformation3::from_reader_with_ctx(&mut reader, ()).unwrap();

            assert_eq!(
                AdditionalInformation3 {
                    media_technology_type: MediaTechnologyType::PhtaloCyanine,
                    media_identification_code: MediaIdentificationCode(0b0111010101010000),
                    product_revision_number: 2,
                },
                val
            );
        }
    }
}

pub mod cdrw {
    use deku::DekuRead;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
    #[deku(id_type = "u8", bits = 2)]
    #[repr(u8)]
    pub enum MediaTechnologyType {
        PhaseChange = 0b00,
        Other = 0b11,
        #[deku(id_pat = "_")]
        Reserved(u8),
    }

    pub mod standard {
        use deku::{deku_derive, DekuRead};

        use crate::orange_book::atip::{CdrwSubtype, DiscApplicationCode, DiscSpeed, DiscType};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
        #[deku(id_type = "u8", bits = 3)]
        #[repr(u8)]
        pub enum WritePower {
            W5_0 = 0b000,
            W6_0 = 0b001,
            W7_0 = 0b010,
            W8_0 = 0b011,
            W9_0 = 0b100,
            W10_0 = 0b101,
            W11_0 = 0b110,
            W12_0 = 0b111,
        }

        impl WritePower {
            pub const fn milliwatt(self) -> f32 {
                match self {
                    Self::W5_0 => 5.0,
                    Self::W6_0 => 6.0,
                    Self::W7_0 => 7.0,
                    Self::W8_0 => 8.0,
                    Self::W9_0 => 9.0,
                    Self::W10_0 => 10.0,
                    Self::W11_0 => 11.0,
                    Self::W12_0 => 12.0,
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
        #[deku(id_type = "u8", bits = 3)]
        #[repr(u8)]
        pub enum PowerMultFactor {
            Rho1_00 = 0b000,
            Rho1_05 = 0b001,
            Rho1_10 = 0b010,
            Rho1_15 = 0b011,
            Rho1_20 = 0b100,
            Rho1_25 = 0b101,
            Rho1_30 = 0b110,
            Rho1_35 = 0b111,
        }

        impl PowerMultFactor {
            pub const fn rho(self) -> f32 {
                match self {
                    Self::Rho1_00 => 1.00,
                    Self::Rho1_05 => 1.05,
                    Self::Rho1_10 => 1.10,
                    Self::Rho1_15 => 1.15,
                    Self::Rho1_20 => 1.20,
                    Self::Rho1_25 => 1.25,
                    Self::Rho1_30 => 1.30,
                    Self::Rho1_35 => 1.35,
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
        #[deku(id_type = "u8", bits = 3)]
        #[repr(u8)]
        pub enum TargetModulationValue {
            Gamma0_50 = 0b000,
            Gamma0_60 = 0b001,
            Gamma0_75 = 0b010,
            Gamma0_90 = 0b011,
            Gamma1_10 = 0b100,
            Gamma1_35 = 0b101,
            Gamma1_65 = 0b110,
            Gamma2_00 = 0b111,
        }

        impl TargetModulationValue {
            pub const fn gamma(self) -> f32 {
                match self {
                    Self::Gamma0_50 => 0.50,
                    Self::Gamma0_60 => 0.60,
                    Self::Gamma0_75 => 0.75,
                    Self::Gamma0_90 => 0.90,
                    Self::Gamma1_10 => 1.10,
                    Self::Gamma1_35 => 1.35,
                    Self::Gamma1_65 => 1.65,
                    Self::Gamma2_00 => 2.00,
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
        #[deku(id_type = "u8", bits = 3)]
        #[repr(u8)]
        pub enum EraseWriteRatio {
            Epsilon0_40 = 0b000,
            Epsilon0_43 = 0b001,
            Epsilon0_46 = 0b010,
            Epsilon0_50 = 0b011,
            Epsilon0_54 = 0b100,
            Epsilon0_58 = 0b101,
            Epsilon0_62 = 0b110,
            Epsilon0_66 = 0b111,
        }

        impl EraseWriteRatio {
            pub const fn epsilon(self) -> f32 {
                match self {
                    Self::Epsilon0_40 => 0.40,
                    Self::Epsilon0_43 => 0.43,
                    Self::Epsilon0_46 => 0.46,
                    Self::Epsilon0_50 => 0.50,
                    Self::Epsilon0_54 => 0.54,
                    Self::Epsilon0_58 => 0.58,
                    Self::Epsilon0_62 => 0.62,
                    Self::Epsilon0_66 => 0.66,
                }
            }
        }

        #[deku_derive(DekuRead)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct SpecialInformation1 {
            #[deku(bits = 1, temp, assert_eq = "1")]
            _m1: u8,
            #[deku(pad_bits_after = "1")]
            pub target_writing_power: WritePower,
            #[deku(ctx = "3")]
            pub reference_speed: DiscSpeed,

            #[deku(bits = 1, temp, assert_eq = "0")]
            _s1: u8,
            pub disc_application_code: DiscApplicationCode,

            #[deku(bits = 1, temp, assert_eq = "1")]
            _f1: u8,
            #[deku(assert_eq = "DiscType::Cdrw(CdrwSubtype::Standard)")]
            pub disc_type: DiscType,
            #[deku(bits = 1)]
            pub a1_valid: bool,
            #[deku(bits = 1)]
            pub a2_valid: bool,
            #[deku(bits = 1)]
            pub a3_valid: bool,
        }

        #[deku_derive(DekuRead)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct AdditionalInformation1 {
            #[deku(bits = 1, temp, assert_eq = "0")]
            _m1: u8,
            #[deku(ctx = "3")]
            pub lowest_clv_speed: DiscSpeed,
            #[deku(ctx = "4")]
            pub highest_clv_speed: DiscSpeed,

            #[deku(bits = 1, temp, assert_eq = "0")]
            _s1: u8,
            pub power_mult_factor_ref_speed: PowerMultFactor,
            #[deku(pad_bits_after = "1")]
            pub target_value_modulation_function: TargetModulationValue,

            #[deku(bits = 1, temp, assert_eq = "1")]
            _f1: u8,
            #[deku(pad_bits_after = "4")]
            pub erase_write_ratio_ref_speed: EraseWriteRatio,
        }

        #[deku_derive(DekuRead)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct AdditionalInformation2 {
            #[deku(bits = 1, temp, assert_eq = "0")]
            _m1: u8,
            pub write_power_min_speed: WritePower,
            #[deku(pad_bits_after = "1")]
            pub write_power_max_speed: WritePower,

            #[deku(bits = 1, temp, assert_eq = "1")]
            _s1: u8,
            pub power_mult_factor_min_speed: PowerMultFactor,
            #[deku(pad_bits_after = "1")]
            pub power_mult_factor_max_speed: PowerMultFactor,

            #[deku(bits = 1, temp, assert_eq = "0")]
            _f1: u8,
            pub erase_write_ratio_min_speed: EraseWriteRatio,
            #[deku(pad_bits_after = "1")]
            pub erase_write_ratio_max_speed: EraseWriteRatio,
        }

        #[cfg(test)]
        mod tests {
            use std::io::Cursor;

            use deku::{reader::Reader, DekuReader};

            use crate::orange_book::atip::{
                cdrw::standard::{
                    AdditionalInformation1, EraseWriteRatio, PowerMultFactor, TargetModulationValue,
                },
                DiscSpeed,
            };

            #[test]
            fn parse_additional_information_1() {
                let data: &[u8] = &[0b0000_0010, 0b0100_1100, 0b1001_0000];
                let mut reader = Reader::new(Cursor::new(data));

                let val = AdditionalInformation1::from_reader_with_ctx(&mut reader, ()).unwrap();

                assert_eq!(
                    AdditionalInformation1 {
                        lowest_clv_speed: DiscSpeed::X1,
                        highest_clv_speed: DiscSpeed::X4,
                        power_mult_factor_ref_speed: PowerMultFactor::Rho1_20,
                        target_value_modulation_function: TargetModulationValue::Gamma1_65,
                        erase_write_ratio_ref_speed: EraseWriteRatio::Epsilon0_43
                    },
                    val
                );
            }
        }
    }

    pub mod high_speed {
        use deku::{deku_derive, DekuRead};

        use crate::orange_book::atip::{
            cdrw::{standard::TargetModulationValue, MediaTechnologyType},
            CdrwSubtype, DiscApplicationCode, DiscSpeed, DiscType, MediaIdentificationCode,
        };

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
        #[deku(id_type = "u8", bits = 3)]
        #[repr(u8)]
        pub enum WritePower {
            W13_0 = 0b000,
            W14_0 = 0b001,
            W15_0 = 0b010,
            W16_0 = 0b011,
            W17_0 = 0b100,
            W18_0 = 0b101,
            W19_0 = 0b110,
            W20_0 = 0b111,
        }

        impl WritePower {
            pub const fn milliwatt(self) -> f32 {
                match self {
                    Self::W13_0 => 13.0,
                    Self::W14_0 => 14.0,
                    Self::W15_0 => 15.0,
                    Self::W16_0 => 16.0,
                    Self::W17_0 => 17.0,
                    Self::W18_0 => 18.0,
                    Self::W19_0 => 19.0,
                    Self::W20_0 => 20.0,
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
        #[deku(id_type = "u8", bits = 3)]
        #[repr(u8)]
        pub enum PowerMultFactor {
            Rho1_10 = 0b000,
            Rho1_18 = 0b001,
            Rho1_26 = 0b010,
            Rho1_35 = 0b011,
            Rho1_44 = 0b100,
            Rho1_54 = 0b101,
            Rho1_64 = 0b110,
            Rho1_75 = 0b111,
        }

        impl PowerMultFactor {
            pub const fn rho(self) -> f32 {
                match self {
                    Self::Rho1_10 => 1.10,
                    Self::Rho1_18 => 1.18,
                    Self::Rho1_26 => 1.26,
                    Self::Rho1_35 => 1.35,
                    Self::Rho1_44 => 1.44,
                    Self::Rho1_54 => 1.54,
                    Self::Rho1_64 => 1.64,
                    Self::Rho1_75 => 1.75,
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
        #[deku(id_type = "u8", bits = 3)]
        #[repr(u8)]
        pub enum EraseWriteRatio {
            Epsilon0_30 = 0b000,
            Epsilon0_33 = 0b001,
            Epsilon0_36 = 0b010,
            Epsilon0_39 = 0b011,
            Epsilon0_42 = 0b100,
            Epsilon0_46 = 0b101,
            Epsilon0_50 = 0b110,
            Epsilon0_55 = 0b111,
        }

        impl EraseWriteRatio {
            pub const fn epsilon(self) -> f32 {
                match self {
                    Self::Epsilon0_30 => 0.30,
                    Self::Epsilon0_33 => 0.33,
                    Self::Epsilon0_36 => 0.36,
                    Self::Epsilon0_39 => 0.39,
                    Self::Epsilon0_42 => 0.42,
                    Self::Epsilon0_46 => 0.46,
                    Self::Epsilon0_50 => 0.50,
                    Self::Epsilon0_55 => 0.55,
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
        #[deku(id_type = "u8", bits = 2)]
        #[repr(u8)]
        pub enum OptimumdTtop {
            M0 = 0b00,
            M1 = 0b01,
            M2 = 0b10,
            M3 = 0b11,
        }

        impl OptimumdTtop {
            pub const fn dt_top(self) -> i8 {
                match self {
                    Self::M0 => 0,
                    Self::M1 => 1,
                    Self::M2 => 2,
                    Self::M3 => 3,
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
        #[deku(id_type = "u8", bits = 2)]
        #[repr(u8)]
        pub enum OptimumdTera {
            N0 = 0b00,
            NP1 = 0b01,
            NN2 = 0b10,
            NN1 = 0b11,
        }

        impl OptimumdTera {
            pub const fn dt_top(self) -> i8 {
                match self {
                    Self::N0 => 0,
                    Self::NP1 => 1,
                    Self::NN2 => -2,
                    Self::NN1 => -1,
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
        pub struct WriteStrategyOptimization(OptimumdTtop, OptimumdTera);

        #[deku_derive(DekuRead)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct SpecialInformation1 {
            #[deku(bits = 1, temp, assert_eq = "1")]
            _m1: u8,
            #[deku(pad_bits_after = "1")]
            pub target_writing_power: WritePower,
            #[deku(ctx = "3")]
            pub reference_speed: DiscSpeed,

            #[deku(bits = 1, temp, assert_eq = "0")]
            _s1: u8,
            pub disc_application_code: DiscApplicationCode,

            #[deku(bits = 1, temp, assert_eq = "1")]
            _f1: u8,
            #[deku(assert_eq = "DiscType::Cdrw(CdrwSubtype::HighSpeed)")]
            pub disc_type: DiscType,
            #[deku(bits = 1)]
            pub a1_valid: bool,
            #[deku(bits = 1)]
            pub a2_valid: bool,
            #[deku(bits = 1)]
            pub a3_valid: bool,
        }

        #[deku_derive(DekuRead)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct AdditionalInformation1 {
            #[deku(bits = 1, temp, assert_eq = "0")]
            _m1: u8,
            #[deku(ctx = "3")]
            pub lowest_clv_speed: DiscSpeed,
            #[deku(ctx = "4")]
            pub highest_clv_speed: DiscSpeed,

            #[deku(bits = 1, temp, assert_eq = "0")]
            _s1: u8,
            pub power_mult_factor_ref_speed: PowerMultFactor,
            #[deku(pad_bits_after = "1")]
            pub target_value_modulation_function: TargetModulationValue,

            #[deku(bits = 1, temp, assert_eq = "1")]
            _f1: u8,
            pub erase_write_ratio_ref_speed: EraseWriteRatio,
            pub write_strategy_optimization: WriteStrategyOptimization,
        }

        #[deku_derive(DekuRead)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct AdditionalInformation2 {
            #[deku(bits = 1, temp, assert_eq = "0")]
            _m1: u8,
            pub write_power_min_speed: WritePower,
            #[deku(pad_bits_after = "1")]
            pub write_power_max_speed: WritePower,

            #[deku(bits = 1, temp, assert_eq = "1")]
            _s1: u8,
            pub power_mult_factor_min_speed: PowerMultFactor,
            #[deku(pad_bits_after = "1")]
            pub power_mult_factor_max_speed: PowerMultFactor,

            #[deku(bits = 1, temp, assert_eq = "0")]
            _f1: u8,
            pub erase_write_ratio_min_speed: EraseWriteRatio,
            #[deku(pad_bits_after = "1")]
            pub erase_write_ratio_max_speed: EraseWriteRatio,
        }

        #[deku_derive(DekuRead)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct AdditionalInformation3 {
            #[deku(bits = 1, temp, assert_eq = "0")]
            _m1: u8,
            pub media_technology_type: MediaTechnologyType,
            #[deku(bits = 5, temp)]
            _q1q5: u16,

            #[deku(bits = 1, temp, assert_eq = "1")]
            _s1: u8,
            #[deku(bits = 7, temp)]
            _q6q12: u16,

            #[deku(bits = 1, temp, assert_eq = "1")]
            _f1: u8,
            #[deku(bits = 4, temp)]
            _q13q16: u16,

            #[deku(
                skip,
                default = "MediaIdentificationCode(*_q1q5 << 11 | *_q6q12 << 4 | *_q13q16)"
            )]
            pub media_identification_code: MediaIdentificationCode,
            #[deku(bits = 3)]
            pub product_revision_number: u8,
        }
    }

    pub mod ultra_speed {
        use deku::{deku_derive, DekuRead};

        use crate::orange_book::atip::{
            cdrw::{high_speed::PowerMultFactor, standard::TargetModulationValue},
            CdrwSubtype, DiscApplicationCode, DiscSpeed, DiscType,
        };

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
        #[deku(id_type = "u8", bits = 3)]
        #[repr(u8)]
        pub enum WritePower {
            W35_0 = 0b000,
            W36_0 = 0b001,
            W38_0 = 0b010,
            W39_0 = 0b011,
            W41_0 = 0b100,
            W42_0 = 0b101,
            W44_0 = 0b110,
            W45_0 = 0b111,
        }

        impl WritePower {
            pub const fn milliwatt(self) -> f32 {
                match self {
                    Self::W35_0 => 35.0,
                    Self::W36_0 => 36.0,
                    Self::W38_0 => 38.0,
                    Self::W39_0 => 39.0,
                    Self::W41_0 => 41.0,
                    Self::W42_0 => 42.0,
                    Self::W44_0 => 44.0,
                    Self::W45_0 => 45.0,
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
        #[deku(id_type = "u8", bits = 3)]
        #[repr(u8)]
        pub enum EraseWriteRatio {
            Epsilon0_125 = 0b000,
            Epsilon0_150 = 0b001,
            Epsilon0_175 = 0b010,
            Epsilon0_200 = 0b011,
            Epsilon0_225 = 0b100,
            Epsilon0_250 = 0b101,
            Epsilon0_300 = 0b110,
            Epsilon0_350 = 0b111,
        }

        impl EraseWriteRatio {
            pub const fn epsilon(self) -> f32 {
                match self {
                    Self::Epsilon0_125 => 0.125,
                    Self::Epsilon0_150 => 0.150,
                    Self::Epsilon0_175 => 0.175,
                    Self::Epsilon0_200 => 0.200,
                    Self::Epsilon0_225 => 0.225,
                    Self::Epsilon0_250 => 0.250,
                    Self::Epsilon0_300 => 0.300,
                    Self::Epsilon0_350 => 0.350,
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
        #[deku(id_type = "u8", bits = 2)]
        #[repr(u8)]
        pub enum OptimumdTtop {
            M0 = 0b00,
            M1 = 0b01,
            M2 = 0b10,
            M3 = 0b11,
        }

        impl OptimumdTtop {
            pub const fn dt_top(self) -> i8 {
                match self {
                    Self::M0 => 0,
                    Self::M1 => 1,
                    Self::M2 => 2,
                    Self::M3 => 3,
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
        #[deku(id_type = "u8", bits = 2)]
        #[repr(u8)]
        pub enum OptimumdTera {
            NN1 = 0b00,
            N0 = 0b01,
            NP1 = 0b10,
            NP2 = 0b11,
        }

        impl OptimumdTera {
            pub const fn dt_top(self) -> i8 {
                match self {
                    Self::NN1 => -1,
                    Self::N0 => 0,
                    Self::NP1 => 1,
                    Self::NP2 => 2,
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
        #[deku(id_type = "u8", bits = 3)]
        #[repr(u8)]
        pub enum WritePowerIndication {
            NotSpecified = 0b000,
            W30_0 = 0b001,
            W32_5 = 0b010,
            W35_0 = 0b011,
            W38_0 = 0b100,
            W41_0 = 0b101,
            W45_0 = 0b110,
            W50_0 = 0b111,
        }

        impl WritePowerIndication {
            pub const fn milliwatt(self) -> Option<f32> {
                match self {
                    Self::NotSpecified => None,
                    Self::W30_0 => Some(30.0),
                    Self::W32_5 => Some(32.5),
                    Self::W35_0 => Some(35.0),
                    Self::W38_0 => Some(38.0),
                    Self::W41_0 => Some(41.0),
                    Self::W45_0 => Some(45.0),
                    Self::W50_0 => Some(50.0),
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
        #[deku(id_type = "u8", bits = 3)]
        #[repr(u8)]
        pub enum ErasePowerIndication {
            NotSpecified = 0b000,
            W6_0 = 0b001,
            W7_0 = 0b010,
            W8_0 = 0b011,
            W9_5 = 0b100,
            W11_0 = 0b101,
            W13_0 = 0b110,
            W15_0 = 0b111,
        }

        impl ErasePowerIndication {
            pub const fn milliwatt(self) -> Option<f32> {
                match self {
                    Self::NotSpecified => None,
                    Self::W6_0 => Some(6.0),
                    Self::W7_0 => Some(7.0),
                    Self::W8_0 => Some(8.0),
                    Self::W9_5 => Some(9.5),
                    Self::W11_0 => Some(11.0),
                    Self::W13_0 => Some(13.0),
                    Self::W15_0 => Some(15.0),
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
        pub struct WriteStrategyOptimization(OptimumdTtop, OptimumdTera);

        #[deku_derive(DekuRead)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct SpecialInformation1 {
            #[deku(bits = 1, temp, assert_eq = "1")]
            _m1: u8,
            #[deku(pad_bits_after = "1")]
            pub target_writing_power: WritePower,
            #[deku(ctx = "3")]
            pub reference_speed: DiscSpeed,

            #[deku(bits = 1, temp, assert_eq = "0")]
            _s1: u8,
            pub disc_application_code: DiscApplicationCode,

            #[deku(bits = 1, temp, assert_eq = "1")]
            _f1: u8,
            #[deku(assert_eq = "DiscType::Cdrw(CdrwSubtype::UltraSpeed)")]
            pub disc_type: DiscType,
            #[deku(bits = 1)]
            pub a1_valid: bool,
            #[deku(bits = 1)]
            pub a2_valid: bool,
            #[deku(bits = 1)]
            pub a3_valid: bool,
        }

        #[deku_derive(DekuRead)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct AdditionalInformation1 {
            #[deku(bits = 1, temp, assert_eq = "0")]
            _m1: u8,
            #[deku(ctx = "3")]
            pub min_1t_test_speed: DiscSpeed,
            #[deku(ctx = "4")]
            pub max_1t_test_speed: DiscSpeed,

            #[deku(bits = 1, temp, assert_eq = "0")]
            _s1: u8,
            pub power_mult_factor_1t_test_speed: PowerMultFactor,
            #[deku(pad_bits_after = "1")]
            pub target_value_modulation_function_1t_test_speed: TargetModulationValue,

            #[deku(bits = 1, temp, assert_eq = "1")]
            _f1: u8,
            pub erase_write_ratio_ref_speed: EraseWriteRatio,
            pub write_strategy_optimization: WriteStrategyOptimization,
        }

        #[deku_derive(DekuRead)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct AdditionalInformation2 {
            #[deku(bits = 1, temp, assert_eq = "0")]
            _m1: u8,
            #[deku(ctx = "3")]
            pub min_2t_test_speed: DiscSpeed,
            #[deku(ctx = "4")]
            pub max_2t_test_speed: DiscSpeed,

            #[deku(bits = 1, temp, assert_eq = "1")]
            _s1: u8,
            pub optimum_write_power_16x_2t: WritePowerIndication,
            #[deku(pad_bits_after = "1")]
            pub optimum_write_power_hts_2t: WritePowerIndication,

            #[deku(bits = 1, temp, assert_eq = "0")]
            _f1: u8,
            pub optimum_erase_power_16x_2t: ErasePowerIndication,
            #[deku(pad_bits_after = "1")]
            pub optimum_erase_power_hts_2t: ErasePowerIndication,
        }

        pub use super::high_speed::AdditionalInformation3;
    }

    pub mod ultra_speed_plus {
        use deku::deku_derive;

        use crate::orange_book::atip::DiscApplicationCode;
        use crate::orange_book::atip::{CdrwSubtype, DiscType};

        #[deku_derive(DekuRead)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct SpecialInformation1 {
            #[deku(bits = 1, temp, assert_eq = "1", pad_bits_after = "7")]
            _m1: u8,

            #[deku(bits = 1, temp, assert_eq = "0")]
            _s1: u8,
            pub disc_application_code: DiscApplicationCode,

            #[deku(bits = 1, temp, assert_eq = "1")]
            _f1: u8,
            #[deku(assert_eq = "DiscType::Cdrw(CdrwSubtype::UltraSpeedPlus)")]
            pub disc_type: DiscType,
            #[deku(bits = 1)]
            pub a1_valid: bool,
            #[deku(bits = 1)]
            pub a2_valid: bool,
            #[deku(bits = 1)]
            pub a3_valid: bool,
        }

        // US24 & US32 use the same Additional Information 2 structure
        pub use super::ultra_speed::AdditionalInformation2;
        pub use super::ultra_speed::AdditionalInformation3;
    }
}
