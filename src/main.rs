use std::{fs::File, io, path::Path};

use crate::{
    addressing::{Lba, Msf},
    constants::CHROMADISC_VERSION,
    toc::{TOCData, read_toc},
};

mod addressing;
mod cdb;
mod cdio;
mod constants;
mod sgio;
mod toc;

fn main() -> io::Result<()> {
    let device = Path::new("/dev/cdrom");
    let display = device.display();

    let file = match File::open(device) {
        Err(err) => panic!("Failed to open {}: {}", display, err),
        Ok(file) => file,
    };

    let toc_cdb = TOCData::<Msf>::new(0, 2048, 0);
    let toc = read_toc(&file, toc_cdb)?;

    println!("ChromaDisc version {}", CHROMADISC_VERSION);
    println!();
    println!("TOC of the disc");
    println!(
        "\t {:^5} | {:^8} | {:^8} | {:^11} | {:^9} ",
        "Track", "Start", "Length", "Start (LBA)", "End (LBA)"
    );
    println!("\t{}", "-".repeat(55));

    for window in toc.track_descriptors.windows(2) {
        let (cur, next) = (&window[0], &window[1]);

        let start_lba = Lba::from(cur.start_addr);
        let end_lba = Lba::from(next.start_addr) - Lba::from(1);
        let length = next.start_addr - cur.start_addr;

        println!(
            "\t {:^5} | {:^8} | {:^8} | {:^11} | {:^9} ",
            format!("{:2}", cur.number),
            cur.start_addr,
            length,
            format!("{:6}", start_lba),
            format!("{:6}", end_lba)
        );
    }

    Ok(())
}
