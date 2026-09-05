use arbitrary_int::{u2, u3, u4, u5};
use bitfield::{Bits, BitsEnum, BitsRepr};
use thiserror::Error;
use tinystr::UnvalidatedTinyAsciiStr;

use super::{Inquiry, InquiryOpCode};

use crate::core::{util::Bit, ReadCommand, TruncationError};

const STANDARD_INQUIRY_MIN_BYTES: usize = 36;
const RESPONSE_DATA_FORMAT: u4 = u4::new(0x2);
const MAX_VERSION_DESCRIPTORS: usize = 8;

#[derive(Debug, Error)]
pub enum StandardInquiryError {
    /// The error returned when attempting to parse fewer than 36 bytes into
    /// [`StandardInquiry`](StandardInquiry).
    ///
    /// The standard INQUIRY data shall contain at least 36 bytes according to SCSI-2 through SPC-7.
    #[error(transparent)]
    Truncated(#[from] TruncationError<STANDARD_INQUIRY_MIN_BYTES>),
    /// The error returned when a RESPONSE DATA FORMAT vlaue other tha 2h is encountered. Codes
    /// less than 2h are obsolete as of SCSI-2 and are not supported by the library. Codes 3h-Fh
    /// are reserved and will result in a breaking data structure change should they every be
    /// allocated.
    #[error("Invalid Response Data Format value `{0:X}`, expected {RESPONSE_DATA_FORMAT:X}")]
    InvalidResponseDataFormat(u4),
}

impl ReadCommand<InquiryOpCode> for Inquiry<StandardInquiry> {
    type Len = u16;
    type Response<'a> = StandardInquiry;
    type Error = StandardInquiryError;

    fn response_len(&self) -> Self::Len {
        self.allocation_length
    }

    fn parse<'a>(&self, buf: &'a [u8]) -> Result<Self::Response<'a>, Self::Error> {
        if buf.len() < STANDARD_INQUIRY_MIN_BYTES {
            return Err(TruncationError::<STANDARD_INQUIRY_MIN_BYTES>(buf.len()).into());
        }

        // Not sure if there is really anything I can do with this other than use it to make an
        // additional "bytes left" return value.
        let _additional_length: usize = buf[4].into();

        let response_data_format = u4::extract_u8(buf[3], 0);
        if response_data_format != RESPONSE_DATA_FORMAT {
            return Err(StandardInquiryError::InvalidResponseDataFormat(
                response_data_format,
            ));
        }

        Ok(StandardInquiry {
            peripheral_qualifier: Bits::<PeripheralQualifier>::new(u3::extract_u8(buf[0], 5)),
            peripheral_device_type: Bits::<PeripheralDeviceType>::new(u5::extract_u8(buf[0], 0)),
            rmb: buf[1].bit(7),
            lu_cong: buf[1].bit(6),
            hot_pluggable: Bits::<HotPluggable>::new(u2::extract_u8(buf[1], 4)),
            version: Bits::<Version>::new(buf[2]),
            aerc: buf[3].bit(7),
            trm_tsk: buf[3].bit(6),
            norm_aca: buf[3].bit(5),
            hi_sup: buf[3].bit(4),
            sccs: buf[5].bit(7),
            acc: buf[5].bit(6),
            tpgs: Bits::<TargetPortGroupSupport>::new(u2::extract_u8(buf[5], 4)),
            third_party_copy: buf[5].bit(3),
            protect: buf[5].bit(0),
            basic_queuing: buf[6].bit(7),
            enclosure_services: buf[6].bit(6),
            multi_port: buf[6].bit(4),
            medium_changer: buf[6].bit(3),
            ackreqq: buf[6].bit(2),
            addr32: buf[6].bit(1),
            addr16: buf[6].bit(0),
            relative_addressing: buf[7].bit(7),
            wbus32: buf[7].bit(6),
            wbus16: buf[7].bit(5),
            sync: buf[7].bit(4),
            linked: buf[7].bit(3),
            transfer_disable: buf[7].bit(2),
            command_queuing: buf[7].bit(1),
            vendor_identification: UnvalidatedTinyAsciiStr::try_from_utf8(&buf[8..=15]).unwrap(),
            product_identification: UnvalidatedTinyAsciiStr::try_from_utf8(&buf[16..=31]).unwrap(),
            product_revision_level: UnvalidatedTinyAsciiStr::try_from_utf8(&buf[32..=35]).unwrap(),
            // Beyond this point the length is unvalidated, use get() not indexing
            clocking: buf
                .get(56)
                .map(|b| Bits::<Clocking>::new(u2::extract_u8(*b, 2))),
            qas: buf.get(56).map(|b| b.bit(1)),
            ius: buf.get(56).map(|b| b.bit(0)),
            version_descriptors: buf
                .get(58..)
                .unwrap_or_default()
                .chunks_exact(2)
                .take(MAX_VERSION_DESCRIPTORS)
                .map(|b| u16::from_be_bytes([b[0], b[1]]))
                .collect(),
        })
    }
}

/// SCSI peripheral qualifier (3 bits, from the INQUIRY data).
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, BitsEnum)]
#[bits(3, repr = u3, reserved = 0b010)]
pub enum PeripheralQualifier {
    /// 000b - the specified peripheral device type is currently connected to this
    /// logical unit, or the device server can't tell whether it is. Does not imply
    /// the device is ready for access.
    Connected = 0b000,
    /// 001b - the device server supports this peripheral device type on this logical
    /// unit, but no physical device is currently connected.
    NotConnected = 0b001,
    /// 011b - the device server cannot support a physical device on this logical unit.
    /// The peripheral device type shall be 1Fh; all other values are reserved here.
    Unsupported = 0b011,
    #[bits(alt = 0b101..=0b111)]
    VendorSpecific = 0b100,
}

/// SCSI PERIPHERAL DEVICE TYPE field (5 bits, INQUIRY byte 0 bits 4:0).
/// Codes per SPC-5 Table 150.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, BitsEnum)]
#[bits(5, repr = u5, reserved = 0x06 | 0x09..=0x0B | 0x10 | 0x15..=0x1D)]
pub enum PeripheralDeviceType {
    /// 00h, SBC-5 - direct access block device (e.g. magnetic disk).
    DirectAccessBlock = 0x00,
    /// 01h, SSC-5 - sequential access device (e.g. magnetic tape).
    SequentialAccess = 0x01,
    /// 02h, SSC - Printer device.
    Printer = 0x02,
    /// 03h, SPC-2 - processor device.
    Processor = 0x03,
    /// 04h, SBC - Write-once device (e.g., some optical disks).
    WriteOnce = 0x04,
    /// 05h, MMC-6 - CD/DVD device.
    CdDvd = 0x05,
    /// 07h, SBC - optical memory device (e.g. some optical disks).
    OpticalMemory = 0x07,
    /// 08h, SMC-3 - media changer device (e.g. jukeboxes).
    MediaChanger = 0x08,
    /// 0Ch, SCC-2 - storage array controller device (e.g. RAID).
    StorageArrayController = 0x0C,
    /// 0Dh, SES-2 - enclosure services device.
    EnclosureServices = 0x0D,
    /// 0Eh, RBC - simplified direct access device (e.g. magnetic disk).
    SimplifiedDirectAccess = 0x0E,
    /// 0Fh, OCRW - optical card reader/writer device.
    OpticalCardReaderWriter = 0x0F,
    /// 11h, OSD-2 - object-based storage device.
    ObjectBasedStorage = 0x11,
    /// 12h, ADC-3 - automation/drive interface.
    AutomationDriveInterface = 0x12,
    /// 13h - Security manager device.
    SecurityManager = 0x13,
    /// 14h, ZBC-3 - host managed zoned block device.
    HostManagedZonedBlock = 0x14,
    /// 1Eh - well known logical unit. All well known logical units use this type.
    WellKnownLogicalUnit = 0x1E,
    /// 1Fh - unknown or no device type.
    Unknown = 0x1F,
}

/// SCSI HOT PLUGGABLE field (2 bits, INQUIRY byte 1 bits 5:4).
/// Codes per SPC-5 Table 151.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, BitsEnum)]
#[bits(2, repr = u2, reserved = 0b11)]
pub enum HotPluggable {
    /// 00b - no information is provided regarding whether the SCSI target device
    /// is hot pluggable.
    NoInformation = 0b00,
    /// 01b - the SCSI target device is designed to be removed from a SCSI domain as
    /// a single object (concurrent removal of its target ports, logical units, and all
    /// other contained objects) while that domain keeps operating for its other target
    /// devices, if any.
    Removable = 0b01,
    /// 10b - the SCSI target device is not designed to be removed from a SCSI domain
    /// while that domain continues to operate.
    NotRemovable = 0b10,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, BitsEnum)]
#[bits(2, repr = u2, reserved = 0b11)]
pub enum IsoVersion {
    /// This device does not claim conformance to any ISO/IEC standard
    NoConformance = 0b00,
    /// ISO/IEC 9316:1989
    Scsi = 0b01,
    /// ISO/IEC 9316:1995
    Scsi2 = 0b10,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, BitsEnum)]
#[bits(3, repr = u3, reserved = 0b010..=0b111)]
pub enum EcmaVersion {
    /// This device does not claim conformance to any ECMA standard
    NoConformance = 0b000,
    /// ECMA-111
    Scsi = 0b001,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Version {
    /// This device does not claim conformance to any ANSI standard.
    NoConformance { iso: IsoVersion, ecma: EcmaVersion },
    /// ANSI INCITS 131-1986
    Scsi { iso: IsoVersion, ecma: EcmaVersion },
    /// ANSI INCITS 131-1994
    Scsi2 { iso: IsoVersion, ecma: EcmaVersion },
    /// ANSI INCITS 301-1997
    Spc { iso: IsoVersion, ecma: EcmaVersion },
    /// ANSI INCITS 351-2001
    Spc2 { iso: IsoVersion, ecma: EcmaVersion },
    /// ANSI INCITS 408-2005
    Spc3,
    /// ANSI INCITS 513-2015
    Spc4,
    /// ANSI INCITS 502-2019
    Spc5,
    /// ANSI INCITS 566-2025
    Spc6,
    /// ANSI INCITS 586-202x (Currently in active development)
    Spc7,
}

impl BitsRepr for Version {
    type Bits = u8;
    const FIELD: &'static str = "Version";
    const BITS: u32 = u8::BITS;

    fn from_bits(bits: Self::Bits) -> Option<Self> {
        Some(match bits {
            // Starting from SPC-2 revision 9, the version field is a flat code.
            // SPC-3 is the revision without the possibility of an ISO/IEC or ECMA subfield
            0x05 => Self::Spc3,
            0x06 => Self::Spc4,
            0x07 => Self::Spc5,
            0x0D => Self::Spc6,
            0x0E => Self::Spc7,
            _ => {
                let iso = IsoVersion::from_bits(u2::extract_u8(bits, 6))?;
                let ecma = EcmaVersion::from_bits(u3::extract_u8(bits, 3))?;

                match bits & 0b111 {
                    0b000 => Self::NoConformance { iso, ecma },
                    0b001 => Self::Scsi { iso, ecma },
                    0b010 => Self::Scsi2 { iso, ecma },
                    0b011 => Self::Spc { iso, ecma },
                    0b100 => Self::Spc2 { iso, ecma },
                    _ => return None,
                }
            }
        })
    }

    fn to_bits(self) -> Self::Bits {
        fn iso_ecma_bitmask(iso: IsoVersion, ecma: EcmaVersion) -> u8 {
            (iso.to_bits().value() << 6) | (ecma.to_bits().value() << 3)
        }

        match self {
            Self::NoConformance { iso, ecma } => iso_ecma_bitmask(iso, ecma),
            Self::Scsi { iso, ecma } => iso_ecma_bitmask(iso, ecma) | 0b001,
            Self::Scsi2 { iso, ecma } => iso_ecma_bitmask(iso, ecma) | 0b010,
            Self::Spc { iso, ecma } => iso_ecma_bitmask(iso, ecma) | 0b011,
            Self::Spc2 { iso, ecma } => iso_ecma_bitmask(iso, ecma) | 0b100,
            Self::Spc3 => 0x05,
            Self::Spc4 => 0x06,
            Self::Spc5 => 0x07,
            Self::Spc6 => 0x0D,
            Self::Spc7 => 0x0E,
        }
    }
}

/// TPGS field — what form of asymmetric logical unit access (ALUA) the logical unit supports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, BitsEnum)]
#[bits(2, repr = u2)]
pub enum TargetPortGroupSupport {
    /// The logical unit does not support asymmetric logical unit access, or supports a form of
    /// asymmetric access that is vendor specific. Neither REPORT TARGET PORT GROUPS nor SET
    /// TARGET PORT GROUPS is supported.
    Unsupported = 0b00,
    /// Only implicit asymmetric logical unit access is supported (see 5.18.2.9). The logical unit
    /// is capable of changing target port asymmetric access states without a SET TARGET PORT
    /// GROUPS command. REPORT TARGET PORT GROUPS is supported; SET TARGET PORT GROUPS is not.
    Implicit = 0b01,
    /// Only explicit asymmetric logical unit access is supported (see 5.18.2.10). The logical unit
    /// changes target port asymmetric access states only as requested by a SET TARGET PORT GROUPS
    /// command. Both REPORT TARGET PORT GROUPS and SET TARGET PORT GROUPS are supported.
    Explicit = 0b10,
    /// Both explicit and implicit asymmetric logical unit access are supported. Both REPORT TARGET
    /// PORT GROUPS and SET TARGET PORT GROUPS are supported.
    Both = 0b11,
}

/// SCSI CLOCKING field (2 bits). ST = single transition, DT = double transition.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, BitsEnum)]
#[bits(2, repr = u2, reserved = 0b10)]
pub enum Clocking {
    /// 00b — the device server supports only ST.
    StOnly = 0b00,
    /// 01b — the device server supports only DT.
    DtOnly = 0b01,
    /// 11b — the device server supports both ST and DT.
    StAndDt = 0b11,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandardInquiry {
    pub peripheral_qualifier: Bits<PeripheralQualifier>,
    pub peripheral_device_type: Bits<PeripheralDeviceType>,
    pub rmb: bool,
    pub lu_cong: bool,
    pub hot_pluggable: Bits<HotPluggable>,
    // byte 1 bits 3:0 reserved
    pub version: Bits<Version>,
    pub aerc: bool,
    pub trm_tsk: bool,
    pub norm_aca: bool,
    pub hi_sup: bool,
    // Response Data Format (2h) - if not 2 this will be an error
    pub sccs: bool,
    pub acc: bool,
    pub tpgs: Bits<TargetPortGroupSupport>,
    pub third_party_copy: bool,
    // byte 5 bits 2:1 reserved
    pub protect: bool,
    pub basic_queuing: bool,
    pub enclosure_services: bool,
    // byte 6 bit 5 vendor specific
    pub multi_port: bool,
    pub medium_changer: bool,
    pub ackreqq: bool,
    pub addr32: bool,
    pub addr16: bool,
    pub relative_addressing: bool,
    pub wbus32: bool,
    pub wbus16: bool,
    pub sync: bool,
    pub linked: bool,
    pub transfer_disable: bool,
    pub command_queuing: bool,
    // byte 7 bit 0 vendor specific
    pub vendor_identification: UnvalidatedTinyAsciiStr<8>,
    pub product_identification: UnvalidatedTinyAsciiStr<16>,
    pub product_revision_level: UnvalidatedTinyAsciiStr<4>,
    // bytes 36:55 reserved
    // byte 56 bit 7:4 reserved
    pub clocking: Option<Bits<Clocking>>,
    pub qas: Option<bool>,
    pub ius: Option<bool>,
    // byte 57 reserved
    // All version descriptors will be included, including 0000h. If there are fewer than 8
    // descriptors that means the response was truncated.
    pub version_descriptors: Vec<u16>,
    // byte 74:95 reserved
    // byte 96:n vendor specific
}
