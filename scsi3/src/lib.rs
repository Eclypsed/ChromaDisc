// Overide paths becasue folder names reflect names that would be used if
// I ever decide to break scsi3 up into its own multi-create project
#[path = "scsi_core/mod.rs"]
pub mod core;
#[path = "scsi_mmc/mod.rs"]
pub mod mmc;
pub mod rainbow_books;
#[path = "scsi_spc/mod.rs"]
pub mod spc;
