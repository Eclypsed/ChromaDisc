use std::io;

use scsi_lib::{
    core::constants::CHROMADISC_VERSION,
    device::{scan_sysfs, Drive},
    scsi::mmc::commands::read_toc_pma_atip::{atip::Atip, raw_toc::RawToc, ReadTocPmaAtip},
    // scsi::mmc::commands::{
    //     execute,
    //     get_configuration::{GetConfiguration, RTField},
    //     read_track_info::ReadTrackInfoResponse,
    // },
};

mod cdio;

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

    let cmd = ReadTocPmaAtip::<Atip>::new(0, 4096, 0.into());

    let result = drive.execute(cmd).unwrap();

    println!("Raw TOC:");
    println!("{result:#?}");

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
