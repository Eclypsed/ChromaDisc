use zerocopy::{BigEndian, FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned, U16};

/// Accessors for a Number of Link Sizes field followed by the Link Size entries and Pad bytes.
macro_rules! link_sizes {
    () => {
        #[inline]
        pub const fn number_of_link_sizes(&self) -> u8 {
            self.number_of_link_sizes
        }

        /// The Link Size entries followed by the Pad bytes.
        #[inline]
        pub const fn link_sizes_and_pad(&self) -> &[u8] {
            &self.link_sizes_and_pad
        }

        /// The first Number of Link Sizes bytes of [`Self::link_sizes_and_pad`].
        ///
        /// Shorter than Number of Link Sizes only if fewer bytes are present, which a drive
        /// conforming to the spec won't report.
        #[inline]
        pub const fn link_sizes(&self) -> &[u8] {
            match self
                .link_sizes_and_pad
                .split_at_checked(self.number_of_link_sizes as usize)
            {
                Some((link_sizes, _pad)) => link_sizes,
                None => &self.link_sizes_and_pad,
            }
        }
    };
}

pub(super) use link_sizes;

/// Incremental Streaming Writable (0021h): write support for sequential recording.
///
/// Laid out as version 3.
#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct IncrementalStreamingWritable {
    data_block_type_supported: U16<BigEndian>,
    byte6: u8,
    number_of_link_sizes: u8,
    link_sizes_and_pad: [u8],
}

impl IncrementalStreamingWritable {
    /// Since version 1.
    #[inline]
    pub const fn data_block_type_supported(&self) -> u16 {
        self.data_block_type_supported.get()
    }

    // TODO: Currently missing copy of MMC that details v2 of the ISW feature.
    // Need a copy from somewhere between MMC-5r01e and MMC-5r02b.
    // It's safe to assume based on v1 and v3 that v2 introduced either the TRIO flag or the ARSV
    // flag, and that the other was added in v3.
    super::bit!(
        /// Since version 2 or 3.
        #[doc(alias = "TRIO")]
        track_resource_info_open: byte6[2]
    );
    super::bit!(
        /// Since version 2 or 3.
        #[doc(alias = "ARSV")]
        address_mode_reservation: byte6[1]
    );
    super::bit!(
        /// Since version 1.
        buf: byte6[0]
    );

    link_sizes!();
}

super::debug_fields!(IncrementalStreamingWritable {
    data_block_type_supported,
    track_resource_info_open,
    address_mode_reservation,
    buf,
    number_of_link_sizes,
    link_sizes
});
