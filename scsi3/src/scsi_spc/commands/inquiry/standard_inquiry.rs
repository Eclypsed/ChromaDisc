use arbitrary_int::{u2, u3, u4, u5};
use bitfield::{Bits, BitsEnum, BitsRepr};
use derive_more::Debug;
use thiserror::Error;
use zerocopy::{CastError, FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

use super::{Inquiry, InquiryOpCode, version_descriptor::VersionDescriptor};

use crate::core::{
    ReadCommand, TruncationError,
    ascii::UnvalidatedSizedAsciiStr,
    util::{bit, debug_fields},
};

/// The length of the fields every response holds, through Product Revision Level.
const STANDARD_INQUIRY_MIN_BYTES: usize = 36;

#[derive(Debug, Error)]
pub enum StandardInquiryError {
    /// The error returned when attempting to parse fewer than 36 bytes into [`StandardInquiry`].
    ///
    /// The standard INQUIRY data shall contain at least 36 bytes according to SCSI-2 through SPC-7.
    #[error(transparent)]
    Truncated(#[from] TruncationError<STANDARD_INQUIRY_MIN_BYTES>),
}

impl ReadCommand<InquiryOpCode> for Inquiry<StandardInquiry> {
    type Len = u16;
    type Response<'a> = (&'a StandardInquiry, &'a [u8]);
    type Error = StandardInquiryError;

    fn response_len(&self) -> Self::Len {
        self.allocation_length
    }

    /// Reads `buf` as a response cut to Additional Length, and returns the bytes after it.
    ///
    /// `buf` should hold only the bytes the device server transferred. Any bytes past the
    /// transfer that still fall within Additional Length, such as an allocated buffer's unused
    /// tail, are read as part of the response.
    fn parse<'a>(&self, buf: &'a [u8]) -> Result<Self::Response<'a>, Self::Error> {
        // Additional Length counts the bytes following its own field, at byte 4. A value too
        // small to cover the first 36 bytes still leaves them readable. A `buf` too short to hold
        // it holds no additional bytes either, so 0 stands in, and the cast reports the
        // truncation.
        let additional_length = buf.get(4).copied().unwrap_or(0);
        let elems = (usize::from(additional_length) + 5)
            .saturating_sub(STANDARD_INQUIRY_MIN_BYTES)
            .min(buf.len().saturating_sub(STANDARD_INQUIRY_MIN_BYTES));

        match StandardInquiry::ref_from_prefix_with_elems(buf, elems) {
            Ok(res) => Ok(res),
            Err(CastError::Size(_)) => Err(TruncationError(buf.len()).into()),
            Err(CastError::Alignment(_)) => {
                unreachable!("`StandardInquiry` should be `Unaligned`")
            }
        }
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

/// Standard INQUIRY data: the response to INQUIRY with EVPD and CMDDT set to zero.
///
/// A response holds at least the first 36 bytes, through Product Revision Level. The fields after
/// those are optional, and their accessors return the part of them that's present, cut to
/// Additional Length.
///
/// The accessors read Response Data Format 2h, whatever [`Self::response_data_format`] holds.
// NOTE: As of spc7r05, AERC, TRMTSK, ACC, BQUE, MCHNGR, ACKREQQ, ADDR32, ADDR16, RELADR, WBUS32,
// WBUS16, SYNC, LINKED, TRANDIS, CLOCKING, QAS and IUS are obsolete. Their accessors are documented
// as obsolete and left out of `Debug`.
//
// AERC, TRMTSK, ACKREQQ, ADDR32, WBUS32 and TRANDIS went from obsolete back to reserved. This is
// the only place in SCSI where obsolete bits have been seen to return to reserved. None of them
// has been reassigned yet. If one is, its accessor here reads the new field.
// TODO: Find the revision that made each of these bits obsolete, and the one that made the six
// reserved.
#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct StandardInquiry {
    peripheral_byte: u8,
    byte1: u8,
    version: u8,
    byte3: u8,
    additional_length: u8,
    byte5: u8,
    byte6: u8,
    byte7: u8,
    t10_vendor_identification: UnvalidatedSizedAsciiStr<8>,
    product_identification: UnvalidatedSizedAsciiStr<16>,
    product_revision_level: UnvalidatedSizedAsciiStr<4>,
    /// Bytes 36 to n, not cut to Additional Length.
    additional_data: [u8],
}

impl StandardInquiry {
    #[inline]
    pub const fn peripheral_qualifier(&self) -> Bits<PeripheralQualifier> {
        Bits::new(u3::extract_u8(self.peripheral_byte, 5))
    }

    #[inline]
    pub const fn peripheral_device_type(&self) -> Bits<PeripheralDeviceType> {
        Bits::new(u5::extract_u8(self.peripheral_byte, 0))
    }

    bit!(
        /// Whether the medium is removable.
        #[doc(alias = "RMB")]
        removable_medium: byte1[7]
    );
    bit!(
        /// Whether the logical unit is a member of a logical unit conglomerate.
        #[doc(alias = "LU_CONG")]
        logical_unit_conglomerate: byte1[6]
    );

    #[inline]
    pub const fn hot_pluggable(&self) -> Bits<HotPluggable> {
        Bits::new(u2::extract_u8(self.byte1, 4))
    }

    #[inline]
    pub const fn version(&self) -> Bits<Version> {
        Bits::new(self.version)
    }

    bit!(
        /// Obsolete.
        #[doc(alias = "AERC")]
        asynchronous_event_reporting_capability: byte3[7]
    );
    bit!(
        /// Obsolete.
        #[doc(alias = "TRMTSK")]
        terminate_task: byte3[6]
    );
    // NOTE: SFF-8020i puts an ATAPI version in bits 7-4 of byte 3. The test drive reports 32h,
    // which reads either as NORMACA and HISUP set or as ATAPI version 3 (quirk 5 in
    // docs/quirks.md).
    // CONFIRM: Send the test drive TEST UNIT READY with NACA set in the CONTROL byte. A rejection
    // with INVALID FIELD IN CDB would contradict NORMACA set to one.
    bit!(
        /// Whether the device server supports a NACA bit set to one in the CONTROL byte.
        #[doc(alias = "NORMACA")]
        normal_aca_supported: byte3[5]
    );
    // NOTE: SPC-3 names this bit hierarchical support, set when the device server uses the
    // hierarchical addressing model to assign LUNs. SPC-5 and later name it historical support.
    // Which revision in between renamed it is unknown.
    bit!(
        /// Whether the SCSI target device uses the LUN structures described in SAM-6.
        #[doc(alias = "HISUP")]
        historical_support: byte3[4]
    );

    /// 2h. Values below 2h are obsolete, and values above it are reserved.
    ///
    /// The accessors read the 2h format whatever this field holds.
    // NOTE: SFF-8020i r2.6 requires a Response Data Format of 1h, with an ATAPI version of 2 in
    // bits 7-4, so an ATAPI device may report 1h here. Linux libata rewrites bytes 2-3 to 05h and
    // 32h whenever an ATAPI device reports a Version of 0 (`atapi_fixup_inquiry` in
    // drivers/ata/libata-scsi.c). The test drive reports 00h and 32h through a USB bridge (quirk 5
    // in docs/quirks.md).
    #[inline]
    pub const fn response_data_format(&self) -> u4 {
        u4::extract_u8(self.byte3, 0)
    }

    /// The length of the response that follows this field.
    ///
    /// Not cut by Allocation Length, so it can exceed the bytes the device server transferred.
    #[inline]
    pub const fn additional_length(&self) -> u8 {
        self.additional_length
    }

    bit!(
        /// Whether the SCSI target device contains an embedded storage array controller component.
        #[doc(alias = "SCCS")]
        scc_supported: byte5[7]
    );
    bit!(
        /// Obsolete.
        #[doc(alias = "ACC")]
        access_controls_coordinator: byte5[6]
    );

    #[doc(alias = "TPGS")]
    #[inline]
    pub const fn target_port_group_support(&self) -> Bits<TargetPortGroupSupport> {
        Bits::new(u2::extract_u8(self.byte5, 4))
    }

    bit!(
        /// Whether the SCSI target device supports third-party copy commands.
        #[doc(alias = "3PC")]
        third_party_copy: byte5[3]
    );
    bit!(
        /// Whether the logical unit supports protection information.
        protect: byte5[0]
    );
    bit!(
        /// Obsolete.
        #[doc(alias = "BQUE")]
        basic_queuing: byte6[7]
    );
    bit!(
        /// Whether the SCSI target device contains an embedded enclosure services component.
        #[doc(alias = "ENCSERV")]
        enclosure_services: byte6[6]
    );
    bit!(
        /// Whether the SCSI target device has more than one SCSI target port.
        #[doc(alias = "MULTIP")]
        multi_port: byte6[4]
    );
    bit!(
        /// Obsolete.
        #[doc(alias = "MCHNGR")]
        medium_changer: byte6[3]
    );
    bit!(
        /// Obsolete.
        #[doc(alias = "ACKREQQ")]
        ackreqq: byte6[2]
    );
    bit!(
        /// Obsolete.
        #[doc(alias = "ADDR32")]
        addr32: byte6[1]
    );
    bit!(
        /// Obsolete.
        #[doc(alias = "ADDR16")]
        addr16: byte6[0]
    );
    bit!(
        /// Obsolete.
        #[doc(alias = "RELADR")]
        relative_addressing: byte7[7]
    );
    bit!(
        /// Obsolete.
        #[doc(alias = "WBUS32")]
        wide_bus_32: byte7[6]
    );
    bit!(
        /// Obsolete.
        #[doc(alias = "WBUS16")]
        wide_bus_16: byte7[5]
    );
    bit!(
        /// Obsolete.
        #[doc(alias = "SYNC")]
        synchronous_transfer: byte7[4]
    );
    bit!(
        /// Obsolete.
        #[doc(alias = "LINKED")]
        linked_commands: byte7[3]
    );
    bit!(
        /// Obsolete.
        #[doc(alias = "TRANDIS")]
        transfer_disable: byte7[2]
    );
    bit!(
        /// Whether the logical unit supports the command management model.
        #[doc(alias = "CMDQUE")]
        command_queuing: byte7[1]
    );

    #[inline]
    pub const fn t10_vendor_identification(&self) -> &UnvalidatedSizedAsciiStr<8> {
        &self.t10_vendor_identification
    }

    #[inline]
    pub const fn product_identification(&self) -> &UnvalidatedSizedAsciiStr<16> {
        &self.product_identification
    }

    #[inline]
    pub const fn product_revision_level(&self) -> &UnvalidatedSizedAsciiStr<4> {
        &self.product_revision_level
    }

    /// Bytes 36-55, shorter if the response ends before byte 56.
    #[inline]
    pub const fn vendor_specific(&self) -> &[u8] {
        self.bytes(36, 56)
    }

    /// Obsolete. `None` if the response ends before byte 56.
    #[inline]
    pub const fn clocking(&self) -> Option<Bits<Clocking>> {
        match self.byte56() {
            Some(byte56) => Some(Bits::new(u2::extract_u8(byte56, 2))),
            None => None,
        }
    }

    /// Obsolete. `None` if the response ends before byte 56.
    #[doc(alias = "QAS")]
    #[inline]
    pub const fn quick_arbitration_and_selection(&self) -> Option<bool> {
        match self.byte56() {
            Some(byte56) => Some(byte56 & 1 << 1 != 0),
            None => None,
        }
    }

    /// Obsolete. `None` if the response ends before byte 56.
    #[doc(alias = "IUS")]
    #[inline]
    pub const fn information_units_supported(&self) -> Option<bool> {
        match self.byte56() {
            Some(byte56) => Some(byte56 & 1 << 0 != 0),
            None => None,
        }
    }

    /// The Version Descriptors, unused ones ([`VersionDescriptor::NOT_SUPPORTED`]) included.
    ///
    /// Fewer than 8 only if the response ends before byte 74. A Version Descriptor cut short is
    /// left out.
    pub fn version_descriptors(&self) -> &[VersionDescriptor] {
        match <[VersionDescriptor]>::ref_from_prefix(self.bytes(58, 74)) {
            Ok((version_descriptors, _cut_short)) => version_descriptors,
            Err(CastError::Size(_)) => unreachable!("an empty slice should fit any bytes"),
            Err(CastError::Alignment(_)) => {
                unreachable!("`VersionDescriptor` should be `Unaligned`")
            }
        }
    }

    /// Bytes 96 to n, empty if the response ends before byte 96.
    #[inline]
    pub const fn vendor_specific_parameters(&self) -> &[u8] {
        match self
            .additional()
            .split_at_checked(96 - STANDARD_INQUIRY_MIN_BYTES)
        {
            Some((_, vendor_specific_parameters)) => vendor_specific_parameters,
            None => &[],
        }
    }

    /// Bytes 36 to n, cut to Additional Length.
    ///
    /// Shorter than Additional Length only if the response itself was cut short.
    const fn additional(&self) -> &[u8] {
        let len = (self.additional_length as usize + 5).saturating_sub(STANDARD_INQUIRY_MIN_BYTES);
        match self.additional_data.split_at_checked(len) {
            Some((additional, _)) => additional,
            None => &self.additional_data,
        }
    }

    /// Bytes `start..end` of the response, or the part of them that's present.
    ///
    /// `start` is at least 36.
    const fn bytes(&self, start: usize, end: usize) -> &[u8] {
        let bytes = match self
            .additional()
            .split_at_checked(start - STANDARD_INQUIRY_MIN_BYTES)
        {
            Some((_, bytes)) => bytes,
            None => &[],
        };
        match bytes.split_at_checked(end - start) {
            Some((bytes, _)) => bytes,
            None => bytes,
        }
    }

    /// Byte 56, which holds CLOCKING, QAS and IUS.
    const fn byte56(&self) -> Option<u8> {
        match self.bytes(56, 57) {
            [byte56] => Some(*byte56),
            _ => None,
        }
    }
}

debug_fields!(StandardInquiry {
    peripheral_qualifier,
    peripheral_device_type,
    removable_medium,
    logical_unit_conglomerate,
    hot_pluggable,
    version,
    normal_aca_supported,
    historical_support,
    response_data_format,
    additional_length,
    scc_supported,
    target_port_group_support,
    third_party_copy,
    protect,
    enclosure_services,
    multi_port,
    command_queuing,
    t10_vendor_identification,
    product_identification,
    product_revision_level,
    vendor_specific,
    version_descriptors,
    vendor_specific_parameters,
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Control;

    fn command() -> Inquiry<StandardInquiry> {
        Inquiry::new(0xFF, Control::default())
    }

    /// The first 36 bytes of a CD/DVD device's response, followed by zeros.
    fn response<const N: usize>(additional_length: u8) -> [u8; N] {
        let mut buf = [0; N];
        // Connected CD/DVD device, RMB, SPC-4, Response Data Format 2h, CMDQUE
        buf[..8].copy_from_slice(&[0x05, 0x80, 0x06, 0x02, additional_length, 0, 0, 0x02]);
        buf[8..16].copy_from_slice(b"VENDOR  ");
        buf[16..32].copy_from_slice(b"PRODUCT         ");
        buf[32..36].copy_from_slice(b"1.00");
        buf
    }

    #[test]
    fn truncated_response_is_an_error() {
        assert!(matches!(
            command().parse(&[0; 35]),
            Err(StandardInquiryError::Truncated(TruncationError(35)))
        ));
    }

    #[test]
    fn response_data_format_other_than_2h_is_read() {
        let mut buf = response::<36>(31);
        for byte3 in [0x01, 0x13] {
            buf[3] = byte3;
            let (inquiry, _) = command().parse(&buf).unwrap();
            assert_eq!(inquiry.response_data_format(), u4::new(byte3 & 0x0F));
        }
    }

    #[test]
    fn minimum_response_has_no_optional_fields() {
        let buf = response::<36>(31);
        let (inquiry, rest) = command().parse(&buf).unwrap();
        assert!(rest.is_empty());

        assert_eq!(
            inquiry.peripheral_qualifier().get(),
            Ok(PeripheralQualifier::Connected)
        );
        assert_eq!(
            inquiry.peripheral_device_type().get(),
            Ok(PeripheralDeviceType::CdDvd)
        );
        assert!(inquiry.removable_medium());
        assert_eq!(inquiry.version().get(), Ok(Version::Spc4));
        assert_eq!(inquiry.response_data_format(), u4::new(2));
        assert!(inquiry.command_queuing());
        assert_eq!(inquiry.t10_vendor_identification().as_bytes(), b"VENDOR  ");
        assert_eq!(
            inquiry.product_identification().as_bytes(),
            b"PRODUCT         "
        );
        assert_eq!(inquiry.product_revision_level().as_bytes(), b"1.00");

        assert!(inquiry.vendor_specific().is_empty());
        assert!(inquiry.clocking().is_none());
        assert!(inquiry.quick_arbitration_and_selection().is_none());
        assert!(inquiry.information_units_supported().is_none());
        assert!(inquiry.version_descriptors().is_empty());
        assert!(inquiry.vendor_specific_parameters().is_empty());
    }

    #[test]
    fn parse_cuts_at_additional_length() {
        let mut buf = response::<99>(91);
        buf[36..56].fill(0xAA);
        buf[56] = 0b1111; // CLOCKING: ST and DT, QAS, IUS
        buf[58..62].copy_from_slice(&[0x00, 0x60, 0x04, 0x60]);
        buf[96..].fill(0xEE);

        let (inquiry, rest) = command().parse(&buf).unwrap();
        assert_eq!(rest, [0xEE; 3]);
        assert_eq!(inquiry.vendor_specific(), [0xAA; 20]);
        assert_eq!(
            inquiry.clocking().map(Bits::get),
            Some(Ok(Clocking::StAndDt))
        );
        assert_eq!(inquiry.quick_arbitration_and_selection(), Some(true));
        assert_eq!(inquiry.information_units_supported(), Some(true));
        assert_eq!(inquiry.version_descriptors().len(), 8);
        assert_eq!(
            inquiry.version_descriptors()[..3],
            [
                VersionDescriptor::SAM_3,
                VersionDescriptor::SPC_4,
                VersionDescriptor::NOT_SUPPORTED,
            ]
        );
        assert!(inquiry.vendor_specific_parameters().is_empty());

        // Additional Length covering the bytes after byte 95
        buf[4] = 94;
        let (inquiry, rest) = command().parse(&buf).unwrap();
        assert!(rest.is_empty());
        assert_eq!(inquiry.vendor_specific_parameters(), [0xEE; 3]);
    }

    #[test]
    fn cut_response_returns_what_is_present() {
        let mut buf = response::<61>(91);
        buf[36..56].fill(0xAA);
        buf[56] = 0b0010; // QAS
        buf[58..61].copy_from_slice(&[0x00, 0x60, 0x04]);

        let (inquiry, rest) = command().parse(&buf).unwrap();
        assert!(rest.is_empty());
        assert_eq!(inquiry.vendor_specific(), [0xAA; 20]);
        assert_eq!(
            inquiry.clocking().map(Bits::get),
            Some(Ok(Clocking::StOnly))
        );
        assert_eq!(inquiry.quick_arbitration_and_selection(), Some(true));
        assert_eq!(inquiry.information_units_supported(), Some(false));
        // The second Version Descriptor is cut short
        assert_eq!(inquiry.version_descriptors(), [VersionDescriptor::SAM_3]);
        assert!(inquiry.vendor_specific_parameters().is_empty());

        // Cut within the Vendor Specific bytes
        let (inquiry, _) = command().parse(&buf[..40]).unwrap();
        assert_eq!(inquiry.vendor_specific(), [0xAA; 4]);
        assert!(inquiry.clocking().is_none());
        assert!(inquiry.version_descriptors().is_empty());
    }

    #[test]
    fn additional_length_below_minimum_still_reads_the_first_36_bytes() {
        let mut buf = response::<40>(20);
        buf[36..].fill(0xEE);

        let (inquiry, rest) = command().parse(&buf).unwrap();
        assert_eq!(rest, [0xEE; 4]);
        assert_eq!(inquiry.additional_length(), 20);
        assert_eq!(inquiry.product_revision_level().as_bytes(), b"1.00");
        assert!(inquiry.vendor_specific().is_empty());
    }

    #[test]
    fn direct_cast_is_cut_to_additional_length() {
        let mut buf = response::<60>(31);
        buf[36..].fill(0xAA);

        let inquiry = StandardInquiry::ref_from_bytes(&buf).unwrap();
        assert!(inquiry.vendor_specific().is_empty());
        assert!(inquiry.clocking().is_none());
    }

    #[test]
    fn writes_reach_the_original_buffer() {
        let mut buf = response::<36>(31);

        // Patch in the obsolete Response Data Format 1h
        let inquiry = StandardInquiry::mut_from_bytes(&mut buf).unwrap();
        inquiry.as_mut_bytes()[3] = 0x01;

        assert_eq!(buf[3], 0x01);
        let (inquiry, _) = command().parse(&buf).unwrap();
        assert_eq!(inquiry.response_data_format(), u4::new(1));
    }
}
