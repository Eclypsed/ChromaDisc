//! Partial coverage with a matching `reserved = ...` assertion compiles.
//! No `From<uN>` because the mapping is deliberately partial.

use bitfield::{Bits, BitsEnum, BitsRepr};

#[derive(Clone, Copy, PartialEq, Eq, Debug, BitsEnum)]
#[bits(3, reserved = 0b110..=0b111)]
enum Partial {
    A = 0b000,
    B = 0b001,
    C = 0b010,
    D = 0b011,
    E = 0b100,
    F = 0b101,
}

fn main() {
    let unmapped = Bits::<Partial>::new(0b110);
    let err = unmapped.get().unwrap_err();
    assert_eq!(err.raw, 0b110);
    assert_eq!(err.field, "Partial");

    assert_eq!(<Partial as BitsRepr>::BITS, 3);
}
