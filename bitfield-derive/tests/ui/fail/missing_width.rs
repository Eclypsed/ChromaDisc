//! Width is required. There is no sensible default (PLAN §5).

use bitfield::BitsEnum;

#[derive(Clone, Copy, PartialEq, Eq, BitsEnum)]
#[bits(repr = u8)]
enum NoWidth {
    A = 0,
    B = 1,
}

fn main() {}
