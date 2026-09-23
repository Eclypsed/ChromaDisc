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

pub trait Zc: FromBytes + IntoBytes + Immutable + KnownLayout + Unaligned {}
impl<T> Zc for T where T: FromBytes + IntoBytes + Immutable + KnownLayout + Unaligned {}

pub trait Presence {
    type Output<T: Zc>: Zc;
    const PRESENT: bool;
}

pub struct Present;
pub struct Absent;

impl Presence for Present {
    type Output<T: Zc> = T;
    const PRESENT: bool = true;
}

impl Presence for Absent {
    type Output<T: Zc> = ();
    const PRESENT: bool = false;
}

pub trait BytesRepr: Sized {
    type Bytes: Zc;
    type Error;

    fn from_bytes(bytes: Self::Bytes) -> Result<Self, Self::Error>;
    fn to_bytes(&self) -> Self::Bytes;
}

#[repr(transparent)]
#[derive(FromBytes, IntoBytes, Immutable, KnownLayout, Unaligned)]
pub struct Bytes<T: BytesRepr>(T::Bytes);

impl<T: BytesRepr> Bytes<T> {
    pub fn get(self) -> Result<T, T::Error> {
        T::from_bytes(self.0)
    }
}

impl<T: BytesRepr> From<T> for Bytes<T> {
    fn from(value: T) -> Self {
        Bytes(value.to_bytes())
    }
}
