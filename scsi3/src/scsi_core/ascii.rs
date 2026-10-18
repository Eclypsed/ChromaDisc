use core::fmt;

use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// A variable-length byte string that is expected to be ASCII but does not enforce that
/// invariant.
///
/// The unsized counterpart to [`UnvalidatedSizedAsciiStr`], for ASCII fields whose length
/// is only known at runtime.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[repr(C)]
pub struct UnvalidatedAsciiStr([u8]);

impl UnvalidatedAsciiStr {
    #[inline]
    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Converts into a `&str`. Fails if the bytes are not valid ASCII.
    #[inline]
    pub const fn try_as_str(&self) -> Option<&str> {
        if !self.0.is_ascii() {
            return None;
        }
        match core::str::from_utf8(&self.0) {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }
}

impl fmt::Debug for UnvalidatedAsciiStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Debug as a string if possible
        match self.try_as_str() {
            Some(s) => fmt::Debug::fmt(s, f),
            None => fmt::Debug::fmt(&self.0, f),
        }
    }
}

#[derive(
    FromBytes,
    IntoBytes,
    KnownLayout,
    Immutable,
    Unaligned,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[repr(transparent)]
pub struct UnvalidatedSizedAsciiStr<const N: usize>([u8; N]);

impl<const N: usize> UnvalidatedSizedAsciiStr<N> {
    #[inline]
    pub const fn as_bytes(&self) -> &[u8; N] {
        &self.0
    }

    /// Converts into a `&str`. Fails if the bytes are not valid ASCII.
    #[inline]
    pub const fn try_as_str(&self) -> Option<&str> {
        if !self.0.is_ascii() {
            return None;
        }
        match core::str::from_utf8(&self.0) {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }
}

impl<const N: usize> fmt::Debug for UnvalidatedSizedAsciiStr<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Debug as a string if possible
        match self.try_as_str() {
            Some(s) => fmt::Debug::fmt(s, f),
            None => fmt::Debug::fmt(&self.0, f),
        }
    }
}
