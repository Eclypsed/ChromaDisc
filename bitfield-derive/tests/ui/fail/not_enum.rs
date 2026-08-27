//! `BitsEnum` only makes sense on enums (PLAN §3.3).

use bitfield::BitsEnum;

#[derive(BitsEnum)]
#[bits(4)]
struct NotAnEnum {
    field: u8,
}

fn main() {}
