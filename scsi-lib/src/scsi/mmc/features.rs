use std::fmt::Debug;

use bitflags::bitflags;
use i24::U24;
use thiserror::Error;

use crate::core::util::BitReader;

use super::types::{LoadingMechanism, PhysicalInterfaceStandard, Profile};

#[derive(Debug, Error)]
pub enum FeatureError {
    #[error("Feature descriptor must be at least 4 bytes")]
    DescriptorSize,
    #[error("Feature Descriptor specified {expected} bytes of feature data, received {received}")]
    MissingData { expected: usize, received: usize },
    #[error(
        "Feature can only have {expected} bytes of feature data, Descriptor specified {received}"
    )]
    DataSize {
        expected: parsing::DataSize,
        received: usize,
    },
}

const HEADER_LEN: usize = 4;

#[derive(Debug)]
struct FeatureHeader {
    feature_code: u16,
    pub version: u8,
    pub persistent: bool,
    pub current: bool,
    additional_length: u8,
}

impl FeatureHeader {
    fn parse(bytes: &[u8; HEADER_LEN]) -> Self {
        let flags = BitReader(bytes[2]);

        FeatureHeader {
            feature_code: u16::from_be_bytes(bytes[0..2].try_into().unwrap()),
            version: (bytes[2] & 0b00111100) >> 2,
            persistent: flags.bit(0b00000010),
            current: flags.bit(0b00000001),
            additional_length: bytes[3],
        }
    }
}

trait HasFeatureHeader {
    fn header(&self) -> &FeatureHeader;
}

macro_rules! impl_feature_header {
    ($t:ty) => {
        impl HasFeatureHeader for $t {
            fn header(&self) -> &FeatureHeader {
                &self.header
            }
        }
    };
}

#[allow(private_bounds)]
pub trait MmcFeature: HasFeatureHeader + Debug {
    fn version(&self) -> u8 {
        self.header().version
    }

    fn persistent(&self) -> bool {
        self.header().persistent
    }

    fn current(&self) -> bool {
        self.header().current
    }
}

impl<T: HasFeatureHeader + Debug> MmcFeature for T {}

/// A 4-byte block representing a specific Profile.
///
/// See MMC-6 §5.3.1, Table 94.
#[derive(Debug)]
pub struct ProfileDescriptor {
    pub profile_number: Profile,
    pub current_p: bool,
}

impl ProfileDescriptor {
    const LEN: usize = 4;

    fn parse(value: &[u8; Self::LEN]) -> Self {
        let flags = BitReader(value[2]);

        Self {
            profile_number: Profile::from(u16::from_be_bytes([value[0], value[1]])),
            current_p: flags.bit(0b00000001),
        }
    }
}

bitflags! {
    /// A field to identify the supported Data Types for a CD.
    ///
    /// See MMC-6 §5.3.11
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct DataBlockTypes: u16 {
        const RAW = 1 << 0;
        const PQ_SUBCHANNEL = 1 << 1;
        const PW_SUBCHANNEL_PACK = 1 << 2;
        const PW_SUBCHANNEL_RAW = 1 << 3;
        const MODE1 = 1 << 8;
        const MODE2 = 1 << 9;
        const MODE2_XA_FORM1 = 1 << 10;
        const MODE2_XA_FORM1_SUBHEADER = 1 << 11;
        const MODE2_XA_FORM2 = 1 << 12;
        const MODE2_XA_MIXED = 1 << 13;
    }

    /// A field to identify the CD-RW subtypes a drive supports writing for.
    ///
    /// See MMC-6 §5.3.28 for field definition
    /// See Orange Book Part III Vol.3 §I.2, Table 1 for subtype definitions
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct CdRwSubtypes: u8 {
        /// Disc type 1-4x
        const STANDARD = 1 << 0;
        /// Disc type 4-10x (High Speed)
        const HIGH_SPEED = 1 << 1;
        /// Disc type 8-24x (Ultra Speed 24)
        const ULTRA_SPEED_24 = 1 << 2;
        /// Disc type 8-32x (Ultra Speed 32)
        const ULTRA_SPEED_32 = 1 << 3;
    }

    /// This is a placeholder bitflags to represent the versions of a certain type and class of
    /// Blu-ray Disc the drive supports for a specific capability (reading or writing).
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct BdVersions: u16 {
        const VERSION0 = 1 << 0;
        const VERSION1 = 1 << 1;
        const VERSION2 = 1 << 2;
        const VERSION3 = 1 << 3;
        const VERSION4 = 1 << 4;
        const VERSION5 = 1 << 5;
        const VERSION6 = 1 << 6;
        const VERSION7 = 1 << 7;
        const VERSION8 = 1 << 8;
        const VERSION9 = 1 << 9;
        const VERSION10 = 1 << 10;
        const VERSION11 = 1 << 11;
        const VERSION12 = 1 << 12;
        const VERSION13 = 1 << 13;
        const VERSION14 = 1 << 14;
        const VERSION15 = 1 << 15;
    }
}

/// A list of all Profiles supported by the Drive
///
/// See MMC-6 §5.3.1
#[derive(Debug)]
pub struct ProfileList {
    header: FeatureHeader,
    pub profile_descriptors: Vec<ProfileDescriptor>,
}

/// Mandatory behavior for all devices
///
/// See MMC-6 §5.3.2
#[derive(Debug)]
pub struct Core {
    header: FeatureHeader,
    pub physical_interface: PhysicalInterfaceStandard,
    pub inq2: bool,
    pub dbe: bool,
}

/// The Drive is able to report operational changes to the Host and accept Host requests to prevent
/// operational changes.
///
/// See MMC-6 §5.3.3
#[derive(Debug)]
pub struct Morphing {
    header: FeatureHeader,
    pub oc_event: bool,
    pub asynchronous: bool,
}

/// The medium may be removed from the device.
///
/// See MMC-6 §5.3.4
#[derive(Debug)]
pub struct RemovableMedium {
    header: FeatureHeader,
    pub loading_mechanism: LoadingMechanism,
    pub load: bool,
    pub eject: bool,
    pub prevent_jumper: bool,
    pub lock: bool,
}

/// The ability to control Write Protection status
///
/// See MMC-6 §5.3.5
#[derive(Debug)]
pub struct WriteProtect {
    header: FeatureHeader,
    pub dwp: bool,
    pub wdcb: bool,
    pub spwp: bool,
    pub sswpp: bool,
}

/// The ability to read sectors with random addressing
///
/// See MMC-6 §5.3.6
#[derive(Debug)]
pub struct RandomReadable {
    header: FeatureHeader,
    pub logical_block_size: u32,
    pub blocking: u16,
    pub page_present: bool,
}

/// The Drive is able to read all CD media types; based on OSTA MultiRead
///
/// See MMC-6 §5.3.7
#[derive(Debug)]
pub struct MultiRead {
    header: FeatureHeader,
}

/// The ability to read CD specific structures
///
/// See MMC-6 §5.3.8
#[derive(Debug)]
pub struct CdRead {
    header: FeatureHeader,
    pub dap: bool,
    pub c2_flags: bool,
    pub cd_text: bool,
}

/// The ability to read DVD specific structures
///
/// See MMC-6 §5.3.9
#[derive(Debug)]
pub struct DvdRead {
    header: FeatureHeader,
    pub multi_110: bool,
    pub dual_rw: bool,
    pub dual_r: bool,
}

/// Write support for randomly addressed writes
///
/// See MMC-6 §5.3.10
#[derive(Debug)]
pub struct RandomWritable {
    header: FeatureHeader,
    pub last_lba: i32,
    pub logical_block_size: u32,
    pub blocking: u16,
    pub page_present: bool,
}

/// Write support for sequential recording
///
/// See MMC-6 §5.3.11
#[derive(Debug)]
pub struct IncrementalStreamingWritable {
    header: FeatureHeader,
    pub data_block_types_supported: DataBlockTypes,
    pub trio: bool,
    pub arsv: bool,
    pub buf: bool,
    pub link_sizes: Vec<u8>,
}

/// Write support for erasable media and media that requires an erase pass before overwrite.
///
/// Legacy as of MMC-6, see MMC-5 §5.3.12
#[derive(Debug)]
pub struct SectorErasable {
    header: FeatureHeader,
}

/// Support for formatting of media.
///
/// See MMC-6 §5.3.12
#[derive(Debug)]
pub struct Formattable {
    header: FeatureHeader,
    pub re_no_sa: bool,
    pub expand: bool,
    pub qcert: bool,
    pub cert: bool,
    pub frf: bool,
    pub rrm: bool,
}

/// Ability of the Drive/media system to provide an apparently defect-free space.
///
/// See MMC-6 §5.3.13
#[derive(Debug)]
pub struct HardwareDefectManagement {
    header: FeatureHeader,
    pub ssa: bool,
}

/// Write support for write-once media that is writable in random order.
///
/// See MMC-6 §5.3.14
#[derive(Debug)]
pub struct WriteOnce {
    header: FeatureHeader,
    pub logical_block_size: u32,
    pub blocking: u16,
    pub page_present: bool,
}

/// Write support for media that shall be written from Blocking boundaries only.
///
/// See MMC-6 §5.3.15
#[derive(Debug)]
pub struct RestrictedOverwrite {
    header: FeatureHeader,
}

/// The ability to write high speed CD-RW media
///
/// See MMC-6 §5.3.16
#[derive(Debug)]
pub struct CdRwCavWrite {
    header: FeatureHeader,
}

/// The ability to recognize and read and optionally write MRW formatted media
///
/// See MMC-6 §5.3.17
#[derive(Debug)]
pub struct Mrw {
    header: FeatureHeader,
    pub dvd_plus_write: bool,
    pub dvd_plus_read: bool,
    pub cd_write: bool,
}

/// The ability to control RECOVERED ERROR reporting
///
/// See MMC-6 §5.3.18
#[derive(Debug)]
pub struct EnhancedDefectReporting {
    header: FeatureHeader,
    pub drt_dm: bool,
    pub num_dbi_cache_zones: u8,
    pub num_entries: u16,
}

/// The ability to recognize, read and optionally write DVD+RW media
///
/// See MMC-6 §5.3.19
#[derive(Debug)]
pub struct DvdPlusRw {
    header: FeatureHeader,
    pub write: bool,
    pub quick_start: bool,
    pub close_only: bool,
}

/// The ability to read DVD+R recorded media formats
///
/// See MMC-6 §5.3.20
#[derive(Debug)]
pub struct DvdPlusR {
    header: FeatureHeader,
    pub write: bool,
}

/// Write support for media that is required to be written from Blocking boundaries with length of
/// integral multiple of Blocking size only.
///
/// See MMC-6 §5.3.21
#[derive(Debug)]
pub struct RigidRestrictedOverwrite {
    header: FeatureHeader,
    pub dsdg: bool,
    pub dsdr: bool,
    pub intermediate: bool,
    pub blank: bool,
}

/// Ability to write CD with Track at Once recording
///
/// See MMC-6 §5.3.22
#[derive(Debug)]
pub struct CdTrackAtOnce {
    header: FeatureHeader,
    pub buf: bool,
    pub r_w_raw: bool,
    pub r_w_pack: bool,
    pub test_write: bool,
    pub cd_rw: bool,
    pub rw_subcode: bool,
    pub data_type_supported: DataBlockTypes,
}

/// The ability to write CD with Session at Once or Raw write methods.
///
/// See MMC-6 §5.3.23
#[derive(Debug)]
pub struct CdMastering {
    header: FeatureHeader,
    pub buf: bool,
    pub sao: bool,
    pub raw_ms: bool,
    pub raw: bool,
    pub test_write: bool,
    pub cd_rw: bool,
    pub r_w: bool,
    pub max_cue_sheet_length: U24,
}

/// The ability to write DVD specific structures
///
/// See MMC-6 §5.3.24
#[derive(Debug)]
pub struct DvdRRwWrite {
    header: FeatureHeader,
    pub buf: bool,
    pub rdl: bool,
    pub test_write: bool,
    pub dvd_rw_sl: bool,
}

/// A Logical Unit that can read DDCD specific information from the media and can read user data
/// from DDCD blocks.
///
/// Legacy as of MMC-5, see MMC-4 §7.3.25
#[derive(Debug)]
pub struct DoubleDensityCdRead {
    header: FeatureHeader,
}

/// A Logical Unit that can write data to DDCD-R.
///
/// Legacy as of MMC-5, see MMC-4 §7.3.26
#[derive(Debug)]
pub struct DoubleDensityCdRWrite {
    header: FeatureHeader,
    pub test_rw: bool,
}

/// A Logical Unit that can write data to DDCD-RW.
///
/// Legacy as of MMC-5, see MMC-4 §7.3.27
#[derive(Debug)]
pub struct DoubleDensityCdRwWrite {
    header: FeatureHeader,
    pub intermediate: bool,
    pub blank: bool,
}

/// The ability to record in layer jump mode.
///
/// See MMC-6 §5.3.25
#[derive(Debug)]
pub struct LayerJumpRecording {
    header: FeatureHeader,
    pub link_sizes: Vec<u8>,
}

/// The ability to perform Layer Jump recording on Rigid Restricted Overwritable media
///
/// See MMC-6 §5.3.26
#[derive(Debug)]
pub struct LayerJumpRigidRestrictedOverwrite {
    header: FeatureHeader,
    pub cljb: bool,
    pub buffer_block_size: u8,
}

/// The ability to stop the long immediate operation by a command.
///
/// See MMC-6 §5.3.27
#[derive(Debug)]
pub struct StopLongOperation {
    header: FeatureHeader,
}

/// The ability to report CD –RW media sub-types that are supported for write
///
/// See MMC-6 §5.3.28
#[derive(Debug)]
pub struct CdRwMediaWriteSupport {
    header: FeatureHeader,
    pub cd_rw_subtype_support: CdRwSubtypes,
}

/// Logical Block overwrite service on BD-R discs formatted as SRM+POW.
///
/// See MMC-6 §5.3.29
#[derive(Debug)]
pub struct BdRPow {
    header: FeatureHeader,
}

/// The ability to read DVD+RW Dual Layer recorded media formats
///
/// See MMC-6 §5.3.30
#[derive(Debug)]
pub struct DvdPlusRwDualLayer {
    header: FeatureHeader,
    pub write: bool,
    pub quick_start: bool,
    pub close_only: bool,
}

/// The ability to read DVD+R Dual Layer recorded media formats
///
/// See MMC-6 §5.3.31
#[derive(Debug)]
pub struct DvdPlusRDualLayer {
    header: FeatureHeader,
    pub write: bool,
}

/// The ability to read control structures and user data from a BD disc
///
/// See MMC-6 §5.3.32
#[derive(Debug)]
pub struct BdRead {
    header: FeatureHeader,
    pub bd_re_class0_support: BdVersions,
    pub bd_re_class1_support: BdVersions,
    pub bd_re_class2_support: BdVersions,
    pub bd_re_class3_support: BdVersions,
    pub bd_r_class0_support: BdVersions,
    pub bd_r_class1_support: BdVersions,
    pub bd_r_class2_support: BdVersions,
    pub bd_r_class3_support: BdVersions,
    pub bd_rom_class0_support: BdVersions,
    pub bd_rom_class1_support: BdVersions,
    pub bd_rom_class2_support: BdVersions,
    pub bd_rom_class3_support: BdVersions,
}

/// The ability to write control structures and user data to certain BD discs
///
/// See MMC-6 §5.3.33
#[derive(Debug)]
pub struct BdWrite {
    header: FeatureHeader,
    pub bd_re_class0_support: BdVersions,
    pub bd_re_class1_support: BdVersions,
    pub bd_re_class2_support: BdVersions,
    pub bd_re_class3_support: BdVersions,
    pub bd_r_class0_support: BdVersions,
    pub bd_r_class1_support: BdVersions,
    pub bd_r_class2_support: BdVersions,
    pub bd_r_class3_support: BdVersions,
}

/// Timely, Safe Recording permits the Host to schedule defect management.
///
/// See MMC-6 §5.3.34
#[derive(Debug)]
pub struct Tsr {
    header: FeatureHeader,
}

/// The ability to read control structures and user data from a HD DVD disc
///
/// See MMC-6 §5.3.35
#[derive(Debug)]
pub struct HdDvdRead {
    header: FeatureHeader,
    pub hd_dvd_r: bool,
    pub hd_dvd_ram: bool,
}

/// The ability to write control structures and user data to certain HD DVD discs
///
/// See MMC-6 §5.3.36
#[derive(Debug)]
pub struct HdDvdWrite {
    header: FeatureHeader,
    pub hd_dvd_r: bool,
    pub hd_dvd_ram: bool,
}

/// The ability to record HD DVD-RW in fragment recording mode
///
/// See MMC-6 §5.3.37
#[derive(Debug)]
pub struct HdDvdRwFragmentRecording {
    header: FeatureHeader,
    pub bgp: bool,
}

/// The ability to access some Hybrid Discs.
///
/// See MMC-6 §5.3.38
#[derive(Debug)]
pub struct HybridDisc {
    header: FeatureHeader,
    pub ri: bool,
}

/// Host and device directed power management
///
/// See MMC-6 §5.3.39
#[derive(Debug)]
pub struct PowerManagement {
    header: FeatureHeader,
}

/// Ability to perform Self Monitoring Analysis and Reporting Technology
///
/// See MMC-6 §5.3.40
#[derive(Debug)]
pub struct Smart {
    header: FeatureHeader,
    pub pp: bool,
}

/// Single mechanism multiple disc changer
///
/// See MMC-6 §5.3.41
#[derive(Debug)]
pub struct EmbeddedChanger {
    header: FeatureHeader,
    pub scc: bool,
    pub sdp: bool,
    pub highest_slot_number: u8,
}

/// Ability to play CD Audio data directly to an external output
///
/// Legacy as of MMC-5, see MMC-4 §7.3.31
#[derive(Debug)]
pub struct CdAudioExternalPlay {
    header: FeatureHeader,
    pub scan: bool,
    pub scm: bool,
    pub sv: bool,
    pub number_of_volume_levels: u16,
}

/// Ability for the device to accept new microcode via the interface
///
/// See MMC-6 §5.3.42
#[derive(Debug)]
pub struct MicrocodeUpgrade {
    header: FeatureHeader,
    pub m5: bool,
}

/// Ability to respond to all commands within a specific time
///
/// See MMC-6 §5.3.43
#[derive(Debug)]
pub struct Timeout {
    header: FeatureHeader,
    pub group3: bool,
    pub unit_length: u16,
}

/// Ability to perform DVD CSS/CPPM authentication and RPC
///
/// See MMC-6 §5.3.44
#[derive(Debug)]
pub struct DvdCss {
    header: FeatureHeader,
    pub css_version: u8,
}

/// Ability to read and write using Host requested performance parameters
///
/// See MMC-6 §5.3.45
#[derive(Debug)]
pub struct RealTimeStreaming {
    header: FeatureHeader,
    pub rbcb: bool,
    pub scs: bool,
    pub mp2a: bool,
    pub wspd: bool,
    pub sw: bool,
}

/// The Drive has a unique identifier
///
/// See MMC-6 §5.3.46
#[derive(Debug)]
pub struct DriveSerialNumber {
    header: FeatureHeader,
    pub serial_number: String,
}

/// Ability to return unique Media Serial Number
///
/// Legacy as of MMC-6, see MMC-6 §5.3.47
#[derive(Debug)]
pub struct MediaSerialNumber {
    header: FeatureHeader,
}

/// The ability to read and/or write DCBs
///
/// See MMC-6 §5.3.48
#[derive(Debug)]
pub struct DiscControlBlocks {
    header: FeatureHeader,
    pub supported_dcb_entries: Vec<u32>,
}

/// The Drive supports DVD CPRM authentication
///
/// See MMC-6 §5.3.49
#[derive(Debug)]
pub struct DvdCprm {
    header: FeatureHeader,
    pub cprm_version: u8,
}

/// Firmware creation date report
///
/// See MMC-6 §5.3.50
#[derive(Debug)]
pub struct FirmwareInformation {
    header: FeatureHeader,
    pub centry: u16,
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
}

/// The ability to decode and optionally encode AACS protected information
///
/// See MMC-6 §5.3.51
#[derive(Debug)]
pub struct Aacs {
    header: FeatureHeader,
    pub bng: bool,
    pub block_count_binding_nonce: u8,
    pub number_of_agids: u8,
    pub aacs_version: u8,
}

/// The ability to perform DVD CSS managed recording
///
/// See MMC-6 §5.3.52
#[derive(Debug)]
pub struct DvdCssManagedRecording {
    header: FeatureHeader,
    pub max_scramble_extent_info_entries: u8,
}

/// The ability to decode and optionally encode VCPS protected information
///
/// See MMC-6 §5.3.53
#[derive(Debug)]
pub struct Vcps {
    header: FeatureHeader,
}

/// The ability to encode and decode SecurDisc protected information
///
/// See MMC-6 §5.3.54
#[derive(Debug)]
pub struct SecurDisc {
    header: FeatureHeader,
}

/// TCG Optical Security Subsystem Class Feature
///
/// See MMC-6 §5.3.55
#[derive(Debug)]
pub struct Ossc {
    header: FeatureHeader,
    pub psau: bool,
    pub lospb: bool,
    pub me: bool,
    pub profile_numbers: Vec<u16>,
}

/// Struct representing an unknown feature descriptor. Could be a Vendor Specific, Reserved, or
/// otherwise Unimplemented Feature.
#[derive(Debug)]
pub struct UnknownFeature {
    pub feature_code: u16,
    header: FeatureHeader,
    pub data: Vec<u8>,
}

impl_feature_header!(ProfileList);
impl_feature_header!(Core);
impl_feature_header!(Morphing);
impl_feature_header!(RemovableMedium);
impl_feature_header!(WriteProtect);
impl_feature_header!(RandomReadable);
impl_feature_header!(MultiRead);
impl_feature_header!(CdRead);
impl_feature_header!(DvdRead);
impl_feature_header!(RandomWritable);
impl_feature_header!(IncrementalStreamingWritable);
impl_feature_header!(SectorErasable);
impl_feature_header!(Formattable);
impl_feature_header!(HardwareDefectManagement);
impl_feature_header!(WriteOnce);
impl_feature_header!(RestrictedOverwrite);
impl_feature_header!(CdRwCavWrite);
impl_feature_header!(Mrw);
impl_feature_header!(EnhancedDefectReporting);
impl_feature_header!(DvdPlusRw);
impl_feature_header!(DvdPlusR);
impl_feature_header!(RigidRestrictedOverwrite);
impl_feature_header!(CdTrackAtOnce);
impl_feature_header!(CdMastering);
impl_feature_header!(DvdRRwWrite);
impl_feature_header!(DoubleDensityCdRead);
impl_feature_header!(DoubleDensityCdRWrite);
impl_feature_header!(DoubleDensityCdRwWrite);
impl_feature_header!(LayerJumpRecording);
impl_feature_header!(LayerJumpRigidRestrictedOverwrite);
impl_feature_header!(StopLongOperation);
impl_feature_header!(CdRwMediaWriteSupport);
impl_feature_header!(BdRPow);
impl_feature_header!(DvdPlusRwDualLayer);
impl_feature_header!(DvdPlusRDualLayer);
impl_feature_header!(BdRead);
impl_feature_header!(BdWrite);
impl_feature_header!(Tsr);
impl_feature_header!(HdDvdRead);
impl_feature_header!(HdDvdWrite);
impl_feature_header!(HdDvdRwFragmentRecording);
impl_feature_header!(HybridDisc);
impl_feature_header!(PowerManagement);
impl_feature_header!(Smart);
impl_feature_header!(EmbeddedChanger);
impl_feature_header!(CdAudioExternalPlay);
impl_feature_header!(MicrocodeUpgrade);
impl_feature_header!(Timeout);
impl_feature_header!(DvdCss);
impl_feature_header!(RealTimeStreaming);
impl_feature_header!(DriveSerialNumber);
impl_feature_header!(MediaSerialNumber);
impl_feature_header!(DiscControlBlocks);
impl_feature_header!(DvdCprm);
impl_feature_header!(FirmwareInformation);
impl_feature_header!(Aacs);
impl_feature_header!(DvdCssManagedRecording);
impl_feature_header!(Vcps);
impl_feature_header!(SecurDisc);
impl_feature_header!(Ossc);
impl_feature_header!(UnknownFeature);

pub struct FeatureParser<'a> {
    bytes: &'a [u8],
}

impl<'a> FeatureParser<'a> {
    pub fn new(descriptors: &'a [u8]) -> Self {
        Self { bytes: descriptors }
    }
}

impl<'a> Iterator for FeatureParser<'a> {
    type Item = Box<dyn MmcFeature>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bytes.is_empty() {
            return None;
        }

        let res = parsing::parse_descriptor(self.bytes);

        if let Ok(feature) = res {
            let data_len: usize = feature.header().additional_length.into();
            let bytes_read = HEADER_LEN + data_len;
            self.bytes = self.bytes.get(bytes_read..).unwrap_or(&[]);
            Some(feature)
        } else {
            self.bytes = &[];
            None
        }
    }
}

mod parsing {
    use std::ops::RangeFrom;

    use derive_more::Display;

    use super::*;
    use crate::core::util::BitReader;
    use crate::scsi::mmc::types::PhysicalInterfaceStandard;

    #[derive(Debug, Display)]
    #[display("{_variant}")]
    pub enum DataSize {
        Fixed(u8),
        #[display("{}..", _0.start)]
        Variable(RangeFrom<u8>),
    }

    impl DataSize {
        pub fn cmp_size(&self, len: usize) -> bool {
            match self {
                Self::Fixed(s) => len == (*s).into(),
                Self::Variable(RangeFrom { start }) => len >= (*start).into(),
            }
        }
    }

    trait ParseFeature: HasFeatureHeader + Sized {
        const DATA_LEN: DataSize;

        fn parse(header: FeatureHeader, data: &[u8]) -> Self;
    }

    impl ParseFeature for ProfileList {
        const DATA_LEN: DataSize = DataSize::Variable(0..);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            let profile_descriptors = data
                .chunks_exact(ProfileDescriptor::LEN)
                .map(|c| ProfileDescriptor::parse(c.try_into().unwrap()))
                .collect::<Vec<ProfileDescriptor>>();

            Self {
                header,
                profile_descriptors,
            }
        }
    }

    impl ParseFeature for Core {
        const DATA_LEN: DataSize = DataSize::Fixed(8);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            let pis_bytes = u32::from_be_bytes(data[0..4].try_into().unwrap());
            let flags = BitReader(data[4]);

            Self {
                header,
                physical_interface: PhysicalInterfaceStandard::from(pis_bytes),
                inq2: flags.bit(0b00000010),
                dbe: flags.bit(0b00000001),
            }
        }
    }

    impl ParseFeature for Morphing {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            let flags = BitReader(data[0]);

            Self {
                header,
                oc_event: flags.bit(0b00000010),
                asynchronous: flags.bit(0b00000001),
            }
        }
    }

    impl ParseFeature for RemovableMedium {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            let loading_mechanism = LoadingMechanism::from((data[0] & 0b11100000) >> 5);
            let flags = BitReader(data[0]);

            Self {
                header,
                loading_mechanism,
                load: flags.bit(0b00010000),
                eject: flags.bit(0b00001000),
                prevent_jumper: flags.bit(0b00000100),
                lock: flags.bit(0b00000001),
            }
        }
    }

    impl ParseFeature for WriteProtect {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            let flags = BitReader(data[0]);

            Self {
                header,
                dwp: flags.bit(0b00001000),
                wdcb: flags.bit(0b00000100),
                spwp: flags.bit(0b00000010),
                sswpp: flags.bit(0b00000001),
            }
        }
    }

    impl ParseFeature for RandomReadable {
        const DATA_LEN: DataSize = DataSize::Fixed(8);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            Self {
                header,
                logical_block_size: u32::from_be_bytes(data[0..4].try_into().unwrap()),
                blocking: u16::from_be_bytes(data[4..6].try_into().unwrap()),
                page_present: BitReader(data[6]).bit(0b00000001),
            }
        }
    }

    impl ParseFeature for MultiRead {
        const DATA_LEN: DataSize = DataSize::Fixed(0);

        fn parse(header: FeatureHeader, _: &[u8]) -> Self {
            Self { header }
        }
    }

    impl ParseFeature for CdRead {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            let flags = BitReader(data[0]);

            Self {
                header,
                dap: flags.bit(0b10000000),
                c2_flags: flags.bit(0b00000010),
                cd_text: flags.bit(0b00000001),
            }
        }
    }

    impl ParseFeature for DvdRead {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            let dual_flags = BitReader(data[2]);

            Self {
                header,
                multi_110: BitReader(data[0]).bit(0b00000001),
                dual_rw: dual_flags.bit(0b00000010),
                dual_r: dual_flags.bit(0b00000001),
            }
        }
    }

    impl ParseFeature for RandomWritable {
        const DATA_LEN: DataSize = DataSize::Fixed(12);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            Self {
                header,
                last_lba: i32::from_be_bytes(data[0..4].try_into().unwrap()),
                logical_block_size: u32::from_be_bytes(data[4..8].try_into().unwrap()),
                blocking: u16::from_be_bytes(data[8..10].try_into().unwrap()),
                page_present: BitReader(data[10]).bit(0b00000001),
            }
        }
    }

    impl ParseFeature for IncrementalStreamingWritable {
        const DATA_LEN: DataSize = DataSize::Variable(4..);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            let data_types_bits = u16::from_be_bytes(data[0..2].try_into().unwrap());
            let flags = BitReader(data[2]);
            let num_link_sizes: usize = data[3].into();
            let link_sizes = data.get(4..(4 + num_link_sizes)).unwrap_or(&[]).to_vec();
            debug_assert_eq!(link_sizes.len(), num_link_sizes);

            Self {
                header,
                data_block_types_supported: DataBlockTypes::from_bits_retain(data_types_bits),
                trio: flags.bit(0b00000100),
                arsv: flags.bit(0b00000010),
                buf: flags.bit(0b00000001),
                link_sizes,
            }
        }
    }

    impl ParseFeature for SectorErasable {
        const DATA_LEN: DataSize = DataSize::Fixed(0);

        fn parse(header: FeatureHeader, _: &[u8]) -> Self {
            Self { header }
        }
    }

    impl ParseFeature for Formattable {
        const DATA_LEN: DataSize = DataSize::Fixed(8);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            let flags = BitReader(data[0]);

            Self {
                header,
                re_no_sa: flags.bit(0b00001000),
                expand: flags.bit(0b00000100),
                qcert: flags.bit(0b00000010),
                cert: flags.bit(0b00000001),
                frf: BitReader(data[1]).bit(0b10000000),
                rrm: BitReader(data[4]).bit(0b00000001),
            }
        }
    }

    impl ParseFeature for HardwareDefectManagement {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            Self {
                header,
                ssa: BitReader(data[0]).bit(0b10000000),
            }
        }
    }

    impl ParseFeature for WriteOnce {
        const DATA_LEN: DataSize = DataSize::Fixed(8);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            Self {
                header,
                logical_block_size: u32::from_be_bytes(data[0..4].try_into().unwrap()),
                blocking: u16::from_be_bytes(data[4..6].try_into().unwrap()),
                page_present: BitReader(data[6]).bit(0b00000001),
            }
        }
    }

    impl ParseFeature for RestrictedOverwrite {
        const DATA_LEN: DataSize = DataSize::Fixed(0);

        fn parse(header: FeatureHeader, _: &[u8]) -> Self {
            Self { header }
        }
    }

    impl ParseFeature for CdRwCavWrite {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, _: &[u8]) -> Self {
            Self { header }
        }
    }

    impl ParseFeature for Mrw {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            let flags = BitReader(data[0]);

            Self {
                header,
                dvd_plus_write: flags.bit(0b00000100),
                dvd_plus_read: flags.bit(0b00000010),
                cd_write: flags.bit(0b00000001),
            }
        }
    }

    impl ParseFeature for EnhancedDefectReporting {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            Self {
                header,
                drt_dm: BitReader(data[0]).bit(0b00000001),
                num_dbi_cache_zones: data[1],
                num_entries: u16::from_be_bytes(data[2..4].try_into().unwrap()),
            }
        }
    }

    impl ParseFeature for DvdPlusRw {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            let flags = BitReader(data[1]);

            Self {
                header,
                write: BitReader(data[0]).bit(0b00000001),
                quick_start: flags.bit(0b00000010),
                close_only: flags.bit(0b00000001),
            }
        }
    }

    impl ParseFeature for DvdPlusR {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            Self {
                header,
                write: BitReader(data[0]).bit(0b00000001),
            }
        }
    }

    impl ParseFeature for RigidRestrictedOverwrite {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            let flags = BitReader(data[0]);

            Self {
                header,
                dsdg: flags.bit(0b00001000),
                dsdr: flags.bit(0b00000100),
                intermediate: flags.bit(0b00000010),
                blank: flags.bit(0b00000001),
            }
        }
    }

    impl ParseFeature for CdTrackAtOnce {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            let flags = BitReader(data[0]);
            let data_type_bits = u16::from_be_bytes(data[2..4].try_into().unwrap());

            Self {
                header,
                buf: flags.bit(0b01000000),
                r_w_raw: flags.bit(0b00010000),
                r_w_pack: flags.bit(0b00001000),
                test_write: flags.bit(0b00000100),
                cd_rw: flags.bit(0b00000010),
                rw_subcode: flags.bit(0b00000001),
                data_type_supported: DataBlockTypes::from_bits_retain(data_type_bits),
            }
        }
    }

    impl ParseFeature for CdMastering {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            let flags = BitReader(data[0]);

            Self {
                header,
                buf: flags.bit(0b01000000),
                sao: flags.bit(0b00100000),
                raw_ms: flags.bit(0b00010000),
                raw: flags.bit(0b00001000),
                test_write: flags.bit(0b00000100),
                cd_rw: flags.bit(0b00000010),
                r_w: flags.bit(0b00000001),
                max_cue_sheet_length: U24::from_be_bytes(data[1..4].try_into().unwrap()),
            }
        }
    }

    impl ParseFeature for DvdRRwWrite {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            let flags = BitReader(data[0]);

            Self {
                header,
                buf: flags.bit(0b01000000),
                rdl: flags.bit(0b00001000),
                test_write: flags.bit(0b00000100),
                dvd_rw_sl: flags.bit(0b00000010),
            }
        }
    }

    impl ParseFeature for DoubleDensityCdRead {
        const DATA_LEN: DataSize = DataSize::Fixed(0);

        fn parse(header: FeatureHeader, _: &[u8]) -> Self {
            Self { header }
        }
    }

    impl ParseFeature for DoubleDensityCdRWrite {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            Self {
                header,
                test_rw: BitReader(data[0]).bit(0b00000100),
            }
        }
    }

    impl ParseFeature for DoubleDensityCdRwWrite {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            let flags = BitReader(data[0]);

            Self {
                header,
                intermediate: flags.bit(0b00000010),
                blank: flags.bit(0b00000001),
            }
        }
    }

    impl ParseFeature for LayerJumpRecording {
        const DATA_LEN: DataSize = DataSize::Variable(4..);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            let num_link_sizes: usize = data[3].into();
            let link_sizes = data.get(4..(4 + num_link_sizes)).unwrap_or(&[]).to_vec();
            debug_assert_eq!(link_sizes.len(), num_link_sizes);

            Self { header, link_sizes }
        }
    }

    impl ParseFeature for LayerJumpRigidRestrictedOverwrite {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            Self {
                header,
                cljb: BitReader(data[0]).bit(0b00000001),
                buffer_block_size: data[3],
            }
        }
    }

    impl ParseFeature for StopLongOperation {
        const DATA_LEN: DataSize = DataSize::Fixed(0);

        fn parse(header: FeatureHeader, _: &[u8]) -> Self {
            Self { header }
        }
    }

    impl ParseFeature for CdRwMediaWriteSupport {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            Self {
                header,
                cd_rw_subtype_support: CdRwSubtypes::from_bits_retain(data[1]),
            }
        }
    }

    impl ParseFeature for BdRPow {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, _: &[u8]) -> Self {
            Self { header }
        }
    }

    impl ParseFeature for DvdPlusRwDualLayer {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            let flags = BitReader(data[1]);

            Self {
                header,
                write: BitReader(data[0]).bit(0b00000001),
                quick_start: flags.bit(0b00000010),
                close_only: flags.bit(0b00000001),
            }
        }
    }

    impl ParseFeature for DvdPlusRDualLayer {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            Self {
                header,
                write: BitReader(data[0]).bit(0b00000001),
            }
        }
    }

    impl ParseFeature for BdRead {
        const DATA_LEN: DataSize = DataSize::Fixed(28);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            let bitmaps = data[4..]
                .chunks_exact(2)
                .map(|c| BdVersions::from_bits_truncate(u16::from_be_bytes(c.try_into().unwrap())))
                .collect::<Vec<BdVersions>>();

            Self {
                header,
                bd_re_class0_support: bitmaps[0],
                bd_re_class1_support: bitmaps[1],
                bd_re_class2_support: bitmaps[2],
                bd_re_class3_support: bitmaps[3],
                bd_r_class0_support: bitmaps[4],
                bd_r_class1_support: bitmaps[5],
                bd_r_class2_support: bitmaps[6],
                bd_r_class3_support: bitmaps[7],
                bd_rom_class0_support: bitmaps[8],
                bd_rom_class1_support: bitmaps[9],
                bd_rom_class2_support: bitmaps[10],
                bd_rom_class3_support: bitmaps[11],
            }
        }
    }

    impl ParseFeature for BdWrite {
        const DATA_LEN: DataSize = DataSize::Fixed(20);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            let bitmaps = data[4..]
                .chunks_exact(2)
                .map(|c| BdVersions::from_bits_truncate(u16::from_be_bytes(c.try_into().unwrap())))
                .collect::<Vec<BdVersions>>();

            Self {
                header,
                bd_re_class0_support: bitmaps[0],
                bd_re_class1_support: bitmaps[1],
                bd_re_class2_support: bitmaps[2],
                bd_re_class3_support: bitmaps[3],
                bd_r_class0_support: bitmaps[4],
                bd_r_class1_support: bitmaps[5],
                bd_r_class2_support: bitmaps[6],
                bd_r_class3_support: bitmaps[7],
            }
        }
    }

    impl ParseFeature for Tsr {
        const DATA_LEN: DataSize = DataSize::Fixed(0);

        fn parse(header: FeatureHeader, _: &[u8]) -> Self {
            Self { header }
        }
    }

    impl ParseFeature for HdDvdRead {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            Self {
                header,
                hd_dvd_r: BitReader(data[0]).bit(0b00000001),
                hd_dvd_ram: BitReader(data[2]).bit(0b00000001),
            }
        }
    }

    impl ParseFeature for HdDvdWrite {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            Self {
                header,
                hd_dvd_r: BitReader(data[0]).bit(0b00000001),
                hd_dvd_ram: BitReader(data[2]).bit(0b00000001),
            }
        }
    }

    impl ParseFeature for HdDvdRwFragmentRecording {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            Self {
                header,
                bgp: BitReader(data[0]).bit(0b00000001),
            }
        }
    }

    impl ParseFeature for HybridDisc {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            Self {
                header,
                ri: BitReader(data[0]).bit(0b00000001),
            }
        }
    }

    impl ParseFeature for PowerManagement {
        const DATA_LEN: DataSize = DataSize::Fixed(0);

        fn parse(header: FeatureHeader, _: &[u8]) -> Self {
            Self { header }
        }
    }

    impl ParseFeature for Smart {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            Self {
                header,
                pp: BitReader(data[0]).bit(0b00000001),
            }
        }
    }

    impl ParseFeature for EmbeddedChanger {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            let flags = BitReader(data[0]);

            Self {
                header,
                scc: flags.bit(0b00010000),
                sdp: flags.bit(0b00000100),
                highest_slot_number: data[3] & 0b00011111,
            }
        }
    }

    impl ParseFeature for CdAudioExternalPlay {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            let flags = BitReader(data[0]);

            Self {
                header,
                scan: flags.bit(0b00000100),
                scm: flags.bit(0b00000010),
                sv: flags.bit(0b00000001),
                number_of_volume_levels: u16::from_be_bytes([data[2], data[3]]),
            }
        }
    }

    impl ParseFeature for MicrocodeUpgrade {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            Self {
                header,
                m5: BitReader(data[0]).bit(0b00000001),
            }
        }
    }

    impl ParseFeature for Timeout {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            Self {
                header,
                group3: BitReader(data[0]).bit(0b00000001),
                unit_length: u16::from_be_bytes([data[2], data[3]]),
            }
        }
    }

    impl ParseFeature for DvdCss {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            Self {
                header,
                css_version: data[3],
            }
        }
    }

    impl ParseFeature for RealTimeStreaming {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            let flags = BitReader(data[0]);

            Self {
                header,
                rbcb: flags.bit(0b00010000),
                scs: flags.bit(0b00001000),
                mp2a: flags.bit(0b00000100),
                wspd: flags.bit(0b00000010),
                sw: flags.bit(0b00000001),
            }
        }
    }

    impl ParseFeature for DriveSerialNumber {
        const DATA_LEN: DataSize = DataSize::Variable(0..);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            Self {
                header,
                serial_number: str::from_utf8(data).unwrap().trim_end().to_string(),
            }
        }
    }

    impl ParseFeature for MediaSerialNumber {
        const DATA_LEN: DataSize = DataSize::Fixed(0);

        fn parse(header: FeatureHeader, _: &[u8]) -> Self {
            Self { header }
        }
    }

    impl ParseFeature for DiscControlBlocks {
        const DATA_LEN: DataSize = DataSize::Variable(0..);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            let entries = data
                .chunks_exact(4)
                .map(|c| u32::from_be_bytes(c.try_into().unwrap()))
                .collect::<Vec<u32>>();

            Self {
                header,
                supported_dcb_entries: entries,
            }
        }
    }

    impl ParseFeature for DvdCprm {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            Self {
                header,
                cprm_version: data[3],
            }
        }
    }

    impl ParseFeature for FirmwareInformation {
        const DATA_LEN: DataSize = DataSize::Fixed(16);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            Self {
                header,
                centry: u16::from_be_bytes([data[0], data[1]]),
                year: u16::from_be_bytes([data[2], data[3]]),
                month: u16::from_be_bytes([data[4], data[5]]),
                day: u16::from_be_bytes([data[6], data[7]]),
                hour: u16::from_be_bytes([data[8], data[9]]),
                minute: u16::from_be_bytes([data[10], data[11]]),
                second: u16::from_be_bytes([data[12], data[13]]),
            }
        }
    }

    impl ParseFeature for Aacs {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            Self {
                header,
                bng: BitReader(data[0]).bit(0b00000001),
                block_count_binding_nonce: data[1],
                number_of_agids: data[2] & 0b00001111,
                aacs_version: data[3],
            }
        }
    }

    impl ParseFeature for DvdCssManagedRecording {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            Self {
                header,
                max_scramble_extent_info_entries: data[0],
            }
        }
    }

    impl ParseFeature for Vcps {
        const DATA_LEN: DataSize = DataSize::Fixed(4);

        fn parse(header: FeatureHeader, _: &[u8]) -> Self {
            Self { header }
        }
    }

    impl ParseFeature for SecurDisc {
        const DATA_LEN: DataSize = DataSize::Fixed(0);

        fn parse(header: FeatureHeader, _: &[u8]) -> Self {
            Self { header }
        }
    }

    impl ParseFeature for Ossc {
        const DATA_LEN: DataSize = DataSize::Variable(2..);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            let flags = BitReader(data[0]);
            let num_profiles: usize = data[1].into();
            let profile_numbers = data
                .get(2..num_profiles)
                .unwrap_or(&[])
                .chunks_exact(2)
                .map(|c| u16::from_be_bytes(c.try_into().unwrap()))
                .collect::<Vec<u16>>();
            debug_assert_eq!(num_profiles, profile_numbers.len());

            Self {
                header,
                psau: flags.bit(0b10000000),
                lospb: flags.bit(0b01000000),
                me: flags.bit(0b00000001),
                profile_numbers,
            }
        }
    }

    fn parse_feature<T: ParseFeature>(
        header: FeatureHeader,
        data_bytes: &[u8],
    ) -> Result<T, FeatureError> {
        let num_bytes = data_bytes.len();

        if !T::DATA_LEN.cmp_size(num_bytes) {
            return Err(FeatureError::DataSize {
                expected: T::DATA_LEN,
                received: num_bytes,
            });
        }

        Ok(T::parse(header, data_bytes))
    }

    pub fn parse_descriptor(bytes: &[u8]) -> Result<Box<dyn MmcFeature>, FeatureError> {
        if bytes.len() < HEADER_LEN {
            return Err(FeatureError::DescriptorSize);
        }

        let header = FeatureHeader::parse(bytes[0..HEADER_LEN].try_into().unwrap());

        let data = bytes.get(HEADER_LEN..).unwrap_or(&[]);

        let Some(data) = data.get(..usize::from(header.additional_length)) else {
            return Err(FeatureError::MissingData {
                expected: header.additional_length.into(),
                received: data.len(),
            });
        };

        Ok(match header.feature_code {
            0x0000 => Box::new(parse_feature::<ProfileList>(header, data)?),
            0x0001 => Box::new(parse_feature::<Core>(header, data)?),
            0x0002 => Box::new(parse_feature::<Morphing>(header, data)?),
            0x0003 => Box::new(parse_feature::<RemovableMedium>(header, data)?),
            0x0004 => Box::new(parse_feature::<WriteProtect>(header, data)?),
            0x0010 => Box::new(parse_feature::<RandomReadable>(header, data)?),
            0x001D => Box::new(parse_feature::<MultiRead>(header, data)?),
            0x001E => Box::new(parse_feature::<CdRead>(header, data)?),
            0x001F => Box::new(parse_feature::<DvdRead>(header, data)?),
            0x0020 => Box::new(parse_feature::<RandomWritable>(header, data)?),
            0x0021 => Box::new(parse_feature::<IncrementalStreamingWritable>(header, data)?),
            0x0022 => Box::new(parse_feature::<SectorErasable>(header, data)?),
            0x0023 => Box::new(parse_feature::<Formattable>(header, data)?),
            0x0024 => Box::new(parse_feature::<HardwareDefectManagement>(header, data)?),
            0x0025 => Box::new(parse_feature::<WriteOnce>(header, data)?),
            0x0026 => Box::new(parse_feature::<RestrictedOverwrite>(header, data)?),
            0x0027 => Box::new(parse_feature::<CdRwCavWrite>(header, data)?),
            0x0028 => Box::new(parse_feature::<Mrw>(header, data)?),
            0x0029 => Box::new(parse_feature::<EnhancedDefectReporting>(header, data)?),
            0x002A => Box::new(parse_feature::<DvdPlusRw>(header, data)?),
            0x002B => Box::new(parse_feature::<DvdPlusR>(header, data)?),
            0x002C => Box::new(parse_feature::<RigidRestrictedOverwrite>(header, data)?),
            0x002D => Box::new(parse_feature::<CdTrackAtOnce>(header, data)?),
            0x002E => Box::new(parse_feature::<CdMastering>(header, data)?),
            0x002F => Box::new(parse_feature::<DvdRRwWrite>(header, data)?),
            0x0030 => Box::new(parse_feature::<DoubleDensityCdRead>(header, data)?),
            0x0031 => Box::new(parse_feature::<DoubleDensityCdRWrite>(header, data)?),
            0x0032 => Box::new(parse_feature::<DoubleDensityCdRwWrite>(header, data)?),
            0x0033 => Box::new(parse_feature::<LayerJumpRecording>(header, data)?),
            0x0034 => Box::new(parse_feature::<LayerJumpRigidRestrictedOverwrite>(
                header, data,
            )?),
            0x0035 => Box::new(parse_feature::<StopLongOperation>(header, data)?),
            0x0037 => Box::new(parse_feature::<CdRwMediaWriteSupport>(header, data)?),
            0x0038 => Box::new(parse_feature::<BdRPow>(header, data)?),
            0x003A => Box::new(parse_feature::<DvdPlusRwDualLayer>(header, data)?),
            0x003B => Box::new(parse_feature::<DvdPlusRDualLayer>(header, data)?),
            0x0040 => Box::new(parse_feature::<BdRead>(header, data)?),
            0x0041 => Box::new(parse_feature::<BdWrite>(header, data)?),
            0x0042 => Box::new(parse_feature::<Tsr>(header, data)?),
            0x0050 => Box::new(parse_feature::<HdDvdRead>(header, data)?),
            0x0051 => Box::new(parse_feature::<HdDvdWrite>(header, data)?),
            0x0052 => Box::new(parse_feature::<HdDvdRwFragmentRecording>(header, data)?),
            0x0080 => Box::new(parse_feature::<HybridDisc>(header, data)?),
            0x0100 => Box::new(parse_feature::<PowerManagement>(header, data)?),
            0x0101 => Box::new(parse_feature::<Smart>(header, data)?),
            0x0102 => Box::new(parse_feature::<EmbeddedChanger>(header, data)?),
            0x0103 => Box::new(parse_feature::<CdAudioExternalPlay>(header, data)?),
            0x0104 => Box::new(parse_feature::<MicrocodeUpgrade>(header, data)?),
            0x0105 => Box::new(parse_feature::<Timeout>(header, data)?),
            0x0106 => Box::new(parse_feature::<DvdCss>(header, data)?),
            0x0107 => Box::new(parse_feature::<RealTimeStreaming>(header, data)?),
            0x0108 => Box::new(parse_feature::<DriveSerialNumber>(header, data)?),
            0x0109 => Box::new(parse_feature::<MediaSerialNumber>(header, data)?),
            0x010A => Box::new(parse_feature::<DiscControlBlocks>(header, data)?),
            0x010B => Box::new(parse_feature::<DvdCprm>(header, data)?),
            0x010C => Box::new(parse_feature::<FirmwareInformation>(header, data)?),
            0x010D => Box::new(parse_feature::<Aacs>(header, data)?),
            0x010E => Box::new(parse_feature::<DvdCssManagedRecording>(header, data)?),
            0x0110 => Box::new(parse_feature::<Vcps>(header, data)?),
            0x0113 => Box::new(parse_feature::<SecurDisc>(header, data)?),
            0x0142 => Box::new(parse_feature::<Ossc>(header, data)?),
            feature_code => Box::new(UnknownFeature {
                feature_code,
                header,
                data: data.to_vec(),
            }),
        })
    }
}
