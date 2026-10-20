use std::{
    io,
    os::fd::{AsRawFd, OwnedFd},
};

use scsi3::{
    core::{Command, Control, ReadCommand, util::Present},
    mmc::{
        commands::{
            get_configuration::GetConfiguration,
            read_cd::{
                Transfer,
                c2::C2ErrorInfo,
                main_channel::{MainChannel, SectorMode, SelectionOf},
                sub_channel::SubChannelSelection,
            },
            read_toc_pma_atip::ReadTocPmaAtip,
        },
        features::FeatureCode,
    },
    spc::commands::inquiry::{Inquiry, standard_inquiry::StandardInquiry},
};
use zerocopy::IntoBytes;

use crate::{
    device::{Drive, scan_sysfs},
    transport::sgio::{DxferDirection, run_sgio},
};

pub mod device;
pub mod transport;

#[allow(dead_code)]
fn print_inquiry(fd: &OwnedFd) {
    let inquiry_cmd = Inquiry::<StandardInquiry>::new(200, Control::default());
    let mut inq_buf = vec![0u8; inquiry_cmd.response_len().into()];

    let inq_received = run_sgio(
        fd.as_raw_fd(),
        inquiry_cmd.as_cdb().as_mut_slice(),
        inq_buf.as_mut_slice(),
        DxferDirection::FromDev,
    )
    .unwrap();

    println!("Byte 3: {:0X}h", inq_buf[3]);

    let (inquiry, _rest) = inquiry_cmd.parse(&inq_buf[..inq_received]).unwrap();
    println!("INQUIRY:\n{:#?}", inquiry);
}

#[allow(dead_code)]
fn print_toc(fd: &OwnedFd) {
    let toc_cmd = ReadTocPmaAtip::atip(1024 * 8, Control::default());
    let mut toc_buf = vec![0u8; toc_cmd.response_len().into()];

    let _toc_received = run_sgio(
        fd.as_raw_fd(),
        toc_cmd.as_cdb().as_mut_slice(),
        toc_buf.as_mut_slice(),
        DxferDirection::FromDev,
    )
    .unwrap();

    let (toc, _rest) = toc_cmd.parse(&toc_buf).unwrap();
    println!("TOC:\n{:#?}", toc);
}

#[allow(dead_code)]
fn print_configuration(fd: &OwnedFd) {
    let cmd = GetConfiguration::all(FeatureCode::from_u16(0), 8096, Control::default());
    let mut buf = vec![0u8; cmd.response_len().into()];

    let _received = run_sgio(
        fd.as_raw_fd(),
        cmd.as_cdb().as_mut_slice(),
        buf.as_mut_slice(),
        DxferDirection::FromDev,
    )
    .unwrap();

    let (config, _rest) = cmd.parse(&buf).unwrap();
    println!("CONFIGURATION:\n{:#?}", config);
}

// This works but is far less efficient than simply interpreting the data in a different
// order i.e. Having a separate zc struct that has c2 after subchannel
#[allow(dead_code)]
fn normalize_c2_subchannel_ordering_quirk<M, L, C, S>(transfer: &mut Transfer<M, L, C, S>)
where
    M: SectorMode,
    L: SelectionOf<M>,
    C: C2ErrorInfo<C2Presence = Present>,
    S: SubChannelSelection<SubChannelPresence = Present>,
{
    transfer.as_mut_bytes()[size_of::<MainChannel<M, L>>()..]
        .rotate_left(size_of::<S::SubChannelData>());
}

fn main() -> io::Result<()> {
    println!("ChromaDisc version {}", env!("CARGO_PKG_VERSION"));
    println!();

    let devices = scan_sysfs()?;

    println!("Discovered Devices:");
    for device in &devices {
        println!("{device}")
    }
    println!();

    let drive = Drive::new(devices[0].clone());
    let fd = drive.get_fd()?;

    // print_inquiry(&fd);
    print_toc(&fd);
    // print_configuration(&fd);

    // TODO: Probably move this to a test
    // let mut q = [0u8; 12];
    // for (i, b) in sector.sub_channel().iter().enumerate() {
    //     q[i / 8] |= ((b >> 6) & 1) << (7 - (i % 8));
    // }
    // println!("Raw Q: {:?}", q);
    // println!("Calculated EDC: {}", sector.main_channel().calculate_edc());
    // println!("EDC matches?: {}", sector.main_channel().edc_matches());

    Ok(())
}
