use num_enum::{IntoPrimitive, TryFromPrimitive};
use thiserror::Error;

use super::{Feature, FeatureCode, FeatureHeader};

const REQUIRED_VERSION: u8 = 0b0000;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Encountered invalid version `0b{0:04b}`, Feature 'Profile List' requires version `0b{ver:04b}`", ver = REQUIRED_VERSION)]
    InvalidVersion(u8),
    #[error("'Persistent' must be true for Feature 'Profile List'")]
    PersistentRequired,
    #[error("'Current' must be true for Feature 'Profile List'")]
    CurrentRequired,
    #[error("Invalid profile number: 0x{0:04X}")]
    InvalidProfile(u16),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ProfileList {
    pub header: FeatureHeader,
    /// The list of all profile descriptors in the order of preferred operation.
    pub profile_descriptors: Vec<ProfileDescriptor>,
}

impl Feature<&[u8]> for ProfileList {
    const FEATURE_CODE: FeatureCode = FeatureCode::ProfileList;

    type Error = Error;

    fn parse(header: FeatureHeader, bytes: &[u8]) -> Result<Self, Self::Error> {
        if header.version != REQUIRED_VERSION {
            return Err(Error::InvalidVersion(header.version));
        }

        if !header.persistent {
            return Err(Error::PersistentRequired);
        }

        if !header.current {
            return Err(Error::CurrentRequired);
        }

        let profile_descriptors = bytes
            .chunks_exact(4)
            .map(|b| ProfileDescriptor::try_from(<&[u8; 4]>::try_from(b).unwrap()))
            .collect::<Result<Vec<ProfileDescriptor>, _>>()?;

        Ok(ProfileList {
            header,
            profile_descriptors,
        })
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ProfileDescriptor {
    /// The identity of a Profile to which the Drive conforms.
    profile_number: Profile,
    /// When true, indicates that this Profile is currently active. If no medium is present then no
    /// descriptor should be active.
    current_profile: bool,
}

impl TryFrom<&[u8; 4]> for ProfileDescriptor {
    type Error = Error;

    fn try_from(value: &[u8; 4]) -> Result<Self, Self::Error> {
        const CURRENT_MASK: u8 = 0b00000001;

        let profile_number = Profile::try_from_primitive(u16::from_be_bytes([value[0], value[1]]))?;
        let current_profile = value[2] & CURRENT_MASK != 0;

        Ok(Self {
            profile_number,
            current_profile,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::InvalidProfile))]
#[repr(u16)]
pub enum Profile {
    /// Re-writable; with removable media
    RemoveableDisk = 0x0002,
    /// Read only Compact Disc capable
    CDrom = 0x0008,
    /// Write once Compact Disc capable
    CDr = 0x0009,
    /// Re-writable Compact Disc capable
    CDrw = 0x000A,
    /// Read only DVD
    DVDrom = 0x0010,
    /// Write once DVD using Sequential recording
    DVDrSequential = 0x0011,
    /// Re-writable DVD
    DVDram = 0x0012,
    /// Re-recordable DVD using Restricted Overwrite
    DVDrwRestricted = 0x0013,
    /// Re-recordable DVD using Sequential recording
    DVDrwSequential = 0x0014,
    /// Dual Layer DVD-R using Sequential recording
    DVDrDualSequential = 0x0015,
    /// Dual Layer DVD-R using Layer Jump recording
    DVDrDualJump = 0x0016,
    /// Dual Layer DVD-RW recording
    DVDrwDual = 0x0017,
    /// Write once DVD for CSS managed recording
    DVDDownload = 0x0018,
    /// DVD+ReWritable
    DVDPlusrw = 0x001A,
    /// DVD+Recordable
    DVDPlusr = 0x001B,
    /// DVD+Rewritable Dual Layer
    DVDPlusrwDual = 0x002A,
    /// DVD+Recordable Dual Layer
    DVDPlusrDual = 0x002B,
    /// Blu-ray Disc ROM
    BDrom = 0x0040,
    /// Blu-ray Disc Recordable – Sequential Recording Mode
    BDrSRM = 0x0041,
    /// Blu-ray Disc Recordable – Random Recording Mode
    BDrRRM = 0x0042,
    /// Blu-ray Disc Rewritable
    BDre = 0x0043,
    /// Read-only HD DVD
    HDDVDrom = 0x0050,
    /// Write-once HD DVD
    HDDVDr = 0x0051,
    /// Rewritable HD DVD
    HDDVDram = 0x0052,
    /// Rewritable HD DVD
    HDDVDrw = 0x0053,
    /// Dual Layer Write-once HD DVD
    HDDVDrDual = 0x0058,
    /// Dual Layer Rewritable HD DVD
    HDDVDrwDual = 0x005A,
    /// Drive does not conform to any Profile
    NonConforming = 0xFFFF,
}
