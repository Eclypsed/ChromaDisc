//! Every variant needs an explicit discriminant — that's the canonical
//! encoding `to_bits` emits (PLAN §5).

use bitfield::BitsEnum;

#[derive(Clone, Copy, PartialEq, Eq, BitsEnum)]
#[bits(2)]
enum Missing {
    A = 0b00,
    B,
    C = 0b10,
    D = 0b11,
}

fn main() {}
