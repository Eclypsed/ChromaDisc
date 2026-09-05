use std::{
    io,
    os::fd::{AsRawFd, OwnedFd},
};

use scsi3::{
    core::{addressing::Lba, Command, Control, ReadCommand},
    mmc::commands::{
        read_cd::{
            cd_da::CdDa, mode1::Mode1, mode2_form1::Mode2Form1, C2ErrorInfo, MainChannelSelection,
            ReadCd, SubChannelSelection,
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

// #[allow(dead_code)]
// fn print_toc(tracks: &[ReadTrackInfoResponse]) {
//     println!("TOC of the extracted CD");
//     println!(
//         "\t {:^5} | {:^8} | {:^8} | {:^11} | {:^9} ",
//         "Track", "Start", "Length", "Start (LBA)", "End (LBA)"
//     );
//     println!("\t{}", "-".repeat(55));
//
//     for track in tracks {
//         let start = Lba::try_from(track.logical_track_start_addr).unwrap();
//         let length = track.logical_track_size;
//         let end = start + i32::try_from(length).unwrap() - 1;
//
//         let mut frames = length;
//         let minutes = frames / FRAMES_PER_MINUTE as u32;
//         frames -= minutes * FRAMES_PER_MINUTE as u32;
//         let seconds = frames / FRAMES_PER_SECOND as u32;
//         frames -= seconds * FRAMES_PER_SECOND as u32;
//
//         println!(
//             "\t {:^5} | {:^8} | {:^8} | {:^11} | {:^9} ",
//             format!("{:2}", track.logical_track_number),
//             Msf::from(start),
//             format!(
//                 "{:2}:{:02}:{:02}",
//                 minutes as u8, seconds as u8, frames as u8
//             ),
//             format!("{:6}", start),
//             format!("{:6}", end)
//         );
//     }
// }

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

    let command = ReadCd::<
        Lba,
        Mode2Form1<{ MainChannelSelection::SUB_HEADER | MainChannelSelection::USER_DATA }>,
    >::new(
        252000.into(),
        1u8.into(),
        C2ErrorInfo::None,
        SubChannelSelection::None,
        Control::default(),
    );
    let mut buf = vec![0u8; 4000];

    let received = run_sgio(
        fd.as_raw_fd(),
        command.as_cdb().as_mut_slice(),
        buf.as_mut_slice(),
        DxferDirection::FromDev,
    )
    .unwrap();

    println!("Received: {received} bytes");

    // println!("Raw TOC:");
    // println!("{result:#?}");

    // let timestamp = Local::now();
    // println!("ChromaDisc extraction logfile from {timestamp}");
    // println!();
    //
    // let config_cmd = GetConfiguration::new(RTField::All, 0, 8096, 0.into());
    //
    // let res = execute(config_cmd, fd.as_raw_fd()).unwrap();
    //
    // println!("{:#?}", res)

    Ok(())
}
