use self::bytes::SpecialInformation1F;
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3)]
pub enum WritePowerReferenceSpeed {
    W5_0 = 0b000,
    W6_0 = 0b001,
    W7_0 = 0b010,
    W8_0 = 0b011,
    W9_0 = 0b100,
    W10_0 = 0b101,
    W11_0 = 0b110,
    W12_0 = 0b111,
}

impl WritePowerReferenceSpeed {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3, reserved = 0b000 | 0b010..=0b111)]
pub enum ReferenceSpeed {
    X2 = 0b001,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3, reserved = 0b010..=0b111)]
pub enum LowestClvSpeed {
    X1 = 0b000,
    X2 = 0b001,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(4, repr = u4, reserved = 0b0000 | 0b0011..=0b1111)]
pub enum HighestClvSpeed {
    X2 = 0b0001,
    X4 = 0b0010,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3)]
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

// See `crate::core::validity` for why this is `pub` in a private module.
mod bytes {
    use crate::core::validity::byte_pattern;

    byte_pattern! {
        /// Byte F of a Standard Speed CD-RW's Special Information 1: ID bit 1, Disc Type 1 and Disc
        /// Sub-Type 000b.
        pub enum SpecialInformation1F = 1 1 0 0 _ 0 x x x
    }
}

/// Special Information 1 of a Standard Speed CD-RW's ATIP.
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
    pub const fn indicative_target_writing_power_at_reference_speed(
        &self,
    ) -> Bits<WritePowerReferenceSpeed> {
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

    bit!(a1_valid: f[2]);
    bit!(a2_valid: f[1]);
    bit!(a3_valid: f[0]);
}

debug_fields!(SpecialInformation1 {
    indicative_target_writing_power_at_reference_speed,
    reference_speed,
    disc_application_code,
    a1_valid,
    a2_valid,
    a3_valid,
});

/// Additional Information 1 of a Standard Speed CD-RW's ATIP.
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
    pub const fn lowest_usable_clv_recording_speed(&self) -> Bits<LowestClvSpeed> {
        Bits::new(u3::extract_u8(self.m as u8, 4))
    }

    #[inline]
    pub const fn highest_usable_clv_recording_speed(&self) -> Bits<HighestClvSpeed> {
        Bits::new(u4::extract_u8(self.m as u8, 0))
    }

    /// Power Multiplication Factor ρ at Reference Speed.
    #[inline]
    pub const fn power_multiplication_factor_at_reference_speed(&self) -> Bits<PowerMultFactor> {
        Bits::new(u3::extract_u8(self.s as u8, 4))
    }

    /// Target γ value of the Modulation/Power function.
    #[inline]
    pub const fn target_gamma_value_of_the_modulation_power_function(
        &self,
    ) -> Bits<TargetModulationValue> {
        Bits::new(u3::extract_u8(self.s as u8, 1))
    }

    /// Recommended erase/write power ratio ε at Reference Speed.
    #[inline]
    pub const fn recommended_erase_write_power_ratio_at_reference_speed(
        &self,
    ) -> Bits<EraseWriteRatio> {
        Bits::new(u3::extract_u8(self.f as u8, 4))
    }
}

debug_fields!(AdditionalInformation1 {
    lowest_usable_clv_recording_speed,
    highest_usable_clv_recording_speed,
    power_multiplication_factor_at_reference_speed,
    target_gamma_value_of_the_modulation_power_function,
    recommended_erase_write_power_ratio_at_reference_speed,
});

/// Additional Information 2 of a Standard Speed CD-RW's ATIP.
#[derive(
    TryFromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct AdditionalInformation2 {
    m: Msb0,
    s: Msb1,
    f: Msb0,
}

impl AdditionalInformation2 {
    #[inline]
    pub const fn indicative_target_writing_power_at_lowest_usable_recording_speed(
        &self,
    ) -> Bits<WritePowerReferenceSpeed> {
        Bits::new(u3::extract_u8(self.m as u8, 4))
    }

    #[inline]
    pub const fn indicative_target_writing_power_at_highest_usable_recording_speed(
        &self,
    ) -> Bits<WritePowerReferenceSpeed> {
        Bits::new(u3::extract_u8(self.m as u8, 1))
    }

    /// Power Multiplication Factor ρ at Lowest Usable Recording Speed.
    #[inline]
    pub const fn power_multiplication_factor_at_lowest_usable_recording_speed(
        &self,
    ) -> Bits<PowerMultFactor> {
        Bits::new(u3::extract_u8(self.s as u8, 4))
    }

    /// Power Multiplication Factor ρ at Highest Usable Recording Speed.
    #[inline]
    pub const fn power_multiplication_factor_at_highest_usable_recording_speed(
        &self,
    ) -> Bits<PowerMultFactor> {
        Bits::new(u3::extract_u8(self.s as u8, 1))
    }

    #[inline]
    pub const fn recommended_erase_write_power_ratio_at_lowest_usable_recording_speed(
        &self,
    ) -> Bits<EraseWriteRatio> {
        Bits::new(u3::extract_u8(self.f as u8, 4))
    }

    #[inline]
    pub const fn recommended_erase_write_power_ratio_at_highest_usable_recording_speed(
        &self,
    ) -> Bits<EraseWriteRatio> {
        Bits::new(u3::extract_u8(self.f as u8, 1))
    }
}

debug_fields!(AdditionalInformation2 {
    indicative_target_writing_power_at_lowest_usable_recording_speed,
    indicative_target_writing_power_at_highest_usable_recording_speed,
    power_multiplication_factor_at_lowest_usable_recording_speed,
    power_multiplication_factor_at_highest_usable_recording_speed,
    recommended_erase_write_power_ratio_at_lowest_usable_recording_speed,
    recommended_erase_write_power_ratio_at_highest_usable_recording_speed,
});

// Additional Information 3 - All reserved
