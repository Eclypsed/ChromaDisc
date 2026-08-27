//! Widths above 128 are out of scope — u128 is the widest primitive we can
//! use as the smallest-holding-primitive (PLAN §6.4).

use bitfield::BitsEnum;

#[derive(Clone, Copy, PartialEq, Eq, BitsEnum)]
#[bits(129)]
enum TooWide {
    A = 0,
}

fn main() {}
