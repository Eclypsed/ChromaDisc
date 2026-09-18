// Realistically this file is temporary and everything in here should be moved elsewhere

use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

pub trait Bit {
    fn bit(self, n: u32) -> bool;
}

impl Bit for u8 {
    #[inline]
    fn bit(self, n: u32) -> bool {
        (self & (1 << n)) != 0
    }
}

pub trait Wire: FromBytes + IntoBytes + Immutable + KnownLayout + Unaligned {}
impl<T> Wire for T where T: FromBytes + IntoBytes + Immutable + KnownLayout + Unaligned {}

pub trait Presence {
    type Output<T: Wire>: Wire;
    const PRESENT: bool;
}

pub struct Present;
pub struct Absent;

impl Presence for Present {
    type Output<T: Wire> = T;
    const PRESENT: bool = true;
}

impl Presence for Absent {
    type Output<T: Wire> = ();
    const PRESENT: bool = false;
}
