use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// DVD CSS (0106h): the ability to perform DVD CSS/CPPM authentication and RPC.
///
/// Laid out as version 0.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct DvdCss {
    _bytes4_6: [u8; 3],
    css_version: u8,
}

impl DvdCss {
    #[inline]
    pub const fn css_version(&self) -> u8 {
        self.css_version
    }
}

super::debug_fields!(DvdCss { css_version });
