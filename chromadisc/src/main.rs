use std::{
    io,
    os::fd::{AsRawFd, OwnedFd},
};

use scsi3::{
    core::{
        addressing::{Lba, Span},
        Command, Control, ReadCommand,
    },
    mmc::commands::{
        read_cd::{
            main_channel::{
                selections::{NoFields, SyncAllHeadersUserDataEdcEcc, UserData},
                CdDa, Mode2Form1,
            },
            BlockC2Pointers, RawPw, ReadCd,
        },
        read_toc_pma_atip::{formatted_toc::FormattedToc, ReadTocPmaAtip},
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

    let command =
        ReadCd::<Lba, Mode2Form1, SyncAllHeadersUserDataEdcEcc, BlockC2Pointers, RawPw>::new(
            252000.into(),
            5u8.into(),
            Control::default(),
        );
    let expected_bytes: usize = command.response_len().try_into().unwrap();
    let mut buf = vec![0u8; 16384];

    let received = run_sgio(
        fd.as_raw_fd(),
        command.as_cdb().as_mut_slice(),
        buf.as_mut_slice(),
        DxferDirection::FromDev,
    )
    .unwrap();

    println!("Received: {received} bytes");
    let mut res = command.parse(&buf[0..(expected_bytes + 100)]).unwrap();
    println!("Sectors: {}", res.len());
    println!();
    let sector = res.next().unwrap();
    println!("Last Sector:");
    print!("{:?}", sector);

    Ok(())
}
