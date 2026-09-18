#![no_std]
#![forbid(unsafe_code)]

// May try to remove this in the future to make scsi3 compatible with no-heap environments
extern crate alloc;

// Overide paths becasue folder names reflect names that would be used if
// I ever decide to break scsi3 up into its own multi-create project
#[path = "scsi_core/mod.rs"]
pub mod core;
#[path = "scsi_mmc/mod.rs"]
pub mod mmc;
#[path = "scsi_spc/mod.rs"]
pub mod spc;
