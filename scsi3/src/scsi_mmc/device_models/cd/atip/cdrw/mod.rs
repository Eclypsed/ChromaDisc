use super::*;

pub mod high_speed;
pub mod reserved;
pub mod standard;
pub mod ultra_speed;
pub mod ultra_speed_plus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(2, repr = u2, reserved = 0b01..=0b10)]
pub enum MediaTechnologyType {
    PhaseChange = 0b00,
    Other = 0b11,
}
