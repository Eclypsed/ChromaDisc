use nix::sys::stat::Mode;
use std::os::fd::{AsRawFd, OwnedFd};
use std::path::Path;
use std::{fs, io};
use thiserror::Error;

use array_concat::*;
use const_format::concatcp;
use nix::fcntl::{open, OFlag};
use seq_macro::seq;

use crate::core::{Command, OpCodeDef, Response};
// use crate::scsi::mmc::commands::{execute, inquiry::Inquiry};
use crate::transport::sgio::{run_sgio, DxferDirection, ScsiError};

macro_rules! device_files {
    ($prefix:expr, $($range:tt)+) => {{
        const P: &str = $prefix;
        seq!(N in $($range)+ {
            [
                #(concatcp!(P, N), )*
            ]
        })
    }};
}

const NAMED_DEVICES: [&str; 2] = ["/dev/cdrom", "/dev/dvd"];
const HD_DEVICES: [&str; 26] = device_files!("/dev/hd", 'a'..='z');
const SCD_DEVICES: [&str; 28] = device_files!("/dev/scd", 0u8..=27u8);
const SR_DEVICES: [&str; 28] = device_files!("/dev/sr", 0u8..=27u8);

pub const DEVICES: [&str; concat_arrays_size!(NAMED_DEVICES, HD_DEVICES, SCD_DEVICES, SR_DEVICES)] =
    concat_arrays!(NAMED_DEVICES, HD_DEVICES, SCD_DEVICES, SR_DEVICES);

#[derive(Debug, Error)]
pub enum ExecuteError<E> {
    #[error(transparent)]
    Scsi(#[from] ScsiError),
    #[error("failed to parse response: {0}")]
    Parse(E),
    #[error(transparent)]
    Io(#[from] io::Error),
}

#[derive(Debug)]
pub struct Drive {
    pub devnode: String,
}

impl Drive {
    pub fn new(devnode: String) -> Self {
        Self { devnode }
    }

    fn get_fd(&self) -> io::Result<OwnedFd> {
        Ok(open(
            self.devnode.as_str(),
            OFlag::O_RDONLY | OFlag::O_NONBLOCK,
            Mode::empty(),
        )?)
    }

    pub fn execute<O, C, E>(&self, command: C) -> Result<C::Response, ExecuteError<E>>
    where
        O: OpCodeDef,
        C: Command<O>,
        C::Response: Response<Error = E>,
    {
        let bytes = run_sgio(
            self.get_fd()?.as_raw_fd(),
            command.as_cdb().as_mut(),
            4096,
            DxferDirection::FromDev,
        )?;
        C::Response::from_bytes(bytes).map_err(ExecuteError::Parse)
    }
}

pub fn scan_sysfs() -> io::Result<Vec<String>> {
    const OPTICAL_DEVICE_TYPE: &str = "5";

    let mut devnodes = Vec::new();
    let base = Path::new("/sys/class/block");

    for entry in fs::read_dir(base)? {
        let Ok(entry) = entry else {
            continue;
        };

        let name = entry.file_name().to_string_lossy().into_owned();

        let device_path = entry.path().join("device");

        let type_path = device_path.join("type");

        let Ok(dev_type) = fs::read_to_string(&type_path).map(|t| t.trim().to_string()) else {
            continue;
        };

        if dev_type == OPTICAL_DEVICE_TYPE {
            devnodes.push(format!("/dev/{}", name));
        }
    }

    Ok(devnodes)
}

// pub fn get_devices() -> Vec<Drive> {
//     let mut devices = Vec::new();

//     for devnode in scan_sysfs().unwrap() {
//         let fd = get_file_descriptor(&devnode).unwrap();
//         let inquiry = Inquiry::new(false, 0, 0.into());
//         let res = execute(inquiry, fd.as_raw_fd()).unwrap();

//         devices.push(Drive {
//             devnode,
//             removeable_medium: res.removable_media,
//             spc_version: res.version,
//             vendor: res.t10_vendor_identification,
//             product_id: res.product_identification,
//             revision: res.product_revision_level,
//         });
//     }

//     devices
// }
