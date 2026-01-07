use std::{fs::File, path::Path};

use crate::{
    addressing::Msf,
    commands::{
        get_configuration::{GetConfiguration, GetConfigurationResponse, RTField},
        toc::{FormattedTOC, Toc},
    },
    constants::CHROMADISC_VERSION,
    sgio::{DxferDirection, run_sgio},
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
    let display = device.display();

    let file = match File::open(device) {
        Err(err) => panic!("Failed to open {}: {}", display, err),
        Ok(file) => file,
    };

    println!("ChromaDisc version {}", CHROMADISC_VERSION);
    println!();

    let toc_cmd = FormattedTOC::<Msf>::new(0, 2048, 0);
    let toc_data = run_sgio(&file, toc_cmd, DxferDirection::FromDev).unwrap();
    let toc = <Toc<Msf> as TryFrom<Vec<u8>>>::try_from(toc_data).unwrap();

    println!("{toc}");
    println!();

    // let start = Lba::from(14300);
    // let sector_bytes = read_audio_range(&file, start, u24!(1)).unwrap();
    //
    // for byte in sector_bytes {
    //     print!("{:02X} ", byte);
    // }

    let config_cmd = GetConfiguration::new(RTField::All, 0, 8096, 0.into());
    let res: GetConfigurationResponse = run_sgio(&file, config_cmd, DxferDirection::FromDev)
        .unwrap()
        .try_into()
        .unwrap();

    println!("Current profile: {:?}", res.current_profile);

    for desc in res.descriptors {
        println!("Feature: {:#?}", desc.feature_data)
    }
}
