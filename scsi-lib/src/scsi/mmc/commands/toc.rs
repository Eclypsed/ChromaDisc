use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::marker::PhantomData;
use thiserror::Error;

use crate::core::addressing::{Address, AddressError, Lba, Msf};
use crate::scsi::mmc::types::q_subchannel;

use super::{Command, Control};

const TOC_HEADER_LEN: usize = 4;

#[derive(Debug, Error)]
pub enum Error<Addr: TOCAddr> {
    #[error(transparent)]
    InvalidAddress(#[from] AddressError<Addr>),
    #[error("Encountered invalid ADR {0:04b}")]
    InvalidAdr(u8),
    #[error("Received {0} bytes of READ TOC response, expected at least {min}", min = TOC_HEADER_LEN)]
    IncompleteHeader(usize),
}

pub trait TOCAddr: Address {
    const MSF_FLAG: bool;

    fn from_be_bytes(bytes: &[u8; 4]) -> Result<Self, AddressError<Self>>;
}

impl TOCAddr for Lba {
    const MSF_FLAG: bool = false;

    fn from_be_bytes(bytes: &[u8; 4]) -> Result<Self, AddressError<Self>> {
        i32::from_be_bytes(*bytes).try_into()
    }
}

impl TOCAddr for Msf {
    const MSF_FLAG: bool = true;

    fn from_be_bytes(bytes: &[u8; 4]) -> Result<Self, AddressError<Self>> {
        Msf::new(bytes[1], bytes[2], bytes[3])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u8)]
pub enum Format {
    FormattedTOC = 0b0000,
    MultiSessionInfo = 0b0001,
    RawTOC = 0b0010,
    Pma = 0b0011,
    Atip = 0b0100,
    CDText = 0b0101,
}

pub trait TOCCommand: Command<10> {
    const FORMAT: Format;
    const MSF_FLAG: bool;
}

#[derive(Debug)]
pub struct FormattedTOC<Addr: TOCAddr> {
    track: u8,
    allocation_len: u16,
    control: Control,
    _msf: PhantomData<Addr>,
}

impl<Addr> FormattedTOC<Addr>
where
    Addr: TOCAddr,
{
    pub fn new(track: u8, allocation_len: u16, control: Control) -> Self {
        FormattedTOC {
            track,
            allocation_len,
            control,
            _msf: PhantomData,
        }
    }
}

impl<Addr> Command<10> for FormattedTOC<Addr>
where
    Addr: TOCAddr,
{
    const OP_CODE: u8 = 0x43;

    type Response = Toc<Addr>;

    fn as_cdb(&self) -> [u8; 10] {
        let mut bytes = [0u8; 10];

        bytes[0] = Self::OP_CODE;
        bytes[1] |= u8::from(Self::MSF_FLAG) << 1;
        bytes[2] |= u8::from(Self::FORMAT) & 0xF;
        bytes[6] = self.track;
        bytes[7] = (self.allocation_len >> 8) as u8;
        bytes[8] = self.allocation_len as u8;
        bytes[9] = self.control.into();

        bytes
    }

    fn allocation_len(&self) -> usize {
        self.allocation_len.into()
    }
}

impl<Addr> TOCCommand for FormattedTOC<Addr>
where
    Addr: TOCAddr,
{
    const FORMAT: Format = Format::FormattedTOC;
    const MSF_FLAG: bool = Addr::MSF_FLAG;
}

#[derive(Debug)]
pub struct TrackDescriptor<Addr: TOCAddr> {
    /// The type of information encoded in the Q Sub-channel of the block where this TOC entry was found
    pub adr: q_subchannel::Adr,
    /// Indicates the attributes of the track.
    pub control: q_subchannel::Control,
    pub number: u8,
    pub start_addr: Addr,
}

#[derive(Debug)]
pub struct Toc<Addr: TOCAddr> {
    // length: u16,
    pub first_track_num: u8,
    pub last_track_num: u8,
    pub track_descriptors: Vec<TrackDescriptor<Addr>>,
}

impl<Addr> TryFrom<Vec<u8>> for Toc<Addr>
where
    Addr: TOCAddr,
{
    type Error = Error<Addr>;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() < 4 {
            return Err(Error::IncompleteHeader(value.len()));
        }

        // let length = u16::from_be_bytes([value[0], value[1]]);
        let first_track_num = value[2];
        let last_track_num = value[3];

        let num_tracks: usize = (last_track_num - first_track_num).into();
        let mut track_descriptors: Vec<TrackDescriptor<Addr>> = Vec::with_capacity(num_tracks);

        for descriptor in value[4..].chunks_exact(8) {
            let adr_bits = (descriptor[1] & 0xF0) >> 4;
            let adr = q_subchannel::Adr::try_from_primitive(adr_bits)
                .map_err(|_| Error::<Addr>::InvalidAdr(adr_bits))?;
            let control = q_subchannel::Control::from_bits_truncate(descriptor[1] & 0x0F);
            let track_num = descriptor[2];

            let start_addr: Addr = Addr::from_be_bytes(&descriptor[4..=7].try_into().unwrap())?;

            track_descriptors.push(TrackDescriptor {
                adr,
                control,
                number: track_num,
                start_addr,
            });
        }

        Ok(Toc {
            // length,
            first_track_num,
            last_track_num,
            track_descriptors,
        })
    }
}
