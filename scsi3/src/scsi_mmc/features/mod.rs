#[rustfmt::skip]
use ::core::{fmt, hash::{Hash, Hasher}};

use arbitrary_int::u4;
use derive_more::Debug;
use zerocopy::{
    BigEndian, FromBytes, Immutable, IntoBytes, KnownLayout, TryFromBytes, Unalign, Unaligned, U16,
};

macro_rules! bit {
    ($(#[$m:meta])* $name:ident: $field:ident[$bit:literal]) => {
        $(#[$m])*
        #[inline]
        pub const fn $name(&self) -> bool { self.$field & (1 << $bit) != 0 }
    };
}

use bit;

/// Implements `Debug` as a struct of what the given accessors return.
macro_rules! debug_fields {
    ($type:ident { $($accessor:ident),* $(,)? }) => {
        impl ::core::fmt::Debug for $type {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(stringify!($type))
                    $(.field(stringify!($accessor).trim_start_matches("r#"), &self.$accessor()))*
                    .finish()
            }
        }
    };
}

use debug_fields;

pub mod aacs;
pub mod bd_read;
pub mod bd_write;
pub mod bdr_pseudo_overwrite;
pub mod cd_audio_external_play;
pub mod cd_mastering;
pub mod cd_read;
pub mod cd_tao;
pub mod cdrw_cav_write;
pub mod cdrw_media_write_support;
pub mod core;
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

/// A Feature, implemented on the type that lays out its Feature Dependent Data.
///
/// Each Feature has one layout: that of the latest version this library knows. Later versions
/// of a Feature only define fields that earlier versions reserved, and reserved fields are zero,
/// so one layout reads every version. A field documented as "Since version N" reads as zero on a
/// Drive that reports an earlier version, and bytes that a newer version appends are left in
/// [`FeatureDescriptor::trailing`]. Versions whose data is shorter than the layout can't be read as
/// it, and [`FeatureDescriptor::data`] is `None` for them.
pub trait Feature:
    sealed::Sealed + FromBytes + IntoBytes + KnownLayout + Immutable + Unaligned
{
    const CODE: FeatureCode;
}

mod sealed {
    use super::*;

    pub trait Sealed {
        /// A 2-byte type whose only valid bit pattern is one big-endian feature code.
        type Code: TryFromBytes + IntoBytes + KnownLayout + Immutable + Copy + 'static;
    }

    /// The registry: the only place a Feature Code is tied to its layout.
    ///
    /// `Type = CONST;`, where `CONST` names the matching [`FeatureCode`] constant.
    macro_rules! features {
        ($($feature:ty = $code_name:ident;)*) => {
            $(
                #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
                #[derive(TryFromBytes, IntoBytes, KnownLayout, Immutable, Clone, Copy)]
                #[repr(u16)]
                pub enum $code_name {
                    BigEndian = u16::from_ne_bytes(u16::to_be_bytes(FeatureCode::$code_name.into_u16())),
                }

                impl Sealed for $feature {
                    type Code = $code_name;
                }

                impl Feature for $feature {
                    const CODE: FeatureCode = FeatureCode::$code_name;
                }
            )*
        };
    }

    features! {
        profile_list::ProfileList = PROFILE_LIST;
        core::Core = CORE;
        morphing::Morphing = MORPHING;
        removable_medium::RemovableMedium = REMOVABLE_MEDIUM;
        write_protect::WriteProtect = WRITE_PROTECT;
        random_readable::RandomReadable = RANDOM_READABLE;
        multi_read::MultiRead = MULTI_READ;
        cd_read::CdRead = CD_READ;
        dvd_read::DvdRead = DVD_READ;
        random_writable::RandomWritable = RANDOM_WRITABLE;
        incremental_streaming_writable::IncrementalStreamingWritable = INCREMENTAL_STREAMING_WRITABLE;
        sector_erasable::SectorErasable = SECTOR_ERASABLE;
        formattable::Formattable = FORMATTABLE;
        hardware_defect_management::HardwareDefectManagement = HARDWARE_DEFECT_MANAGEMENT;
        write_once::WriteOnce = WRITE_ONCE;
        restricted_overwrite::RestrictedOverwrite = RESTRICTED_OVERWRITE;
        cdrw_cav_write::CdrwCavWrite = CDRW_CAV_WRITE;
        mrw::Mrw = MRW;
        enhanced_defect_reporting::EnhancedDefectReporting = ENHANCED_DEFECT_REPORTING;
        dvd_plus_rw::DvdPlusRw = DVD_PLUS_RW;
        dvd_plus_r::DvdPlusR = DVD_PLUS_R;
        rigid_restricted_overwrite::RigidRestrictedOverwrite = RIGID_RESTRICTED_OVERWRITE;
        cd_tao::CdTao = CD_TAO;
        cd_mastering::CdMastering = CD_MASTERING;
        dvd_r_rw_write::DvdRRwWrite = DVD_R_RW_WRITE;
        double_density_cd_read::DoubleDensityCdRead = DOUBLE_DENSITY_CD_READ;
        double_density_cdr_write::DoubleDensityCdrWrite = DOUBLE_DENSITY_CDR_WRITE;
        double_density_cdrw_write::DoubleDensityCdrwWrite = DOUBLE_DENSITY_CDRW_WRITE;
        layer_jump_recording::LayerJumpRecording = LAYER_JUMP_RECORDING;
        layer_jump_rigid_restricted_overwrite::LayerJumpRigidRestrictedOverwrite =
            LAYER_JUMP_RIGID_RESTRICTED_OVERWRITE;
        stop_long_operation::StopLongOperation = STOP_LONG_OPERATION;
        cdrw_media_write_support::CdrwMediaWriteSupport = CDRW_MEDIA_WRITE_SUPPORT;
        bdr_pseudo_overwrite::BdrPseudoOverwrite = BDR_PSEUDO_OVERWRITE;
        dvd_plus_rw_dual_layer::DvdPlusRwDualLayer = DVD_PLUS_RW_DUAL_LAYER;
        dvd_plus_r_dual_layer::DvdPlusRDualLayer = DVD_PLUS_R_DUAL_LAYER;
        bd_read::BdRead = BD_READ;
        bd_write::BdWrite = BD_WRITE;
        tsr::Tsr = TSR;
        hddvd_read::HddvdRead = HDDVD_READ;
        hddvd_write::HddvdWrite = HDDVD_WRITE;
        hddvdrw_fragment_recording::HddvdrwFragmentRecording = HDDVDRW_FRAGMENT_RECORDING;
        hybrid_disc::HybridDisc = HYBRID_DISC;
        power_management::PowerManagement = POWER_MANAGEMENT;
        smart::Smart = SMART;
        embedded_changer::EmbeddedChanger = EMBEDDED_CHANGER;
        cd_audio_external_play::CdAudioExternalPlay = CD_AUDIO_EXTERNAL_PLAY;
        microcode_upgrade::MicrocodeUpgrade = MICROCODE_UPGRADE;
        timeout::Timeout = TIMEOUT;
        dvd_css::DvdCss = DVD_CSS;
        real_time_streaming::RealTimeStreaming = REAL_TIME_STREAMING;
        drive_serial_number::DriveSerialNumber = DRIVE_SERIAL_NUMBER;
        media_serial_number::MediaSerialNumber = MEDIA_SERIAL_NUMBER;
        disc_control_blocks::DiscControlBlocks = DISC_CONTROL_BLOCKS;
        dvd_cprm::DvdCprm = DVD_CPRM;
        firmware_information::FirmwareInformation = FIRMWARE_INFORMATION;
        aacs::Aacs = AACS;
        dvd_css_managed_recording::DvdCssManagedRecording = DVD_CSS_MANAGED_RECORDING;
        vcps::Vcps = VCPS;
        secur_disc::SecurDisc = SECUR_DISC;
        ossc::Ossc = OSSC;
    }
}

macro_rules! header {
    () => {
        #[inline]
        pub const fn version(&self) -> u4 {
            u4::extract_u8(self.ver_per_cur, 2)
        }

        /// Whether the Feature is always current.
        #[inline]
        pub const fn persistent(&self) -> bool {
            self.ver_per_cur & 1 << 1 != 0
        }

        /// Whether the Feature is currently active.
        #[inline]
        pub const fn current(&self) -> bool {
            self.ver_per_cur & 1 << 0 != 0
        }

        #[inline]
        pub const fn additional_length(&self) -> u8 {
            self.additional_length
        }

        /// The Feature Dependent Data, cut to Additional Length.
        ///
        /// Shorter than Additional Length only if the descriptor itself was cut short.
        #[inline]
        pub const fn raw_data(&self) -> &[u8] {
            match self.data.split_at_checked(self.additional_length as usize) {
                Some((data, _)) => data,
                None => &self.data,
            }
        }

        /// The Feature Dependent Data, cut to Additional Length.
        #[inline]
        pub const fn raw_data_mut(&mut self) -> &mut [u8] {
            let len = self.raw_data().len();
            self.data.split_at_mut(len).0
        }

        /// Adds the header fields after Feature Code.
        fn debug_header(&self, s: &mut fmt::DebugStruct<'_, '_>) {
            s.field("version", &self.version())
                .field("persistent", &self.persistent())
                .field("current", &self.current())
                .field("additional_length", &self.additional_length());
        }
    };
}

/// A Feature Descriptor in the generic format every Feature Descriptor shares, whatever its
/// Feature.
#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct GenericFeatureDescriptor {
    feature_code: FeatureCode,
    ver_per_cur: u8,
    additional_length: u8,
    data: [u8],
}

impl GenericFeatureDescriptor {
    #[inline]
    pub const fn feature_code(&self) -> FeatureCode {
        self.feature_code
    }

    header!();

    /// These same bytes as a descriptor of `F`, or `None` if this descriptor is for another
    /// Feature.
    pub fn get<F: Feature + ?Sized>(&self) -> Option<&FeatureDescriptor<F>> {
        FeatureDescriptor::<F>::try_ref_from_bytes(self.as_bytes()).ok()
    }

    /// These same bytes as a descriptor of `F`, or `None` if this descriptor is for another
    /// Feature.
    pub fn get_mut<F: Feature + ?Sized>(&mut self) -> Option<&mut FeatureDescriptor<F>> {
        FeatureDescriptor::<F>::try_mut_from_bytes(self.as_mut_bytes()).ok()
    }
}

impl fmt::Debug for GenericFeatureDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("GenericFeatureDescriptor");
        s.field("feature_code", &self.feature_code());
        self.debug_header(&mut s);
        s.field("raw_data", &self.raw_data()).finish()
    }
}

/// A Feature Descriptor whose Feature Code is that of `F`.
#[derive(TryFromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C)]
pub struct FeatureDescriptor<F: Feature + ?Sized> {
    feature_code: Unalign<F::Code>,
    ver_per_cur: u8,
    additional_length: u8,
    data: [u8],
}

impl<F: Feature + ?Sized> FeatureDescriptor<F> {
    header!();

    /// The Feature Dependent Data as `F`.
    ///
    /// `None` if the data is shorter than `F`, which happens for versions that predate `F`'s
    /// layout.
    pub fn data(&self) -> Option<&F> {
        F::ref_from_prefix(self.raw_data())
            .ok()
            .map(|(data, _)| data)
    }

    /// The Feature Dependent Data as `F`.
    ///
    /// `None` if the data is shorter than `F`, which happens for versions that predate `F`'s
    /// layout.
    pub fn data_mut(&mut self) -> Option<&mut F> {
        F::mut_from_prefix(self.raw_data_mut())
            .ok()
            .map(|(data, _)| data)
    }

    /// The Feature Dependent Data that follows [`Self::data`].
    ///
    /// Non-empty when a Drive reports a newer version than `F` describes, or when a list ends in a
    /// partial entry. Empty when [`Self::data`] is `None`.
    pub fn trailing(&self) -> &[u8] {
        F::ref_from_prefix(self.raw_data())
            .ok()
            .map_or(&[], |(_, trailing)| trailing)
    }
}

// Not derived: the Feature Code field is only comparable as bytes.
impl<F: Feature + ?Sized> PartialEq for FeatureDescriptor<F> {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl<F: Feature + ?Sized> Eq for FeatureDescriptor<F> {}

impl<F: Feature + ?Sized> Hash for FeatureDescriptor<F> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_bytes().hash(state);
    }
}

/// Shows the Feature Dependent Data as `F` with its trailing bytes, or as raw bytes when it's too
/// short to read as `F`.
impl<F: Feature + fmt::Debug + ?Sized> fmt::Debug for FeatureDescriptor<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("FeatureDescriptor");
        s.field("feature_code", &F::CODE);
        self.debug_header(&mut s);
        match self.data() {
            Some(data) => s.field("data", &data).field("trailing", &self.trailing()),
            None => s.field("raw_data", &self.raw_data()),
        };
        s.finish()
    }
}

/// A feature code as defined by MMC.
///
/// Feature codes identify the type of a Feature Descriptor returned by the
/// GET CONFIGURATION command. The set of codes is enumerable but
/// non-exhaustive as reserved ranges may be defined by future spec
/// revisions, and the `FF00h`–`FFFFh` range is permanently reserved for
/// vendor-specific use.
///
/// Ordered by code, the order a Drive returns Feature Descriptors in.
#[derive(
    FromBytes,
    IntoBytes,
    KnownLayout,
    Immutable,
    Unaligned,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[repr(transparent)]
pub struct FeatureCode(#[debug("{_0:04X}h")] U16<BigEndian>);

impl FeatureCode {
    /// A list of all Profiles supported by the Drive
    pub const PROFILE_LIST: Self = Self::from_u16(0x0000);
    /// Mandatory behavior for all devices
    pub const CORE: Self = Self::from_u16(0x0001);
    /// The Drive is able to report operational changes to the Host and accept. Host requests to
    /// prevent operational changes.
    pub const MORPHING: Self = Self::from_u16(0x0002);
    /// The medium may be removed from the device
    pub const REMOVABLE_MEDIUM: Self = Self::from_u16(0x0003);
    /// The ability to control Write Protection status
    pub const WRITE_PROTECT: Self = Self::from_u16(0x0004);

    // --- 0005h - 000Fh Reserved ---

    /// The ability to read sectors with random addressing
    pub const RANDOM_READABLE: Self = Self::from_u16(0x0010);

    // --- 0011h - 001Ch Reserved ---

    /// The Drive is able to read all CD media types; based on OSTA MultiRead
    pub const MULTI_READ: Self = Self::from_u16(0x001D);
    /// The ability to read CD specific structures
    pub const CD_READ: Self = Self::from_u16(0x001E);
    /// The ability to read DVD specific structures
    pub const DVD_READ: Self = Self::from_u16(0x001F);
    /// Write support for randomly addressed writes
    pub const RANDOM_WRITABLE: Self = Self::from_u16(0x0020);
    /// Write support for sequential recording
    pub const INCREMENTAL_STREAMING_WRITABLE: Self = Self::from_u16(0x0021);
    /// Write support for erasable media and media that requires an erase pass
    /// before overwrite.
    pub const SECTOR_ERASABLE: Self = Self::from_u16(0x0022);
    /// Support for formatting of media.
    pub const FORMATTABLE: Self = Self::from_u16(0x0023);
    /// Ability of the Drive/media system to provide an apparently defect-free space.
    pub const HARDWARE_DEFECT_MANAGEMENT: Self = Self::from_u16(0x0024);
    /// Write support for write-once media that is writable in random order.
    pub const WRITE_ONCE: Self = Self::from_u16(0x0025);
    /// Write support for media that shall be written from Blocking boundaries only.
    pub const RESTRICTED_OVERWRITE: Self = Self::from_u16(0x0026);
    /// The ability to write high speed CD-RW media
    pub const CDRW_CAV_WRITE: Self = Self::from_u16(0x0027);
    /// The ability to recognize and read and optionally write MRW formatted media
    pub const MRW: Self = Self::from_u16(0x0028);
    /// The ability to control RECOVERED ERROR reporting
    pub const ENHANCED_DEFECT_REPORTING: Self = Self::from_u16(0x0029);
    /// The ability to recognize, read and optionally write DVD+RW media
    pub const DVD_PLUS_RW: Self = Self::from_u16(0x002A);
    /// The ability to read DVD+R recorded media formats
    pub const DVD_PLUS_R: Self = Self::from_u16(0x002B);
    /// Write support for media that is required to be written from Blocking boundaries with length
    /// of integral multiple Blocking size only.
    pub const RIGID_RESTRICTED_OVERWRITE: Self = Self::from_u16(0x002C);
    /// Ability to write CD with Track at Once recording
    pub const CD_TAO: Self = Self::from_u16(0x002D);
    /// The ability to write CD with Session at Once or Raw write methods.
    pub const CD_MASTERING: Self = Self::from_u16(0x002E);
    /// The ability to write DVD specific structures
    pub const DVD_R_RW_WRITE: Self = Self::from_u16(0x002F);
    /// The ability to read user data from DDCD blocks.
    pub const DOUBLE_DENSITY_CD_READ: Self = Self::from_u16(0x0030);
    /// The ability to write and read DDCD-R media.
    pub const DOUBLE_DENSITY_CDR_WRITE: Self = Self::from_u16(0x0031);
    /// The ability to write and read DDCD-RW media
    pub const DOUBLE_DENSITY_CDRW_WRITE: Self = Self::from_u16(0x0032);
    /// The ability to record in layer jump mode
    pub const LAYER_JUMP_RECORDING: Self = Self::from_u16(0x0033);
    /// The ability to perform Layer Jump recording on Rigid Restricted Overwritable media
    pub const LAYER_JUMP_RIGID_RESTRICTED_OVERWRITE: Self = Self::from_u16(0x0034);
    /// The ability to stop the long immediate operation by a command.
    pub const STOP_LONG_OPERATION: Self = Self::from_u16(0x0035);

    // --- 0036h Reserved ---

    /// The ability to report CD–RW media sub-types that are supported for write
    pub const CDRW_MEDIA_WRITE_SUPPORT: Self = Self::from_u16(0x0037);
    /// Logical Block overwrite service on BD-R discs formatted as SRM+POW.
    pub const BDR_PSEUDO_OVERWRITE: Self = Self::from_u16(0x0038);

    // --- 0039h Reserved ---

    /// The ability to read DVD+RW Dual Layer recorded media formats
    pub const DVD_PLUS_RW_DUAL_LAYER: Self = Self::from_u16(0x003A);
    /// The ability to read DVD+R Dual Layer recorded media formats
    pub const DVD_PLUS_R_DUAL_LAYER: Self = Self::from_u16(0x003B);

    // --- 003Ch - 003Fh Reserved ---

    /// The ability to read control structures and user data from a BD disc
    pub const BD_READ: Self = Self::from_u16(0x0040);
    /// The ability to write control structures and user data to certain BD discs
    pub const BD_WRITE: Self = Self::from_u16(0x0041);
    /// Timely, Safe Recording permits the Host to schedule defect management.
    pub const TSR: Self = Self::from_u16(0x0042);

    // --- 0043h - 004Fh Reserved ---

    /// The ability to read control structures and user data from a HD DVD disc
    pub const HDDVD_READ: Self = Self::from_u16(0x0050);
    /// The ability to write control structures and user data to certain HD DVD discs
    pub const HDDVD_WRITE: Self = Self::from_u16(0x0051);
    /// The ability to record HD DVD-RW in fragment recording mode
    pub const HDDVDRW_FRAGMENT_RECORDING: Self = Self::from_u16(0x0052);

    // --- 0053h - 007Fh Reserved ---

    /// The ability to access some Hybrid Discs.
    pub const HYBRID_DISC: Self = Self::from_u16(0x0080);

    // --- 0081h - 00FFh Reserved ---

    /// Host and device directed power management
    pub const POWER_MANAGEMENT: Self = Self::from_u16(0x0100);
    /// Ability to perform Self Monitoring Analysis and Reporting Technology
    pub const SMART: Self = Self::from_u16(0x0101);
    /// Single mechanism multiple disc changer
    pub const EMBEDDED_CHANGER: Self = Self::from_u16(0x0102);
    /// Ability to play audio CDs via the Logical Unit's own analog output
    pub const CD_AUDIO_EXTERNAL_PLAY: Self = Self::from_u16(0x0103);
    /// Ability for the device to accept new microcode via the interface
    pub const MICROCODE_UPGRADE: Self = Self::from_u16(0x0104);
    /// Ability to respond to all commands within a specific time
    pub const TIMEOUT: Self = Self::from_u16(0x0105);
    /// Ability to perform DVD CSS/CPPM authentication and RPC
    pub const DVD_CSS: Self = Self::from_u16(0x0106);
    /// Ability to read and write using Host requested performance parameters
    pub const REAL_TIME_STREAMING: Self = Self::from_u16(0x0107);
    /// The Drive has a unique identifier
    pub const DRIVE_SERIAL_NUMBER: Self = Self::from_u16(0x0108);
    /// Ability to return unique Media Serial Number
    pub const MEDIA_SERIAL_NUMBER: Self = Self::from_u16(0x0109);
    /// The ability to read and/or write DCBs
    pub const DISC_CONTROL_BLOCKS: Self = Self::from_u16(0x010A);
    /// The Drive supports DVD CPRM authentication
    pub const DVD_CPRM: Self = Self::from_u16(0x010B);
    /// Firmware creation date report
    pub const FIRMWARE_INFORMATION: Self = Self::from_u16(0x010C);
    /// The ability to decode and optionally encode AACS protected information
    pub const AACS: Self = Self::from_u16(0x010D);
    /// The ability to perform DVD CSS managed recording
    pub const DVD_CSS_MANAGED_RECORDING: Self = Self::from_u16(0x010E);

    // --- 010Fh Reserved ---

    /// The ability to decode and optionally encode VCPS protected information
    pub const VCPS: Self = Self::from_u16(0x0110);

    // --- 0111h - 0112h Reserved ---

    /// The ability to encode and decode SecurDisc protected information
    pub const SECUR_DISC: Self = Self::from_u16(0x0113);

    // --- 0114h - 0141h Reserved ---

    /// TCG Optical Security Subsystem Class Feature
    pub const OSSC: Self = Self::from_u16(0x0142);

    // --- 0143h - FEFFh Reserved ---

    // --- FF00h – FFFFh ---
    // Vendor Specific range; individual values intentionally not named.

    /// Returns `true` if this code falls within the `FF00h`–`FFFFh`
    /// vendor-specific range reserved by MMC.
    #[inline]
    pub const fn is_vendor_specific(self) -> bool {
        self.into_u16() >= 0xFF00
    }

    #[inline]
    pub const fn from_u16(number: u16) -> Self {
        Self(U16::new(number))
    }

    #[inline]
    pub const fn into_u16(self) -> u16 {
        self.0.get()
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use arbitrary_int::u2;

    use super::{
        bd_read::BdRead,
        cd_read::CdRead,
        cdrw_media_write_support::CdrwMediaWriteSupport,
        core::{Core, PhysicalInterfaceStandard},
        drive_serial_number::DriveSerialNumber,
        incremental_streaming_writable::IncrementalStreamingWritable,
        morphing::Morphing,
        ossc::Ossc,
        profile_list::ProfileList,
        random_writable::RandomWritable,
        sector_erasable::SectorErasable,
        *,
    };
    use crate::mmc::profiles::ProfileNumber;

    /// `buf` as one descriptor, which its Additional Length must span exactly.
    fn descriptor(buf: &[u8]) -> &GenericFeatureDescriptor {
        GenericFeatureDescriptor::ref_from_bytes_with_elems(buf, buf[3].into()).unwrap()
    }

    #[test]
    fn newer_version_reads_through_latest_layout() {
        // Core v3, 4 bytes longer than the v2 layout
        let buf = [
            0x00, 0x01, 0b1111, 12, 0, 0, 0, 2, 0b11, 0, 0, 0, 0xAA, 0xBB, 0xCC, 0xDD,
        ];
        let core = descriptor(&buf).get::<Core>().unwrap();
        assert_eq!(core.version(), u4::new(3));

        let data = core.data().unwrap();
        assert_eq!(
            data.physical_interface_standard(),
            PhysicalInterfaceStandard::ATAPI
        );
        assert!(data.inq2());
        assert!(data.device_busy_event());
        assert_eq!(core.trailing(), [0xAA, 0xBB, 0xCC, 0xDD]);
    }

    #[test]
    fn core_v0_reads_through_v0_layout() {
        let v0 = [0x00, 0x01, 0b0011, 4, 0, 0, 0, 8];
        let core = descriptor(&v0).get::<Core>().unwrap();
        assert!(core.data().is_none());
        assert!(core.trailing().is_empty());
        assert_eq!(
            core.data_v0().unwrap().physical_interface_standard(),
            PhysicalInterfaceStandard::USB
        );

        let v2 = [0x00, 0x01, 0b1011, 8, 0, 0, 0, 8, 0b11, 0, 0, 0];
        let core = descriptor(&v2).get::<Core>().unwrap();
        assert_eq!(
            core.data_v0().unwrap().physical_interface_standard(),
            PhysicalInterfaceStandard::USB
        );
    }

    #[test]
    fn mismatched_feature_code_is_rejected() {
        let buf = [0x00, 0x01, 0b0011, 4, 0, 0, 0, 8];
        assert!(descriptor(&buf).get::<CdRead>().is_none());
    }

    #[test]
    fn version_without_data() {
        let v0 = [0x00, 0x1E, 0b0011, 0];
        assert!(descriptor(&v0).get::<CdRead>().unwrap().data().is_none());

        let v2 = [0x00, 0x1E, 0b1011, 4, 0x83, 0, 0, 0];
        let data = descriptor(&v2).get::<CdRead>().unwrap().data().unwrap();
        assert!(data.digital_audio_play());
        assert!(data.c2_flags());
        assert!(data.cd_text());
    }

    #[test]
    fn feature_without_data() {
        let buf = [0x00, 0x22, 0b0011, 0];
        assert!(descriptor(&buf)
            .get::<SectorErasable>()
            .unwrap()
            .data()
            .is_some());
    }

    #[test]
    fn list_leaves_partial_entry_in_trailing() {
        let buf = [0x00, 0x00, 0b0011, 6, 0x00, 0x08, 0b1, 0, 0x00, 0x10];
        let list = descriptor(&buf).get::<ProfileList>().unwrap();

        let profiles = list.data().unwrap().profiles();
        assert_eq!(profiles.len(), 1);
        assert_eq!(
            profiles[0].profile_number(),
            ProfileNumber::from_u16(0x0008)
        );
        assert!(profiles[0].current_profile());
        assert_eq!(list.trailing(), [0x00, 0x10]);
    }

    #[test]
    fn list_with_fixed_head() {
        // ISW v3: TRIO and BUF set, 2 Link Sizes, 2 pad bytes
        let buf = [0x00, 0x21, 0b1111, 8, 0x00, 0x01, 0b101, 2, 1, 7, 0, 0];
        let data = descriptor(&buf)
            .get::<IncrementalStreamingWritable>()
            .unwrap()
            .data()
            .unwrap();
        assert_eq!(data.data_block_type_supported(), 0x0001);
        assert!(data.track_resource_info_open());
        assert!(!data.address_mode_reservation());
        assert!(data.buf());
        assert_eq!(data.link_sizes(), [1, 7]);

        // Number of Link Sizes larger than the bytes present
        let buf = [0x00, 0x21, 0b1111, 6, 0x00, 0x01, 0b101, 5, 1, 7];
        let data = descriptor(&buf)
            .get::<IncrementalStreamingWritable>()
            .unwrap()
            .data()
            .unwrap();
        assert_eq!(data.number_of_link_sizes(), 5);
        assert_eq!(data.link_sizes(), [1, 7]);
    }

    #[test]
    fn list_with_count_ignores_entries_past_count() {
        // OSSC: ME set, 2 Profile Numbers, then an entry that would make Additional Length a
        // multiple of 4
        let buf = [0x01, 0x42, 0b0011, 8, 0b1, 2, 0x00, 0x41, 0x00, 0x43, 0, 0];
        let data = descriptor(&buf).get::<Ossc>().unwrap().data().unwrap();
        assert!(data.mandatory_encryption());
        assert_eq!(
            data.profile_numbers(),
            [ProfileNumber::BD_R_SRM, ProfileNumber::BD_RE]
        );

        // Number of Profiles larger than the list
        let buf = [0x01, 0x42, 0b0011, 4, 0, 2, 0x00, 0x41];
        let data = descriptor(&buf).get::<Ossc>().unwrap().data().unwrap();
        assert_eq!(data.number_of_profiles(), 2);
        assert_eq!(data.profile_numbers(), [ProfileNumber::BD_R_SRM]);
    }

    #[test]
    fn string_takes_all_data() {
        let buf = [
            0x01, 0x08, 0b0011, 8, b'S', b'N', b'0', b'1', b'2', b'3', b' ', b' ',
        ];
        let data = descriptor(&buf)
            .get::<DriveSerialNumber>()
            .unwrap()
            .data()
            .unwrap();
        assert_eq!(data.serial_number().as_bytes(), b"SN0123  ");
    }

    #[test]
    fn explicit_flags_read_the_obsolete_bitmaps() {
        // BD Read v1, indexed by descriptor byte
        let mut buf = [0; 32];
        buf[..4].copy_from_slice(&[0x00, 0x40, 0b0111, 28]);
        buf[4] = 0b1; // BCA
        buf[9] = 0b110; // Class 0 BD-RE: RE2, RE1
        buf[17] = 0b10; // Class 0 BD-R: R
        buf[18..20].copy_from_slice(&[0x80, 0x01]); // Class 1 BD-R: obsolete bits
        buf[25] = 0b10; // Class 0 BD-ROM: ROM

        let data = descriptor(&buf).get::<BdRead>().unwrap().data().unwrap();
        assert!(data.bca() && data.re2() && data.re1() && data.r() && data.rom());
        assert_eq!(data.bd_re_read_support(u2::new(0)), 0b110);
        assert_eq!(data.bd_r_read_support(u2::new(1)), 0x8001);
        assert_eq!(data.bd_rom_read_support(u2::new(0)), 0b10);
        assert_eq!(data.bd_rom_read_support(u2::new(3)), 0);
    }

    #[test]
    fn data_is_cut_to_additional_length() {
        // A Profile List cast together with the header of the descriptor after it
        let buf = [
            0x00, 0x00, 0b0011, 4, 0x00, 0x08, 0b1, 0, 0x00, 0x01, 0b0011, 0,
        ];
        let list = FeatureDescriptor::<ProfileList>::try_ref_from_bytes(&buf).unwrap();
        assert_eq!(list.raw_data().len(), 4);
        assert_eq!(list.data().unwrap().profiles().len(), 1);
    }

    #[test]
    fn data_shorter_than_layout_is_not_read() {
        // Random Writable v0 as laid out in MMC-2r09
        let v0 = [0x00, 0x20, 0b0011, 8, 0, 0, 0x10, 0, 0, 0x10, 0b1, 0];
        let random_writable = descriptor(&v0).get::<RandomWritable>().unwrap();
        assert!(random_writable.data().is_none());
        assert_eq!(random_writable.raw_data().len(), 8);

        let v1 = [
            0x00, 0x20, 0b0111, 12, 0, 0, 0x10, 0, 0, 0, 0x08, 0, 0, 0x10, 0b1, 0,
        ];
        let data = descriptor(&v1)
            .get::<RandomWritable>()
            .unwrap()
            .data()
            .unwrap();
        assert_eq!(data.last_logical_block_address(), 0x1000);
        assert_eq!(data.logical_block_size(), 2048);
        assert_eq!(data.blocking(), 0x10);
        assert!(data.page_present());
    }

    #[test]
    fn debug_shows_accessors() {
        let buf = [0x00, 0x1E, 0b1011, 4, 0x83, 0, 0, 0];
        assert_eq!(
            format!("{:?}", descriptor(&buf).get::<CdRead>().unwrap()),
            "FeatureDescriptor { feature_code: FeatureCode(001Eh), version: 2, persistent: true, \
             current: true, additional_length: 4, data: CdRead { digital_audio_play: true, \
             c2_flags: true, cd_text: true }, trailing: [] }"
        );

        // Raw identifiers print without `r#`
        let buf = [0x00, 0x02, 0b0111, 4, 0b11, 0, 0, 0];
        let data = descriptor(&buf).get::<Morphing>().unwrap().data().unwrap();
        assert_eq!(
            format!("{data:?}"),
            "Morphing { oc_event: true, async: true }"
        );

        // Flags print by name
        let buf = [0x00, 0x37, 0b0011, 4, 0, 0b11, 0, 0];
        let data = descriptor(&buf)
            .get::<CdrwMediaWriteSupport>()
            .unwrap()
            .data()
            .unwrap();
        assert_eq!(
            format!("{data:?}"),
            "CdrwMediaWriteSupport { cd_rw_media_subtype_support: \
             CdrwMediaSubtypeSupport(STANDARD | HIGH_SPEED) }"
        );
    }

    #[test]
    fn debug_shows_raw_data_shorter_than_layout() {
        // Random Writable v0 as laid out in MMC-2r09
        let buf = [0x00, 0x20, 0b0011, 8, 0, 0, 0x10, 0, 0, 0x10, 0b1, 0];
        assert_eq!(
            format!("{:?}", descriptor(&buf).get::<RandomWritable>().unwrap()),
            "FeatureDescriptor { feature_code: FeatureCode(0020h), version: 0, persistent: true, \
             current: true, additional_length: 8, raw_data: [0, 0, 16, 0, 0, 16, 1, 0] }"
        );
    }

    #[test]
    fn equality_includes_bytes_past_additional_length() {
        // A Profile List cast together with the header of the descriptor after it
        let a = [
            0x00, 0x00, 0b0011, 4, 0x00, 0x08, 0b1, 0, 0x00, 0x01, 0b0011, 0,
        ];
        let mut b = a;
        b[8..].fill(0xFF);

        let typed = FeatureDescriptor::<ProfileList>::try_ref_from_bytes;
        assert_ne!(typed(&a).unwrap(), typed(&b).unwrap());
        let generic = GenericFeatureDescriptor::ref_from_bytes;
        assert_ne!(generic(&a).unwrap(), generic(&b).unwrap());
    }

    #[test]
    fn writes_reach_the_original_buffer() {
        let mut buf = [0x00, 0x01, 0b1011, 8, 0, 0, 0, 2, 0, 0, 0, 0];

        let additional_length = buf[3].into();
        let generic =
            GenericFeatureDescriptor::mut_from_bytes_with_elems(&mut buf, additional_length)
                .unwrap();
        let core = generic.get_mut::<Core>().unwrap().data_mut().unwrap();
        core.as_mut_bytes()[4] |= 1;

        assert_eq!(buf[8], 1);
        assert!(descriptor(&buf)
            .get::<Core>()
            .unwrap()
            .data()
            .unwrap()
            .device_busy_event());
    }
}
