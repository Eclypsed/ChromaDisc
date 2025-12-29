use bitflags::bitflags;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fs::File;
use std::io;
use std::marker::PhantomData;
use std::os::fd::AsRawFd;

use crate::addressing::{Address, Lba, Msf};
use crate::cdb::Cdb;
use crate::sgio::{DxferDirection, SG_INFO_CHECK, SgIoHeader, ioctl_sg_io};

pub trait TOCAddr: Address {
    const MSF_FLAG: bool;

    fn from_be_bytes(bytes: &[u8; 4]) -> Self;
}

impl TOCAddr for Lba {
    const MSF_FLAG: bool = false;

    fn from_be_bytes(bytes: &[u8; 4]) -> Self {
        i32::from_be_bytes(*bytes).into()
    }
}

impl TOCAddr for Msf {
    const MSF_FLAG: bool = true;

    fn from_be_bytes(bytes: &[u8; 4]) -> Self {
        Msf::new_unchecked(bytes[1], bytes[2], bytes[3])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Format {
    FormattedTOC = 0b0000,
    MultiSessionInfo = 0b0001,
    RawTOC = 0b0010,
    Pma = 0b0011,
    Atip = 0b0100,
    CDText = 0b0101,
}

pub trait TOCCdb: Cdb<10> {
    const FORMAT: Format;
    const MSF_FLAG: bool;

    type ResponseData: TOCResponse;

    fn allocation_len(&self) -> u16;
}

pub trait TOCResponse: Sized {
    fn parse(bytes: &[u8]) -> io::Result<Self>;
}

#[derive(Debug)]
pub struct FormattedTOC<Addr: TOCAddr> {
    track: u8,
    allocation_len: u16,
    control: u8,
    _msf: PhantomData<Addr>,
}

impl<Addr> FormattedTOC<Addr>
where
    Addr: TOCAddr,
{
    pub fn new(track: u8, allocation_len: u16, control: u8) -> Self {
        FormattedTOC {
            track,
            allocation_len,
            control,
            _msf: PhantomData,
        }
    }
}

impl<Addr> Cdb<10> for FormattedTOC<Addr>
where
    Addr: TOCAddr,
{
    const OP_CODE: u8 = 0x43;

    fn to_bytes(&self) -> [u8; 10] {
        let mut bytes = [0u8; 10];

        bytes[0] = Self::OP_CODE;
        bytes[1] |= u8::from(Self::MSF_FLAG) << 1;
        bytes[2] |= u8::from(Self::FORMAT) & 0xF;
        bytes[6] = self.track;
        bytes[7..=8].copy_from_slice(&self.allocation_len.to_be_bytes());
        bytes[9] = self.control;

        bytes
    }
}

impl<Addr> TOCCdb for FormattedTOC<Addr>
where
    Addr: TOCAddr,
{
    const FORMAT: Format = Format::FormattedTOC;
    const MSF_FLAG: bool = Addr::MSF_FLAG;

    type ResponseData = Toc<Addr>;

    fn allocation_len(&self) -> u16 {
        self.allocation_len
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Control: u8 {
        /// If set and track is audio, the track has 4 channels, otherwise 2. Not set when track is data.
        ///
        /// # Examples
        ///
        /// - 00xx = 2 audio channels
        /// - 10xx = 4 audio channels
        const FOUR_CHANNELS = 1 << 3;
        /// If set, track is data, otherwise audio.
        ///
        /// # Examples
        ///
        /// - x0xx = Audio Track
        /// - 01xx = Data Track
        const IS_DATA = 1 << 2;
        /// If set digital copy is permitted, otherwise prohibited.
        ///
        /// # Examples
        ///
        /// - xx0x = Digital copy prohibited
        /// - xx1x = Digital copy permitted
        const COPY_PERMITTED = 1 << 1;
        /// If set and track is audio, pre-emphasis is enabled. If set and track is data, track is recorded incrementally, otherwise uninterrupted.
        ///
        /// # Examples
        ///
        /// - x0x0 = Audio without pre-emphasis
        /// - x0x1 = Audio with pre-emphasis
        /// - 01x0 = Data track recorded uninterrupted
        /// - 01x1 = Data track recorded incrementally
        const PREEMPHASIS_OR_INCREMENTAL = 1 << 0;
    }
}

#[derive(Debug)]
pub struct TrackDescriptor<Addr: TOCAddr> {
    /// The type of information encoded in the Q Sub-channel of the block where this TOC entry was found
    #[allow(dead_code)]
    pub adr: u8,
    /// Indicates the attributes of the track.
    #[allow(dead_code)]
    pub control: Control,
    pub number: u8,
    pub start_addr: Addr,
}

#[derive(Debug)]
pub struct Toc<Addr: TOCAddr> {
    #[allow(dead_code)]
    pub length: u16,
    #[allow(dead_code)]
    pub first_track_num: u8,
    #[allow(dead_code)]
    pub last_track_num: u8,
    pub track_descriptors: Vec<TrackDescriptor<Addr>>,
}

impl<Addr> TOCResponse for Toc<Addr>
where
    Addr: TOCAddr,
{
    fn parse(bytes: &[u8]) -> io::Result<Self> {
        if bytes.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "TOC must be at least four bytes long",
            ));
        }

        let length = u16::from_be_bytes([bytes[0], bytes[1]]);
        let first_track_num = bytes[2];
        let last_track_num = bytes[3];

        let mut track_descriptors: Vec<TrackDescriptor<Addr>> = vec![];

        for descriptor in bytes[4..].chunks_exact(8) {
            let adr = (descriptor[1] & 0xF0) >> 4;
            let control = Control::from_bits_truncate(descriptor[1] & 0x0F);
            let track_num = descriptor[2];

            let start_addr: Addr = Addr::from_be_bytes(&descriptor[4..=7].try_into().unwrap());

            track_descriptors.push(TrackDescriptor {
                adr,
                control,
                number: track_num,
                start_addr,
            });
        }

        Ok(Toc {
            length,
            first_track_num,
            last_track_num,
            track_descriptors,
        })
    }
}

pub fn read_toc<Cdb: TOCCdb>(file: &File, cdb: Cdb) -> io::Result<Cdb::ResponseData> {
    let mut cdb_bytes = cdb.to_bytes();

    let mut data = vec![0u8; cdb.allocation_len().into()];
    let mut sense = [0u8; 64];

    let mut hdr = SgIoHeader::new(
        DxferDirection::FromDev,
        &mut cdb_bytes,
        &mut data,
        &mut sense,
    );

    unsafe {
        ioctl_sg_io(file.as_raw_fd(), &mut hdr)?;
    }

    if hdr.info & SG_INFO_CHECK != 0 {
        return Err(io::Error::other("SG_IO check failed"));
    }

    // TODO: Change error checking implementation. This section is temporarily borrowed from:
    // https://github.com/bloomca/rust-cd-da-reader

    // Check SCSI status
    if hdr.status != 0 {
        let error_msg = match hdr.status {
            0x02 => "Check Condition",
            0x08 => "Busy",
            0x18 => "Reservation Conflict",
            0x28 => "Task Set Full",
            0x30 => "ACA Active",
            0x40 => "Task Aborted",
            _ => "Unknown SCSI error",
        };

        // If there's sense data, parse it for more details
        if hdr.sb_len_wr > 0 {
            let sense_key = sense[2] & 0x0F;
            let asc = sense[12]; // Additional Sense Code
            let ascq = sense[13]; // Additional Sense Code Qualifier

            return Err(io::Error::other(format!(
                "SCSI error: {} (status=0x{:02x}, sense_key=0x{:x}, asc=0x{:02x}, ascq=0x{:02x})",
                error_msg, hdr.status, sense_key, asc, ascq
            )));
        } else {
            return Err(io::Error::other(format!("SCSI error: {}", error_msg)));
        }
    }

    // From the SCSI HOWTO: "In practice it only reports underruns (i.e. positive number) as data
    // overruns should never happen"
    let residual: usize = hdr.resid.try_into().map_err(|_| {
        io::Error::other(format!(
            "SCSI error: Received invalid residual {}",
            hdr.resid
        ))
    })?;

    if data.len() <= residual {
        return Err(io::Error::other(format!(
            "SCSI error: Received residual ({}) >= allocation length ({})",
            residual,
            cdb.allocation_len()
        )));
    }

    Cdb::ResponseData::parse(&data[0..(data.len() - residual)])
}
