use self::bytes::SpecialInformation1F;
use super::standard::TargetModulationValue;
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3, reserved = 0b000..=0b010 | 0b100..=0b111)]
pub enum ReferenceSpeed {
    X8 = 0b011,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3, reserved = 0b000..=0b001 | 0b011..=0b111)]
pub enum LowestClvSpeed {
    X4 = 0b010,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(4, repr = u4, reserved = 0b0000..=0b0011 | 0b0101..=0b1111)]
pub enum HighestClvSpeed {
    X10 = 0b0100,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(2, repr = u2)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(2, repr = u2)]
pub enum OptimumdTera {
    N0 = 0b00,
    NP1 = 0b01,
    NN2 = 0b10,
    NN1 = 0b11,
}

impl OptimumdTera {
    pub const fn dt_era(self) -> i8 {
        match self {
            Self::N0 => 0,
            Self::NP1 => 1,
            Self::NN2 => -2,
            Self::NN1 => -1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WriteStrategyOptimization(pub Bits<OptimumdTtop>, pub Bits<OptimumdTera>);

// See `crate::core::validity` for why this is `pub` in a private module.
mod bytes {
    use crate::core::validity::byte_pattern;

    byte_pattern! {
        /// Byte F of a High Speed CD-RW's Special Information 1: ID bit 1, Disc Type 1 and Disc
        /// Sub-Type 001b.
        pub enum SpecialInformation1F = 1 1 0 0 _ 1 x x x
    }
}

/// Special Information 1 of a High Speed CD-RW's ATIP.
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
    pub const fn indicative_target_writing_power_at_reference_speed(&self) -> Bits<WritePower> {
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

/// Additional Information 1 of a High Speed CD-RW's ATIP.
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

    /// Target γ value of the modulation/power function for all speeds.
    #[inline]
    pub const fn target_gamma_value_of_the_modulation_power_function_for_all_speeds(
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

    #[inline]
    pub const fn write_strategy_optimization(&self) -> WriteStrategyOptimization {
        WriteStrategyOptimization(
            Bits::new(u2::extract_u8(self.f as u8, 2)),
            Bits::new(u2::extract_u8(self.f as u8, 0)),
        )
    }
}

debug_fields!(AdditionalInformation1 {
    lowest_usable_clv_recording_speed,
    highest_usable_clv_recording_speed,
    power_multiplication_factor_at_reference_speed,
    target_gamma_value_of_the_modulation_power_function_for_all_speeds,
    recommended_erase_write_power_ratio_at_reference_speed,
    write_strategy_optimization,
});

/// Additional Information 2 of a High Speed CD-RW's ATIP.
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
    ) -> Bits<WritePower> {
        Bits::new(u3::extract_u8(self.m as u8, 4))
    }

    #[inline]
    pub const fn indicative_target_writing_power_at_highest_usable_recording_speed(
        &self,
    ) -> Bits<WritePower> {
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

/// Additional Information 3 of a High Speed, Ultra Speed or Ultra Speed+ CD-RW's ATIP.
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
