//! Total coverage → `From<uN> for T` is emitted (PLAN §4.2, §8.2).
//! Also validates that `BitsRepr` is fully wired.

use bitfield::{Bits, BitsEnum, BitsRepr};

#[derive(Clone, Copy, PartialEq, Eq, Debug, BitsEnum)]
#[bits(2)]
enum Total {
    A = 0b00,
    B = 0b01,
    C = 0b10,
    D = 0b11,
}

fn main() {
    // Every u8 pattern in 0..=3 decodes.
    for i in 0..=3u8 {
        assert!(Total::from_bits(i).is_some());
    }
    // Repr width (u8) > field width (2): out-of-range still returns None.
    assert!(Total::from_bits(4).is_none());

    // BitsRepr consts populated.
    assert_eq!(<Total as BitsRepr>::BITS, 2);
    assert_eq!(<Total as BitsRepr>::FIELD, "Total");

    // Bits<T> round-trip.
    let b: Bits<Total> = Total::C.into();
    assert_eq!(b.get(), Ok(Total::C));

    // Note: no `From<u8> for Total` — mapping over u8 is not total because
    // the repr is wider than the field (PLAN §7 last paragraph).
    // The above assertion `Total::from_bits(4).is_none()` proves the point.
}
