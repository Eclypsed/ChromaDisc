use bitflags::bitflags;

pub mod q_subchannel {
    use super::bitflags;
    use num_enum::TryFromPrimitive;

    /// 4 bits of identification for DATA-Q. This is also known as the Mode (ADR) Q.
    ///
    /// See MMC-6 §4.2.3.4, Table 17.
    #[repr(u8)]
    #[derive(Debug, Clone, Copy, TryFromPrimitive)]
    pub enum Adr {
        /// See:
        /// | Area     | Reference        |
        /// | -------- | ---------------- |
        /// | Program  | MMC-6 §4.2.3.5.2 |
        /// | Lead-out | MMC-6 §4.2.3.6   |
        /// | Lead-in  | MMC-6 §4.2.3.7.2 |
        Mode1Q = 0b0001,
        /// See:
        /// | Area     | Reference        |
        /// | -------- | ---------------- |
        /// | Program  | MMC-6 §4.2.3.5.3 |
        /// | Lead-out | MMC-6 §4.2.3.6   |
        /// | Lead-in  | MMC-6 §4.2.3.7.3 |
        Mode2Q = 0b0010,
        /// See:
        /// | Area    | Reference        |
        /// | ------- | ---------------- |
        /// | Program | MMC-6 §4.2.3.5.4 |
        Mode3Q = 0b0011,
        /// See:
        /// | Area    | Reference        |
        /// | ------- | ---------------- |
        /// | Lead-in | MMC-6 §4.2.3.7.4 |
        Mode5Q = 0b0101,
    }

    bitflags! {
        /// The Control Field has 4 bits that define the type of information in the frame.
        ///
        /// See MMC-6 §4.2.3.4, Table 17.
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct Control: u8 {
            /// If set and track is audio, the track has 4 channels, otherwise 2. Not set when
            /// track is data.
            ///
            /// # Examples
            ///
            /// - 00xx = 2 audio channels
            /// - 10xx = 4 audio channels
            const FOUR_CHANNELS = 1 << 3;
            /// If set, track is data, otherwise audio.
            ///
            /// # Examples
            ///
            /// - x0xx = Audio Track
            /// - 01xx = Data Track
            const IS_DATA = 1 << 2;
            /// If set digital copy is permitted, otherwise prohibited.
            ///
            /// # Examples
            ///
            /// - xx0x = Digital copy prohibited
            /// - xx1x = Digital copy permitted
            const COPY_PERMITTED = 1 << 1;
            /// If set and track is audio, pre-emphasis is enabled. If set and track is data, track
            /// is recorded incrementally, otherwise uninterrupted.
            ///
            /// # Examples
            ///
            /// - x0x0 = Audio without pre-emphasis
            /// - x0x1 = Audio with pre-emphasis
            /// - 01x0 = Data track recorded uninterrupted
            /// - 01x1 = Data track recorded incrementally
            const PREEMPHASIS_OR_INCREMENTAL = 1 << 0;
        }
    }
}

pub mod spc {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum Version {
        /// The device does not claim conformance to any standard.
        NoConformance = 0x00,
        /// The device complies to ANSI INCITS 301-1997 (SPC).
        Spc = 0x03,
        /// The device complies to ANSI INCITS 351-2001 (SPC-2).
        Spc2 = 0x04,
        /// The device complies to ANSI INCITS 408-2005 (SPC-3).
        Spc3 = 0x05,
        /// The device complies to ANSI INCITS 513-2015 (SPC-4)
        Spc4 = 0x06,
        /// The device complies to T10/BSR INCITS 503 (SPC-6)
        Spc6 = 0x07,
        Obselete(u8),
        Reserved(u8),
    }

    impl From<u8> for Version {
        fn from(value: u8) -> Self {
            match value {
                0x00 => Self::NoConformance,
                0x03 => Self::Spc,
                0x04 => Self::Spc2,
                0x05 => Self::Spc3,
                0x06 => Self::Spc4,
                0x07 => Self::Spc6,
                v @ (0x01
                | 0x02
                | 0x08..=0x0C
                | 0x40..=0x44
                | 0x48..=0x4C
                | 0x80..=0x84
                | 0x88..=0x8C) => Self::Obselete(v),
                v => Self::Reserved(v),
            }
        }
    }
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureCode {
    /// A list of all Profiles supported by the Drive
    ProfileList = 0x0000,
    /// Mandatory behavior for all devices
    Core = 0x0001,
    /// The Drive is able to report operational changes to the Host and accept. Host requests to
    /// prevent operational changes.
    Morphing = 0x0002,
    /// The medium may be removed from the device
    RemoveableMedium = 0x0003,
    /// The ability to control Write Protection status
    WriteProtected = 0x0004,
    /// The ability to read sectors with random addressing
    RandomReadable = 0x0010,
    /// The Drive is able to read all CD media types; based on OSTA MultiRead
    MultiRead = 0x001D,
    /// The ability to read CD specific structures
    CDRead = 0x001E,
    /// The ability to read DVD specific structures
    DVDRead = 0x001F,
    /// Write support for randomly addressed writes
    RandomWriteable = 0x0020,
    /// Write support for sequential recording
    IncrementalStreamingWritable = 0x0021,
    /// Support for formatting of media.
    Formattable = 0x0023,
    /// Ability of the Drive/media system to provide an apparently defect-free space.
    HardwareDefectManagement = 0x0024,
    /// Write support for write-once media that is writable in random order.
    WriteOnce = 0x0025,
    /// Write support for media that shall be written from Blocking boundaries only.
    RestrictedOverwrite = 0x0026,
    /// The ability to write high speed CD-RW media
    CDrwCAVWrite = 0x0027,
    /// The ability to recognize and read and optionally write MRW formatted media
    Mrw = 0x0028,
    /// The ability to control RECOVERED ERROR reporting
    EnhancedDefectReporting = 0x0029,
    /// The ability to recognize, read and optionally write DVD+RW media
    DVDPlusrw = 0x002A,
    /// The ability to read DVD+R recorded media formats
    DVDPlusr = 0x002B,
    /// Write support for media that is required to be written from Blocking boundaries with length
    /// of integral multiple Blocking size only.
    RigidRestrictedOverwrite = 0x002C,
    /// Ability to write CD with Track at Once recording
    CDTrackAtOnce = 0x002D,
    /// The ability to write CD with Session at Once or Raw write methods.
    CDMastering = 0x002E,
    /// The ability to write DVD specific structures
    DVDrrwWrite = 0x002F,
    /// The ability to record in layer jump mode
    LayerJumpRecording = 0x0033,
    /// The ability to perform Layer Jump recording on Rigid Restricted Overwritable media
    LJRigidRestrictedOverwrite = 0x0034,
    /// The ability to stop the long immediate operation by a command.
    StopLongOperation = 0x0035,
    /// The ability to report CD –RW media sub-types that are supported for write
    CDrwMediaWriteSupport = 0x0037,
    /// Logical Block overwrite service on BD-R discs formatted as SRM+POW.
    BDrPOW = 0x0038,
    /// The ability to read DVD+RW Dual Layer recorded media formats
    DVDPlusrwDualLayer = 0x003A,
    /// The ability to read DVD+R Dual Layer recorded media formats
    DVDPlusrDualLayer = 0x003B,
    /// The ability to read control structures and user data from a BD disc
    BDReadFeature = 0x0040,
    /// The ability to write control structures and user data to certain BD discs
    BDWriteFeature = 0x0041,
    /// Timely, Safe Recording permits the Host to schedule defect management.
    Tsr = 0x0042,
    /// The ability to read control structures and user data from a HD DVD disc
    HDDVDRead = 0x0050,
    /// The ability to write control structures and user data to certain HD DVD discs
    HDDVDWrite = 0x0051,
    /// The ability to record HD DVD-RW in fragment recording mode
    HDDVDrwFragmentRecording = 0x0052,
    /// The ability to access some Hybrid Discs.
    HybridDisc = 0x0080,
    /// Host and device directed power management
    PowerManagement = 0x0100,
    /// Ability to perform Self Monitoring Analysis and Reporting Technology
    Smart = 0x0101,
    /// Single mechanism multiple disc changer
    EmbeddedChanger = 0x0102,
    /// Ability for the device to accept new microcode via the interface
    MicrocodeUpgrade = 0x0104,
    /// Ability to respond to all commands within a specific time
    Timeout = 0x0105,
    /// Ability to perform DVD CSS/CPPM authentication and RPC
    DVDcss = 0x0106,
    /// Ability to read and write using Host requested performance parameters
    RealTimeStreaming = 0x0107,
    /// The Drive has a unique identifier
    DriveSerialNumber = 0x0108,
    /// The ability to read and/or write DCBs
    DCBs = 0x010A,
    /// The Drive supports DVD CPRM authentication
    DVDcprm = 0x010B,
    /// Firmware creation date report
    FirmwareInformation = 0x010C,
    /// The ability to decode and optionally encode AACS protected information
    Aacs = 0x010D,
    /// The ability to perform DVD CSS managed recording
    DVDcssManagedRecording = 0x010E,
    /// The ability to decode and optionally encode VCPS protected information
    Vcps = 0x0110,
    /// The ability to encode and decode SecurDisc protected information
    SecurDisc = 0x0113,
    /// TCG Optical Security Subsystem Class Feature
    OSSCFeature = 0x0142,
}

/// A 16-bit value representing a Drive Profile.
///
/// See MMC-6 §5.3.1, Table 95.
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Profile {
    /// Unknown Profile with a currently reserved Profile Number.
    Reserved(u16),
    /// Non-removable disk profile.
    ///
    /// Obsolete as of MMC-6, see MMC-5 §5.4.3
    NonRemovableDisk = 0x0001,
    /// Re-writable; with removable media
    ///
    /// See MMC-6 §5.4.3
    RemovableDisk = 0x0002,
    /// Magneto-Optical disk with sector erase capability.
    ///
    /// Legacy as of MMC-6, see MMC-5 §5.4.5
    MoErasable = 0x0003,
    /// Optical write once.
    ///
    /// Legacy as of MMC-6, see MMC-5 §5.4.6
    OpticalWriteOnce = 0x0004,
    /// Advance Storage – Magneto-Optical.
    ///
    /// Legacy as of MMC06, see MMC-5 §5.4.7
    AsMo = 0x0005,
    /// Read only Compact Disc capable
    ///
    /// See MMC-6 §5.4.4
    CdRom = 0x0008,
    /// Write once Compact Disc capable
    ///
    /// See MMC-6 §5.4.5
    CdR = 0x0009,
    /// Re-writable Compact Disc capable
    ///
    /// See MMC-6 §5.4.6
    CdRw = 0x000A,
    /// Read only DVD
    ///
    /// See MMC-6 §5.4.7
    DvdRom = 0x0010,
    /// Write once DVD using Sequential recording
    ///
    /// See MMC-6 §5.4.8
    DvdRSequentialRecording = 0x0011,
    /// Re-writable DVD
    ///
    /// See MMC-6 §5.4.9
    DvdRam = 0x0012,
    /// Re-recordable DVD using Restricted Overwrite
    ///
    /// See MMC-6 §5.4.10
    DvdRwRestrictedOverwrite = 0x0013,
    /// Re-recordable DVD using Sequential recording
    ///
    /// See MMC-6 §5.4.11
    DvdRwSequentialRecording = 0x0014,
    /// Dual Layer DVD-R using Sequential recording
    ///
    /// See MMC-6 §5.4.12
    DvdRDualLayerSequentialRecording = 0x0015,
    /// Dual Layer DVD-R using Layer Jump recording
    ///
    /// See MMC-6 §5.4.13
    DvdRDualLayerJumpRecording = 0x0016,
    /// Dual Layer DVD-RW recording
    ///
    /// See MMC-6 §5.4.14
    DvdRwDualLayer = 0x0017,
    /// Write once DVD for CSS managed recording
    ///
    /// See MMC-6 §5.4.15
    DvdDownloadDiscRecording = 0x0018,
    /// DVD+ReWritable
    ///
    /// See MMC-6 §5.4.16
    DvdPlusRw = 0x001A,
    /// DVD+Recordable
    ///
    /// See MMC-6 §5.4.17
    DvdPlusR = 0x001B,
    /// The DDCD-ROM Profile
    ///
    /// Legacy as of MMC-5, see MMC-4 §7.4.16
    DdcdRom = 0x0020,
    /// The DDCD-R Profile
    ///
    /// Legacy as of MMC-5, see MMC-4 §7.4.17
    DdcdR = 0x0021,
    /// The DDCD-RW Profile
    ///
    /// Legacy as of MMC-5, see MMC-4 §7.4.18
    DdcdRw = 0x0022,
    /// DVD+Rewritable Dual Layer
    ///
    /// See MMC-6 §5.4.18
    DvdPlusRwDualLayer = 0x002A,
    /// DVD+Recordable Dual Layer
    ///
    /// See MMC-6 §5.4.19
    DvdPlusRDualLayer = 0x002B,
    /// Blu-ray Disc ROM
    ///
    /// See MMC-6 §5.4.20
    BdRom = 0x0040,
    /// Blu-ray Disc Recordable – Sequential Recording Mode
    ///
    /// See MMC-6 §5.4.21
    BdRSrm = 0x0041,
    /// Blu-ray Disc Recordable – Random Recording Mode
    ///
    /// See MMC-6 §5.4.22
    BdRRrm = 0x0042,
    /// Blu-ray Disc Rewritable
    ///
    /// See MMC-6 §5.4.23
    BdRe = 0x0043,
    /// Read-only HD DVD
    ///
    /// See MMC-6 §5.4.24
    HdDvdRom = 0x0050,
    /// Write-once HD DVD
    ///
    /// See MMC-6 §5.4.25
    HdDvdR = 0x0051,
    /// Rewritable HD DVD
    ///
    /// See MMC-6 §5.4.26
    HdDvdRam = 0x0052,
    /// Rewritable HD DVD
    ///
    /// See MMC-6 §5.4.27
    HdDvdRw = 0x0053,
    /// Dual Layer Write-once HD DVD
    ///
    /// See MMC-6 §5.4.28
    HdDvdRDualLayer = 0x0058,
    /// Dual Layer Rewritable HD DVD
    ///
    /// See MMC-6 §5.4.29
    HdDvdRwDualLayer = 0x005A,
    /// The Drive does not conform to any Profile.
    ///
    /// See MMC-6 §5.4.30
    NonConforming = 0xFFFF,
}

impl From<u16> for Profile {
    fn from(value: u16) -> Self {
        match value {
            0x0001 => Self::NonRemovableDisk,
            0x0002 => Self::RemovableDisk,
            0x0003 => Self::MoErasable,
            0x0004 => Self::OpticalWriteOnce,
            0x0005 => Self::AsMo,
            0x0008 => Self::CdRom,
            0x0009 => Self::CdR,
            0x000A => Self::CdRw,
            0x0010 => Self::DvdRom,
            0x0011 => Self::DvdRSequentialRecording,
            0x0012 => Self::DvdRam,
            0x0013 => Self::DvdRwRestrictedOverwrite,
            0x0014 => Self::DvdRwSequentialRecording,
            0x0015 => Self::DvdRDualLayerSequentialRecording,
            0x0016 => Self::DvdRDualLayerJumpRecording,
            0x0017 => Self::DvdRwDualLayer,
            0x0018 => Self::DvdDownloadDiscRecording,
            0x001A => Self::DvdPlusRw,
            0x001B => Self::DvdPlusR,
            0x0020 => Self::DdcdRom,
            0x0021 => Self::DdcdR,
            0x0022 => Self::DdcdRw,
            0x002A => Self::DvdPlusRwDualLayer,
            0x002B => Self::DvdPlusRDualLayer,
            0x0040 => Self::BdRom,
            0x0041 => Self::BdRSrm,
            0x0042 => Self::BdRRrm,
            0x0043 => Self::BdRe,
            0x0050 => Self::HdDvdRom,
            0x0051 => Self::HdDvdR,
            0x0052 => Self::HdDvdRam,
            0x0053 => Self::HdDvdRw,
            0x0058 => Self::HdDvdRDualLayer,
            0x005A => Self::HdDvdRwDualLayer,
            0xFFFF => Self::NonConforming,
            v => Self::Reserved(v),
        }
    }
}

/// A 32-bit value representing what physical interface the drive is using.
///
/// See MMC-6 §5.3.2, Table 97.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalInterfaceStandard {
    Unspecified = 0x00000000,
    ScsiFamily = 0x00000001,
    Atapi = 0x00000002,
    Ieee1394_1995 = 0x00000003,
    Ieee1394A = 0x00000004,
    FibreChannel = 0x00000005,
    Ieee1394B = 0x00000006,
    SerialAtapi = 0x00000007,
    Usb = 0x00000008,
    IncitsDefined(u32),
    SffDefined(u32),
    IeeDefined(u32),
    VendorUnique = 0x0000FFFF,
    Reserved(u32),
}

impl From<u32> for PhysicalInterfaceStandard {
    fn from(value: u32) -> Self {
        match value {
            0x00000000 => Self::Unspecified,
            0x00000001 => Self::ScsiFamily,
            0x00000002 => Self::Atapi,
            0x00000003 => Self::Ieee1394_1995,
            0x00000004 => Self::Ieee1394A,
            0x00000005 => Self::FibreChannel,
            0x00000006 => Self::Ieee1394B,
            0x00000007 => Self::SerialAtapi,
            0x00000008 => Self::Usb,
            v @ 0x00010000..=0x0001FFFF => Self::IncitsDefined(v),
            v @ 0x00020000..=0x0002FFFF => Self::SffDefined(v),
            v @ 0x00030000..=0x0003FFFF => Self::IeeDefined(v),
            0x0000FFFF => Self::VendorUnique,
            v => Self::Reserved(v),
        }
    }
}

/// A 3-bit value representing the phyical Loading Mechanism Type used by drives that support
/// Removable Mediums.
///
/// See MMC-6 §5.3.4, Table 102.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadingMechanism {
    CaddySlot = 0b000,
    Tray = 0b001,
    PopUp = 0b010,
    EmbeddedIndividuallyChangeable = 0b100,
    EmbeddedMagazine = 0b101,
    Reserved(u8),
}

impl From<u8> for LoadingMechanism {
    fn from(value: u8) -> Self {
        match value & 0b111 {
            0b000 => Self::CaddySlot,
            0b001 => Self::Tray,
            0b010 => Self::PopUp,
            0b100 => Self::EmbeddedIndividuallyChangeable,
            0b101 => Self::EmbeddedMagazine,
            v => Self::Reserved(v),
        }
    }
}
