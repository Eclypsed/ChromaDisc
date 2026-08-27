//! `#[bits(fallback)]` and `reserved = ...` are mutually exclusive:
//! the fallback absorbs the complement so asserting it is meaningless
//! (PLAN §6.3).

use bitfield::BitsEnum;

#[derive(Clone, Copy, PartialEq, Eq, BitsEnum)]
#[bits(2, reserved = 0b11)]
enum Conflict {
    A = 0b00,
    B = 0b01,
    #[bits(fallback)]
    Other = 0b10,
}

fn main() {}
