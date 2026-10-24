use core::ops::Range;

use bitfield::BitsEnum;

use self::bytes::SpecialInformation1F;
use super::*;

/// A 3-bit value representing the optimum recording power in mW for CD-R and CD-RW discs.
///
/// See CD-R/WO System Description (Orange Book Part II Volume 1) §4.4
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3)]
pub enum WritePowerReferenceSpeed {
    W4_0 = 0b000,
    W4_4 = 0b001,
    W4_9 = 0b010,
    W5_4 = 0b011,
    W5_9 = 0b100,
    W6_6 = 0b101,
    W7_2 = 0b110,
    W8_0 = 0b111,
}

impl WritePowerReferenceSpeed {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3, reserved = 0b001..=0b111)]
pub enum ReferenceSpeed {
    X1 = 0b000,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3, reserved = 0b000..=0b001 | 0b011..=0b111)]
pub enum LowestTestSpeed {
    X4 = 0b010,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(4, repr = u4, reserved = 0b0000..=0b0101 | 0b1100..=0b1111)]
pub enum HighestTestSpeed {
    X16 = 0b0110,
    X20 = 0b0111,
    X24 = 0b1000,
    X32 = 0b1001,
    X40 = 0b1010,
    X48 = 0b1011,
}

/// A 3-bit value representing a sub-class within the Multi-Speed Recordable disc types
///
/// See CD-R System Description (Orange Book Part II Volume 2) §4.4.4.3
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3, reserved = 0b001..=0b111)]
pub enum HighSpeedSubtype {
    CdrMultiSpeed = 0b000,
}

/// A 2-bit value representing the optimum β range of the medium
///
/// See CD-R System Description (Orange Book Part II Volume 2) §4.4.4.4
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(2, repr = u2)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(4, repr = u4, reserved = 0b0001..=0b1111)]
pub enum AdditionalCapacityLength {
    Minutes2 = 0b0000,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3)]
pub enum WritePowerLowestTestSpeed {
    W7_0 = 0b000,
    W8_0 = 0b001,
    W9_0 = 0b010,
    W10_0 = 0b011,
    W11_0 = 0b100,
    W12_0 = 0b101,
    W13_0 = 0b110,
    W14_0 = 0b111,
}

impl WritePowerLowestTestSpeed {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(4, repr = u4)]
pub enum WritePowerHighestTestSpeed {
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

impl WritePowerHighestTestSpeed {
    pub const fn milliwatt(self) -> f32 {
        match self {
            Self::W16_0 => 16.0,
            Self::W18_0 => 18.0,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3)]
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
/// recording at the Highest Test Speed.
///
/// See CD-R System Description (Orange Book Part II Volume 2) §4.4.5.4
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(2, repr = u2)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(2, repr = u2)]
pub enum MediaTechnologyType {
    Cyanine = 0b00,
    PhthaloCyanine = 0b01,
    Reserved = 0b10,
    Other = 0b11,
}

// See `crate::core::validity` for why this is `pub` in a private module.
mod bytes {
    use crate::core::validity::byte_pattern;

    byte_pattern! {
        /// Byte F of a CD-R's Special Information 1: ID bit 1 and Disc Type 0.
        pub enum SpecialInformation1F = 1 0 x x _ x x x x
    }
}

/// Special Information 1 of a CD-R's ATIP.
#[derive(
    TryFromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct SpecialInformation1 {
    m: SpecialInformation1M,
    s: Msb0,
    f: SpecialInformation1F,
}

impl SpecialInformation1 {
    #[inline]
    pub const fn indicative_optimum_writing_power(&self) -> Bits<WritePowerReferenceSpeed> {
        Bits::new(u3::extract_u8(self.m as u8, 4))
    }

    #[inline]
    pub const fn reference_speed(&self) -> Bits<ReferenceSpeed> {
        Bits::new(u3::extract_u8(self.m as u8, 0))
    }

    #[inline]
    pub const fn disc_application_code(&self) -> Bits<DiscApplicationCode> {
        Bits::new(u7::extract_u8(self.s as u8, 0))
    }

    #[inline]
    pub const fn medium_type(&self) -> Bits<CdRSubtype> {
        Bits::new(u3::extract_u8(self.f as u8, 3))
    }

    bit!(a1_valid: f[2]);
    bit!(a2_valid: f[1]);
    bit!(a3_valid: f[0]);
}

debug_fields!(SpecialInformation1 {
    indicative_optimum_writing_power,
    reference_speed,
    disc_application_code,
    medium_type,
    a1_valid,
    a2_valid,
    a3_valid,
});

/// Additional Information 1 of a CD-R's ATIP.
#[derive(
    TryFromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct AdditionalInformation1 {
    m: Msb0,
    s: Msb0,
    f: Msb1,
}

impl AdditionalInformation1 {
    #[inline]
    pub const fn lowest_test_speed(&self) -> Bits<LowestTestSpeed> {
        Bits::new(u3::extract_u8(self.m as u8, 4))
    }

    #[inline]
    pub const fn highest_test_speed(&self) -> Bits<HighestTestSpeed> {
        Bits::new(u4::extract_u8(self.m as u8, 0))
    }

    #[inline]
    pub const fn high_speed_subtype(&self) -> Bits<HighSpeedSubtype> {
        Bits::new(u3::extract_u8(self.s as u8, 4))
    }

    #[inline]
    pub const fn optimum_beta_range(&self) -> Bits<OptimumBetaRange> {
        Bits::new(u2::extract_u8(self.s as u8, 0))
    }

    #[inline]
    pub const fn optimum_pulse_length(&self) -> Bits<OptimumPulseLength> {
        Bits::new(u3::extract_u8(self.f as u8, 4))
    }

    /// Length of Additional Capacity & Lead-out Area.
    #[inline]
    pub const fn length_of_additional_capacity_and_lead_out_area(
        &self,
    ) -> Bits<AdditionalCapacityLength> {
        Bits::new(u4::extract_u8(self.f as u8, 0))
    }
}

debug_fields!(AdditionalInformation1 {
    lowest_test_speed,
    highest_test_speed,
    high_speed_subtype,
    optimum_beta_range,
    optimum_pulse_length,
    length_of_additional_capacity_and_lead_out_area,
});

/// Additional Information 2 of a CD-R's ATIP.
#[derive(
    TryFromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct AdditionalInformation2 {
    m: Msb0,
    s: Msb1,
    _f: Msb0,
}

impl AdditionalInformation2 {
    #[inline]
    pub const fn indicative_optimum_writing_power_at_lowest_test_speed(
        &self,
    ) -> Bits<WritePowerLowestTestSpeed> {
        Bits::new(u3::extract_u8(self.m as u8, 4))
    }

    #[inline]
    pub const fn indicative_optimum_writing_power_at_highest_test_speed(
        &self,
    ) -> Bits<WritePowerHighestTestSpeed> {
        Bits::new(u4::extract_u8(self.m as u8, 0))
    }

    #[inline]
    pub const fn power_boost_for_i3_pits(&self) -> Bits<PowerBoostI3> {
        Bits::new(u3::extract_u8(self.s as u8, 4))
    }

    #[inline]
    pub const fn pulse_length_correction_after_i3_lands(&self) -> Bits<PulseLengthCorrectionI3> {
        Bits::new(u2::extract_u8(self.s as u8, 2))
    }
}

debug_fields!(AdditionalInformation2 {
    indicative_optimum_writing_power_at_lowest_test_speed,
    indicative_optimum_writing_power_at_highest_test_speed,
    power_boost_for_i3_pits,
    pulse_length_correction_after_i3_lands,
});

/// Additional Information 3 of a CD-R's ATIP.
#[derive(
    TryFromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct AdditionalInformation3 {
    m: Msb0,
    s: Msb1,
    f: Msb1,
}

impl AdditionalInformation3 {
    #[inline]
    pub const fn media_technology_type(&self) -> Bits<MediaTechnologyType> {
        Bits::new(u2::extract_u8(self.m as u8, 5))
    }

    #[inline]
    pub const fn media_identification_code(&self) -> MediaIdentificationCode {
        MediaIdentificationCode::from_section(self.m as u8, self.s as u8, self.f as u8)
    }

    #[inline]
    pub const fn product_revision_number(&self) -> u3 {
        u3::extract_u8(self.f as u8, 0)
    }
}

debug_fields!(AdditionalInformation3 {
    media_technology_type,
    media_identification_code,
    product_revision_number,
});
