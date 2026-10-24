use self::bytes::SpecialInformation1F;
use super::high_speed::{OptimumdTtop, PowerMultFactor};
use super::standard::TargetModulationValue;
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3)]
pub enum WritePower1tTestSpeed {
    W35_0 = 0b000,
    W36_0 = 0b001,
    W38_0 = 0b010,
    W39_0 = 0b011,
    W41_0 = 0b100,
    W42_0 = 0b101,
    W44_0 = 0b110,
    W45_0 = 0b111,
}

impl WritePower1tTestSpeed {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3, reserved = 0b000..=0b101 | 0b111)]
pub enum TestSpeed1t {
    X16 = 0b110,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3, reserved = 0b000..=0b101 | 0b111)]
pub enum Lowest1tTestSpeed {
    X16 = 0b110,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(4, repr = u4, reserved = 0b0000..=0b0101 | 0b0111..=0b1111)]
pub enum Highest1tTestSpeed {
    X16 = 0b0110,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(2, repr = u2)]
pub enum OptimumdTera {
    NN1 = 0b00,
    N0 = 0b01,
    NP1 = 0b10,
    NP2 = 0b11,
}

impl OptimumdTera {
    pub const fn dt_era(self) -> i8 {
        match self {
            Self::NN1 => -1,
            Self::N0 => 0,
            Self::NP1 => 1,
            Self::NP2 => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3, reserved = 0b000..=0b010 | 0b100..=0b111)]
pub enum Lowest2tTestSpeed {
    X8 = 0b011,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(4, repr = u4, reserved = 0b0000..=0b0111 | 0b1001..=0b1111)]
pub enum Highest2tTestSpeed {
    X24 = 0b1000,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WriteStrategyOptimization(pub Bits<OptimumdTtop>, pub Bits<OptimumdTera>);

// See `crate::core::validity` for why this is `pub` in a private module.
mod bytes {
    use crate::core::validity::byte_pattern;

    byte_pattern! {
        /// Byte F of an Ultra Speed CD-RW's Special Information 1: ID bit 1, Disc Type 1 and Disc
        /// Sub-Type 010b.
        pub enum SpecialInformation1F = 1 1 0 1 _ 0 x x x
    }
}

/// Special Information 1 of an Ultra Speed CD-RW's ATIP.
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
    /// Indicative Target Writing Power at '1T' Test Speed.
    #[inline]
    pub const fn indicative_target_writing_power_at_1t_test_speed(
        &self,
    ) -> Bits<WritePower1tTestSpeed> {
        Bits::new(u3::extract_u8(self.m as u8, 4))
    }

    /// '1T' Test Speed.
    #[doc(alias = "1T Test Speed")]
    #[inline]
    pub const fn test_speed_1t(&self) -> Bits<TestSpeed1t> {
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
    indicative_target_writing_power_at_1t_test_speed,
    test_speed_1t,
    disc_application_code,
    a1_valid,
    a2_valid,
    a3_valid,
});

/// Additional Information 1 of an Ultra Speed CD-RW's ATIP.
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
    /// Lowest '1T' Test Speed.
    #[inline]
    pub const fn lowest_1t_test_speed(&self) -> Bits<Lowest1tTestSpeed> {
        Bits::new(u3::extract_u8(self.m as u8, 4))
    }

    /// Highest '1T' Test Speed.
    #[inline]
    pub const fn highest_1t_test_speed(&self) -> Bits<Highest1tTestSpeed> {
        Bits::new(u4::extract_u8(self.m as u8, 0))
    }

    /// Power Multiplication Factor ρ at '1T' Test Speed.
    #[inline]
    pub const fn power_multiplication_factor_at_1t_test_speed(&self) -> Bits<PowerMultFactor> {
        Bits::new(u3::extract_u8(self.s as u8, 4))
    }

    /// Target γ value of the modulation/power function at '1T' Test Speed.
    #[inline]
    pub const fn target_gamma_value_of_the_modulation_power_function_at_1t_test_speed(
        &self,
    ) -> Bits<TargetModulationValue> {
        Bits::new(u3::extract_u8(self.s as u8, 1))
    }

    /// Recommended erase/write power ratio ε at '1T' Test Speed.
    #[inline]
    pub const fn recommended_erase_write_power_ratio_at_1t_test_speed(
        &self,
    ) -> Bits<EraseWriteRatio> {
        Bits::new(u3::extract_u8(self.f as u8, 4))
    }

    /// Write-strategy optimization at '1T' Test Speed.
    #[inline]
    pub const fn write_strategy_optimization_at_1t_test_speed(&self) -> WriteStrategyOptimization {
        WriteStrategyOptimization(
            Bits::new(u2::extract_u8(self.f as u8, 2)),
            Bits::new(u2::extract_u8(self.f as u8, 0)),
        )
    }
}

debug_fields!(AdditionalInformation1 {
    lowest_1t_test_speed,
    highest_1t_test_speed,
    power_multiplication_factor_at_1t_test_speed,
    target_gamma_value_of_the_modulation_power_function_at_1t_test_speed,
    recommended_erase_write_power_ratio_at_1t_test_speed,
    write_strategy_optimization_at_1t_test_speed,
});

/// Additional Information 2 of an Ultra Speed CD-RW's ATIP.
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
    /// Lowest '2T' Write-strategy Test Speed.
    #[inline]
    pub const fn lowest_2t_write_strategy_test_speed(&self) -> Bits<Lowest2tTestSpeed> {
        Bits::new(u3::extract_u8(self.m as u8, 4))
    }

    /// Highest '2T' Write-strategy Test Speed.
    #[inline]
    pub const fn highest_2t_write_strategy_test_speed(&self) -> Bits<Highest2tTestSpeed> {
        Bits::new(u4::extract_u8(self.m as u8, 0))
    }

    /// Optimum '2T' Write-strategy write power indication at 16x.
    #[inline]
    pub const fn optimum_2t_write_strategy_write_power_indication_at_16x(
        &self,
    ) -> Bits<WritePowerIndication> {
        Bits::new(u3::extract_u8(self.s as u8, 4))
    }

    /// Optimum '2T' Write-strategy write power indication at HTS.
    #[inline]
    pub const fn optimum_2t_write_strategy_write_power_indication_at_hts(
        &self,
    ) -> Bits<WritePowerIndication> {
        Bits::new(u3::extract_u8(self.s as u8, 1))
    }

    /// Optimum '2T' Write-strategy erase power indication at 16x.
    #[inline]
    pub const fn optimum_2t_write_strategy_erase_power_indication_at_16x(
        &self,
    ) -> Bits<ErasePowerIndication> {
        Bits::new(u3::extract_u8(self.f as u8, 4))
    }

    /// Optimum '2T' Write-strategy erase power indication at HTS.
    #[inline]
    pub const fn optimum_2t_write_strategy_erase_power_indication_at_hts(
        &self,
    ) -> Bits<ErasePowerIndication> {
        Bits::new(u3::extract_u8(self.f as u8, 1))
    }
}

debug_fields!(AdditionalInformation2 {
    lowest_2t_write_strategy_test_speed,
    highest_2t_write_strategy_test_speed,
    optimum_2t_write_strategy_write_power_indication_at_16x,
    optimum_2t_write_strategy_write_power_indication_at_hts,
    optimum_2t_write_strategy_erase_power_indication_at_16x,
    optimum_2t_write_strategy_erase_power_indication_at_hts,
});

pub use super::high_speed::AdditionalInformation3;
