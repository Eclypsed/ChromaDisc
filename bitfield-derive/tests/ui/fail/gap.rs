//! Partial coverage with no `reserved` assertion and no `fallback` variant
//! is an error — coverage is a fact and the user needs to acknowledge it
//! (PLAN §6.1, §6.2).

use bitfield::BitsEnum;

#[derive(Clone, Copy, PartialEq, Eq, BitsEnum)]
#[bits(2)]
enum Partial {
    A = 0b00,
    B = 0b01,
    // 0b10 and 0b11 are uncovered
}

fn main() {}
