use std::fmt::Display;

use deku::{DekuRead, DekuWrite};
use derive_more::{Debug, From, Into};

pub mod aacs;
pub mod bd_read;
pub mod bd_write;
pub mod bdr_psuedo_overwrite;
pub mod cd_audio_external_play;
pub mod cd_mastering;
pub mod cd_read;
pub mod cd_tao;
pub mod cdrw_cav_write;
pub mod cdrw_media_write_support;
pub mod core_feature;
pub mod disc_control_blocks;
pub mod double_density_cd_read;
pub mod double_density_cdr_write;
pub mod double_density_cdrw_write;
pub mod drive_serial_number;
pub mod dvd_cprm;
pub mod dvd_css;
pub mod dvd_css_managed_recording;
pub mod dvd_plus_r;
pub mod dvd_plus_r_dual_layer;
pub mod dvd_plus_rw;
pub mod dvd_plus_rw_dual_layer;
pub mod dvd_r_rw_write;
pub mod dvd_read;
pub mod embedded_changer;
pub mod enhanced_defect_reporting;
pub mod firmware_information;
pub mod formattable;
pub mod hardware_defect_management;
pub mod hddvd_read;
pub mod hddvd_write;
pub mod hddvdrw_fragment_recording;
pub mod hybrid_disc;
pub mod incremental_streaming_writable;
pub mod layer_jump_recording;
pub mod layer_jump_rigid_restricted_overwrite;
pub mod media_serial_number;
pub mod microcode_upgrade;
pub mod morphing;
pub mod mrw;
pub mod multi_read;
pub mod ossc;
pub mod power_management;
pub mod profile_list;
pub mod random_readable;
pub mod random_writable;
pub mod real_time_streaming;
pub mod removable_medium;
pub mod restricted_overwrite;
pub mod rigid_restricted_overwrite;
pub mod sector_erasable;
pub mod secur_disc;
pub mod smart;
pub mod stop_long_operation;
pub mod timeout;
pub mod tsr;
pub mod vcps;
pub mod write_once;
pub mod write_protect;

#[derive(Debug, DekuRead)]
pub struct FeatureHeader {
    pub feature_code: FeatureCode,
    #[deku(pad_bits_before = "2", bits = 4)]
    pub version: u8,
    #[deku(bits = 1)]
    pub persistent: bool,
    #[deku(bits = 1)]
    pub current: bool,
    pub additional_length: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FeatureStatus<T> {
    pub persistent: bool,
    pub current: bool,
    pub feature: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(ctx = "header: FeatureHeader", id = "header.feature_code")]
pub enum Feature {
    #[deku(id = "FeatureCode::PROFILE_LIST")]
    ProfileList(#[deku(ctx = "header")] profile_list::ProfileListDescriptor),
    #[deku(id = "FeatureCode::CORE")]
    Core(#[deku(ctx = "header")] core_feature::CoreDescriptor),
    #[deku(id = "FeatureCode::MORPHING")]
    Morphing(#[deku(ctx = "header")] morphing::MorphingDescriptor),
    #[deku(id = "FeatureCode::REMOVABLE_MEDIUM")]
    RemovableMedium(#[deku(ctx = "header")] removable_medium::RemovableMediumDescriptor),
    #[deku(id = "FeatureCode::WRITE_PROTECT")]
    WriteProtect(#[deku(ctx = "header")] write_protect::WriteProtectDescriptor),
    #[deku(id = "FeatureCode::RANDOM_READABLE")]
    RandomReadable(#[deku(ctx = "header")] random_readable::RandomReadableDescriptor),
    #[deku(id = "FeatureCode::MULTI_READ")]
    MultiRead(#[deku(ctx = "header")] multi_read::MultiReadDescriptor),
    #[deku(id = "FeatureCode::CD_READ")]
    CdRead(#[deku(ctx = "header")] cd_read::CdReadDescriptor),
    #[deku(id = "FeatureCode::DVD_READ")]
    DvdRead(#[deku(ctx = "header")] dvd_read::DvdReadDescriptor),
    #[deku(id = "FeatureCode::RANDOM_WRITABLE")]
    RandomWritable(#[deku(ctx = "header")] random_writable::RandomWritableDescriptor),
    #[deku(id = "FeatureCode::INCREMENTAL_STREAMING_WRITABLE")]
    IncrementalStreamingWritable(
        #[deku(ctx = "header")]
        incremental_streaming_writable::IncrementalStreamingWritableDescriptor,
    ),
    #[deku(id = "FeatureCode::SECTOR_ERASABLE")]
    SectorErasable(#[deku(ctx = "header")] sector_erasable::SectorErasableDescriptor),
    #[deku(id = "FeatureCode::FORMATTABLE")]
    Formattable(#[deku(ctx = "header")] formattable::FormattableDescriptor),
    #[deku(id = "FeatureCode::HARDWARE_DEFECT_MANAGEMENT")]
    HardwareDefectManagement(
        #[deku(ctx = "header")] hardware_defect_management::HardwareDefectManagementDescriptor,
    ),
    #[deku(id = "FeatureCode::WRITE_ONCE")]
    WriteOnce(#[deku(ctx = "header")] write_once::WriteOnceDescriptor),
    #[deku(id = "FeatureCode::RESTRICTED_OVERWRITE")]
    RestrictedOverwrite(
        #[deku(ctx = "header")] restricted_overwrite::RestrictedOverwriteDescriptor,
    ),
    #[deku(id = "FeatureCode::CD_RW_CAV_WRITE")]
    CdrwCavWrite(#[deku(ctx = "header")] cdrw_cav_write::CdrwCavWriteDescriptor),
    #[deku(id = "FeatureCode::MRW")]
    Mrw(#[deku(ctx = "header")] mrw::MrwDescriptor),
    #[deku(id = "FeatureCode::ENHANCED_DEFECT_REPORTING")]
    EnhancedDefectReporting(
        #[deku(ctx = "header")] enhanced_defect_reporting::EnhancedDefectReportingDescriptor,
    ),
    #[deku(id = "FeatureCode::DVD_PLUS_RW")]
    DvdPlusRw(#[deku(ctx = "header")] dvd_plus_rw::DvdPlusRwDescriptor),
    #[deku(id = "FeatureCode::DVD_PLUS_R")]
    DvdPlusR(#[deku(ctx = "header")] dvd_plus_r::DvdPlusRDescriptor),
    #[deku(id = "FeatureCode::RIGID_RESTRICTED_OVERWRITE")]
    RigidRestrictedOverwrite(
        #[deku(ctx = "header")] rigid_restricted_overwrite::RigidRestrictedOverwriteDescriptor,
    ),
    #[deku(id = "FeatureCode::CD_TRACK_AT_ONCE")]
    CdTao(#[deku(ctx = "header")] cd_tao::CdTaoDescriptor),
    #[deku(id = "FeatureCode::CD_MASTERING")]
    CdMastering(#[deku(ctx = "header")] cd_mastering::CdMasteringDescriptor),
    #[deku(id = "FeatureCode::DVD_R_RW_WRITE")]
    DvdRRwWrite(#[deku(ctx = "header")] dvd_r_rw_write::DvdRRwWriteDescriptor),
    #[deku(id = "FeatureCode::DDCD_READ")]
    DoubleDensityCdRead(
        #[deku(ctx = "header")] double_density_cd_read::DoubleDensityCdReadDescriptor,
    ),
    #[deku(id = "FeatureCode::DDCD_R_WRITE")]
    DoubleDensityCdrWrite(
        #[deku(ctx = "header")] double_density_cdr_write::DoubleDensityCdrWriteDescriptor,
    ),
    #[deku(id = "FeatureCode::DDCD_RW_WRITE")]
    DoubleDensityCdrwWrite(
        #[deku(ctx = "header")] double_density_cdrw_write::DoubleDensityCdrwWriteDescriptor,
    ),
    #[deku(id = "FeatureCode::LAYER_JUMP_RECORDING")]
    LayerJumpRecording(#[deku(ctx = "header")] layer_jump_recording::LayerJumpRecordingDescriptor),
    #[deku(id = "FeatureCode::LJ_RIGID_RESTRICTED_OVERWRITE")]
    LayerJumpRigidRestrictedOverwrite(
        #[deku(ctx = "header")]
        layer_jump_rigid_restricted_overwrite::LayerJumpRigidRestrictedOverwriteDescriptor,
    ),
    #[deku(id = "FeatureCode::STOP_LONG_OPERATION")]
    StopLongOperation(#[deku(ctx = "header")] stop_long_operation::StopLongOperationDescriptor),
    #[deku(id = "FeatureCode::CD_RW_MEDIA_WRITE_SUPPORT")]
    CdrwMediaWriteSupport(
        #[deku(ctx = "header")] cdrw_media_write_support::CdrwMediaWriteSupportDescriptor,
    ),
    #[deku(id = "FeatureCode::BD_R_POW")]
    BdrPsuedoOverwrite(#[deku(ctx = "header")] bdr_psuedo_overwrite::BdrPsuedoOverwriteDescriptor),
    #[deku(id = "FeatureCode::DVD_PLUS_RW_DUAL_LAYER")]
    DvdPlusRWDualLayer(
        #[deku(ctx = "header")] dvd_plus_rw_dual_layer::DvdPlusRWDualLayerDescriptor,
    ),
    #[deku(id = "FeatureCode::DVD_PLUS_R_DUAL_LAYER")]
    DvdPlusRDualLayer(#[deku(ctx = "header")] dvd_plus_r_dual_layer::DvdPlusRDualLayerDescriptor),
    #[deku(id = "FeatureCode::BD_READ")]
    BdRead(#[deku(ctx = "header")] bd_read::BdReadDescriptor),
    #[deku(id = "FeatureCode::BD_WRITE")]
    BdWrite(#[deku(ctx = "header")] bd_write::BdWriteDescriptor),
    #[deku(id = "FeatureCode::TSR")]
    Tsr(#[deku(ctx = "header")] tsr::TsrDescriptor),
    #[deku(id = "FeatureCode::HD_DVD_READ")]
    HddvdRead(#[deku(ctx = "header")] hddvd_read::HddvdReadDescriptor),
    #[deku(id = "FeatureCode::HD_DVD_WRITE")]
    HddvdWrite(#[deku(ctx = "header")] hddvd_write::HddvdWriteDescriptor),
    #[deku(id = "FeatureCode::HD_DVD_RW_FRAGMENT_RECORDING")]
    HddvdrwFragmentRecording(
        #[deku(ctx = "header")] hddvdrw_fragment_recording::HddvdrwFragmentRecordingDescriptor,
    ),
    #[deku(id = "FeatureCode::HYBRID_DISC")]
    HybridDisc(#[deku(ctx = "header")] hybrid_disc::HybridDiscDescriptor),
    #[deku(id = "FeatureCode::POWER_MANAGEMENT")]
    PowerManagement(#[deku(ctx = "header")] power_management::PowerManagementDescriptor),
    #[deku(id = "FeatureCode::SMART")]
    Smart(#[deku(ctx = "header")] smart::SmartDescriptor),
    #[deku(id = "FeatureCode::EMBEDDED_CHANGER")]
    EmbeddedChanger(#[deku(ctx = "header")] embedded_changer::EmbeddedChangerDescriptor),
    #[deku(id = "FeatureCode::CD_AUDIO_EXTERNAL_PLAY")]
    CdAudioExternalPlay(
        #[deku(ctx = "header")] cd_audio_external_play::CdAudioExternalPlayDescriptor,
    ),
    #[deku(id = "FeatureCode::MICROCODE_UPGRADE")]
    MicrocodeUpgrade(#[deku(ctx = "header")] microcode_upgrade::MicrocodeUpgradeDescriptor),
    #[deku(id = "FeatureCode::TIMEOUT")]
    Timeout(#[deku(ctx = "header")] timeout::TimeoutDescriptor),
    #[deku(id = "FeatureCode::DVD_CSS")]
    DvdCss(#[deku(ctx = "header")] dvd_css::DvdCssDescriptor),
    #[deku(id = "FeatureCode::REAL_TIME_STREAMING")]
    RealTimeStreaming(#[deku(ctx = "header")] real_time_streaming::RealTimeStreamingDescriptor),
    #[deku(id = "FeatureCode::DRIVE_SERIAL_NUMBER")]
    DriveSerialNumber(#[deku(ctx = "header")] drive_serial_number::DriveSerialNumberDescriptor),
    #[deku(id = "FeatureCode::MEDIA_SERIAL_NUMBER")]
    MediaSerialNumber(#[deku(ctx = "header")] media_serial_number::MediaSerialNumberDescriptor),
    #[deku(id = "FeatureCode::DCBS")]
    DiscControlBlocks(#[deku(ctx = "header")] disc_control_blocks::DiscControlBlocksDescriptor),
    #[deku(id = "FeatureCode::DVD_CPRM")]
    DvdCprm(#[deku(ctx = "header")] dvd_cprm::DvdCprmDescriptor),
    #[deku(id = "FeatureCode::FIRMWARE_INFORMATION")]
    FirmwareInformation(
        #[deku(ctx = "header")] firmware_information::FirmwareInformationDescriptor,
    ),
    #[deku(id = "FeatureCode::AACS")]
    Aacs(#[deku(ctx = "header")] aacs::AacsDescriptor),
    #[deku(id = "FeatureCode::DVD_CSS_MANAGED_RECORDING")]
    DvdCssManagedRecording(
        #[deku(ctx = "header")] dvd_css_managed_recording::DvdCssManagedRecordingDescriptor,
    ),
    #[deku(id = "FeatureCode::VCPS")]
    Vcps(#[deku(ctx = "header")] vcps::VcpsDescriptor),
    #[deku(id = "FeatureCode::SECUR_DISC")]
    SecurDisc(#[deku(ctx = "header")] secur_disc::SecurDiscDescriptor),
    #[deku(id = "FeatureCode::OSSC_FEATURE")]
    Ossc(#[deku(ctx = "header")] ossc::OsscDescriptor),
}

/// A feature code as defined by MMC.
///
/// Feature codes identify the type of a Feature Descriptor returned by the
/// GET CONFIGURATION command. The set of codes is enumerable but
/// non-exhaustive as reserved ranges may be defined by future spec
/// revisions, and the `FF00h`–`FFFFh` range is permanently reserved for
/// vendor-specific use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, From, Into, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct FeatureCode(#[debug("{_0:04X}h")] u16);

impl FeatureCode {
    /// A list of all Profiles supported by the Drive
    pub const PROFILE_LIST: Self = Self(0x0000);
    /// Mandatory behavior for all devices
    pub const CORE: Self = Self(0x0001);
    /// The Drive is able to report operational changes to the Host and accept. Host requests to
    /// prevent operational changes.
    pub const MORPHING: Self = Self(0x0002);
    /// The medium may be removed from the device
    pub const REMOVABLE_MEDIUM: Self = Self(0x0003);
    /// The ability to control Write Protection status
    pub const WRITE_PROTECT: Self = Self(0x0004);

    // --- 0005h - 000Fh Reserved ---

    /// The ability to read sectors with random addressing
    pub const RANDOM_READABLE: Self = Self(0x0010);

    // --- 0011h - 001Ch Reserved ---

    /// The Drive is able to read all CD media types; based on OSTA MultiRead
    pub const MULTI_READ: Self = Self(0x001D);
    /// The ability to read CD specific structures
    pub const CD_READ: Self = Self(0x001E);
    /// The ability to read DVD specific structures
    pub const DVD_READ: Self = Self(0x001F);
    /// Write support for randomly addressed writes
    pub const RANDOM_WRITABLE: Self = Self(0x0020);
    /// Write support for sequential recording
    pub const INCREMENTAL_STREAMING_WRITABLE: Self = Self(0x0021);
    /// Write support for erasable media and media that requires an erase pass
    /// before overwrite.
    pub const SECTOR_ERASABLE: Self = Self(0x0022);
    /// Support for formatting of media.
    pub const FORMATTABLE: Self = Self(0x0023);
    /// Ability of the Drive/media system to provide an apparently defect-free space.
    pub const HARDWARE_DEFECT_MANAGEMENT: Self = Self(0x0024);
    /// Write support for write-once media that is writable in random order.
    pub const WRITE_ONCE: Self = Self(0x0025);
    /// Write support for media that shall be written from Blocking boundaries only.
    pub const RESTRICTED_OVERWRITE: Self = Self(0x0026);
    /// The ability to write high speed CD-RW media
    pub const CD_RW_CAV_WRITE: Self = Self(0x0027);
    /// The ability to recognize and read and optionally write MRW formatted media
    pub const MRW: Self = Self(0x0028);
    /// The ability to control RECOVERED ERROR reporting
    pub const ENHANCED_DEFECT_REPORTING: Self = Self(0x0029);
    /// The ability to recognize, read and optionally write DVD+RW media
    pub const DVD_PLUS_RW: Self = Self(0x002A);
    /// The ability to read DVD+R recorded media formats
    pub const DVD_PLUS_R: Self = Self(0x002B);
    /// Write support for media that is required to be written from Blocking boundaries with length
    /// of integral multiple Blocking size only.
    pub const RIGID_RESTRICTED_OVERWRITE: Self = Self(0x002C);
    /// Ability to write CD with Track at Once recording
    pub const CD_TRACK_AT_ONCE: Self = Self(0x002D);
    /// The ability to write CD with Session at Once or Raw write methods.
    pub const CD_MASTERING: Self = Self(0x002E);
    /// The ability to write DVD specific structures
    pub const DVD_R_RW_WRITE: Self = Self(0x002F);
    /// The ability to read user data from DDCD blocks.
    pub const DDCD_READ: Self = Self(0x0030);
    /// The ability to write and read DDCD-R media.
    pub const DDCD_R_WRITE: Self = Self(0x0031);
    /// The ability to write and read DDCD-RW media
    pub const DDCD_RW_WRITE: Self = Self(0x0032);
    /// The ability to record in layer jump mode
    pub const LAYER_JUMP_RECORDING: Self = Self(0x0033);
    /// The ability to perform Layer Jump recording on Rigid Restricted Overwritable media
    pub const LJ_RIGID_RESTRICTED_OVERWRITE: Self = Self(0x0034);

    // --- 0036h Reserved ---

    /// The ability to stop the long immediate operation by a command.
    pub const STOP_LONG_OPERATION: Self = Self(0x0035);
    /// The ability to report CD–RW media sub-types that are supported for write
    pub const CD_RW_MEDIA_WRITE_SUPPORT: Self = Self(0x0037);
    /// Logical Block overwrite service on BD-R discs formatted as SRM+POW.
    pub const BD_R_POW: Self = Self(0x0038);

    // --- 0039h Reserved ---

    /// The ability to read DVD+RW Dual Layer recorded media formats
    pub const DVD_PLUS_RW_DUAL_LAYER: Self = Self(0x003A);
    /// The ability to read DVD+R Dual Layer recorded media formats
    pub const DVD_PLUS_R_DUAL_LAYER: Self = Self(0x003B);

    // --- 003Ch - 003Fh Reserved ---

    /// The ability to read control structures and user data from a BD disc
    pub const BD_READ: Self = Self(0x0040);
    /// The ability to write control structures and user data to certain BD discs
    pub const BD_WRITE: Self = Self(0x0041);
    /// Timely, Safe Recording permits the Host to schedule defect management.
    pub const TSR: Self = Self(0x0042);

    // --- 0043h - 004Fh Reserved ---

    /// The ability to read control structures and user data from a HD DVD disc
    pub const HD_DVD_READ: Self = Self(0x0050);
    /// The ability to write control structures and user data to certain HD DVD discs
    pub const HD_DVD_WRITE: Self = Self(0x0051);
    /// The ability to record HD DVD-RW in fragment recording mode
    pub const HD_DVD_RW_FRAGMENT_RECORDING: Self = Self(0x0052);

    // --- 0053h - 007Fh Reserved ---

    /// The ability to access some Hybrid Discs.
    pub const HYBRID_DISC: Self = Self(0x0080);

    // --- 0081h - 00FFh Reserved ---

    /// Host and device directed power management
    pub const POWER_MANAGEMENT: Self = Self(0x0100);
    /// Ability to perform Self Monitoring Analysis and Reporting Technology
    pub const SMART: Self = Self(0x0101);
    /// Single mechanism multiple disc changer
    pub const EMBEDDED_CHANGER: Self = Self(0x0102);
    /// Ability to play audio CDs via the Logical Unitís own analog output
    pub const CD_AUDIO_EXTERNAL_PLAY: Self = Self(0x0103);
    /// Ability for the device to accept new microcode via the interface
    pub const MICROCODE_UPGRADE: Self = Self(0x0104);
    /// Ability to respond to all commands within a specific time
    pub const TIMEOUT: Self = Self(0x0105);
    /// Ability to perform DVD CSS/CPPM authentication and RPC
    pub const DVD_CSS: Self = Self(0x0106);
    /// Ability to read and write using Host requested performance parameters
    pub const REAL_TIME_STREAMING: Self = Self(0x0107);
    /// The Drive has a unique identifier
    pub const DRIVE_SERIAL_NUMBER: Self = Self(0x0108);
    /// Ability to return unique Media Serial Number
    pub const MEDIA_SERIAL_NUMBER: Self = Self(0x0109);
    /// The ability to read and/or write DCBs
    pub const DCBS: Self = Self(0x010A);
    /// The Drive supports DVD CPRM authentication
    pub const DVD_CPRM: Self = Self(0x010B);
    /// Firmware creation date report
    pub const FIRMWARE_INFORMATION: Self = Self(0x010C);
    /// The ability to decode and optionally encode AACS protected information
    pub const AACS: Self = Self(0x010D);
    /// The ability to perform DVD CSS managed recording
    pub const DVD_CSS_MANAGED_RECORDING: Self = Self(0x010E);

    // --- 010Fh Reserved ---

    /// The ability to decode and optionally encode VCPS protected information
    pub const VCPS: Self = Self(0x0110);

    // --- 0111h - 0112h Reserved ---

    /// The ability to encode and decode SecurDisc protected information
    pub const SECUR_DISC: Self = Self(0x0113);

    // --- 0114h - 0141h Reserved ---

    /// TCG Optical Security Subsystem Class Feature
    pub const OSSC_FEATURE: Self = Self(0x0142);

    // --- 0143h - FEFFh Reserved ---

    // --- FF00h – FFFFh ---
    // Vendor Specific range; individual values intentionally not named.

    /// Returns `true` if this code falls within the `FF00h`–`FFFFh`
    /// vendor-specific range reserved by MMC.
    pub const fn is_vendor_specific(self) -> bool {
        self.0 >= 0xFF00
    }
}

// Might make sense to move this into the consuming crate. Don't necessarily want to force users into a specific Display impl.
impl Display for FeatureCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({:04X}h)",
            match *self {
                Self::PROFILE_LIST => "Profile List",
                Self::CORE => "Core",
                Self::MORPHING => "Morphing",
                c if c.is_vendor_specific() => "Vendor Specific",
                _ => todo!(),
            },
            self.0
        )
    }
}
