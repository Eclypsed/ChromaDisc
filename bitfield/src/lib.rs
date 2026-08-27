//! Wire-protocol bitfield enums with lossless round-tripping.
//!
//! See `PLAN.md` for the full design. The short version:
//!
//! - [`BitsRepr`] is implemented on user enums (typically via
//!   [`#[derive(BitsEnum)]`](BitsEnum)) to describe how a fieldless enum
//!   maps to and from a fixed-width bit pattern.
//! - [`Bits<T>`] wraps the raw pattern verbatim so unmapped values are still
//!   reachable and round-trips are exact even when the mapping is
//!   non-injective.
//! - [`UnmappedBits`] is what falls out of [`Bits::get`] when the stored
//!   pattern has no variant.

// Make `::bitfield::` resolvable from within this crate so the derive can
// emit an absolute path that works everywhere (downstream deps, this crate's
// own tests, and this crate's own examples).
extern crate self as bitfield;

pub use bitfield_derive::BitsEnum;

use std::fmt;
use std::hash::{Hash, Hasher};

/// A fieldless enum that models a fixed-width bit pattern.
///
/// The width of the field ([`BITS`](Self::BITS)) is a semantic property of the
/// mapping and is *not* required to equal the width of [`Bits`](Self::Bits).
/// A 4-bit field can carry `Bits = u8` — decoding will simply reject any
/// pattern outside `0..16`.
pub trait BitsRepr: Sized + Copy {
    /// Conversion boundary type: a primitive (`u8`, `u16`, …) or a wrapper
    /// such as `arbitrary_int::u4`.
    type Bits: Copy + Eq + fmt::Binary;

    /// Field width in bits.
    const BITS: u32;

    /// Field name, used in [`Bits`]'s `Debug` and in [`UnmappedBits`].
    const FIELD: &'static str;

    /// Decode a raw pattern into a variant. Returns `None` for any pattern the
    /// mapping does not claim.
    fn from_bits(bits: Self::Bits) -> Option<Self>;

    /// Encode a variant to its canonical pattern.
    fn to_bits(self) -> Self::Bits;
}

/// A raw bit pattern annotated with the enum it maps against.
///
/// Storage is exactly `T::Bits`, so patterns round-trip losslessly even when
/// `T` is non-injective or partial. Interpretation is deferred to
/// [`get`](Self::get).
pub struct Bits<T: BitsRepr>(T::Bits);

impl<T: BitsRepr> Bits<T> {
    /// Wrap a raw pattern. No validation.
    pub const fn new(bits: T::Bits) -> Self {
        Self(bits)
    }

    /// The raw pattern, verbatim.
    pub fn bits(self) -> T::Bits {
        self.0
    }

    /// Interpret the stored pattern as a variant of `T`.
    ///
    /// When `T` is total (either every pattern is claimed, or a
    /// `#[bits(fallback)]` variant absorbs the rest) this can never return
    /// `Err` — that is expected, not a defect.
    pub fn get(self) -> Result<T, UnmappedBits<T::Bits>> {
        T::from_bits(self.0).ok_or(UnmappedBits {
            raw: self.0,
            field: T::FIELD,
        })
    }
}

impl<T: BitsRepr> From<T> for Bits<T> {
    fn from(v: T) -> Self {
        Self(v.to_bits())
    }
}

// The derives you'd normally reach for here bound `T`, not `T::Bits` — which
// silently forces users to derive Clone/Copy/etc. on their enum. Hand-write
// them bounded on `T::Bits` instead. See PLAN §4.6.

impl<T: BitsRepr> Clone for Bits<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: BitsRepr> Copy for Bits<T> {}

impl<T: BitsRepr> PartialEq for Bits<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: BitsRepr> Eq for Bits<T> {}

impl<T: BitsRepr> Hash for Bits<T>
where
    T::Bits: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

/// `Debug` reports the storage — the raw pattern, zero-padded to the field
/// width and prefixed with `0b`. It deliberately does *not* call [`get`](Bits::get);
/// interpretation is a separate concern. See PLAN §4.6.
impl<T: BitsRepr> fmt::Debug for Bits<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({:#0width$b})",
            T::FIELD,
            self.0,
            width = T::BITS as usize + 2, // `+ 2` for the `0b` prefix
        )
    }
}

/// The error returned by [`Bits::get`] when the stored pattern has no variant.
///
/// This is *unmapped*, not *reserved* — whether an unmapped pattern is
/// spec-reserved, vendor-reserved, or simply not-yet-allocated is a
/// domain-level distinction the crate deliberately takes no position on.
/// See PLAN §3.2.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct UnmappedBits<B> {
    pub raw: B,
    pub field: &'static str,
}

impl<B: fmt::Binary> fmt::Display for UnmappedBits<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "unmapped bit pattern {:#b} in field `{}`",
            self.raw, self.field,
        )
    }
}

impl<B: fmt::Debug + fmt::Binary> std::error::Error for UnmappedBits<B> {}
