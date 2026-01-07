use std::{
    ffi::{c_uchar, c_void},
    fs::File,
    os::fd::AsRawFd,
    ptr,
};

use nix::ioctl_read_bad;
use num_enum::TryFromPrimitive;
use thiserror::Error;

use crate::{commands::Command, error::MMCError};

#[derive(Debug, Error)]
pub enum SCSIError {
    #[error("CDB must be < 256 bytes in length, received: {0}")]
    InvalidCDB(usize),
    #[error("Data must be < 2^32 bytes in length, received: {0}")]
    InvalidData(usize),
    #[error("Syscall to ioctl failed")]
    IOCTLFailed(#[from] nix::errno::Errno),
    #[error("Residual must be non-negative and < allocation ({allocated}), received: {resid}")]
    InvalidResidual { resid: i32, allocated: u32 },
    #[error("SG IO failed with status code `{_0:?}`")]
    BadStatus(StatusCondition),
    #[error("MMC Error: {0:?}")]
    MMCError(MMCError),
    #[error("Unknown SCSI error, `masked_status`: {_0:02X}")]
    UnknownStatus(u8),
    #[error(
        "Unknown SCSI error, status={status:?}, sense_key=0x{sk:X}, asc=0x{asc:02X}, ascq=0x{ascq:02X})"
    )]
    UnknownSenseData {
        status: StatusCondition,
        sk: u8,
        asc: u8,
        ascq: u8,
    },
}

// Many of these are straight from the linux source code in linux/include/scsi/sg.h

const SG_IO: u64 = 0x2285;

#[repr(i32)]
#[allow(dead_code)]
pub enum DxferDirection {
    /// SCSI Test Unit Ready command
    None = -1,
    /// SCSI WRITE command
    ToDev = -2,
    /// SCSI READ command
    FromDev = -3,
    ToFromDev = -4,
    Unknown = -5,
}

/// Linux SCSI status codes used for comparison with `masked_status`, which strips vendor
/// information and shifts right one position. `masked_status == ((status & 0x3e) >> 1)`
#[derive(Debug, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum StatusCondition {
    Good = 0x00,
    CheckCondition = 0x01,
    ConditionGood = 0x02,
    Busy = 0x04,
    IntermediateGood = 0x08,
    IntermediateCGood = 0x0A,
    ReservationConflict = 0x0C,
    CommandTerminated = 0x11,
    QueueFull = 0x14,
    ACAActive = 0x18,
    TaskAborted = 0x20,
}

#[repr(C)]
struct SgIoHeader {
    interface_id: i32,               /* [i] 'S' for SCSI generic (required) */
    dxfer_direction: DxferDirection, /* [i] data transfer direction */
    cmd_len: u8,                     /* [i] SCSI command length */
    mx_sb_len: u8,                   /* [i] max length to write to sbp */
    iovec_count: u16,                /* [i] 0 implies no scatter gather */
    dxfer_len: u32,                  /* [i] byte count of data transfer */
    dxferp: *mut c_void, /* [i], [*io] points to data transfer memory or scatter gather list */
    cmdp: *mut c_uchar,  /* [i], [*i] points to command to perform */
    sbp: *mut c_uchar,   /* [i], [*o] points to sense_buffer memory */
    timeout: u32,        /* [i] MAX_UINT->no timeout (unit: millisec) */
    flags: u32,          /* [i] 0 -> default, see SG_FLAG... */
    pack_id: i32,        /* [i->o] unused internally (normally) */
    usr_ptr: *mut c_void, /* [i->o] unused internally */
    status: u8,          /* [o] scsi status */
    masked_status: u8,   /* [o] shifted, masked scsi status */
    msg_status: u8,      /* [o] messaging level data (optional) */
    sb_len_wr: u8,       /* [o] byte count actually written to sbp */
    host_status: u16,    /* [o] errors from host adapter */
    driver_status: u16,  /* [o] errors from software driver */
    resid: i32,          /* [o] dxfer_len - actual_transferred */
    duration: u32,       /* [o] time taken by cmd (unit: millisec) */
    info: u32,           /* [o] auxiliary information */
}

ioctl_read_bad!(ioctl_sg_io, SG_IO, SgIoHeader);

pub fn run_sgio<Cmd: Command<CMD_LEN>, const CMD_LEN: usize>(
    file: &File,
    cmd: Cmd,
    dxfer_direction: DxferDirection,
) -> Result<Vec<u8>, SCSIError> {
    const SENSE_BUF_SIZE: u8 = 64;

    let mut sense = [0u8; SENSE_BUF_SIZE as usize];

    let mut cdb = cmd.as_cdb();
    let mut data = vec![0u8; cmd.allocation_len()];

    let cdb_len = cdb.len();
    let Ok(cmd_len) = u8::try_from(cdb_len) else {
        return Err(SCSIError::InvalidCDB(cdb_len));
    };

    let data_len = data.len();

    let Ok(dxfer_len) = u32::try_from(data_len) else {
        return Err(SCSIError::InvalidData(data_len));
    };

    let mut header = SgIoHeader {
        interface_id: 'S' as i32,
        dxfer_direction,
        cmd_len,
        mx_sb_len: SENSE_BUF_SIZE,
        iovec_count: 0,
        dxfer_len,
        dxferp: data.as_mut_ptr() as *mut c_void,
        cmdp: cdb.as_mut_ptr(),
        sbp: sense.as_mut_ptr(),
        timeout: 10_000,
        flags: 0,
        pack_id: 0,
        usr_ptr: ptr::null_mut(),
        status: 0,
        masked_status: 0,
        msg_status: 0,
        sb_len_wr: 0,
        host_status: 0,
        driver_status: 0,
        resid: 0,
        duration: 0,
        info: 0,
    };

    unsafe {
        ioctl_sg_io(file.as_raw_fd(), &mut header)?;
    }

    let Ok(status) = StatusCondition::try_from_primitive(header.masked_status) else {
        return Err(SCSIError::UnknownStatus(header.masked_status));
    };

    // Note: If status == ConditionGood, then there *is* sense data available, but idk if I really
    // care about that.
    if status == StatusCondition::Good || status == StatusCondition::ConditionGood {
        // From the SCSI HOWTO: "In practice it only reports underruns (i.e. positive number) as data
        // overruns should never happen"

        if let Ok(residual) = usize::try_from(header.resid)
            && data_len > residual
        {
            data.truncate(data_len - residual);
            return Ok(data);
        };

        return Err(SCSIError::InvalidResidual {
            resid: header.resid,
            allocated: dxfer_len,
        });
    }

    // If there's sense data, parse it for more details
    if header.sb_len_wr > 0 {
        let sk = sense[2] & 0x0F; // Sense key
        let asc = sense[12]; // Additional Sense Code
        let ascq = sense[13]; // Additional Sense Code Qualifier

        let Some(mmc_error) = MMCError::from_codes(sk, asc, ascq) else {
            return Err(SCSIError::UnknownSenseData {
                status,
                sk,
                asc,
                ascq,
            });
        };

        return Err(SCSIError::MMCError(mmc_error));
    }

    Err(SCSIError::BadStatus(status))
}
