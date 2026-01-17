pub mod get_configuration;
pub mod inquiry;
pub mod read_capacity;
pub mod read_cd;
pub mod read_track_info;
pub mod start_stop_unit;
pub mod toc;

use derive_more::{Debug, From, Into};
use thiserror::Error;

use crate::transport::sgio::{DxferDirection, SCSIError, run_sgio};

#[derive(Debug, Error)]
pub enum ExecuteError<Cmd: Command<N>, const N: usize> {
    #[error(transparent)]
    SCSIError(#[from] SCSIError),
    #[error("Failed to parse the response from the command")]
    ParseError(#[source] <Cmd::Response as TryFrom<Vec<u8>>>::Error),
}

pub trait Command<const CDB_LEN: usize>: Sized {
    /// OPERATION CODE enum for valid MMCs
    /// ```text
    ///   7   6   5   4   3   2   1   0
    /// +---+---+---+---+---+---+---+---+
    /// | GROUPCODE |    COMMAND CODE   |
    /// +---+---+---+---+---+---+---+---+
    /// ```
    /// See: [SAM-6]
    const OP_CODE: u8;

    type Response: TryFrom<Vec<u8>>;

    fn as_cdb(&self) -> [u8; CDB_LEN];

    fn allocation_len(&self) -> usize;
}

pub fn execute<Cmd: Command<CDB_LEN>, const CDB_LEN: usize>(
    cmd: Cmd,
    fd: i32,
) -> Result<Cmd::Response, ExecuteError<Cmd, CDB_LEN>> {
    let bytes = run_sgio(fd, cmd, DxferDirection::FromDev)?;
    <Cmd::Response as TryFrom<Vec<u8>>>::try_from(bytes).map_err(|e| ExecuteError::ParseError(e))
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
/// See: [SAM-6]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, From, Into)]
pub struct Control(u8);
