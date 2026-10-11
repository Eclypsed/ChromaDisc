use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// Real Time Streaming (0107h): the ability to read and write using Host requested performance
/// parameters.
///
/// Laid out as version 5. Version 0 has no Feature Dependent Data.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct RealTimeStreaming {
    byte4: u8,
    _bytes5_7: [u8; 3],
}

impl RealTimeStreaming {
    super::bit!(
        /// Since version 5.
        #[doc(alias = "SMP")]
        set_minimum_performance: byte4[5]
    );
    super::bit!(
        /// Since version 3.
        #[doc(alias = "RBCB")]
        read_buffer_capacity_block: byte4[4]
    );
    super::bit!(
        /// Since version 2.
        #[doc(alias = "SCS")]
        set_cd_speed: byte4[3]
    );
    super::bit!(
        /// Since version 2.
        #[doc(alias = "MP2A")]
        mode_page_2a: byte4[2]
    );
    super::bit!(
        /// Since version 2.
        #[doc(alias = "WSPD")]
        write_speed_performance_descriptor: byte4[1]
    );
    super::bit!(
        /// Since version 1.
        #[doc(alias = "SW")]
        stream_writing: byte4[0]
    );
}

super::debug_fields!(RealTimeStreaming {
    set_minimum_performance,
    read_buffer_capacity_block,
    set_cd_speed,
    mode_page_2a,
    write_speed_performance_descriptor,
    stream_writing
});
