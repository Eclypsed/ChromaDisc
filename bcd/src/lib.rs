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
//! - [`Bcd::try_from_u8`] / … — from a primitive integer via the double-dabble
//!   algorithm, returning `None` on overflow.
//! - The [`bcd!`] macro — a convenience wrapper that selects the appropriate
//!   primitive conversion based on the value's magnitude and panics on overflow.
//!
//! # Feature Flags
//!
//! | Feature | Description |
//! |---------|-------------|
//! | `deku`  | Implements [`deku::DekuReader`] for [`Bcd`], enabling zero-copy parsing from binary formats. |
//!
//! # Examples
//!
//! ```rust
//! use bcd::{Bcd, bcd};
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
//! let n: Bcd<2> = bcd!(1234);
//! assert_eq!(n.to_string(), "1234");
//!
//! // Convert back to a primitive
//! let value: u16 = n.try_into_u16().unwrap();
//! assert_eq!(value, 1234);
//! ```

use std::{error::Error, fmt::Display};

const DIGIT_MAX: u8 = 9;

/// A packed Binary-Coded Decimal (BCD) value backed by `BYTES` bytes.
///
/// Each byte encodes two decimal digits: the high nibble holds the more-significant
/// digit and the low nibble holds the less-significant digit. For example, the decimal
/// value `42` is stored as `0x42`.
///
/// # Zero
///
/// `Bcd<0>` is a valid type representing a zero-byte BCD with no digits. Its numeric
/// conversions consistently return `0`. It is documented here for completeness; in
/// most use cases `BYTES >= 1` is expected.
///
/// # Examples
///
/// ```rust
/// use bcd::Bcd;
///
/// let a: Bcd<2> = Bcd::try_from_u16(100).unwrap();
/// let b: Bcd<2> = Bcd::try_from_u16(200).unwrap();
/// assert!(a < b);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "deku", derive(deku::DekuRead))]
pub struct Bcd<const BYTES: usize>(
    #[cfg_attr(
        feature = "deku",
        deku(reader = "deku_features::read_bcd_bytes(deku::reader)")
    )]
    [u8; BYTES],
);

/// Error returned when a byte contains a nibble with a value greater than 9.
///
/// BCD digits must be in the range `0..=9`. Any nibble value in `10..=15` (`0xA`–`0xF`)
/// is invalid and will produce this error.
///
/// # Examples
///
/// ```rust
/// use bcd::Bcd;
///
/// // 0xAB contains nibbles 0xA and 0xB, both invalid BCD digits
/// let err = Bcd::<1>::from_bcd_bytes([0xAB]).unwrap_err();
/// assert_eq!(err.to_string(), "Invalid BCD digit 10. Must be <= 9");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidBcdDigit(u8);

impl Display for InvalidBcdDigit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid BCD digit {}. Must be <= {}", self.0, DIGIT_MAX)
    }
}

impl Error for InvalidBcdDigit {}

macro_rules! impl_bcd_try_from {
    ($(#[$attr:meta])* $name:ident, $ty:ty) => {
        $(#[$attr])*
        pub const fn $name(value: $ty) -> Option<Self> {
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
                None
            } else {
                Some(Bcd(bcd))
            }
        }
    };
}

macro_rules! impl_bcd_try_into {
    ($(#[$attr:meta])* $name:ident, $ty:ty) => {
        $(#[$attr])*
        pub const fn $name(self) -> Option<$ty> {
            let mut result: $ty = 0;

            let mut i = 0;
            while i < BYTES {
                let hi = (self.0[i] >> 4) & 0x0f;
                let lo = self.0[i] & 0x0f;

                result = match result.checked_mul(10) {
                    Some(v) => v,
                    None => return None,
                };

                result = match result.checked_add(hi as $ty) {
                    Some(v) => v,
                    None => return None,
                };

                result = match result.checked_mul(10) {
                    Some(v) => v,
                    None => return None,
                };

                result = match result.checked_add(lo as $ty) {
                    Some(v) => v,
                    None => return None,
                };

                i += 1;
            }

            Some(result)
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
    /// use bcd::Bcd;
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
            let low = bytes[i] & 0x0f;
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
        /// Returns `None` if `value` has more decimal digits than `BYTES * 2` can hold.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use bcd::Bcd;
        ///
        /// let n = Bcd::<1>::try_from_u8(99).unwrap();
        /// assert_eq!(n.to_string(), "99");
        ///
        /// // 100 requires 3 digits but Bcd<1> holds at most 2
        /// assert!(Bcd::<1>::try_from_u8(100).is_none());
        /// ```
        try_from_u8, u8
    );

    impl_bcd_try_from!(
        /// Converts a [`u16`] to BCD using the double-dabble algorithm.
        ///
        /// Returns `None` if `value` has more decimal digits than `BYTES * 2` can hold.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use bcd::Bcd;
        ///
        /// let n = Bcd::<2>::try_from_u16(9999).unwrap();
        /// assert_eq!(n.to_string(), "9999");
        ///
        /// // Bcd<1> can hold at most 99
        /// assert!(Bcd::<1>::try_from_u16(100).is_none());
        /// ```
        try_from_u16, u16
    );

    impl_bcd_try_from!(
        /// Converts a [`u32`] to BCD using the double-dabble algorithm.
        ///
        /// Returns `None` if `value` has more decimal digits than `BYTES * 2` can hold.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use bcd::Bcd;
        ///
        /// let n = Bcd::<4>::try_from_u32(12_345_678).unwrap();
        /// assert_eq!(n.to_string(), "12345678");
        /// ```
        try_from_u32, u32
    );

    impl_bcd_try_from!(
        /// Converts a [`u64`] to BCD using the double-dabble algorithm.
        ///
        /// Returns `None` if `value` has more decimal digits than `BYTES * 2` can hold.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use bcd::Bcd;
        ///
        /// let n = Bcd::<8>::try_from_u64(1_000_000_000_000u64).unwrap();
        /// assert_eq!(n.to_string(), "1000000000000");
        /// ```
        try_from_u64, u64
    );

    impl_bcd_try_from!(
        /// Converts a [`u128`] to BCD using the double-dabble algorithm.
        ///
        /// Returns `None` if `value` has more decimal digits than `BYTES * 2` can hold.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use bcd::Bcd;
        ///
        /// let n = Bcd::<20>::try_from_u128(u128::MAX).unwrap();
        /// assert_eq!(n.to_string(), u128::MAX.to_string());
        /// ```
        try_from_u128, u128
    );

    impl_bcd_try_from!(
        /// Converts a [`usize`] to BCD using the double-dabble algorithm.
        ///
        /// Returns `None` if `value` has more decimal digits than `BYTES * 2` can hold.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use bcd::Bcd;
        ///
        /// let n = Bcd::<4>::try_from_usize(12345usize).unwrap();
        /// assert_eq!(n.to_string(), "12345");
        /// ```
        try_from_usize, usize
    );

    impl_bcd_try_into!(
        /// Converts this BCD value to a [`u8`].
        ///
        /// Returns `None` if the decoded value exceeds [`u8::MAX`].
        ///
        /// # Examples
        ///
        /// ```rust
        /// use bcd::Bcd;
        ///
        /// let n = Bcd::<1>::try_from_u8(42).unwrap();
        /// assert_eq!(n.try_into_u8(), Some(42u8));
        ///
        /// // A large Bcd<2> value won't fit in a u8
        /// let big = Bcd::<2>::try_from_u16(1000).unwrap();
        /// assert!(big.try_into_u8().is_none());
        /// ```
        try_into_u8, u8
    );

    impl_bcd_try_into!(
        /// Converts this BCD value to a [`u16`].
        ///
        /// Returns `None` if the decoded value exceeds [`u16::MAX`].
        ///
        /// # Examples
        ///
        /// ```rust
        /// use bcd::Bcd;
        ///
        /// let n = Bcd::<2>::try_from_u16(1234).unwrap();
        /// assert_eq!(n.try_into_u16(), Some(1234u16));
        /// ```
        try_into_u16, u16
    );

    impl_bcd_try_into!(
        /// Converts this BCD value to a [`u32`].
        ///
        /// Returns `None` if the decoded value exceeds [`u32::MAX`].
        ///
        /// # Examples
        ///
        /// ```rust
        /// use bcd::Bcd;
        ///
        /// let n = Bcd::<4>::try_from_u32(100_000).unwrap();
        /// assert_eq!(n.try_into_u32(), Some(100_000u32));
        /// ```
        try_into_u32, u32
    );

    impl_bcd_try_into!(
        /// Converts this BCD value to a [`u64`].
        ///
        /// Returns `None` if the decoded value exceeds [`u64::MAX`].
        ///
        /// # Examples
        ///
        /// ```rust
        /// use bcd::Bcd;
        ///
        /// let n = Bcd::<10>::try_from_u64(u64::MAX).unwrap();
        /// assert_eq!(n.try_into_u64(), Some(u64::MAX));
        /// ```
        try_into_u64, u64
    );

    impl_bcd_try_into!(
        /// Converts this BCD value to a [`u128`].
        ///
        /// Returns `None` if the decoded value exceeds [`u128::MAX`].
        ///
        /// # Examples
        ///
        /// ```rust
        /// use bcd::Bcd;
        ///
        /// let n = Bcd::<10>::try_from_u128(12_34_56_78_90_00u128).unwrap();
        /// assert_eq!(n.try_into_u128(), Some(12_34_56_78_90_00u128));
        /// ```
        try_into_u128, u128
    );

    impl_bcd_try_into!(
        /// Converts this BCD value to a [`usize`].
        ///
        /// Returns `None` if the decoded value exceeds [`usize::MAX`].
        ///
        /// # Examples
        ///
        /// ```rust
        /// use bcd::Bcd;
        ///
        /// let n = Bcd::<4>::try_from_usize(99999usize).unwrap();
        /// assert_eq!(n.try_into_usize(), Some(99999usize));
        /// ```
        try_into_usize, usize
    );
}

/// Formats the BCD value as a decimal string without leading zeros.
///
/// If the value is zero, `"0"` is produced. Standard [`std::fmt`] width and
/// alignment specifiers are respected via [`std::fmt::Formatter::pad`], so
/// zero-padding and field widths work as expected.
///
/// # Examples
///
/// ```rust
/// use bcd::Bcd;
///
/// let n = Bcd::<4>::try_from_u32(42).unwrap();
/// assert_eq!(format!("{n}"),     "42");
/// assert_eq!(format!("{n:0>8}"), "00000042");
/// assert_eq!(format!("{n:<8}"),  "42      ");
///
/// // Zero displays as "0", not ""
/// let zero = Bcd::<2>::try_from_u16(0).unwrap();
/// assert_eq!(format!("{zero}"), "0");
/// ```
impl<const BYTES: usize> Display for Bcd<BYTES> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::with_capacity(BYTES * 2);
        for nibble in self
            .0
            .iter()
            .flat_map(|b| [b >> 4, b & 0x0F])
            .skip_while(|n| *n == 0)
        {
            buf.push(char::from(b'0' + nibble));
        }
        if buf.is_empty() {
            buf.push('0');
        }
        f.pad(&buf)
    }
}

macro_rules! impl_bcd_from {
    ($prim:ty, $fn_name:ident, $try_into_fn:ident, $($bytes:literal),+) => {
        $(
            impl Bcd<$bytes> {
                /// Safely converts this BCD value to a primitive integer.
                pub const fn $fn_name(self) -> $prim {
                    self.$try_into_fn().expect("Overflowed during BCD conversion")
                }
            }
        )+
    };
}

// Ranges derived from ilog10 of each type's MAX value.
// Each byte holds 2 digits, so BYTES =< digits / 2.
//
// u8:   ilog10(u8::MAX)   = 2 digits  -> 1 byte
// u16:  ilog10(u16::MAX)  = 4 digits  -> 2 bytes
// u32:  ilog10(u32::MAX)  = 9 digits  -> 4 bytes
// u64:  ilog10(u64::MAX)  = 19 digits -> 9 bytes
// u128: ilog10(u128::MAX) = 38 digits -> 19 bytes
impl_bcd_from!(u8, into_u8, try_into_u8, 0, 1);
impl_bcd_from!(u16, into_u16, try_into_u16, 0, 1, 2);
impl_bcd_from!(u32, into_u32, try_into_u32, 0, 1, 2, 3, 4);
impl_bcd_from!(u64, into_u64, try_into_u64, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
impl_bcd_from!(
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
#[cfg(target_pointer_width = "16")]
impl_bcd_from!(usize, into_usize, try_into_usize, 0, 1, 2);
#[cfg(target_pointer_width = "32")]
impl_bcd_from!(usize, into_usize, try_into_usize, 0, 1, 2, 3, 4);
#[cfg(target_pointer_width = "64")]
impl_bcd_from!(
    usize,
    into_usize,
    try_into_usize,
    0,
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9
);

/// Constructs a [`Bcd`] from an integer literal or expression, panicking on overflow.
///
/// The macro inspects the magnitude of the value at runtime and delegates to the
/// appropriate `try_from_*` method. If the value does not fit within the `BYTES`
/// of the inferred [`Bcd`] type, the macro panics.
///
/// This is primarily useful for constructing `Bcd` values from constants where
/// you know the value is in range and want to avoid the ergonomic overhead of
/// unwrapping an `Option`.
///
/// # Panics
///
/// Panics if the value overflows the `Bcd<BYTES>` that type inference selects.
///
/// # Examples
///
/// ```rust
/// use bcd::{Bcd, bcd};
///
/// let n: Bcd<2> = bcd!(1234u16);
/// assert_eq!(n.to_string(), "1234");
///
/// // Works with expressions, not just literals
/// let x = 56u8;
/// let n: Bcd<1> = bcd!(x);
/// assert_eq!(n.to_string(), "56");
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
            Some(bcd) => bcd,
            None => panic!("Overflow during BCD conversion"),
        }
    }};
}

#[cfg(feature = "deku")]
mod deku_features {
    use super::Bcd;
    use deku::{reader::Reader, DekuError, DekuReader};

    /// [`deku::DekuReader`] implementation for [`Bcd`].
    ///
    /// Reads exactly `BYTES` bytes from the reader and validates each nibble as a
    /// legal BCD digit. Returns a [`deku::DekuError::Parse`] if any nibble exceeds `9`.
    ///
    /// Requires the `deku` feature flag.
    pub fn read_bcd_bytes<const BYTES: usize, R: std::io::Read + std::io::Seek>(
        reader: &mut Reader<R>,
    ) -> Result<[u8; BYTES], DekuError> {
        Bcd::<BYTES>::from_bcd_bytes(<[u8; BYTES]>::from_reader_with_ctx(reader, ())?)
            .map(|b| b.0)
            .map_err(|e| DekuError::Parse(e.to_string().into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Construction ---

    #[test]
    fn from_bcd_bytes_valid() {
        let n = Bcd::<2>::from_bcd_bytes([0x12, 0x34]).unwrap();
        assert_eq!(n.to_string(), "1234");
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
    fn try_from_u8_roundtrip() {
        for v in [0u8, 1, 9, 10, 99] {
            let bcd = Bcd::<1>::try_from_u8(v).unwrap();
            assert_eq!(bcd.try_into_u8(), Some(v));
        }
    }

    #[test]
    fn try_from_u8_overflow() {
        // Bcd<1> holds at most 2 digits (0–99)
        assert!(Bcd::<1>::try_from_u8(100).is_none());
        assert!(Bcd::<1>::try_from_u8(255).is_none());
    }

    #[test]
    fn try_from_u16_roundtrip() {
        for v in [0u16, 1, 999, 9999, u16::MAX] {
            let bcd = Bcd::<3>::try_from_u16(v).unwrap();
            assert_eq!(bcd.try_into_u16(), Some(v));
        }
    }

    #[test]
    fn try_from_u32_max() {
        let bcd = Bcd::<5>::try_from_u32(u32::MAX).unwrap();
        assert_eq!(bcd.to_string(), u32::MAX.to_string());
    }

    #[test]
    fn try_from_u64_max() {
        let bcd = Bcd::<10>::try_from_u64(u64::MAX).unwrap();
        assert_eq!(bcd.to_string(), u64::MAX.to_string());
    }

    #[test]
    fn try_from_u128_max() {
        let bcd = Bcd::<20>::try_from_u128(u128::MAX).unwrap();
        assert_eq!(bcd.to_string(), u128::MAX.to_string());
    }

    // --- Conversion back to primitives ---

    #[test]
    fn try_into_u8_overflow() {
        // Bcd<2> can hold 1000, which overflows u8
        let bcd = Bcd::<2>::try_from_u16(1000).unwrap();
        assert!(bcd.try_into_u8().is_none());
    }

    #[test]
    fn try_into_u16_overflow() {
        // Bcd<3> can hold values up to 999999, which overflows u16
        let bcd = Bcd::<3>::try_from_u32(100_000).unwrap();
        assert!(bcd.try_into_u16().is_none());
    }

    #[test]
    fn infallible_into_u8() {
        let bcd = Bcd::<1>::try_from_u8(42).unwrap();
        assert_eq!(bcd.into_u8(), 42u8);
    }

    #[test]
    fn infallible_into_u32() {
        let bcd = Bcd::<4>::try_from_u32(12_34_56_78).unwrap();
        assert_eq!(bcd.into_u32(), 12_34_56_78_u32);
    }

    // --- Display ---

    #[test]
    fn display_zero() {
        let n = Bcd::<2>::try_from_u16(0).unwrap();
        assert_eq!(n.to_string(), "0");
    }

    #[test]
    fn display_no_leading_zeros() {
        let n = Bcd::<4>::try_from_u32(42).unwrap();
        assert_eq!(n.to_string(), "42");
    }

    #[test]
    fn display_padding_right_aligned() {
        let n = Bcd::<2>::try_from_u16(42).unwrap();
        assert_eq!(format!("{n:0>6}"), "000042");
    }

    #[test]
    fn display_padding_left_aligned() {
        let n = Bcd::<2>::try_from_u16(42).unwrap();
        assert_eq!(format!("{n:<6}"), "42    ");
    }

    #[test]
    fn display_max_u32() {
        let n = Bcd::<5>::try_from_u32(u32::MAX).unwrap();
        assert_eq!(n.to_string(), u32::MAX.to_string());
    }

    // --- Ordering ---

    #[test]
    fn ordering() {
        let a = Bcd::<2>::try_from_u16(100).unwrap();
        let b = Bcd::<2>::try_from_u16(200).unwrap();
        let c = Bcd::<2>::try_from_u16(200).unwrap();
        assert!(a < b);
        assert!(b > a);
        assert_eq!(b, c);
    }

    // --- InvalidBcdDigit error ---

    #[test]
    fn invalid_bcd_digit_display() {
        let e = InvalidBcdDigit(10);
        assert_eq!(e.to_string(), "Invalid BCD digit 10. Must be <= 9");
    }

    #[test]
    fn invalid_bcd_digit_is_error() {
        // Ensure it satisfies the Error trait bound
        let e: Box<dyn std::error::Error> = Box::new(InvalidBcdDigit(15));
        assert!(e.to_string().contains("15"));
    }

    // --- bcd! macro ---

    #[test]
    fn bcd_macro_u8_range() {
        let n: Bcd<1> = bcd!(99u8);
        assert_eq!(n.to_string(), "99");
    }

    #[test]
    fn bcd_macro_u16_range() {
        let n: Bcd<2> = bcd!(1234u16);
        assert_eq!(n.to_string(), "1234");
    }

    #[test]
    fn bcd_macro_expression() {
        let x = 56u8;
        let n: Bcd<1> = bcd!(x);
        assert_eq!(n.to_string(), "56");
    }

    #[test]
    #[should_panic(expected = "Overflow during BCD conversion")]
    fn bcd_macro_panics_on_overflow() {
        // 100 won't fit in Bcd<1> which holds at most 2 digits
        let _: Bcd<1> = bcd!(100u8);
    }
}
