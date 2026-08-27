//! Two variants claiming the same pattern — coverage says every pattern
//! belongs to at most one variant (PLAN §6.1).

use bitfield::BitsEnum;

#[derive(Clone, Copy, PartialEq, Eq, BitsEnum)]
#[bits(2)]
enum Overlap {
    A = 0b00,
    #[bits(alt = 0b00)]
    B = 0b01,
    C = 0b10,
    D = 0b11,
}

fn main() {}
