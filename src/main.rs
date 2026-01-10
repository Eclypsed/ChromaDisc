use indicatif::ProgressBar;
use std::{fs::File, path::Path, time::Instant};

use i24::U24;

use crate::{
    addressing::Lba,
    commands::{
        Command,
        read_cd::SectorReader,
        read_track_info::{AddressType, ReadTrackInfo, ReadTrackInfoResponse},
        toc::FormattedTOC,
    },
    constants::CHROMADISC_VERSION,
};

mod addressing;
mod cdio;
mod commands;
mod constants;
mod error;
mod features;
mod sgio;

fn main() {
    let device = Path::new("/dev/cdrom");

    let file = match File::open(device) {
        Err(err) => panic!("Failed to open {}: {}", device.display(), err),
        Ok(file) => file,
    };

    println!("ChromaDisc version {}", CHROMADISC_VERSION);
    println!();

    let toc = FormattedTOC::<Lba>::new(0, 2048, 0).execute(&file).unwrap();

    let track_info: Vec<ReadTrackInfoResponse> = (toc.first_track_num..=toc.last_track_num)
        .map(|n| ReadTrackInfo::new(false, AddressType::LTN, n.into(), 0.into()).execute(&file))
        .collect::<Result<Vec<ReadTrackInfoResponse>, _>>()
        .unwrap();

    for track in track_info {
        let start = track.logical_track_start_addr;
        let end = track.logical_track_start_addr + track.logical_track_size - 1;

        let sectors = U24::try_from_u32(end - start).unwrap();

        let bar = ProgressBar::new(sectors.to_u32().into());

        let start_lba = Lba::try_from(i32::try_from(start).unwrap()).unwrap();
        let reader = SectorReader::new(&file, start_lba, sectors);

        println!("Reading Track {}", track.logical_track_number);
        let now = Instant::now();

        for res in reader {
            let (_, remain) = res.unwrap();
            let read = sectors - remain;
            bar.set_position(read.into());
        }

        let elapsed_time = now.elapsed();

        bar.finish();
        println!(
            "Reading Track {} took {:.3} seconds",
            track.logical_track_number,
            elapsed_time.as_secs_f32()
        )
    }
}
