//! Exact-width repr (u4) + total coverage → `From<u4> for T` is emitted
//! (PLAN §7 last paragraph).

use arbitrary_int::u4;
use bitfield::{Bits, BitsEnum};

#[derive(Clone, Copy, PartialEq, Eq, Debug, BitsEnum)]
#[bits(4, repr = u4)]
enum Nibble {
    V0 = 0x0,
    V1 = 0x1,
    V2 = 0x2,
    V3 = 0x3,
    V4 = 0x4,
    V5 = 0x5,
    V6 = 0x6,
    V7 = 0x7,
    V8 = 0x8,
    V9 = 0x9,
    VA = 0xA,
    VB = 0xB,
    VC = 0xC,
    VD = 0xD,
    VE = 0xE,
    VF = 0xF,
}

fn main() {
    // Total + exact-width repr → From<u4> exists.
    let v: Nibble = u4::new(0x5).into();
    assert_eq!(v, Nibble::V5);

    // Bits<T> round-trip still works.
    let wrapped: Bits<Nibble> = Nibble::VA.into();
    assert_eq!(wrapped.get(), Ok(Nibble::VA));
}
