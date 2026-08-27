//! A literal that doesn't fit in the declared width — caught during
//! analysis with a span pointing at the offending literal (PLAN §6.4).

use bitfield::BitsEnum;

#[derive(Clone, Copy, PartialEq, Eq, BitsEnum)]
#[bits(3)]
enum TooWide {
    A = 0b000,
    B = 0b001,
    C = 0b010,
    D = 0b011,
    E = 0b100,
    F = 0b101,
    G = 0b110,
    H = 0b1000, // 4 bits, doesn't fit in 3
}

fn main() {}
