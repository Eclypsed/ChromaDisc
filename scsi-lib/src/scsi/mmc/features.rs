use std::fmt::Debug;

use thiserror::Error;

use crate::core::util::BitReader;

use super::types::{LoadingMechanism, PhysicalInterfaceStandard, Profile};

#[derive(Debug, Error)]
pub enum FeatureError {
    #[error("Feature descriptor must be at least 4 bytes")]
    DescriptorSize,
    #[error("Expected {expected} bytes of descriptor data, received {received}")]
    DataSize { expected: usize, received: usize },
}

const HEADER_LEN: usize = 4;

#[derive(Debug)]
struct FeatureHeader {
    pub version: u8,
    pub persistent: bool,
    pub current: bool,
    additional_length: u8,
}

impl FeatureHeader {
    fn parse(bytes: &[u8; HEADER_LEN]) -> Self {
        let flags = BitReader(bytes[2]);

        FeatureHeader {
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
    pub data_block_types_supported: u16,
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
    use super::*;
    use crate::core::util::BitReader;
    use crate::scsi::mmc::types::PhysicalInterfaceStandard;

    trait ParseFeature: Sized {
        const DATA_LEN: Option<usize>;

        fn parse(header: FeatureHeader, data: &[u8]) -> Self;
    }

    impl ParseFeature for ProfileList {
        const DATA_LEN: Option<usize> = None;

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
        const DATA_LEN: Option<usize> = Some(8);

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
        const DATA_LEN: Option<usize> = Some(4);

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
        const DATA_LEN: Option<usize> = Some(4);

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
        const DATA_LEN: Option<usize> = Some(4);

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
        const DATA_LEN: Option<usize> = Some(8);

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
        const DATA_LEN: Option<usize> = Some(0);

        fn parse(header: FeatureHeader, _: &[u8]) -> Self {
            Self { header }
        }
    }

    impl ParseFeature for CdRead {
        const DATA_LEN: Option<usize> = Some(4);

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
        const DATA_LEN: Option<usize> = Some(4);

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
        const DATA_LEN: Option<usize> = Some(12);

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
        const DATA_LEN: Option<usize> = Some(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            let flags = BitReader(data[2]);
            let num_link_sizes: usize = data[3].into();
            let link_sizes = data.get(4..(4 + num_link_sizes)).unwrap_or(&[]).to_vec();
            debug_assert_eq!(link_sizes.len(), num_link_sizes);

            Self {
                header,
                data_block_types_supported: u16::from_be_bytes(data[0..2].try_into().unwrap()),
                trio: flags.bit(0b00000100),
                arsv: flags.bit(0b00000010),
                buf: flags.bit(0b00000001),
                link_sizes,
            }
        }
    }

    impl ParseFeature for SectorErasable {
        const DATA_LEN: Option<usize> = Some(0);

        fn parse(header: FeatureHeader, _: &[u8]) -> Self {
            Self { header }
        }
    }

    impl ParseFeature for Formattable {
        const DATA_LEN: Option<usize> = Some(8);

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
        const DATA_LEN: Option<usize> = Some(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            Self {
                header,
                ssa: BitReader(data[0]).bit(0b10000000),
            }
        }
    }

    impl ParseFeature for WriteOnce {
        const DATA_LEN: Option<usize> = Some(8);

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
        const DATA_LEN: Option<usize> = Some(0);

        fn parse(header: FeatureHeader, _: &[u8]) -> Self {
            Self { header }
        }
    }

    impl ParseFeature for CdRwCavWrite {
        const DATA_LEN: Option<usize> = Some(4);

        fn parse(header: FeatureHeader, _: &[u8]) -> Self {
            Self { header }
        }
    }

    impl ParseFeature for Mrw {
        const DATA_LEN: Option<usize> = Some(4);

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
        const DATA_LEN: Option<usize> = Some(4);

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
        const DATA_LEN: Option<usize> = Some(4);

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
        const DATA_LEN: Option<usize> = Some(4);

        fn parse(header: FeatureHeader, data: &[u8]) -> Self {
            Self {
                header,
                write: BitReader(data[0]).bit(0b00000001),
            }
        }
    }

    impl ParseFeature for RigidRestrictedOverwrite {
        const DATA_LEN: Option<usize> = Some(4);

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

    fn parse_feature<T: ParseFeature>(
        header_bytes: &[u8; 4],
        data_bytes: &[u8],
    ) -> Result<T, FeatureError> {
        let header = FeatureHeader::parse(header_bytes);

        if let Some(data_len) = T::DATA_LEN
            && data_bytes.len() < data_len
        {
            return Err(FeatureError::DataSize {
                expected: data_len,
                received: data_bytes.len(),
            });
        }

        Ok(T::parse(header, data_bytes))
    }

    pub fn parse_descriptor(bytes: &[u8]) -> Result<Box<dyn MmcFeature>, FeatureError> {
        if bytes.len() < HEADER_LEN {
            return Err(FeatureError::DescriptorSize);
        }

        let header: &[u8; HEADER_LEN] = &bytes[0..HEADER_LEN].try_into().unwrap();
        let feature_code = u16::from_be_bytes([header[0], header[1]]);

        let additional_length: usize = header[3].into();
        let data = bytes.get(HEADER_LEN..).unwrap_or(&[]);

        let Some(data) = data.get(..additional_length) else {
            return Err(FeatureError::DataSize {
                expected: additional_length,
                received: data.len(),
            });
        };

        Ok(match feature_code {
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
            code => Box::new(UnknownFeature {
                feature_code: code,
                header: FeatureHeader::parse(header),
                data: data.to_vec(),
            }),
        })
    }
}
