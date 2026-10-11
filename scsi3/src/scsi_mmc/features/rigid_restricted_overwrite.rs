use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// Rigid Restricted Overwrite (002Ch): write support for media that is required to be written
/// from Blocking boundaries with length of integral multiple Blocking size only.
///
/// Laid out as version 0.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct RigidRestrictedOverwrite {
    byte4: u8,
    _bytes5_7: [u8; 3],
}

impl RigidRestrictedOverwrite {
    super::bit!(#[doc(alias = "DSDG")] defect_status_data_generate: byte4[3]);
    super::bit!(#[doc(alias = "DSDR")] defect_status_data_read: byte4[2]);
    super::bit!(intermediate: byte4[1]);
    super::bit!(blank: byte4[0]);
}

super::debug_fields!(RigidRestrictedOverwrite {
    defect_status_data_generate,
    defect_status_data_read,
    intermediate,
    blank
});
