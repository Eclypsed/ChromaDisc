//! Binary-Coded Decimal (BCD) representation and conversion utilities.
//!
//! This crate provides [`Bcd<BYTES>`], a fixed-size, const-generic type for storing
//! unsigned integers in packed BCD format. Each byte holds two decimal digits, one
//! per nibble. The `BYTES` const parameter controls the storage size and therefore
//! the maximum representable value.
//!
//! # BCD Format
//!
//! In packed BCD, the value `1234` is stored as `[0x12, 0x34]`. The high nibble of
//! each byte holds the more-significant digit. Leading zero bytes are permitted and
//! are transparent to formatting and arithmetic conversions.
//!
//! # Constructing a `Bcd`
//!
//! There are three ways to build a [`Bcd`]:
//!
//! - [`Bcd::from_bcd_bytes`] — from a raw byte array already in BCD format.
//! - [`Bcd::try_from_u8`] and its `try_from_u16` / `try_from_u32` / `try_from_u64`
//!   / `try_from_u128` / `try_from_usize` counterparts — from a primitive integer
//!   via the double-dabble algorithm, returning [`BcdOverflowError`] on overflow.
//! - The [`bcd!`] macro — a convenience wrapper that dispatches to the smallest
//!   primitive conversion capable of holding the value and panics on overflow.
//!
//! Conversion back to primitives uses the matching `try_into_*` methods. For the
//! `Bcd<BYTES>` sizes that are guaranteed to fit in a target primitive, infallible
//! `into_*` methods are also provided.
//!
//! # `const` support
//!
//! All construction and conversion methods are `const fn`, and the [`bcd!`] macro
//! works in `const` context:
//!
//! ```rust
//! use const_bcd::{Bcd, bcd};
//!
//! const N: Bcd<2> = bcd!(1234u16);
//! ```
//!
//! # `no_std`
//!
//! This crate is `no_std` compatible. The `std` feature is enabled by default but
//! has no functional effect — the crate uses only `core` internally. Disable it
//! for `no_std` targets:
//!
//! ```toml
//! [dependencies]
//! const-bcd = { version = "0.1", default-features = false }
//! ```
//!
//! The crate never allocates, regardless of feature flags.
//!
//! # Examples
//!
//! ```rust
//! use const_bcd::{Bcd, bcd};
//!
//! // Construct from a primitive integer
//! let n: Bcd<2> = Bcd::try_from_u16(1234).unwrap();
//! assert_eq!(n.to_string(), "1234");
//!
//! // Construct directly from raw BCD bytes
//! let n: Bcd<2> = Bcd::from_bcd_bytes([0x12, 0x34]).unwrap();
//! assert_eq!(n.to_string(), "1234");
//!
//! // Convenience macro
//! let n: Bcd<2> = bcd!(1234u16);
//! assert_eq!(n.to_string(), "1234");
//!
//! // Convert back to a primitive
//! let value: u16 = n.try_into_u16().unwrap();
//! assert_eq!(value, 1234);
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

use core::{error::Error, fmt, str};

const DIGIT_MAX: u8 = 9;

/// A packed Binary-Coded Decimal (BCD) value backed by `BYTES` bytes.
///
/// Each byte encodes two decimal digits: the high nibble holds the more-significant
/// digit and the low nibble holds the less-significant digit. For example, the decimal
/// value `42` is stored as `0x42`.
///
/// # Zero-byte `Bcd<0>`
///
/// `Bcd<0>` is a valid, zero-sized type that stores no digits. `try_into_*` always
/// yields `0`, and `try_from_*` accepts only `0` (any other value overflows). It
/// exists so the const-generic bound stays uniform; in most use cases `BYTES >= 1`
/// is expected.
///
/// # Examples
///
/// ```rust
/// use const_bcd::Bcd;
///
/// let a: Bcd<2> = Bcd::try_from_u16(100).unwrap();
/// let b: Bcd<2> = Bcd::try_from_u16(200).unwrap();
/// assert!(a < b);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bcd<const BYTES: usize>([u8; BYTES]);

/// Error returned when a byte contains a nibble with a value greater than 9.
///
/// BCD digits must be in the range `0..=9`. Any nibble value in `10..=15` (`0xA`–`0xF`)
/// is invalid and will produce this error.
///
/// # Examples
///
/// ```rust
/// use const_bcd::Bcd;
///
/// // 0xAB contains nibbles 0xA and 0xB, both invalid BCD digits
/// let err = Bcd::<1>::from_bcd_bytes([0xAB]).unwrap_err();
/// assert_eq!(err.to_string(), "Invalid BCD digit 10. Must be <= 9");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidBcdDigit(u8);

impl fmt::Display for InvalidBcdDigit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid BCD digit {}. Must be <= {}", self.0, DIGIT_MAX)
    }
}

impl Error for InvalidBcdDigit {}

/// Error returned when a conversion between an integer and a BCD overflows.
///
/// # Examples
///
/// ```rust
/// use const_bcd::{Bcd, bcd};
///
/// // A Bcd<1> can only hold two decimal digits (a value <= 99)
/// let err = Bcd::<1>::try_from_u8(240).unwrap_err();
/// assert_eq!(err.to_string(), "Overflow during BCD conversion");
///
/// let bcd: Bcd<2> = bcd!(1234u16);
/// let err = bcd.try_into_u8().unwrap_err();
/// assert_eq!(err.to_string(), "Overflow during BCD conversion");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BcdOverflowError;

impl fmt::Display for BcdOverflowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Overflow during BCD conversion")
    }
}

impl Error for BcdOverflowError {}

macro_rules! impl_bcd_try_from {
    ($(#[$attr:meta])* $name:ident, $ty:ty) => {
        $(#[$attr])*
        pub const fn $name(value: $ty) -> Result<Self, BcdOverflowError> {
            const BITS: u32 = <$ty>::BITS;

            let mut bcd = [0u8; BYTES];
            let mut overflow = false;

            let mut bit = BITS;
            while bit > 0 {
                bit -= 1;

                let mut byte = 0;
                while byte < BYTES {
                    let lo = bcd[byte] & 0x0f;
                    let hi = (bcd[byte] >> 4) & 0x0f;

                    if lo >= 5 {
                        bcd[byte] += 0x03;
                    }

                    if hi >= 5 {
                        bcd[byte] += 0x30;
                    }

                    byte += 1;
                }

                let mut carry = ((value >> bit) & 1) as u8;

                let mut i = BYTES;
                while i > 0 {
                    i -= 1;

                    let next_carry = (bcd[i] >> 7) & 1;
                    bcd[i] = (bcd[i] << 1) | carry;
                    carry = next_carry;
                }

                if carry != 0 {
                    overflow = true;
                }
            }

            if overflow {
                Err(BcdOverflowError)
            } else {
                Ok(Bcd(bcd))
            }
        }
    };
}

macro_rules! impl_bcd_try_into {
    ($(#[$attr:meta])* $name:ident, $ty:ty) => {
        $(#[$attr])*
        pub const fn $name(self) -> Result<$ty, BcdOverflowError> {
            let mut result: $ty = 0;

            let mut i = 0;
            while i < BYTES {
                let hi = (self.0[i] >> 4) & 0x0f;
                let lo = self.0[i] & 0x0f;

                result = match result.checked_mul(10) {
                    Some(v) => v,
                    None => return Err(BcdOverflowError),
                };

                result = match result.checked_add(hi as $ty) {
                    Some(v) => v,
                    None => return Err(BcdOverflowError),
                };

                result = match result.checked_mul(10) {
                    Some(v) => v,
                    None => return Err(BcdOverflowError),
                };

                result = match result.checked_add(lo as $ty) {
                    Some(v) => v,
                    None => return Err(BcdOverflowError),
                };

                i += 1;
            }

            Ok(result)
        }
    };
}

impl<const BYTES: usize> Bcd<BYTES> {
    /// Constructs a `Bcd` from a raw byte array already in packed BCD format.
    ///
    /// Each nibble in `bytes` must be a valid decimal digit (`0..=9`). If any nibble
    /// is in the range `10..=15`, an [`InvalidBcdDigit`] error is returned containing
    /// the offending nibble value.
    ///
    /// # Errors
    ///
    /// Returns [`Err(InvalidBcdDigit)`](InvalidBcdDigit) if any nibble exceeds `9`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use const_bcd::Bcd;
    ///
    /// // Valid BCD bytes
    /// let n = Bcd::<2>::from_bcd_bytes([0x12, 0x34]).unwrap();
    /// assert_eq!(n.to_string(), "1234");
    ///
    /// // Invalid: high nibble of first byte is 0xA
    /// assert!(Bcd::<2>::from_bcd_bytes([0xA0, 0x00]).is_err());
    ///
    /// // Invalid: low nibble of second byte is 0xF
    /// assert!(Bcd::<2>::from_bcd_bytes([0x00, 0x0F]).is_err());
    /// ```
    pub const fn from_bcd_bytes(bytes: [u8; BYTES]) -> Result<Self, InvalidBcdDigit> {
        let mut i = 0;

        while i < BYTES {
            let high = bytes[i] >> 4;
            let low = bytes[i] & 0x0F;
            if high > DIGIT_MAX {
                return Err(InvalidBcdDigit(high));
            }
            if low > DIGIT_MAX {
                return Err(InvalidBcdDigit(low));
            }
            i += 1;
        }

        Ok(Self(bytes))
    }

    impl_bcd_try_from!(
        /// Converts a [`u8`] to BCD using the double-dabble algorithm.
        ///
        /// Returns `Err(BcdOverflowError)` if `value` has more decimal digits than `BYTES * 2` can hold.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use const_bcd::Bcd;
        ///
        /// let n = Bcd::<1>::try_from_u8(99).unwrap();
        /// assert_eq!(n.to_string(), "99");
        ///
        /// // 100 requires 3 digits but Bcd<1> holds at most 2
        /// assert!(Bcd::<1>::try_from_u8(100).is_err());
        /// ```
        try_from_u8, u8
    );

    impl_bcd_try_from!(
        /// Converts a [`u16`] to BCD using the double-dabble algorithm.
        ///
        /// Returns `Err(BcdOverflowError)` if `value` has more decimal digits than `BYTES * 2` can hold.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use const_bcd::Bcd;
        ///
        /// let n = Bcd::<2>::try_from_u16(9999).unwrap();
        /// assert_eq!(n.to_string(), "9999");
        ///
        /// // Bcd<1> can hold at most 99
        /// assert!(Bcd::<1>::try_from_u16(100).is_err());
        /// ```
        try_from_u16, u16
    );

    impl_bcd_try_from!(
        /// Converts a [`u32`] to BCD using the double-dabble algorithm.
        ///
        /// Returns `Err(BcdOverflowError)` if `value` has more decimal digits than `BYTES * 2` can hold.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use const_bcd::Bcd;
        ///
        /// let n = Bcd::<4>::try_from_u32(12_345_678).unwrap();
        /// assert_eq!(n.to_string(), "12345678");
        /// ```
        try_from_u32, u32
    );

    impl_bcd_try_from!(
        /// Converts a [`u64`] to BCD using the double-dabble algorithm.
        ///
        /// Returns `Err(BcdOverflowError)` if `value` has more decimal digits than `BYTES * 2` can hold.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use const_bcd::Bcd;
        ///
        /// let n = Bcd::<8>::try_from_u64(1_000_000_000_000u64).unwrap();
        /// assert_eq!(n.to_string(), "1000000000000");
        /// ```
        try_from_u64, u64
    );

    impl_bcd_try_from!(
        /// Converts a [`u128`] to BCD using the double-dabble algorithm.
        ///
        /// Returns `Err(BcdOverflowError)` if `value` has more decimal digits than `BYTES * 2` can hold.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use const_bcd::Bcd;
        ///
        /// let n = Bcd::<20>::try_from_u128(u128::MAX).unwrap();
        /// assert_eq!(n.to_string(), u128::MAX.to_string());
        /// ```
        try_from_u128, u128
    );

    impl_bcd_try_from!(
        /// Converts a [`usize`] to BCD using the double-dabble algorithm.
        ///
        /// Returns `Err(BcdOverflowError)` if `value` has more decimal digits than `BYTES * 2` can hold.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use const_bcd::Bcd;
        ///
        /// let n = Bcd::<4>::try_from_usize(12345usize).unwrap();
        /// assert_eq!(n.to_string(), "12345");
        /// ```
        try_from_usize, usize
    );

    impl_bcd_try_into!(
        /// Converts this BCD value to a [`u8`].
        ///
        /// Returns `Err(BcdOverflowError)` if the decoded value exceeds [`u8::MAX`].
        ///
        /// # Examples
        ///
        /// ```rust
        /// use const_bcd::Bcd;
        ///
        /// let n = Bcd::<1>::try_from_u8(42).unwrap();
        /// assert_eq!(n.try_into_u8(), Ok(42u8));
        ///
        /// // A large Bcd<2> value won't fit in a u8
        /// let big = Bcd::<2>::try_from_u16(1000).unwrap();
        /// assert!(big.try_into_u8().is_err());
        /// ```
        try_into_u8, u8
    );

    impl_bcd_try_into!(
        /// Converts this BCD value to a [`u16`].
        ///
        /// Returns `Err(BcdOverflowError)` if the decoded value exceeds [`u16::MAX`].
        ///
        /// # Examples
        ///
        /// ```rust
        /// use const_bcd::Bcd;
        ///
        /// let n = Bcd::<2>::try_from_u16(1234).unwrap();
        /// assert_eq!(n.try_into_u16(), Ok(1234u16));
        /// ```
        try_into_u16, u16
    );

    impl_bcd_try_into!(
        /// Converts this BCD value to a [`u32`].
        ///
        /// Returns `Err(BcdOverflowError)` if the decoded value exceeds [`u32::MAX`].
        ///
        /// # Examples
        ///
        /// ```rust
        /// use const_bcd::Bcd;
        ///
        /// let n = Bcd::<4>::try_from_u32(100_000).unwrap();
        /// assert_eq!(n.try_into_u32(), Ok(100_000u32));
        /// ```
        try_into_u32, u32
    );

    impl_bcd_try_into!(
        /// Converts this BCD value to a [`u64`].
        ///
        /// Returns `Err(BcdOverflowError)` if the decoded value exceeds [`u64::MAX`].
        ///
        /// # Examples
        ///
        /// ```rust
        /// use const_bcd::Bcd;
        ///
        /// let n = Bcd::<10>::try_from_u64(u64::MAX).unwrap();
        /// assert_eq!(n.try_into_u64(), Ok(u64::MAX));
        /// ```
        try_into_u64, u64
    );

    impl_bcd_try_into!(
        /// Converts this BCD value to a [`u128`].
        ///
        /// Returns `Err(BcdOverflowError)` if the decoded value exceeds [`u128::MAX`].
        ///
        /// # Examples
        ///
        /// ```rust
        /// use const_bcd::Bcd;
        ///
        /// // A value larger than u64::MAX that still fits in Bcd<10> (max 10^20 - 1).
        /// let v = 20_000_000_000_000_000_000u128;
        /// let n = Bcd::<10>::try_from_u128(v).unwrap();
        /// assert_eq!(n.try_into_u128(), Ok(v));
        /// ```
        try_into_u128, u128
    );

    impl_bcd_try_into!(
        /// Converts this BCD value to a [`usize`].
        ///
        /// Returns `Err(BcdOverflowError)` if the decoded value exceeds [`usize::MAX`].
        ///
        /// # Examples
        ///
        /// ```rust
        /// use const_bcd::Bcd;
        ///
        /// let n = Bcd::<4>::try_from_usize(99999usize).unwrap();
        /// assert_eq!(n.try_into_usize(), Ok(99999usize));
        /// ```
        try_into_usize, usize
    );
}

/// Formats the BCD value as a decimal string.
///
/// Behaves like a native unsigned integer: width, alignment, sign (`+`), and
/// numeric zero-padding are honored via [`core::fmt::Formatter::pad_integral`].
///
/// # Examples
///
/// ```rust
/// use const_bcd::Bcd;
///
/// let n = Bcd::<4>::try_from_u32(42).unwrap();
/// assert_eq!(format!("{n}"),    "42");
/// assert_eq!(format!("{n:8}"),  "      42");
/// assert_eq!(format!("{n:08}"), "00000042");
/// assert_eq!(format!("{n:<8}"), "42      ");
/// assert_eq!(format!("{n:+}"),  "+42");
///
/// // Zero displays as "0", not ""
/// let zero = Bcd::<2>::try_from_u16(0).unwrap();
/// assert_eq!(format!("{zero}"), "0");
/// ```
impl<const BYTES: usize> fmt::Display for Bcd<BYTES> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = [[0u8; 2]; BYTES];
        let mut len = 0;
        for nibble in self
            .0
            .iter()
            .flat_map(|b| [b >> 4, b & 0x0F])
            .skip_while(|n| *n == 0)
        {
            // Division and modulo will get compiled away to len >> 1 and len & 1, so don't worry about performance.
            buf[len / 2][len % 2] = b'0' + nibble;
            len += 1;
        }
        let s = if len == 0 {
            // No non-zero nibbles (or `BYTES == 0`): emit a single "0".
            "0"
        } else {
            str::from_utf8(&buf.as_flattened()[..len]).unwrap()
        };
        f.pad_integral(true, "", s)
    }
}

macro_rules! impl_bcd_into {
    ($prim:ty, $fn_name:ident, $try_into_fn:ident, $($bytes:literal),+) => {
        $(
            impl Bcd<$bytes> {
                /// Infallibly converts this BCD value to a primitive integer.
                ///
                /// Available only for `BYTES` values whose maximum representable BCD
                /// value is statically guaranteed to fit in the target primitive, so
                /// this method cannot overflow.
                pub const fn $fn_name(self) -> $prim {
                    match self.$try_into_fn() {
                        Ok(val) => val,
                        // Unreachable: `BYTES` is bounded so that overflow is impossible.
                        // A panic branch is required to keep this function total for `const fn`.
                        Err(_) => panic!("BUG: infallible BCD conversion overflowed")
                    }
                }
            }
        )+
    };
}

// An infallible `into_*` exists for every BYTES value whose maximum representable
// BCD value (10.pow(2 * BYTES) - 1) is guaranteed to fit in the target primitive.
// The largest safe BYTES is `ilog10(PRIM::MAX) / 2`:
//
//   u8:   ilog10(u8::MAX)   =  2  ->  BYTES <= 1
//   u16:  ilog10(u16::MAX)  =  4  ->  BYTES <= 2
//   u32:  ilog10(u32::MAX)  =  9  ->  BYTES <= 4
//   u64:  ilog10(u64::MAX)  = 19  ->  BYTES <= 9
//   u128: ilog10(u128::MAX) = 38  ->  BYTES <= 19
//
// `usize` is treated as if it were `u16` so the impls remain sound on 16-bit targets.
impl_bcd_into!(u8, into_u8, try_into_u8, 0, 1);
impl_bcd_into!(u16, into_u16, try_into_u16, 0, 1, 2);
impl_bcd_into!(u32, into_u32, try_into_u32, 0, 1, 2, 3, 4);
impl_bcd_into!(u64, into_u64, try_into_u64, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
impl_bcd_into!(
    u128,
    into_u128,
    try_into_u128,
    0,
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
    17,
    18,
    19
);
impl_bcd_into!(usize, into_usize, try_into_usize, 0, 1, 2);

/// Constructs a [`Bcd`] from an unsigned integer expression, panicking on overflow.
///
/// The value is cast to `u128` and then dispatched at runtime to the smallest
/// `try_from_*` method capable of holding it. This keeps the double-dabble loop
/// short for small values without requiring the caller to pick the right method.
///
/// This macro is `const`-usable, so it can construct `Bcd` values in `const`
/// items and other `const` contexts.
///
/// # Signed values
///
/// This macro accepts any expression that can be cast to `u128`. Passing a
/// negative value is a logic error: the value is reinterpreted as an unsigned
/// integer via the `as` cast, which will almost always overflow the target
/// `Bcd<BYTES>` and panic. Use one of the `try_from_*` methods directly if you
/// need explicit control over the source type.
///
/// # Panics
///
/// Panics if the value overflows the `Bcd<BYTES>` that type inference selects.
///
/// # Examples
///
/// ```rust
/// use const_bcd::{Bcd, bcd};
///
/// let n: Bcd<2> = bcd!(1234u16);
/// assert_eq!(n.to_string(), "1234");
///
/// // Works with expressions, not just literals
/// let x = 56u8;
/// let n: Bcd<1> = bcd!(x);
/// assert_eq!(n.to_string(), "56");
///
/// // Works in const context
/// const M: Bcd<4> = bcd!(12345678u32);
/// assert_eq!(M.to_string(), "12345678");
///
/// // Works with u128-range values
/// let n: Bcd<20> = bcd!(u128::MAX);
/// assert_eq!(n.to_string(), u128::MAX.to_string());
/// ```
#[macro_export]
macro_rules! bcd {
    ($v:expr) => {{
        let v: u128 = $v as u128;
        match if v <= u8::MAX as u128 {
            $crate::Bcd::try_from_u8(v as u8)
        } else if v <= u16::MAX as u128 {
            $crate::Bcd::try_from_u16(v as u16)
        } else if v <= u32::MAX as u128 {
            $crate::Bcd::try_from_u32(v as u32)
        } else if v <= u64::MAX as u128 {
            $crate::Bcd::try_from_u64(v as u64)
        } else {
            $crate::Bcd::try_from_u128(v)
        } {
            Ok(bcd) => bcd,
            Err(_) => panic!("Overflow during BCD conversion"),
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_format::assert_display_fmt;

    // --- Construction: from_bcd_bytes ---

    #[test]
    fn from_bcd_bytes_valid() {
        let n = Bcd::<2>::from_bcd_bytes([0x12, 0x34]).unwrap();
        assert_display_fmt!(n, "1234");
    }

    #[test]
    fn from_bcd_bytes_all_zeros() {
        let n = Bcd::<4>::from_bcd_bytes([0; 4]).unwrap();
        assert_eq!(n.try_into_u32(), Ok(0));
    }

    #[test]
    fn from_bcd_bytes_rejects_invalid_high_nibble() {
        let err = Bcd::<1>::from_bcd_bytes([0xA0]).unwrap_err();
        assert_eq!(err, InvalidBcdDigit(0xA));
    }

    #[test]
    fn from_bcd_bytes_rejects_invalid_low_nibble() {
        let err = Bcd::<1>::from_bcd_bytes([0x0B]).unwrap_err();
        assert_eq!(err, InvalidBcdDigit(0xB));
    }

    #[test]
    fn from_bcd_bytes_reports_first_invalid_nibble() {
        // The high nibble of the second byte (0xC) is the first bad nibble.
        let err = Bcd::<2>::from_bcd_bytes([0x12, 0xC3]).unwrap_err();
        assert_eq!(err, InvalidBcdDigit(0xC));
    }

    // --- Construction: try_from_* roundtrips and overflow ---

    #[test]
    fn try_from_u8_exhaustive() {
        for v in 0..=99u8 {
            let bcd = Bcd::<1>::try_from_u8(v).unwrap();
            assert_eq!(bcd.try_into_u8(), Ok(v));
        }
        for v in 100..=255u8 {
            assert!(Bcd::<1>::try_from_u8(v).is_err());
        }
    }

    #[test]
    fn try_from_u8_wider_bcd_never_overflows() {
        for v in 0..=255u8 {
            let bcd = Bcd::<2>::try_from_u8(v).unwrap();
            assert_eq!(bcd.try_into_u16(), Ok(v as u16));
        }
    }

    #[test]
    fn try_from_u16_roundtrip() {
        for v in [0u16, 1, 999, 9999, u16::MAX] {
            let bcd = Bcd::<3>::try_from_u16(v).unwrap();
            assert_eq!(bcd.try_into_u16(), Ok(v));
        }
    }

    #[test]
    fn try_from_u16_overflow() {
        // Bcd<2> holds at most 9999.
        assert!(Bcd::<2>::try_from_u16(10_000).is_err());
        assert!(Bcd::<2>::try_from_u16(u16::MAX).is_err());
    }

    #[test]
    fn try_from_u32_roundtrip() {
        for v in [0u32, 1, 9_999, 99_999_999, u32::MAX] {
            let bcd = Bcd::<5>::try_from_u32(v).unwrap();
            assert_eq!(bcd.try_into_u32(), Ok(v));
        }
    }

    #[test]
    fn try_from_u32_overflow() {
        // Bcd<4> holds at most 99_999_999.
        assert!(Bcd::<4>::try_from_u32(100_000_000).is_err());
        assert!(Bcd::<4>::try_from_u32(u32::MAX).is_err());
    }

    #[test]
    fn try_from_u64_roundtrip() {
        for v in [0u64, 1, u16::MAX as u64, u32::MAX as u64, u64::MAX] {
            let bcd = Bcd::<10>::try_from_u64(v).unwrap();
            assert_eq!(bcd.try_into_u64(), Ok(v));
        }
    }

    #[test]
    fn try_from_u64_overflow() {
        // Bcd<9> holds at most 10^18 - 1, which is less than u64::MAX.
        assert!(Bcd::<9>::try_from_u64(u64::MAX).is_err());
    }

    #[test]
    fn try_from_u128_roundtrip() {
        for v in [0u128, 1, u32::MAX as u128, u64::MAX as u128, u128::MAX] {
            let bcd = Bcd::<20>::try_from_u128(v).unwrap();
            assert_eq!(bcd.try_into_u128(), Ok(v));
        }
    }

    #[test]
    fn try_from_u128_overflow() {
        // Bcd<19> holds at most 10^38 - 1, which is less than u128::MAX.
        assert!(Bcd::<19>::try_from_u128(u128::MAX).is_err());
    }

    #[test]
    fn try_from_usize_roundtrip() {
        for v in [0usize, 1, 99, 99_999] {
            let bcd = Bcd::<3>::try_from_usize(v).unwrap();
            assert_eq!(bcd.try_into_usize(), Ok(v));
        }
    }

    #[test]
    fn try_from_usize_overflow() {
        assert!(Bcd::<2>::try_from_usize(10_000).is_err());
    }

    // --- try_into_* overflow ---

    #[test]
    fn try_into_u8_overflow() {
        let bcd = Bcd::<2>::try_from_u16(1000).unwrap();
        assert!(bcd.try_into_u8().is_err());
    }

    #[test]
    fn try_into_u16_overflow() {
        let bcd = Bcd::<3>::try_from_u32(100_000).unwrap();
        assert!(bcd.try_into_u16().is_err());
    }

    #[test]
    fn try_into_u32_overflow() {
        // Bcd<5> filled with 9s = 9_999_999_999 > u32::MAX.
        let bcd = Bcd::<5>::from_bcd_bytes([0x99; 5]).unwrap();
        assert!(bcd.try_into_u32().is_err());
    }

    #[test]
    fn try_into_u64_overflow() {
        // Bcd<10> filled with 9s = 10^20 - 1 > u64::MAX.
        let bcd = Bcd::<10>::from_bcd_bytes([0x99; 10]).unwrap();
        assert!(bcd.try_into_u64().is_err());
    }

    #[test]
    fn try_into_u128_overflow() {
        // Bcd<20> filled with 9s represents 10^40 - 1, which overflows u128.
        let bcd = Bcd::<20>::from_bcd_bytes([0x99; 20]).unwrap();
        assert!(bcd.try_into_u128().is_err());
    }

    #[test]
    fn try_into_usize_overflow_on_any_target() {
        // Overflows usize on every currently supported target (max 64-bit).
        let bcd = Bcd::<20>::from_bcd_bytes([0x99; 20]).unwrap();
        assert!(bcd.try_into_usize().is_err());
    }

    // --- Infallible into_* ---

    #[test]
    fn infallible_into_u8() {
        let bcd = Bcd::<1>::try_from_u8(42).unwrap();
        assert_eq!(bcd.into_u8(), 42u8);
    }

    #[test]
    fn infallible_into_u16() {
        let bcd = Bcd::<2>::try_from_u16(9999).unwrap();
        assert_eq!(bcd.into_u16(), 9999u16);
    }

    #[test]
    fn infallible_into_u32() {
        let bcd = Bcd::<4>::try_from_u32(12_345_678).unwrap();
        assert_eq!(bcd.into_u32(), 12_345_678u32);
    }

    #[test]
    fn infallible_into_u64() {
        let v = 999_999_999_999_999_999u64;
        let bcd = Bcd::<9>::try_from_u64(v).unwrap();
        assert_eq!(bcd.into_u64(), v);
    }

    #[test]
    fn infallible_into_u128() {
        let v = 10u128.pow(38) - 1;
        let bcd = Bcd::<19>::try_from_u128(v).unwrap();
        assert_eq!(bcd.into_u128(), v);
    }

    #[test]
    fn infallible_into_usize() {
        let bcd = Bcd::<2>::try_from_usize(9999).unwrap();
        assert_eq!(bcd.into_usize(), 9999usize);
    }

    // --- Display ---

    #[test]
    fn display_zero() {
        let n = Bcd::<2>::try_from_u16(0).unwrap();
        assert_display_fmt!(n, "0");
    }

    #[test]
    fn display_no_leading_zeros() {
        let n = Bcd::<4>::try_from_u32(42).unwrap();
        assert_display_fmt!(n, "42");
    }

    #[test]
    fn display_single_digit() {
        let n = Bcd::<4>::try_from_u32(7).unwrap();
        assert_display_fmt!(n, "7");
    }

    #[test]
    fn display_padding_right_aligned() {
        let n = Bcd::<2>::try_from_u16(42).unwrap();
        assert_display_fmt!(format_args!("{:0>6}", n), "000042");
    }

    #[test]
    fn display_padding_left_aligned() {
        let n = Bcd::<2>::try_from_u16(42).unwrap();
        assert_display_fmt!(format_args!("{:<6}", n), "42    ");
    }

    #[test]
    fn display_default_right_aligned_like_integer() {
        // Native uints default to right-align with space fill.
        let n = Bcd::<2>::try_from_u16(42).unwrap();
        assert_display_fmt!(format_args!("{:8}", n), "      42");
    }

    #[test]
    fn display_numeric_zero_padding() {
        // `{:08}` should zero-pad like an integer, not space-pad like a string.
        let n = Bcd::<2>::try_from_u16(42).unwrap();
        assert_display_fmt!(format_args!("{:08}", n), "00000042");
    }

    #[test]
    fn display_positive_sign_flag() {
        // `{:+}` should emit `+` since BCD values are always non-negative.
        let n = Bcd::<2>::try_from_u16(42).unwrap();
        assert_display_fmt!(format_args!("{:+}", n), "+42");
    }

    #[test]
    fn display_positive_sign_with_zero_pad() {
        // Sign counts against width when combined with zero-padding.
        let n = Bcd::<2>::try_from_u16(42).unwrap();
        assert_display_fmt!(format_args!("{:+08}", n), "+0000042");
    }

    #[test]
    fn display_zero_pad_zero_value() {
        let n = Bcd::<2>::try_from_u16(0).unwrap();
        assert_display_fmt!(format_args!("{:04}", n), "0000");
    }

    #[test]
    fn display_u128_max() {
        let bcd = Bcd::<20>::try_from_u128(u128::MAX).unwrap();
        assert_eq!(bcd.to_string(), u128::MAX.to_string());
    }

    #[test]
    fn display_internal_zeros() {
        // Guards the `skip_while(|n| *n == 0)` logic: only leading zeros should
        // be trimmed, not zeros between non-zero digits.
        let n = Bcd::<2>::try_from_u16(1002).unwrap();
        assert_display_fmt!(n, "1002");

        let n = Bcd::<4>::try_from_u32(1_000_000).unwrap();
        assert_display_fmt!(n, "1000000");
    }

    // --- Debug ---

    #[test]
    fn debug_shows_internal_bytes() {
        // Derived Debug: prints the byte array in decimal.
        let bcd = Bcd::<2>::from_bcd_bytes([0x12, 0x34]).unwrap();
        assert_eq!(format!("{bcd:?}"), "Bcd([18, 52])");
    }

    // --- Ordering / Eq / Copy / Hash ---

    #[test]
    fn ordering() {
        let a = Bcd::<2>::try_from_u16(100).unwrap();
        let b = Bcd::<2>::try_from_u16(200).unwrap();
        let c = Bcd::<2>::try_from_u16(200).unwrap();
        assert!(a < b);
        assert!(b > a);
        assert_eq!(b, c);
    }

    #[test]
    fn copy_and_clone() {
        let a = Bcd::<2>::try_from_u16(42).unwrap();
        let b = a;
        #[allow(clippy::clone_on_copy)]
        let c = a.clone();
        assert_eq!(a, b);
        assert_eq!(a, c);
    }

    #[test]
    fn hash_consistent_with_eq() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(Bcd::<2>::try_from_u16(1234).unwrap(), "hi");
        assert_eq!(map.get(&Bcd::<2>::try_from_u16(1234).unwrap()), Some(&"hi"),);
    }

    // --- Error types ---

    #[test]
    fn invalid_bcd_digit_display() {
        let e = InvalidBcdDigit(10);
        assert_display_fmt!(e, "Invalid BCD digit 10. Must be <= 9");
    }

    #[test]
    fn bcd_overflow_error_display() {
        let e = BcdOverflowError;
        assert_display_fmt!(e, "Overflow during BCD conversion");
    }

    // --- Bcd<0> edge case ---

    #[test]
    fn zero_byte_bcd_accepts_zero_and_overflows_on_one() {
        assert!(Bcd::<0>::try_from_u8(0).is_ok());
        assert!(Bcd::<0>::try_from_u8(1).is_err());

        let z = Bcd::<0>::try_from_u8(0).unwrap();
        assert_eq!(z.try_into_u8(), Ok(0));
        assert_eq!(z.into_u8(), 0);
        assert_display_fmt!(z, "0");
    }

    // --- bcd! macro ---

    #[test]
    fn bcd_macro_u8_range() {
        let n: Bcd<1> = bcd!(99u8);
        assert_display_fmt!(n, "99");
    }

    #[test]
    fn bcd_macro_u16_range() {
        let n: Bcd<2> = bcd!(1234u16);
        assert_display_fmt!(n, "1234");
    }

    #[test]
    fn bcd_macro_u32_range() {
        let n: Bcd<5> = bcd!(100_000u32);
        assert_display_fmt!(n, "100000");
    }

    #[test]
    fn bcd_macro_u64_range() {
        let n: Bcd<10> = bcd!(u64::MAX);
        assert_eq!(n.to_string(), u64::MAX.to_string());
    }

    #[test]
    fn bcd_macro_u128_range() {
        let n: Bcd<20> = bcd!(u128::MAX);
        assert_eq!(n.to_string(), u128::MAX.to_string());
    }

    #[test]
    fn bcd_macro_expression() {
        let x = 56u8;
        let n: Bcd<1> = bcd!(x);
        assert_display_fmt!(n, "56");
    }

    #[test]
    fn bcd_macro_zero() {
        let n: Bcd<1> = bcd!(0u8);
        assert_display_fmt!(n, "0");
    }

    #[test]
    #[should_panic(expected = "Overflow during BCD conversion")]
    fn bcd_macro_panics_on_overflow() {
        // 100 won't fit in Bcd<1> which holds at most 2 digits.
        let _: Bcd<1> = bcd!(100u8);
    }

    // --- const context ---

    #[test]
    fn const_construction_and_conversion() {
        const RAW: Bcd<2> = match Bcd::<2>::from_bcd_bytes([0x12, 0x34]) {
            Ok(v) => v,
            Err(_) => panic!(),
        };
        const FROM_INT: Bcd<2> = match Bcd::<2>::try_from_u16(1234) {
            Ok(v) => v,
            Err(_) => panic!(),
        };
        const FROM_MACRO: Bcd<2> = bcd!(1234u16);
        const AS_INT: u16 = match FROM_MACRO.try_into_u16() {
            Ok(v) => v,
            Err(_) => 0,
        };
        const AS_INFALLIBLE: u16 = FROM_MACRO.into_u16();

        assert_eq!(RAW, FROM_INT);
        assert_eq!(RAW, FROM_MACRO);
        assert_eq!(AS_INT, 1234);
        assert_eq!(AS_INFALLIBLE, 1234);
    }
}
