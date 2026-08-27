//! End-to-end check on the derive: PLAN §5's `ExampleField` verbatim,
//! decoded and encoded through `Bits<T>`.

use arbitrary_int::u4;
use bitfield::{Bits, BitsEnum, UnmappedBits};

#[derive(Clone, Copy, PartialEq, Eq, Debug, BitsEnum)]
#[bits(4, repr = u4, reserved = 0b1110..=0b1111)]
#[non_exhaustive]
#[repr(u8)]
pub enum ExampleField {
    ValueA = 0b0000,
    ValueB = 0b0001,
    #[bits(alt = 0b0100)]
    ValueC = 0b0010,
    ValueD = 0b0011,
    #[bits(alt = 0b0110 | 0b1000..=0b1010)]
    ValueE = 0b0101,
    ValueF = 0b0111,
    #[bits(alt = 0b1100..=0b1101)]
    VendorSpecific = 0b1011,
}

fn main() {
    // 1. Canonical round-trip: enum → bits → enum.
    for v in [
        ExampleField::ValueA,
        ExampleField::ValueB,
        ExampleField::ValueC,
        ExampleField::ValueD,
        ExampleField::ValueE,
        ExampleField::ValueF,
        ExampleField::VendorSpecific,
    ] {
        let wrapped: Bits<ExampleField> = v.into();
        assert_eq!(
            wrapped.get(),
            Ok(v),
            "canonical round-trip failed for {v:?}"
        );
    }

    // 2. Non-injective decode: alt patterns decode to the same variant.
    let alt = Bits::<ExampleField>::new(u4::new(0b0100));
    assert_eq!(alt.get(), Ok(ExampleField::ValueC));

    let alt2 = Bits::<ExampleField>::new(u4::new(0b1010));
    assert_eq!(alt2.get(), Ok(ExampleField::ValueE));

    // 3. Bit equality is not semantic equality.
    let canonical = Bits::<ExampleField>::from(ExampleField::ValueC);
    let alt_pattern = Bits::<ExampleField>::new(u4::new(0b0100));
    assert_ne!(canonical, alt_pattern, "Bits equality is bit equality");
    assert_eq!(canonical.get(), alt_pattern.get());

    // 4. Unmapped patterns surface as UnmappedBits, carrying the field name.
    let unmapped = Bits::<ExampleField>::new(u4::new(0b1110));
    let err = unmapped.get().unwrap_err();
    assert_eq!(
        err,
        UnmappedBits {
            raw: u4::new(0b1110),
            field: "ExampleField",
        },
    );
    assert_eq!(
        err.to_string(),
        "unmapped bit pattern 0b1110 in field `ExampleField`",
    );

    // 5. Debug prints storage, zero-padded to the field width, prefixed with 0b.
    assert_eq!(
        format!("{:?}", Bits::<ExampleField>::new(u4::new(0b0010))),
        "ExampleField(0b0010)",
    );
    assert_eq!(format!("{:?}", unmapped), "ExampleField(0b1110)");

    println!("phase 4 validation gate: derive round-trips cleanly");
}
