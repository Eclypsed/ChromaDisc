use std::fmt::Debug;

use bitflags::bitflags;
use i24::U24;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use thiserror::Error;

use super::types::{LoadingMechanism, PhysicalInterfaceStandard, Profile};

#[derive(Debug, Error)]
pub enum FeatureError {
    #[error("Feature descriptor must be at least 4 bytes")]
    DescriptorSize,
    #[error("Feature Descriptor specified {expected} bytes of feature data, received {received}")]
    MissingData { expected: usize, received: usize },
    #[error(
        "{feature} Feature can only have {expected} bytes of feature data, Descriptor specified {received}"
    )]
    DataSize {
        feature: String,
        expected: parsing::DataSize,
        received: usize,
    },
}

const HEADER_LEN: usize = 4;

/// A set of the defined feature codes.
///
/// See MMC-6 §5.3.2, Table 92.
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
#[non_exhaustive]
pub enum FeatureCode {
    ProfileList = 0x0000,
    Core = 0x00001,
    Morphing = 0x0002,
    RemovableMedium = 0x0003,
    WriteProtect = 0x0004,
    RandomReadable = 0x0010,
    MultiRead = 0x001D,
    CdRead = 0x001E,
    DvdRead = 0x001F,
    RandomWritable = 0x0020,
    IncrementalStreamingWritable = 0x0021,
    SectorErasable = 0x0022,
    Formattable = 0x0023,
    HardwareDefectManagement = 0x0024,
    WriteOnce = 0x0025,
    RestrictedOverwrite = 0x0026,
    CdRwCavWrite = 0x0027,
    Mrw = 0x0028,
    EnhancedDefectReporting = 0x0029,
    DvdPlusRw = 0x002A,
    DvdPlusR = 0x002B,
    RigidRestrictedOverwrite = 0x002C,
    CdTrackAtOnce = 0x002D,
    CdMastering = 0x002E,
    DvdRRwWrite = 0x002F,
    DoubleDensityCdRead = 0x0030,
    DoubleDensityCdRWrite = 0x0031,
    DoubleDensityCdRwWrite = 0x0032,
    LayerJumpRecording = 0x0033,
    LayerJumpRigidRestrictedOverwrite = 0x0034,
    StopLongOperation = 0x0035,
    CdRwMediaWriteSupport = 0x0037,
    BdRPow = 0x0038,
    DvdPlusRwDualLayer = 0x003A,
    DvdPlusRDualLayer = 0x003B,
    BdRead = 0x0040,
    BdWrite = 0x0041,
    Tsr = 0x0042,
    HdDvdRead = 0x0050,
    HdDvdWrite = 0x0051,
    HdDvdRwFragmentRecording = 0x0052,
    HybridDisc = 0x0080,
    PowerManagement = 0x0100,
    Smart = 0x0101,
    EmbeddedChanger = 0x0102,
    CdAudioExternalPlay = 0x0103,
    MicrocodeUpgrade = 0x0104,
    Timeout = 0x0105,
    DvdCss = 0x0106,
    RealTimeStreaming = 0x0107,
    DriveSerialNumber = 0x0108,
    MediaSerialNumber = 0x0109,
    DiscControlBlocks = 0x010A,
    DvdCprm = 0x010B,
    FirmwareInformation = 0x010C,
    Aacs = 0x010D,
    DvdCssManagedRecording = 0x010E,
    Vcps = 0x0110,
    SecurDisc = 0x0113,
    Ossc = 0x0142,
}

/// A set of the defined feature descriptors as well as a catch-all unknown descriptor for vendor
/// specific or otherwise unknown descriptors.
///
/// See MMC-6 §5.2
#[derive(Debug)]
#[non_exhaustive]
pub enum MmcFeature {
    ProfileList(ProfileList),
    Core(Core),
    Morphing(Morphing),
    RemovableMedium(RemovableMedium),
    WriteProtect(WriteProtect),
    RandomReadable(RandomReadable),
    MultiRead(MultiRead),
    CdRead(CdRead),
    DvdRead(DvdRead),
    RandomWritable(RandomWritable),
    IncrementalStreamingWritable(IncrementalStreamingWritable),
    SectorErasable(SectorErasable),
    Formattable(Formattable),
    HardwareDefectManagement(HardwareDefectManagement),
    WriteOnce(WriteOnce),
    RestrictedOverwrite(RestrictedOverwrite),
    CdRwCavWrite(CdRwCavWrite),
    Mrw(Mrw),
    EnhancedDefectReporting(EnhancedDefectReporting),
    DvdPlusRw(DvdPlusRw),
    DvdPlusR(DvdPlusR),
    RigidRestrictedOverwrite(RigidRestrictedOverwrite),
    CdTrackAtOnce(CdTrackAtOnce),
    CdMastering(CdMastering),
    DvdRRwWrite(DvdRRwWrite),
    DoubleDensityCdRead(DoubleDensityCdRead),
    DoubleDensityCdRWrite(DoubleDensityCdRWrite),
    DoubleDensityCdRwWrite(DoubleDensityCdRwWrite),
    LayerJumpRecording(LayerJumpRecording),
    LayerJumpRigidRestrictedOverwrite(LayerJumpRigidRestrictedOverwrite),
    StopLongOperation(StopLongOperation),
    CdRwMediaWriteSupport(CdRwMediaWriteSupport),
    BdRPow(BdRPow),
    DvdPlusRwDualLayer(DvdPlusRwDualLayer),
    DvdPlusRDualLayer(DvdPlusRDualLayer),
    BdRead(BdRead),
    BdWrite(BdWrite),
    Tsr(Tsr),
    HdDvdRead(HdDvdRead),
    HdDvdWrite(HdDvdWrite),
    HdDvdRwFragmentRecording(HdDvdRwFragmentRecording),
    HybridDisc(HybridDisc),
    PowerManagement(PowerManagement),
    Smart(Smart),
    EmbeddedChanger(EmbeddedChanger),
    CdAudioExternalPlay(CdAudioExternalPlay),
    MicrocodeUpgrade(MicrocodeUpgrade),
    Timeout(Timeout),
    DvdCss(DvdCss),
    RealTimeStreaming(RealTimeStreaming),
    DriveSerialNumber(DriveSerialNumber),
    MediaSerialNumber(MediaSerialNumber),
    DiscControlBlocks(DiscControlBlocks),
    DvdCprm(DvdCprm),
    FirmwareInformation(FirmwareInformation),
    Aacs(Aacs),
    DvdCssManagedRecording(DvdCssManagedRecording),
    Vcps(Vcps),
    SecurDisc(SecurDisc),
    Ossc(Ossc),
    Unknown {
        feature_code: u16,
        version: u8,
        persistent: bool,
        current: bool,
        data: Vec<u8>,
    },
}

#[derive(Debug)]
struct FeatureHeader {
    pub version: u8,
    pub persistent: bool,
    pub current: bool,
}

macro_rules! impl_feature {
    ($t:ty, $name:literal, $code:expr) => {
        impl FeatureDescriptor for $t {
            const NAME: &'static str = $name;
            const CODE: FeatureCode = $code;

            fn version(&self) -> u8 {
                self.header.version
            }

            fn persistent(&self) -> bool {
                self.header.persistent
            }

            fn current(&self) -> bool {
                self.header.current
            }
        }
    };
}

/// A trait representing the generic descriptor properties
///
/// See MMC-6 §5.2.2
pub trait FeatureDescriptor: Debug {
    const NAME: &'static str;
    const CODE: FeatureCode;

    fn version(&self) -> u8;
    fn persistent(&self) -> bool;
    fn current(&self) -> bool;
}

/// A 4-byte block representing a specific Profile.
///
/// See MMC-6 §5.3.1, Table 94.
#[derive(Debug)]
pub struct ProfileDescriptor {
    pub profile_number: Profile,
    pub current_p: bool,
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

impl_feature!(ProfileList, "Profile List", FeatureCode::ProfileList);
impl_feature!(Core, "Core", FeatureCode::Core);
impl_feature!(Morphing, "Morphing", FeatureCode::Morphing);
impl_feature!(
    RemovableMedium,
    "Removable Medium",
    FeatureCode::RemovableMedium
);
impl_feature!(WriteProtect, "Write Protect", FeatureCode::WriteProtect);
impl_feature!(
    RandomReadable,
    "Random Readable",
    FeatureCode::RandomReadable
);
impl_feature!(MultiRead, "Multi-Read", FeatureCode::MultiRead);
impl_feature!(CdRead, "CD Read", FeatureCode::CdRead);
impl_feature!(DvdRead, "DVD Read", FeatureCode::DvdRead);
impl_feature!(
    RandomWritable,
    "Random Writable",
    FeatureCode::RandomWritable
);
impl_feature!(
    IncrementalStreamingWritable,
    "Incremental Streaming Writable",
    FeatureCode::IncrementalStreamingWritable
);
impl_feature!(
    SectorErasable,
    "Sector Erasable",
    FeatureCode::SectorErasable
);
impl_feature!(Formattable, "Formattable", FeatureCode::Formattable);
impl_feature!(
    HardwareDefectManagement,
    "Hardware Defect Management",
    FeatureCode::HardwareDefectManagement
);
impl_feature!(WriteOnce, "Write Once", FeatureCode::WriteOnce);
impl_feature!(
    RestrictedOverwrite,
    "Restricted Overwrite",
    FeatureCode::RestrictedOverwrite
);
impl_feature!(CdRwCavWrite, "CD-RW CAV Write", FeatureCode::CdRwCavWrite);
impl_feature!(Mrw, "MRW", FeatureCode::Mrw);
impl_feature!(
    EnhancedDefectReporting,
    "Enhanced Defect Reporting",
    FeatureCode::EnhancedDefectReporting
);
impl_feature!(DvdPlusRw, "DVD+RW", FeatureCode::DvdPlusRw);
impl_feature!(DvdPlusR, "DVD+R", FeatureCode::DvdPlusR);
impl_feature!(
    RigidRestrictedOverwrite,
    "Rigid Restricted Overwrite",
    FeatureCode::RigidRestrictedOverwrite
);
impl_feature!(
    CdTrackAtOnce,
    "CD Track at Once",
    FeatureCode::CdTrackAtOnce
);
impl_feature!(
    CdMastering,
    "CD Mastering (Session at Once)",
    FeatureCode::CdMastering
);
impl_feature!(DvdRRwWrite, "DVD-R/-RW Write", FeatureCode::DvdRRwWrite);
impl_feature!(
    DoubleDensityCdRead,
    "Double Density CD Read",
    FeatureCode::DoubleDensityCdRead
);
impl_feature!(
    DoubleDensityCdRWrite,
    "Double Density CD-R Write",
    FeatureCode::DoubleDensityCdRWrite
);
impl_feature!(
    DoubleDensityCdRwWrite,
    "Double Density CD-RW Write",
    FeatureCode::DoubleDensityCdRwWrite
);
impl_feature!(
    LayerJumpRecording,
    "Layer Jump Recording",
    FeatureCode::LayerJumpRecording
);
impl_feature!(
    LayerJumpRigidRestrictedOverwrite,
    "Layer Jump Rigid Restricted Overwrite",
    FeatureCode::LayerJumpRigidRestrictedOverwrite
);
impl_feature!(
    StopLongOperation,
    "Stop Long Operation",
    FeatureCode::StopLongOperation
);
impl_feature!(
    CdRwMediaWriteSupport,
    "CD-RW Media Write Support",
    FeatureCode::CdRwMediaWriteSupport
);
impl_feature!(BdRPow, "BD-R Pseudo-Overwrite (POW)", FeatureCode::BdRPow);
impl_feature!(
    DvdPlusRwDualLayer,
    "DVD+RW Dual Layer",
    FeatureCode::DvdPlusRwDualLayer
);
impl_feature!(
    DvdPlusRDualLayer,
    "DVD+R Dual Layer",
    FeatureCode::DvdPlusRDualLayer
);
impl_feature!(BdRead, "BD Read", FeatureCode::BdRead);
impl_feature!(BdWrite, "BD Write", FeatureCode::BdWrite);
impl_feature!(Tsr, "TSR", FeatureCode::Tsr);
impl_feature!(HdDvdRead, "HD DVD Read", FeatureCode::HdDvdRead);
impl_feature!(HdDvdWrite, "HD DVD Write", FeatureCode::HdDvdWrite);
impl_feature!(
    HdDvdRwFragmentRecording,
    "HD DVD-RW Fragment Recording",
    FeatureCode::HdDvdRwFragmentRecording
);
impl_feature!(HybridDisc, "Hybrid Disc", FeatureCode::HybridDisc);
impl_feature!(
    PowerManagement,
    "Power Management",
    FeatureCode::PowerManagement
);
impl_feature!(Smart, "S.M.A.R.T.", FeatureCode::Smart);
impl_feature!(
    EmbeddedChanger,
    "Embedded Changer",
    FeatureCode::EmbeddedChanger
);
impl_feature!(
    CdAudioExternalPlay,
    "CD Audio External Play",
    FeatureCode::CdAudioExternalPlay
);
impl_feature!(
    MicrocodeUpgrade,
    "Microcode Upgrade",
    FeatureCode::MicrocodeUpgrade
);
impl_feature!(Timeout, "Timeout", FeatureCode::Timeout);
impl_feature!(DvdCss, "DVD CSS", FeatureCode::DvdCss);
impl_feature!(
    RealTimeStreaming,
    "Real Time Streaming",
    FeatureCode::RealTimeStreaming
);
impl_feature!(
    DriveSerialNumber,
    "Drive Serial Number",
    FeatureCode::DriveSerialNumber
);
impl_feature!(
    MediaSerialNumber,
    "Media Serial Number",
    FeatureCode::MediaSerialNumber
);
impl_feature!(
    DiscControlBlocks,
    "Disc Control Blocks (DCBs)",
    FeatureCode::DiscControlBlocks
);
impl_feature!(DvdCprm, "DVD CPRM", FeatureCode::DvdCprm);
impl_feature!(
    FirmwareInformation,
    "Firmware Information",
    FeatureCode::FirmwareInformation
);
impl_feature!(Aacs, "AACS", FeatureCode::Aacs);
impl_feature!(
    DvdCssManagedRecording,
    "DVD CSS Managed Recording",
    FeatureCode::DvdCssManagedRecording
);
impl_feature!(Vcps, "VCPS", FeatureCode::Vcps);
impl_feature!(SecurDisc, "SecurDisc", FeatureCode::SecurDisc);
impl_feature!(Ossc, "OSSC", FeatureCode::Ossc);

pub struct FeatureParser<'a> {
    bytes: &'a [u8],
}

impl<'a> FeatureParser<'a> {
    pub fn new(descriptors: &'a [u8]) -> Self {
        Self { bytes: descriptors }
    }
}

impl<'a> Iterator for FeatureParser<'a> {
    type Item = MmcFeature;

    fn next(&mut self) -> Option<Self::Item> {
        let _ = self.bytes.get(..HEADER_LEN)?;

        let desc_len = HEADER_LEN + usize::from(self.bytes[3]); // Additional Length
        let desc_bytes = self.bytes.get(..desc_len)?;

        let res = parsing::parse_descriptor(desc_bytes);

        if let Ok(feature) = res {
            self.bytes = self.bytes.get(desc_len..).unwrap_or(&[]);
            Some(feature)
        } else {
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

    trait ParseFeature: FeatureDescriptor + Sized {
        const DATA_LEN: DataSize;

        fn parse(header: FeatureHeader, data: &[u8]) -> Self;
    }

    impl ParseFeature for ProfileList {
        const DATA_LEN: DataSize = DataSize::Variable(0..);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            let profile_descriptors = data
                .chunks_exact(4)
                .map(|c| ProfileDescriptor {
                    profile_number: Profile::from(u16::from_be_bytes([c[0], c[1]])),
                    current_p: BitReader(c[2]).bit(0b00000001),
                })
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
                feature: T::NAME.to_string(),
                expected: T::DATA_LEN,
                received: num_bytes,
            });
        }

        Ok(T::parse(header, data_bytes))
    }

    macro_rules! map_feature {
        ($code:expr, $header:expr, $data:expr, {
            $($ft:ty => $variant:expr),* $(,)?
        }) => {
            match $code {
                $(
                    <$ft>::CODE => $variant(
                        parse_feature::<$ft>($header, $data)?
                    )
                ),*
            }
        };
    }

    pub fn parse_descriptor(bytes: &[u8]) -> Result<MmcFeature, FeatureError> {
        if bytes.len() < HEADER_LEN {
            return Err(FeatureError::DescriptorSize);
        }

        let feature_code = u16::from_be_bytes([bytes[0], bytes[1]]);
        let version = (bytes[2] & 0b00111100) >> 2;

        let flags = BitReader(bytes[2]);
        let persistent = flags.bit(0b00000010);
        let current = flags.bit(0b00000001);

        let additional_length: usize = bytes[3].into();

        let header = FeatureHeader {
            version,
            persistent,
            current,
        };

        let data = bytes.get(HEADER_LEN..).unwrap_or(&[]);

        let Some(data) = data.get(..additional_length) else {
            return Err(FeatureError::MissingData {
                expected: additional_length,
                received: data.len(),
            });
        };

        let Ok(feature_code) = FeatureCode::try_from(feature_code) else {
            return Ok(MmcFeature::Unknown {
                feature_code,
                version,
                persistent,
                current,
                data: data.to_vec(),
            });
        };

        Ok(map_feature!(feature_code, header, data, {
            ProfileList => MmcFeature::ProfileList,
            Core => MmcFeature::Core,
            Morphing => MmcFeature::Morphing,
            RemovableMedium => MmcFeature::RemovableMedium,
            WriteProtect => MmcFeature::WriteProtect,
            RandomReadable => MmcFeature::RandomReadable,
            MultiRead => MmcFeature::MultiRead,
            CdRead => MmcFeature::CdRead,
            DvdRead => MmcFeature::DvdRead,
            RandomWritable => MmcFeature::RandomWritable,
            IncrementalStreamingWritable => MmcFeature::IncrementalStreamingWritable,
            SectorErasable => MmcFeature::SectorErasable,
            Formattable => MmcFeature::Formattable,
            HardwareDefectManagement => MmcFeature::HardwareDefectManagement,
            WriteOnce => MmcFeature::WriteOnce,
            RestrictedOverwrite => MmcFeature::RestrictedOverwrite,
            CdRwCavWrite => MmcFeature::CdRwCavWrite,
            Mrw => MmcFeature::Mrw,
            EnhancedDefectReporting => MmcFeature::EnhancedDefectReporting,
            DvdPlusRw => MmcFeature::DvdPlusRw,
            DvdPlusR => MmcFeature::DvdPlusR,
            RigidRestrictedOverwrite => MmcFeature::RigidRestrictedOverwrite,
            CdTrackAtOnce => MmcFeature::CdTrackAtOnce,
            CdMastering => MmcFeature::CdMastering,
            DvdRRwWrite => MmcFeature::DvdRRwWrite,
            DoubleDensityCdRead => MmcFeature::DoubleDensityCdRead,
            DoubleDensityCdRWrite => MmcFeature::DoubleDensityCdRWrite,
            DoubleDensityCdRwWrite => MmcFeature::DoubleDensityCdRwWrite,
            LayerJumpRecording => MmcFeature::LayerJumpRecording,
            LayerJumpRigidRestrictedOverwrite => MmcFeature::LayerJumpRigidRestrictedOverwrite,
            StopLongOperation => MmcFeature::StopLongOperation,
            CdRwMediaWriteSupport => MmcFeature::CdRwMediaWriteSupport,
            BdRPow => MmcFeature::BdRPow,
            DvdPlusRwDualLayer => MmcFeature::DvdPlusRwDualLayer,
            DvdPlusRDualLayer => MmcFeature::DvdPlusRDualLayer,
            BdRead => MmcFeature::BdRead,
            BdWrite => MmcFeature::BdWrite,
            Tsr => MmcFeature::Tsr,
            HdDvdRead => MmcFeature::HdDvdRead,
            HdDvdWrite => MmcFeature::HdDvdWrite,
            HdDvdRwFragmentRecording => MmcFeature::HdDvdRwFragmentRecording,
            HybridDisc => MmcFeature::HybridDisc,
            PowerManagement => MmcFeature::PowerManagement,
            Smart => MmcFeature::Smart,
            EmbeddedChanger => MmcFeature::EmbeddedChanger,
            CdAudioExternalPlay => MmcFeature::CdAudioExternalPlay,
            MicrocodeUpgrade => MmcFeature::MicrocodeUpgrade,
            Timeout => MmcFeature::Timeout,
            DvdCss => MmcFeature::DvdCss,
            RealTimeStreaming => MmcFeature::RealTimeStreaming,
            DriveSerialNumber => MmcFeature::DriveSerialNumber,
            MediaSerialNumber => MmcFeature::MediaSerialNumber,
            DiscControlBlocks => MmcFeature::DiscControlBlocks,
            DvdCprm => MmcFeature::DvdCprm,
            FirmwareInformation => MmcFeature::FirmwareInformation,
            Aacs => MmcFeature::Aacs,
            DvdCssManagedRecording => MmcFeature::DvdCssManagedRecording,
            Vcps => MmcFeature::Vcps,
            SecurDisc => MmcFeature::SecurDisc,
            Ossc => MmcFeature::Ossc,
        }))
    }
}
