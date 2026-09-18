use crate::core::util::{Absent, Presence, Present};
use arbitrary_int::u2;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

mod private {
    use super::*;

    pub trait C2Marker {
        const C2_SELECTION: u2;
        type C2Data: FromBytes + IntoBytes + Immutable + KnownLayout + Unaligned;
        type C2Presence: Presence;
    }
}

pub trait C2ErrorInfo: private::C2Marker {}

pub struct NoC2;
impl private::C2Marker for NoC2 {
    const C2_SELECTION: u2 = u2::new(0b00);
    type C2Data = ();
    type C2Presence = Absent;
}
impl C2ErrorInfo for NoC2 {}

pub struct C2Pointers;
impl private::C2Marker for C2Pointers {
    const C2_SELECTION: u2 = u2::new(0b01);
    type C2Data = [u8; 294];
    type C2Presence = Present;
}
impl C2ErrorInfo for C2Pointers {}

pub struct BlockC2Pointers;
impl private::C2Marker for BlockC2Pointers {
    const C2_SELECTION: u2 = u2::new(0b10);
    type C2Data = [u8; 296];
    type C2Presence = Present;
}
impl C2ErrorInfo for BlockC2Pointers {}
