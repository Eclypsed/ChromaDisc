use deku::DekuRead;
use num_traits::{PrimInt, Unsigned};
use std::{any::type_name, fmt::Display};
use thiserror::Error;

const DIGIT_MAX: u8 = 9;

/// A packed binary-coded decimal (BCD) number of fixed byte length.
///
/// Each byte stores two decimal digits, one per nibble (high nibble first).
/// For example, the decimal value `1234` is stored as `[0x12, 0x34]`.
///
/// The const parameter `BYTES` determines the number of bytes (and therefore
/// the number of decimal digits, `BYTES * 2`) the value can represent.
///
/// # Conversion to binary
///
/// [`From`] is implemented for the smallest primitive that can hold the
/// represented range without overflow:
///
/// | `BYTES` | Target type |
/// |---------|-------------|
/// | 1       | `u8`        |
/// | 2       | `u16`       |
/// | 3-4     | `u32`       |
/// | 5-9     | `u64`       |
/// | 10-19   | `u128`      |
///
/// # Example
/// ```
/// use rainbow_books::bcd::Bcd;
///
/// let bcd = Bcd::from_bcd_bytes([0x12, 0x34]).unwrap();
/// assert_eq!(u16::from(bcd), 1234);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, DekuRead)]
pub struct Bcd<const BYTES: usize>([u8; BYTES]);

/// Error returned when a byte contains an invalid BCD digit (nibble > 9).
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
#[error("BCD digit {0} must be <= {max}", max = DIGIT_MAX)]
pub struct InvalidBcdDigit(u8);

impl<const BYTES: usize> Bcd<BYTES> {
    /// Constructs a [`Bcd`] from raw packed BCD bytes, validating that every
    /// nibble is a valid decimal digit (0–9).
    ///
    /// # Errors
    ///
    /// Returns [`InvalidBcdDigit`] if any nibble in `bytes` exceeds `9`.
    ///
    /// # Example
    /// ```
    /// use rainbow_books::bcd::Bcd;
    ///
    /// assert!(Bcd::from_bcd_bytes([0x99]).is_ok());
    /// assert!(Bcd::from_bcd_bytes([0xA0]).is_err()); // high nibble = 10
    /// ```
    pub const fn from_bcd_bytes(bytes: [u8; BYTES]) -> Result<Self, InvalidBcdDigit> {
        assert!(BYTES > 0, "Bcd<0> is not a valid type");
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
}

impl<const BYTES: usize> Display for Bcd<BYTES> {
    /// Formats the BCD value as its decimal string representation.
    ///
    /// A [`Bcd<2>`] containing `[0x01, 0x23]` displays as `"0123"`.
    /// Leading zeros are always included so the output is always `BYTES * 2`
    /// digits wide.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in self.0 {
            write!(f, "{}{}", byte >> 4, byte & 0x0F)?;
        }
        Ok(())
    }
}

/// Iterates over the nibbles of a byte slice, high nibble first.
struct NibbleIterator<'a> {
    bytes: &'a [u8],
    index: usize,
    high: bool,
}

impl<'a> NibbleIterator<'a> {
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            index: 0,
            high: true,
        }
    }
}

impl<'a> Iterator for NibbleIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let byte = self.bytes.get(self.index)?;
        if self.high {
            self.high = false;
            Some(byte >> 4)
        } else {
            self.index += 1;
            self.high = true;
            Some(byte & 0x0F)
        }
    }
}

/// Converts packed BCD bytes to a binary integer using Horner's method.
///
/// Panics if any nibble exceeds [`DIGIT_MAX`] or if the result overflows `N`,
/// neither of which should be reachable via the public API.
// This seems far less annoying to implement than Reverse Double Dabble don't @ me
fn bcd_to_binary<N: Unsigned + PrimInt>(bytes: &[u8]) -> N {
    let ten = N::from(10u8).unwrap_or_else(|| panic!("Couldn't fit 10 in {}", type_name::<N>()));
    NibbleIterator::new(bytes).fold(N::zero(), |acc, d| {
        assert!(d <= DIGIT_MAX);
        let d = N::from(d)
            .unwrap_or_else(|| panic!("Couldn't fit digit {} in {}", d, type_name::<N>()));
        acc.checked_mul(&ten)
            .and_then(|a| a.checked_add(&d))
            .expect("Overflowed during BCD -> Binary summation")
    })
}

macro_rules! impl_bcd_from {
    ($prim:ty, $($bytes:literal),+) => {
        $(
            impl From<Bcd<$bytes>> for $prim {
                fn from(bcd: Bcd<$bytes>) -> $prim {
                    bcd_to_binary(&bcd.0)
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
// u32:  ilog10(u32::MAX)  = 9 digits  -> 3-4 bytes
// u64:  ilog10(u64::MAX)  = 19 digits -> 5-9 bytes
// u128: ilog10(u128::MAX) = 38 digits -> 10-19 bytes
impl_bcd_from!(u8, 1);
impl_bcd_from!(u16, 2);
impl_bcd_from!(u32, 3, 4);
impl_bcd_from!(u64, 5, 6, 7, 8, 9);
impl_bcd_from!(u128, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19);
