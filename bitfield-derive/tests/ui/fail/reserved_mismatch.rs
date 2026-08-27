//! `reserved = ...` is an assertion: it must equal the computed complement
//! exactly. Drift detection is the whole point of the key (PLAN §6.2).

use bitfield::BitsEnum;

#[derive(Clone, Copy, PartialEq, Eq, BitsEnum)]
#[bits(3, reserved = 0b110)]
enum Drifted {
    A = 0b000,
    B = 0b001,
    C = 0b010,
    D = 0b011,
    E = 0b100,
    F = 0b101,
    // 0b110 AND 0b111 are actually uncovered, but reserved only names one.
}

fn main() {}
