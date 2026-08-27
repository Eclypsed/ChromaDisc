//! Payload variants are out of scope for v1: `Bits<T>` already holds the
//! raw pattern losslessly, so a payload would be a second, unvalidated
//! copy (PLAN §3.3).

use bitfield::BitsEnum;

#[derive(Clone, Copy, PartialEq, Eq, BitsEnum)]
#[bits(2)]
#[repr(u8)] // silences E0732 so the test focuses on the derive's rejection
enum WithPayload {
    A = 0b00,
    B(u8) = 0b01,
    C = 0b10,
    D = 0b11,
}

fn main() {}
