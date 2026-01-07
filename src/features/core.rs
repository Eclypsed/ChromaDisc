use thiserror::Error;

use super::{FeatureCode, FeatureData, FeatureDataError};

const DATA_LEN: usize = 8;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Received {received} bytes of Core Feature data, expected {expected}", expected = DATA_LEN)]
    LengthMismatch { received: usize },
    #[error("Unknown Physical Interface Standard: 0x{0:08X}")]
    UnknownPhysicalInterfaceStandard(u32),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Core {
    physical_interface_standard: PhysicalInterfaceStandard,
    inq2: bool,
}

impl FeatureData for Core {
    const FEATURE_CODE: FeatureCode = FeatureCode::Core;

    fn parse(bytes: &[u8]) -> Result<Self, FeatureDataError> {
        const INQ2_MASK: u8 = 0b00000010;

        let num_bytes = bytes.len();

        if num_bytes != DATA_LEN {
            return Err(Error::LengthMismatch {
                received: num_bytes,
            }
            .into());
        }

        let physical_interface_standard =
            PhysicalInterfaceStandard::try_from(u32::from_be_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3],
            ]))?;
        let inq2 = (bytes[4] & INQ2_MASK) >> 1 != 0;

        Ok(Core {
            physical_interface_standard,
            inq2,
        })
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalInterfaceStandard {
    Unspecified,
    SCSI,
    ATAPI,
    IEEE1394_1995,
    IEEE1394A,
    FibreChannel,
    IEEE1394B,
    SerialATAPI,
    USB,
    VendorUnique,
    INCITS(u32),
    SFF(u32),
    IEEE(u32),
}

impl TryFrom<u32> for PhysicalInterfaceStandard {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x00000000 => Ok(Self::Unspecified),
            0x00000001 => Ok(Self::SCSI),
            0x00000002 => Ok(Self::ATAPI),
            0x00000003 => Ok(Self::IEEE1394_1995),
            0x00000004 => Ok(Self::IEEE1394A),
            0x00000005 => Ok(Self::FibreChannel),
            0x00000006 => Ok(Self::IEEE1394B),
            0x00000007 => Ok(Self::SerialATAPI),
            0x00000008 => Ok(Self::USB),
            0x0000FFFF => Ok(Self::VendorUnique),
            0x00010000..=0x0001FFFF => Ok(Self::INCITS(value)),
            0x00020000..=0x0002FFFF => Ok(Self::SFF(value)),
            0x00030000..=0x0003FFFF => Ok(Self::IEEE(value)),
            _ => Err(Error::UnknownPhysicalInterfaceStandard(value)),
        }
    }
}

impl From<PhysicalInterfaceStandard> for u32 {
    fn from(value: PhysicalInterfaceStandard) -> Self {
        match value {
            PhysicalInterfaceStandard::Unspecified => 0x00000000,
            PhysicalInterfaceStandard::SCSI => 0x00000001,
            PhysicalInterfaceStandard::ATAPI => 0x00000002,
            PhysicalInterfaceStandard::IEEE1394_1995 => 0x00000003,
            PhysicalInterfaceStandard::IEEE1394A => 0x00000004,
            PhysicalInterfaceStandard::FibreChannel => 0x00000005,
            PhysicalInterfaceStandard::IEEE1394B => 0x00000006,
            PhysicalInterfaceStandard::SerialATAPI => 0x00000007,
            PhysicalInterfaceStandard::USB => 0x00000008,
            PhysicalInterfaceStandard::VendorUnique => 0x0000FFFF,
            PhysicalInterfaceStandard::INCITS(v)
            | PhysicalInterfaceStandard::SFF(v)
            | PhysicalInterfaceStandard::IEEE(v) => v,
        }
    }
}
