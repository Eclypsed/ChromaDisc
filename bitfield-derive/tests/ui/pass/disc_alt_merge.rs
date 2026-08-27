//! When a variant's discriminant is contiguous with any of its `alt` cases,
//! the derive folds them into a single range in the generated match arm.
//! This keeps `clippy::manual_range_patterns` quiet in downstream crates
//! (`0b1011 | 0b1100..=0b1101` → `0b1011..=0b1101`).
//!
//! Adjacencies *within* `alt` are deliberately preserved — that's a lint the
//! user wants to see if they authored it.

use bitfield::{Bits, BitsEnum};

#[derive(Clone, Copy, PartialEq, Eq, Debug, BitsEnum)]
#[bits(3, reserved = 0b101)]
enum Merged {
    A = 0b000,
    // 0b001 is contiguous with disc 0b010 → generated arm is `0b001..=0b010`.
    #[bits(alt = 0b001)]
    B = 0b010,
    // 0b100 is contiguous on the right (0b011+1); 0b110 is not.
    // Generated: `0b011..=0b100 | 0b110`.
    #[bits(alt = 0b100 | 0b110)]
    C = 0b011,
    // 0b101 is asserted reserved; 0b111 is D.
    D = 0b111,
}

fn main() {
    assert_eq!(Bits::<Merged>::new(0b001).get(), Ok(Merged::B));
    assert_eq!(Bits::<Merged>::new(0b010).get(), Ok(Merged::B));

    assert_eq!(Bits::<Merged>::new(0b011).get(), Ok(Merged::C));
    assert_eq!(Bits::<Merged>::new(0b100).get(), Ok(Merged::C));
    assert_eq!(Bits::<Merged>::new(0b110).get(), Ok(Merged::C));

    // Reserved pattern stays unmapped.
    assert!(Bits::<Merged>::new(0b101).get().is_err());
}
