use thiserror::Error;

use super::{Feature, FeatureCode, FeatureHeader};

const REQUIRED_VERSION: u8 = 0b0010;
const REQUIRED_ADD_LEN: usize = 8;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Encountered invalid version `0b{0:04b}`, Feature 'Core' requires version `0b{ver:04b}`", ver = REQUIRED_VERSION)]
    InvalidVersion(u8),
    #[error("'Persistent' must be true for Feature 'Core'")]
    PersistentRequired,
    #[error("'Current' must be true for Feature 'Core'")]
    CurrentRequired,
    #[error("Encountered invalid Additional Length `{0}`, Feature 'CORE' requires version `{len}`", len = REQUIRED_ADD_LEN)]
    InvalidAdditionalLength(u8),
    #[error("Unknown Physical Interface Standard: 0x{0:08X}")]
    UnknownPhysicalInterfaceStandard(u32),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Core {
    pub header: FeatureHeader,
    /// The field that specifies what kind of physical interface the drive is using.
    pub physical_interface_standard: PhysicalInterfaceStandard,
    /// Permits the Drive to indicate support for certain features of the INQUIRY command. If INQ2
    /// is true, the Drive shall support validation of EVPD, Page Code, and the 16-bit Allocation
    /// Lenght fields as described in [SPC-3].
    pub inq2: bool,
    /// The Device Busy Event. Should be set to true; false is legacy though may still be reported.
    pub dbe: bool,
}

impl Feature<&[u8; 8]> for Core {
    const FEATURE_CODE: FeatureCode = FeatureCode::Core;

    type Error = Error;

    fn parse(header: FeatureHeader, bytes: &[u8; 8]) -> Result<Self, Self::Error> {
        const INQ2_MASK: u8 = 0b00000010;
        const DBE_MASK: u8 = 0b00000001;

        if header.version != REQUIRED_VERSION {
            return Err(Error::InvalidVersion(header.version));
        }

        if !header.persistent {
            return Err(Error::PersistentRequired);
        }

        if !header.current {
            return Err(Error::CurrentRequired);
        }

        let physical_interface_standard =
            PhysicalInterfaceStandard::try_from(u32::from_be_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3],
            ]))?;

        let inq2 = (bytes[4] & INQ2_MASK) >> 1 != 0;
        // Not doing a check for DBE because even tho DBE = 0 is legacy some drives may still
        // report it. Legacy features are not errors
        let dbe = (bytes[4] & DBE_MASK) != 0;

        Ok(Core {
            header,
            physical_interface_standard,
            inq2,
            dbe,
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
