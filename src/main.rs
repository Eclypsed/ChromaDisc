use chrono::Local;
use std::{fs::File, path::Path};

use crate::{
    addressing::{Lba, Msf},
    commands::{
        Command,
        inquiry::Inquiry,
        read_track_info::{AddressType, ReadTrackInfo, ReadTrackInfoResponse},
        toc::FormattedTOC,
    },
    constants::{CHROMADISC_VERSION, FRAMES_PER_MINUTE, FRAMES_PER_SECOND},
};

mod addressing;
mod cdio;
mod commands;
mod constants;
mod error;
mod features;
mod sgio;

fn print_toc(tracks: &[ReadTrackInfoResponse]) {
    println!("TOC of the extracted CD");
    println!(
        "\t {:^5} | {:^8} | {:^8} | {:^11} | {:^9} ",
        "Track", "Start", "Length", "Start (LBA)", "End (LBA)"
    );
    println!("\t{}", "-".repeat(55));

    for track in tracks {
        let start = Lba::try_from(track.logical_track_start_addr).unwrap();
        let length = track.logical_track_size;
        let end = Lba::try_from(i32::from(start) + i32::try_from(length).unwrap() - 1).unwrap();

        let mut frames = length;
        let minutes = frames / FRAMES_PER_MINUTE as u32;
        frames -= minutes * FRAMES_PER_MINUTE as u32;
        let seconds = frames / FRAMES_PER_SECOND as u32;
        frames -= seconds * FRAMES_PER_SECOND as u32;

        println!(
            "\t {:^5} | {:^8} | {:^8} | {:^11} | {:^9} ",
            format!("{:2}", track.logical_track_number),
            Msf::from(start),
            format!(
                "{:2}:{:02}:{:02}",
                minutes as u8, seconds as u8, frames as u8
            ),
            format!("{:6}", start),
            format!("{:6}", end)
        );
    }
}

fn main() {
    let device = Path::new("/dev/cdrom");

    let file = match File::open(device) {
        Err(err) => panic!("Failed to open {}: {}", device.display(), err),
        Ok(file) => file,
    };

    println!("ChromaDisc version {}", CHROMADISC_VERSION);
    println!();

    let timestamp = Local::now();
    println!("ChromaDisc extraction logfile from {timestamp}");
    println!();

    let inquiry = Inquiry::new(false, 0, 0.into()).execute(&file).unwrap();
    println!(
        "Used drive : {} {} (revision {})",
        inquiry.t10_vendor_identification,
        inquiry.product_identification,
        inquiry.product_revision_level
    );
    println!();

    let toc = FormattedTOC::<Lba>::new(0, 2048, 0).execute(&file).unwrap();

    let track_info: Vec<ReadTrackInfoResponse> = (toc.first_track_num..=toc.last_track_num)
        .map(|n| ReadTrackInfo::new(false, AddressType::LTN, n.into(), 0.into()).execute(&file))
        .collect::<Result<Vec<ReadTrackInfoResponse>, _>>()
        .unwrap();

    print_toc(&track_info);
    println!();
}
