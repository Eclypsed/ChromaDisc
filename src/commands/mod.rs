#![doc = include_str!("../../docs/scsi.md")]

pub mod get_configuration;
pub mod read_cd;
pub mod toc;

use derive_more::{Debug, From, Into};

pub trait Command<const CDB_LEN: usize> {
    /// OPERATION CODE enum for valid MMCs
    /// ```text
    ///   7   6   5   4   3   2   1   0
    /// +---+---+---+---+---+---+---+---+
    /// | GROUPCODE |    COMMAND CODE   |
    /// +---+---+---+---+---+---+---+---+
    /// ```
    /// See: [SAM-2]
    const OP_CODE: u8;

    type Response: TryFrom<Vec<u8>>;

    fn as_cdb(&self) -> [u8; CDB_LEN];

    fn allocation_len(&self) -> usize;
}

/// CONTROL byte newtype
/// ```text
///   7   6   5   4   3   2   1   0
/// +---+---+---+---+---+---+---+---+
/// |   VS  |  Reserved | N | O | L |
/// +---+---+---+---+---+---+---+---+
/// ```
/// * **VS** - Vendor Specific
/// * **N**  - NACA (Normal Auto Contingent Allegiance)
/// * **O**  - Obsolete
/// * **L**  - Link
///
/// See: [SAM-2]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, From, Into)]
pub struct Control(u8);
