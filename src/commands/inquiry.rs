use std::str::Utf8Error;

use thiserror::Error;

use super::{Command, Control};

const MIN_RESPONSE_LENGTH: usize = 36;

#[derive(Debug, Error)]
pub enum Error {
    #[error("INQUIRY Response must be at least {size} bytes long, received {0}", size = MIN_RESPONSE_LENGTH)]
    IncompleteResponse(usize),
    #[error(transparent)]
    InvalidASCIISequence(#[from] Utf8Error),
}

#[derive(Debug)]
pub struct Inquiry {
    evpd: bool,
    page_code: u8,
    allocation_length: u16,
    control: Control,
}

impl Inquiry {
    pub fn new(evpd: bool, page_code: u8, control: Control) -> Self {
        Self {
            evpd,
            page_code,
            allocation_length: MIN_RESPONSE_LENGTH as u16,
            control,
        }
    }
}

impl Command<6> for Inquiry {
    const OP_CODE: u8 = 0x12;

    type Response = InquiryResponse;

    fn as_cdb(&self) -> [u8; 6] {
        let mut bytes = [0u8; 6];

        bytes[0] = Self::OP_CODE;
        bytes[1] |= u8::from(self.evpd);
        bytes[2] = self.page_code;
        bytes[3] = (self.allocation_length >> 8) as u8;
        bytes[4] = self.allocation_length as u8;
        bytes[5] = self.control.into();

        bytes
    }

    fn allocation_len(&self) -> usize {
        self.allocation_length.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PeripheralQualifier {
    /// A peripheral device having the specified peripheral device type is connected to this
    /// logical unit. If the device server is unable to determine whether or not a peripheral
    /// device is connected, it also shall use this peripheral qualifier. This peripheral qualifier
    /// does not mean that the peripheral device connected to the logical unit is ready for access.
    ConnectedOrUnknown = 0b000,
    NotConnectedButSupported = 0b001,
    Reserved = 0b010,
    NotSupported = 0b011,
    VendorSpecific(u8),
}

impl From<u8> for PeripheralQualifier {
    fn from(value: u8) -> Self {
        match value & 0b111 {
            0b000 => Self::ConnectedOrUnknown,
            0b001 => Self::NotConnectedButSupported,
            0b010 => Self::Reserved,
            0b011 => Self::NotSupported,
            v @ 0b100..=0b111 => Self::VendorSpecific(v),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PeripheralDeviceType {
    DirectAccessBlockDevice = 0x00, // e.g. Magnetic disk
    SequentialAccessDevice = 0x01,  // e.g. Magnetic tape
    PrinterDevice = 0x02,
    ProcessorDevice = 0x03,
    WriteOnceDevice = 0x04, // e.g. Some optical disks
    CDOrDVDDevice = 0x05,
    ScannerDevice = 0x06,        // obsolete
    OpticalMemoryDevice = 0x07,  // e.g. Some optical disks
    MediumChangerDevice = 0x08,  // e.g. Jukeboxes
    CommunicationsDevice = 0x09, // obsolete
    Obsolete(u8),
    StorageArrayControllerDevice = 0x0C, // e.g. RAID
    EnclosureServicesDevice = 0x0D,
    SimplifiedDirectAccessDevice = 0x0E, // e.g. Magnetic disk
    OpticalCardReaderWriterDevice = 0x0F,
    BridgeControllerCommands = 0x10,
    ObjectBasedStorageDevice = 0x11,
    AutomationDriveInterface = 0x12,
    Reserved(u8),
    WellKnownLogicalUnit = 0x1E,
    UnknownOrNoDeviceType = 0x1F,
}

impl From<u8> for PeripheralDeviceType {
    fn from(value: u8) -> Self {
        match value & 0x1F {
            0x00 => Self::DirectAccessBlockDevice,
            0x01 => Self::SequentialAccessDevice,
            0x02 => Self::PrinterDevice,
            0x03 => Self::ProcessorDevice,
            0x04 => Self::WriteOnceDevice,
            0x05 => Self::CDOrDVDDevice,
            0x06 => Self::ScannerDevice,
            0x07 => Self::OpticalMemoryDevice,
            0x08 => Self::MediumChangerDevice,
            0x09 => Self::CommunicationsDevice,
            v @ 0x0A..=0x0B => Self::Obsolete(v),
            0x0C => Self::StorageArrayControllerDevice,
            0x0D => Self::EnclosureServicesDevice,
            0x0E => Self::SimplifiedDirectAccessDevice,
            0x0F => Self::OpticalCardReaderWriterDevice,
            0x10 => Self::BridgeControllerCommands,
            0x11 => Self::ObjectBasedStorageDevice,
            0x12 => Self::AutomationDriveInterface,
            v @ 0x13..=0x1D => Self::Reserved(v),
            0x1E => Self::WellKnownLogicalUnit,
            0x1F => Self::UnknownOrNoDeviceType,
            _ => unreachable!(),
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Version {
    /// The device does not claim conformance to any standard.
    NoConformance = 0x00,
    /// The device complies to ANSI INCITS 301-1997 (SPC).
    SPC = 0x03,
    /// The device complies to ANSI INCITS 351-2001 (SPC-2).
    SPC2 = 0x04,
    /// The device complies to ANSI INCITS 408-2005 (SPC-3).
    SPC3 = 0x05,
    /// The device complies to ANSI INCITS 513-2015 (SPC-4)
    SPC4 = 0x06,
    /// The device complies to T10/BSR INCITS 503 (SPC-6)
    SPC6 = 0x07,
    Obselete(u8),
    Reserved(u8),
}

impl From<u8> for Version {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::NoConformance,
            0x03 => Self::SPC,
            0x04 => Self::SPC2,
            0x05 => Self::SPC3,
            0x06 => Self::SPC4,
            0x07 => Self::SPC6,
            v @ (0x01
            | 0x02
            | 0x08..=0x0C
            | 0x40..=0x44
            | 0x48..=0x4C
            | 0x80..=0x84
            | 0x88..=0x8C) => Self::Obselete(v),
            v @ (0x0D..=0x3F | 0x45..=0x47 | 0x4D..=0x7F | 0x85..=0x87 | 0x8D..=0xFF) => {
                Self::Reserved(v)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TargetPortGroupSupport {
    NotSupported = 0b00,
    ImplicitOnly = 0b01,
    ExplicitOnly = 0b10,
    ImplicitAndExplicit = 0b11,
}

impl From<u8> for TargetPortGroupSupport {
    fn from(value: u8) -> Self {
        match value & 0b11 {
            0b00 => Self::NotSupported,
            0b01 => Self::ImplicitOnly,
            0b10 => Self::ExplicitOnly,
            0b11 => Self::ImplicitAndExplicit,
            _ => unreachable!(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct InquiryResponse {
    pub peripheral_qualifier: PeripheralQualifier,
    pub peripheral_device_type: PeripheralDeviceType,
    pub removable_media: bool,
    pub version: Version,
    pub normal_aca: bool,
    pub hierarchical_support: bool,
    pub response_data_format: u8,
    pub scc_supported: bool,
    pub access_controls_coordinator: bool,
    pub target_port_group_support: TargetPortGroupSupport,
    pub third_party_copy: bool,
    pub protect: bool,
    pub enclosure_services: bool,
    pub multi_port: bool,
    pub command_queuing: bool,
    /// The 8 ASCII character Vendor ID registered with T10.
    ///
    /// Valid ASCII characters are codes 0x21 through 0x7E. Vendor ID's are left aligned and may be
    /// padded on the end with spaces (0x20) if the VendorID is shorter than 8 characters. This
    /// padding is trimmed during parsing.
    ///
    /// Vendor ID assignments available at [T10 Vendor ID Assignments](https://www.t10.org/lists/vid-alph.htm)
    pub t10_vendor_identification: String,
    /// The 16 ASCII characters of left-aligned Product Indentification defined by the vendor.
    /// Trailing whitespace is trimmed.
    pub product_identification: String,
    /// The 4 ASCII characters of left-aligned Product Revision data defined by the vendor.
    /// Trailing whitespace is trimmed.
    pub product_revision_level: String,
    // These fields bring us to byte 36 of the response, the minimum size the INQUIRY command
    // should recieve. There are potential additional fields after this but virtually all of them
    // are vendor specific and not particularly useful.
}

impl TryFrom<Vec<u8>> for InquiryResponse {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let res_len = value.len();
        if res_len < MIN_RESPONSE_LENGTH {
            return Err(Error::IncompleteResponse(res_len));
        }

        let peripheral_qualifier = PeripheralQualifier::from((value[0] & 0b11100000) >> 5);
        let peripheral_device_type = PeripheralDeviceType::from(value[0] & 0b00011111);
        let removable_media = (value[1] & 0b10000000) != 0;
        let version = Version::from(value[2]);
        let normal_aca = (value[3] & 0b00100000) != 0;
        let hierarchical_support = (value[3] & 0b00010000) != 0;
        let response_data_format = value[3] & 0b00001111;
        // Additional Length available in value[4], but we only read the first 36 bytes
        let scc_supported = (value[5] & 0b10000000) != 0;
        let access_controls_coordinator = (value[5] & 0b01000000) != 0;
        let target_port_group_support = TargetPortGroupSupport::from((value[5] & 0b00110000) >> 4);
        let third_party_copy = (value[5] & 0b00001000) != 0;
        let protect = (value[5] & 0b00000001) != 0;
        let enclosure_services = (value[6] & 0b01000000) != 0;
        let multi_port = (value[6] & 0b00010000) != 0;
        let command_queuing = (value[7] & 0b00000010) != 0;
        let t10_vendor_identification = str::from_utf8(&value[8..=15])?.trim_end().to_string();
        let product_identification = str::from_utf8(&value[16..=31])?.trim_end().to_string();
        let product_revision_level = str::from_utf8(&value[32..=35])?.trim_end().to_string();

        Ok(Self {
            peripheral_qualifier,
            peripheral_device_type,
            removable_media,
            version,
            normal_aca,
            hierarchical_support,
            response_data_format,
            scc_supported,
            access_controls_coordinator,
            target_port_group_support,
            third_party_copy,
            protect,
            enclosure_services,
            multi_port,
            command_queuing,
            t10_vendor_identification,
            product_identification,
            product_revision_level,
        })
    }
}
