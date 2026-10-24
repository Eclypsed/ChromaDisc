//! Byte types that zerocopy's `TryFromBytes` accepts only with certain bits set or clear, for
//! fields whose bits decide what their bytes are, such as the ID bits of an ATIP section.
// zerocopy checks validity per field and per byte, and has no custom validators yet, so a
// constraint on some of a byte's bits takes a `#[repr(u8)]` enum of the bytes that meet it.
//
// The types are `pub` only because zerocopy's `KnownLayout` derive names a struct's last field
// type in a public associated type, where a private type is an error (E0446). They're declared in
// modules that aren't public, this one and each `mod bytes` beside the structs that use them, so
// nothing outside the crate can name them.
// TODO: Replace these with custom validators, once zerocopy has them.

/// Defines a `#[repr(u8)]` enum whose variants are the bytes that match a bit pattern, so that
/// zerocopy's `TryFromBytes` accepts only those bytes.
///
/// The pattern lists bits 7 to 0, each `0`, `1`, or `x` for either, and `_` may separate them:
///
/// ```ignore
/// byte_pattern! {
///     /// Byte M of a CD's Special Information 1: ID bit 1, and the DDCD bit, bit 3, clear.
///     pub enum SpecialInformation1M = 1 x x x _ 0 x x x
/// }
/// ```
///
/// The variants are hidden, since only casts create the enum. Fields of its type are read with
/// `as u8`, as `bit!` does.
// A `macro_rules!` macro can't build identifiers, so the variants come from a table of all 256
// bytes, laid out as a binary tree that splits on bit 7 first. Each bit of the pattern keeps the
// half of every subtree it allows: the left for `0`, the right for `1`, and both for `x`.
macro_rules! byte_pattern {
    ($(#[$meta:meta])* $vis:vis enum $name:ident = $($bit:tt)*) => {
        $crate::core::validity::byte_pattern!(@select [$(#[$meta])* $vis $name] [$($bit)*] [
            {{{{{{{{(_00 0x00) (_01 0x01)} {(_02 0x02) (_03 0x03)}}
            {{(_04 0x04) (_05 0x05)} {(_06 0x06) (_07 0x07)}}}
            {{{(_08 0x08) (_09 0x09)} {(_0A 0x0A) (_0B 0x0B)}}
            {{(_0C 0x0C) (_0D 0x0D)} {(_0E 0x0E) (_0F 0x0F)}}}}
            {{{{(_10 0x10) (_11 0x11)} {(_12 0x12) (_13 0x13)}}
            {{(_14 0x14) (_15 0x15)} {(_16 0x16) (_17 0x17)}}}
            {{{(_18 0x18) (_19 0x19)} {(_1A 0x1A) (_1B 0x1B)}}
            {{(_1C 0x1C) (_1D 0x1D)} {(_1E 0x1E) (_1F 0x1F)}}}}}
            {{{{{(_20 0x20) (_21 0x21)} {(_22 0x22) (_23 0x23)}}
            {{(_24 0x24) (_25 0x25)} {(_26 0x26) (_27 0x27)}}}
            {{{(_28 0x28) (_29 0x29)} {(_2A 0x2A) (_2B 0x2B)}}
            {{(_2C 0x2C) (_2D 0x2D)} {(_2E 0x2E) (_2F 0x2F)}}}}
            {{{{(_30 0x30) (_31 0x31)} {(_32 0x32) (_33 0x33)}}
            {{(_34 0x34) (_35 0x35)} {(_36 0x36) (_37 0x37)}}}
            {{{(_38 0x38) (_39 0x39)} {(_3A 0x3A) (_3B 0x3B)}}
            {{(_3C 0x3C) (_3D 0x3D)} {(_3E 0x3E) (_3F 0x3F)}}}}}}
            {{{{{{(_40 0x40) (_41 0x41)} {(_42 0x42) (_43 0x43)}}
            {{(_44 0x44) (_45 0x45)} {(_46 0x46) (_47 0x47)}}}
            {{{(_48 0x48) (_49 0x49)} {(_4A 0x4A) (_4B 0x4B)}}
            {{(_4C 0x4C) (_4D 0x4D)} {(_4E 0x4E) (_4F 0x4F)}}}}
            {{{{(_50 0x50) (_51 0x51)} {(_52 0x52) (_53 0x53)}}
            {{(_54 0x54) (_55 0x55)} {(_56 0x56) (_57 0x57)}}}
            {{{(_58 0x58) (_59 0x59)} {(_5A 0x5A) (_5B 0x5B)}}
            {{(_5C 0x5C) (_5D 0x5D)} {(_5E 0x5E) (_5F 0x5F)}}}}}
            {{{{{(_60 0x60) (_61 0x61)} {(_62 0x62) (_63 0x63)}}
            {{(_64 0x64) (_65 0x65)} {(_66 0x66) (_67 0x67)}}}
            {{{(_68 0x68) (_69 0x69)} {(_6A 0x6A) (_6B 0x6B)}}
            {{(_6C 0x6C) (_6D 0x6D)} {(_6E 0x6E) (_6F 0x6F)}}}}
            {{{{(_70 0x70) (_71 0x71)} {(_72 0x72) (_73 0x73)}}
            {{(_74 0x74) (_75 0x75)} {(_76 0x76) (_77 0x77)}}}
            {{{(_78 0x78) (_79 0x79)} {(_7A 0x7A) (_7B 0x7B)}}
            {{(_7C 0x7C) (_7D 0x7D)} {(_7E 0x7E) (_7F 0x7F)}}}}}}}
            {{{{{{{(_80 0x80) (_81 0x81)} {(_82 0x82) (_83 0x83)}}
            {{(_84 0x84) (_85 0x85)} {(_86 0x86) (_87 0x87)}}}
            {{{(_88 0x88) (_89 0x89)} {(_8A 0x8A) (_8B 0x8B)}}
            {{(_8C 0x8C) (_8D 0x8D)} {(_8E 0x8E) (_8F 0x8F)}}}}
            {{{{(_90 0x90) (_91 0x91)} {(_92 0x92) (_93 0x93)}}
            {{(_94 0x94) (_95 0x95)} {(_96 0x96) (_97 0x97)}}}
            {{{(_98 0x98) (_99 0x99)} {(_9A 0x9A) (_9B 0x9B)}}
            {{(_9C 0x9C) (_9D 0x9D)} {(_9E 0x9E) (_9F 0x9F)}}}}}
            {{{{{(_A0 0xA0) (_A1 0xA1)} {(_A2 0xA2) (_A3 0xA3)}}
            {{(_A4 0xA4) (_A5 0xA5)} {(_A6 0xA6) (_A7 0xA7)}}}
            {{{(_A8 0xA8) (_A9 0xA9)} {(_AA 0xAA) (_AB 0xAB)}}
            {{(_AC 0xAC) (_AD 0xAD)} {(_AE 0xAE) (_AF 0xAF)}}}}
            {{{{(_B0 0xB0) (_B1 0xB1)} {(_B2 0xB2) (_B3 0xB3)}}
            {{(_B4 0xB4) (_B5 0xB5)} {(_B6 0xB6) (_B7 0xB7)}}}
            {{{(_B8 0xB8) (_B9 0xB9)} {(_BA 0xBA) (_BB 0xBB)}}
            {{(_BC 0xBC) (_BD 0xBD)} {(_BE 0xBE) (_BF 0xBF)}}}}}}
            {{{{{{(_C0 0xC0) (_C1 0xC1)} {(_C2 0xC2) (_C3 0xC3)}}
            {{(_C4 0xC4) (_C5 0xC5)} {(_C6 0xC6) (_C7 0xC7)}}}
            {{{(_C8 0xC8) (_C9 0xC9)} {(_CA 0xCA) (_CB 0xCB)}}
            {{(_CC 0xCC) (_CD 0xCD)} {(_CE 0xCE) (_CF 0xCF)}}}}
            {{{{(_D0 0xD0) (_D1 0xD1)} {(_D2 0xD2) (_D3 0xD3)}}
            {{(_D4 0xD4) (_D5 0xD5)} {(_D6 0xD6) (_D7 0xD7)}}}
            {{{(_D8 0xD8) (_D9 0xD9)} {(_DA 0xDA) (_DB 0xDB)}}
            {{(_DC 0xDC) (_DD 0xDD)} {(_DE 0xDE) (_DF 0xDF)}}}}}
            {{{{{(_E0 0xE0) (_E1 0xE1)} {(_E2 0xE2) (_E3 0xE3)}}
            {{(_E4 0xE4) (_E5 0xE5)} {(_E6 0xE6) (_E7 0xE7)}}}
            {{{(_E8 0xE8) (_E9 0xE9)} {(_EA 0xEA) (_EB 0xEB)}}
            {{(_EC 0xEC) (_ED 0xED)} {(_EE 0xEE) (_EF 0xEF)}}}}
            {{{{(_F0 0xF0) (_F1 0xF1)} {(_F2 0xF2) (_F3 0xF3)}}
            {{(_F4 0xF4) (_F5 0xF5)} {(_F6 0xF6) (_F7 0xF7)}}}
            {{{(_F8 0xF8) (_F9 0xF9)} {(_FA 0xFA) (_FB 0xFB)}}
            {{(_FC 0xFC) (_FD 0xFD)} {(_FE 0xFE) (_FF 0xFF)}}}}}}}}
        ]);
    };
    (@select $item:tt [_ $($bit:tt)*] $nodes:tt) => {
        $crate::core::validity::byte_pattern!(@select $item [$($bit)*] $nodes);
    };
    (@select $item:tt [0 $($bit:tt)*] [$({$zero:tt $one:tt})*]) => {
        $crate::core::validity::byte_pattern!(@select $item [$($bit)*] [$($zero)*]);
    };
    (@select $item:tt [1 $($bit:tt)*] [$({$zero:tt $one:tt})*]) => {
        $crate::core::validity::byte_pattern!(@select $item [$($bit)*] [$($one)*]);
    };
    (@select $item:tt [x $($bit:tt)*] [$({$zero:tt $one:tt})*]) => {
        $crate::core::validity::byte_pattern!(@select $item [$($bit)*] [$($zero $one)*]);
    };
    (@select [$(#[$meta:meta])* $vis:vis $name:ident] [] [$(($variant:ident $byte:literal))*]) => {
        $(#[$meta])*
        #[derive(
            ::zerocopy::TryFromBytes,
            ::zerocopy::IntoBytes,
            ::zerocopy::KnownLayout,
            ::zerocopy::Immutable,
            ::zerocopy::Unaligned,
            Clone,
            Copy,
            PartialEq,
            Eq,
            Hash,
        )]
        #[repr(u8)]
        $vis enum $name {
            $(#[doc(hidden)] $variant = $byte,)*
        }
    };
}

pub(crate) use byte_pattern;

byte_pattern! {
    /// A byte whose most significant bit, bit 7, is 0.
    pub enum Msb0 = 0 x x x _ x x x x
}

byte_pattern! {
    /// A byte whose most significant bit, bit 7, is 1.
    pub enum Msb1 = 1 x x x _ x x x x
}
