use std::ffi::{c_uchar, c_void};

use nix::ioctl_read_bad;

// Many of these are straight from the linux source code in linux/include/scsi/sg.h

const SG_IO: u64 = 0x2285;
pub const SG_INFO_CHECK: u32 = 0x1;

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

#[repr(C)]
pub struct SgIoHeader {
    pub interface_id: i32,               /* [i] 'S' for SCSI generic (required) */
    pub dxfer_direction: DxferDirection, /* [i] data transfer direction */
    pub cmd_len: u8,                     /* [i] SCSI command length */
    pub mx_sb_len: u8,                   /* [i] max length to write to sbp */
    pub iovec_count: u16,                /* [i] 0 implies no scatter gather */
    pub dxfer_len: u32,                  /* [i] byte count of data transfer */
    pub dxferp: *mut c_void, /* [i], [*io] points to data transfer memory or scatter gather list */
    pub cmdp: *mut c_uchar,  /* [i], [*i] points to command to perform */
    pub sbp: *mut c_uchar,   /* [i], [*o] points to sense_buffer memory */
    pub timeout: u32,        /* [i] MAX_UINT->no timeout (unit: millisec) */
    pub flags: u32,          /* [i] 0 -> default, see SG_FLAG... */
    pub pack_id: i32,        /* [i->o] unused internally (normally) */
    pub usr_ptr: *mut c_void, /* [i->o] unused internally */
    pub status: u8,          /* [o] scsi status */
    pub masked_status: u8,   /* [o] shifted, masked scsi status */
    pub msg_status: u8,      /* [o] messaging level data (optional) */
    pub sb_len_wr: u8,       /* [o] byte count actually written to sbp */
    pub host_status: u16,    /* [o] errors from host adapter */
    pub driver_status: u16,  /* [o] errors from software driver */
    pub resid: i32,          /* [o] dxfer_len - actual_transferred */
    pub duration: u32,       /* [o] time taken by cmd (unit: millisec) */
    pub info: u32,           /* [o] auxiliary information */
}

impl SgIoHeader {
    pub fn new(
        dxfer_direction: DxferDirection,
        cdb_bytes: &mut [u8],
        data_buf: &mut [u8],
        sense_buf: &mut [u8],
    ) -> Self {
        SgIoHeader {
            interface_id: 'S' as i32,
            dxfer_direction,
            cmd_len: cdb_bytes.len() as u8,
            mx_sb_len: sense_buf.len() as u8,
            iovec_count: 0,
            dxfer_len: data_buf.len() as u32,
            dxferp: data_buf.as_mut_ptr() as *mut c_void,
            cmdp: cdb_bytes.as_mut_ptr(),
            sbp: sense_buf.as_mut_ptr(),
            timeout: 10_000,
            flags: 0,
            pack_id: 0,
            usr_ptr: std::ptr::null_mut(),
            status: 0,
            masked_status: 0,
            msg_status: 0,
            sb_len_wr: 0,
            host_status: 0,
            driver_status: 0,
            resid: 0,
            duration: 0,
            info: 0,
        }
    }
}

ioctl_read_bad!(ioctl_sg_io, SG_IO, SgIoHeader);
