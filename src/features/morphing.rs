use thiserror::Error;

use super::{Feature, FeatureCode, FeatureHeader};

const REQUIRED_VERSION: u8 = 0b0001;
const REQUIRED_ADD_LEN: u8 = 0x04;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Encountered invalid version `0b{0:04b}`, Feature 'Morphing' requires version `0b{ver:04b}`", ver = REQUIRED_VERSION)]
    InvalidVersion(u8),
    #[error("'Persistent' must be true for Feature 'Morphing'")]
    PersistentRequired,
    #[error("'Current' must be true for Feature 'Morphing'")]
    CurrentRequired,
    #[error("Encountered invalid Additional Length `0x{0:02X}`, Feature 'Morphing' requires version `0x{len:02X}`", len = REQUIRED_ADD_LEN)]
    InvalidAdditionalLength(u8),
    #[error("'OCEvent' must be true for Feature 'Morphing'")]
    OCEventRequired,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Morphing {
    /// The first four bytes of the generic Feature format. For the Morphing Feature, the following
    /// field must be set accordingly:
    ///
    /// - `version = 0b0001`
    /// - `persistent = true`
    /// - `current = true`
    /// - `additional_length = 0x04`
    pub header: FeatureHeader,
    /// Operational Change Request/Notification Class Events, should always be true
    pub oc_event: bool,
    /// When false, indicates that the Drive supports only the polling implementation of GET EVENT
    /// STATUS NOTIFICATION. When true, indicates that the Drive supports both polling and
    /// asynchronous GET EVENT STATUS NOTIFICATION. For ATAPI implementations, this should be false
    pub asynchronous: bool,
}

impl Feature<&[u8; 4]> for Morphing {
    const FEATURE_CODE: FeatureCode = FeatureCode::Morphing;

    type Error = Error;

    fn parse(header: FeatureHeader, data: &[u8; 4]) -> Result<Self, Self::Error> {
        const OC_EVENT_MASK: u8 = 0b00000010;
        const ASYNC_MASK: u8 = 0b00000001;

        if header.version != REQUIRED_VERSION {
            return Err(Error::InvalidVersion(header.version));
        }

        if !header.persistent {
            return Err(Error::PersistentRequired);
        }

        if !header.current {
            return Err(Error::CurrentRequired);
        }

        if header.additional_length != REQUIRED_ADD_LEN {
            return Err(Error::InvalidAdditionalLength(header.additional_length));
        }

        let oc_event = (data[0] & OC_EVENT_MASK) >> 1 != 0;

        if !oc_event {
            return Err(Error::OCEventRequired);
        }

        let asynchronous = (data[0] & ASYNC_MASK) != 0;

        Ok(Self {
            header,
            oc_event,
            asynchronous,
        })
    }
}
