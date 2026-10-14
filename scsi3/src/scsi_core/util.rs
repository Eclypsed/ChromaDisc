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

macro_rules! bit {
    ($(#[$m:meta])* $name:ident: $field:ident[$bit:literal]) => {
        $(#[$m])*
        #[inline]
        pub const fn $name(&self) -> bool { self.$field & (1 << $bit) != 0 }
    };
}

pub(crate) use bit;

/// Implements `Debug` as a struct of what the given accessors return.
macro_rules! debug_fields {
    ($type:ident { $($accessor:ident),* $(,)? }) => {
        impl ::core::fmt::Debug for $type {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(stringify!($type))
                    $(.field(stringify!($accessor).trim_start_matches("r#"), &self.$accessor()))*
                    .finish()
            }
        }
    };
}

pub(crate) use debug_fields;

#[macro_export]
macro_rules! bit_set {
    ($byte:expr, $bit:literal) => {
        (($byte & (1 << $bit)) != 0)
    };
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
