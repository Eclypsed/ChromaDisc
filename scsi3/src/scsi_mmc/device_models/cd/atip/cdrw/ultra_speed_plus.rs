use self::bytes::SpecialInformation1F;
use super::ultra_speed::{ErasePowerIndication, Lowest2tTestSpeed, WritePowerIndication};
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(4, repr = u4, reserved = 0b0000..=0b1000 | 0b1010..=0b1111)]
pub enum Highest2tTestSpeed {
    X32 = 0b1001,
}

// See `crate::core::validity` for why this is `pub` in a private module.
mod bytes {
    use crate::core::validity::byte_pattern;

    byte_pattern! {
        /// Byte F of an Ultra Speed+ CD-RW's Special Information 1: ID bit 1, Disc Type 1 and Disc
        /// Sub-Type 011b.
        pub enum SpecialInformation1F = 1 1 0 1 _ 1 x x x
    }
}

/// Special Information 1 of an Ultra Speed+ CD-RW's ATIP.
#[derive(
    TryFromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct SpecialInformation1 {
    // M - all reserved
    _m: SpecialInformation1M,
    s: Msb0,
    f: SpecialInformation1F,
}

impl SpecialInformation1 {
    #[inline]
    pub const fn disc_application_code(&self) -> Bits<DiscApplicationCode> {
        Bits::new(u7::extract_u8(self.s as u8, 0))
    }

    bit!(a1_valid: f[2]);
    bit!(a2_valid: f[1]);
    bit!(a3_valid: f[0]);
}

debug_fields!(SpecialInformation1 {
    disc_application_code,
    a1_valid,
    a2_valid,
    a3_valid,
});

// Additional Information 1 - All reserved

/// Additional Information 2 of an Ultra Speed+ CD-RW's ATIP.
///
/// Laid out as an Ultra Speed CD-RW's, except for the values of the Highest '2T'
/// Write-strategy Test Speed.
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

pub use super::ultra_speed::AdditionalInformation3;
