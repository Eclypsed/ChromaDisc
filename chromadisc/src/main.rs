use std::{
    io,
    mem::offset_of,
    os::fd::{AsRawFd, OwnedFd},
};

use scsi3::{
    core::{
        addressing::{Lba, Span},
        Command, Control, ReadCommand,
    },
    mmc::{
        commands::{
            read_cd::{
                c2::{BlockC2Pointers, C2Pointers, NoC2},
                main_channel::{
                    selections::{NoFields, SyncAllHeadersUserDataEdcEcc, UserData},
                    CdDa, Mode2Form1,
                },
                sub_channel::{FormattedQ, NoSubChannel, RawPw},
                ReadCd, Transfer,
            },
            read_toc_pma_atip::{formatted_toc::FormattedToc, ReadTocPmaAtip},
        },
        device_models::cd::addressing::Msf,
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
fn read_toc(fd: &OwnedFd) -> FormattedToc<Msf> {
    let toc_cmd = ReadTocPmaAtip::<FormattedToc<Msf>>::new(0, 1024 * 4, Control::default());
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
    println!("TOC:\n{:#?}", read_toc(&fd));

    // let start = Msf::try_new(56, 15, 0).unwrap();
    // let end = Msf::try_new(56, 15, 2).unwrap();

    let command = ReadCd::<Mode2Form1, NoFields, C2Pointers, NoSubChannel>::new(
        251950.into(),
        1u8.into(),
        Control::default(),
    );
    // let expected_bytes: usize = command.response_len().try_into().unwrap();
    let mut buf = vec![0u8; 16384];

    let received = run_sgio(
        fd.as_raw_fd(),
        command.as_cdb().as_mut_slice(),
        buf.as_mut_slice(),
        DxferDirection::FromDev,
    )
    .unwrap();

    println!("Received: {received} bytes");
    let (sectors, _rem) = command.parse(&buf[0..(received as usize)]).unwrap();
    println!("Sectors: {}", sectors.len());
    println!();
    println!("Sector:");
    println!("{:?}", sectors[0]);

    // let mut q = [0u8; 12];
    // for (i, b) in sector.sub_channel().iter().enumerate() {
    //     q[i / 8] |= ((b >> 6) & 1) << (7 - (i % 8));
    // }
    // println!("Raw Q: {:?}", q);
    // println!("Calculated EDC: {}", sector.main_channel().calculate_edc());
    // println!("EDC matches?: {}", sector.main_channel().edc_matches());

    Ok(())
}
