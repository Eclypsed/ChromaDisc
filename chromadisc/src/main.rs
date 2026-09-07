use std::{
    io,
    os::fd::{AsRawFd, OwnedFd},
};

#[allow(unused_imports)]
use scsi3::{
    core::{
        addressing::{Lba, Span},
        Command, Control, ReadCommand,
    },
    mmc::{
        commands::{
            read_cd::{
                cd_da::CdDa, mode1::Mode1, mode2_form1::Mode2Form1, BlockC2Pointers, C2ErrorInfo,
                MainChannel, NoC2, NoSubChannel, ReadCd, SubChannelSelection,
            },
            read_toc_pma_atip::{formatted_toc::FormattedToc, ReadTocPmaAtip},
        },
        msf::{Minute, Msf},
    },
    spc::commands::inquiry::{standard_inquiry::StandardInquiry, Inquiry},
};

use crate::{
    device::{scan_sysfs, Drive},
    transport::sgio::{run_sgio, DxferDirection},
};

pub mod device;
pub mod transport;

pub const CHROMADISC_VERSION: &str = "0.1.0";

#[allow(dead_code)]
fn inquiry(fd: &OwnedFd) -> StandardInquiry {
    let inquiry_cmd = Inquiry::<StandardInquiry>::new(200, Control::default());
    let mut inq_buf = vec![0u8; inquiry_cmd.response_len().into()];

    let _inq_received = run_sgio(
        fd.as_raw_fd(),
        inquiry_cmd.as_cdb().as_mut_slice(),
        inq_buf.as_mut_slice(),
        DxferDirection::FromDev,
    )
    .unwrap();

    inquiry_cmd.parse(&inq_buf).unwrap()
}

#[allow(dead_code)]
fn read_toc(fd: &OwnedFd) -> FormattedToc<Lba> {
    let toc_cmd = ReadTocPmaAtip::<FormattedToc<Lba>>::new(0, 1024 * 4, Control::default());
    let mut toc_buf = vec![0u8; toc_cmd.response_len().into()];

    let _toc_received = run_sgio(
        fd.as_raw_fd(),
        toc_cmd.as_cdb().as_mut_slice(),
        toc_buf.as_mut_slice(),
        DxferDirection::FromDev,
    )
    .unwrap();

    toc_cmd.parse(&toc_buf).unwrap()
}

fn main() -> io::Result<()> {
    println!("ChromaDisc version {}", CHROMADISC_VERSION);
    println!();

    let devices = scan_sysfs()?;

    println!("Discovered Devices:");
    for device in &devices {
        println!("{device}")
    }
    println!();

    let drive = Drive::new(devices[0].clone());
    let fd = drive.get_fd()?;

    // println!("INQUIRY:\n{:#?}", inquiry(&fd));
    // println!("TOC:\n{:#?}", read_toc(&fd));

    let command = ReadCd::<Lba, CdDa>::new(false, 1000.into(), 27u8.into(), Control::default());
    let expected_bytes: usize = command.response_len().try_into().unwrap();
    let mut buf = vec![0u8; expected_bytes];

    let received = run_sgio(
        fd.as_raw_fd(),
        command.as_cdb().as_mut_slice(),
        buf.as_mut_slice(),
        DxferDirection::FromDev,
    )
    .unwrap();

    println!("Received: {received} bytes");
    let mut res = command.parse(&buf).unwrap();
    println!("Sectors: {}", res.len());
    println!();
    let sector1 = res.next().unwrap();
    println!("Sector 1:");
    println!("User Data: {} bytes", sector1.user_data().len());

    Ok(())
}
