//! Patterns that can't be reduced to intervals — bindings, `_`, paths —
//! are rejected with a message pointing at `#[bits(fallback)]` (PLAN §5).

use bitfield::BitsEnum;

#[derive(Clone, Copy, PartialEq, Eq, BitsEnum)]
#[bits(2)]
enum Nope {
    A = 0b00,
    #[bits(alt = _)]
    B = 0b01,
    C = 0b10,
    D = 0b11,
}

fn main() {}
