use num_enum::{IntoPrimitive, TryFromPrimitive};
use thiserror::Error;

pub mod core;
pub mod profile_list;

#[derive(Debug, Error)]
pub enum Error {
    #[error(
        "Feature descriptor must be at least {min} bytes, received {0}",
        min = 4
    )]
    DescriptorSize(usize),
    #[error("Expected {expected} bytes of feature data, received {received}")]
    DataSize { expected: usize, received: usize },
    #[error("Encountered unknown opcode: 0x{0:04X}")]
    UnknownOpcode(u16),
    #[error("Feature {0:?} not implemented")]
    UnimplementedFeature(FeatureCode),
    #[error(transparent)]
    FeatureData(#[from] FeatureDataError),
}

#[derive(Debug, Error)]
pub enum FeatureDataError {
    #[error(transparent)]
    ProfileList(#[from] profile_list::Error),
    #[error(transparent)]
    Core(#[from] core::Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::UnknownOpcode))]
#[repr(u16)]
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

#[allow(dead_code)]
trait FeatureData: Sized {
    const FEATURE_CODE: FeatureCode;

    fn parse(bytes: &[u8]) -> Result<Self, FeatureDataError>;
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum FeatureDataType {
    ProfileList(profile_list::ProfileList),
    Core(core::Core),
}

fn parse_feature_data(code: FeatureCode, bytes: &[u8]) -> Result<FeatureDataType, Error> {
    Ok(match code {
        FeatureCode::ProfileList => {
            FeatureDataType::ProfileList(profile_list::ProfileList::parse(bytes)?)
        }
        FeatureCode::Core => FeatureDataType::Core(core::Core::parse(bytes)?),
        _ => Err(Error::UnimplementedFeature(code))?,
    })
}

#[allow(dead_code)]
pub struct FeatureDescriptor {
    pub version: u8,
    pub persistent: bool,
    pub current: bool,
    pub additional_length: u8,
    pub feature_data: FeatureDataType,
}

impl TryFrom<&[u8]> for FeatureDescriptor {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        const VERSION_MASK: u8 = 0b00111100;
        const PERSISTENT_MASK: u8 = 0b00000010;
        const CURRENT_MASK: u8 = 0b00000001;

        let Some(header_bytes) = value.get(0..4) else {
            return Err(Error::DescriptorSize(value.len()));
        };

        let feature_code = FeatureCode::try_from_primitive(u16::from_be_bytes([
            header_bytes[0],
            header_bytes[1],
        ]))?;

        let version = (header_bytes[2] & VERSION_MASK) >> 2;
        let persistent = (header_bytes[2] & PERSISTENT_MASK) >> 1 != 0;
        let current = header_bytes[2] & CURRENT_MASK != 0;

        let additional_length = header_bytes[3];

        let end: usize = (additional_length + 4).into();

        let Some(data_bytes) = value.get(4..end) else {
            return Err(Error::DataSize {
                expected: end - 4,
                received: value.len() - 4,
            });
        };

        let feature_data = parse_feature_data(feature_code, data_bytes)?;

        Ok(Self {
            version,
            persistent,
            current,
            additional_length,
            feature_data,
        })
    }
}
