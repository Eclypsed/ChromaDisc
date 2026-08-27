//! `#[bits(fallback)]` makes the mapping total by construction (PLAN §6.3).
//! Every pattern maps to some variant; `get()` never returns Err.

use bitfield::{Bits, BitsEnum};

#[derive(Clone, Copy, PartialEq, Eq, Debug, BitsEnum)]
#[bits(2)]
enum WithFallback {
    A = 0b00,
    B = 0b01,
    #[bits(fallback)]
    Other = 0b11,
}

fn main() {
    // Explicit variants win.
    assert_eq!(Bits::<WithFallback>::new(0b00).get(), Ok(WithFallback::A));
    assert_eq!(Bits::<WithFallback>::new(0b01).get(), Ok(WithFallback::B));

    // Everything else falls into the catch-all.
    assert_eq!(Bits::<WithFallback>::new(0b10).get(), Ok(WithFallback::Other));
    assert_eq!(Bits::<WithFallback>::new(0b11).get(), Ok(WithFallback::Other));
}
