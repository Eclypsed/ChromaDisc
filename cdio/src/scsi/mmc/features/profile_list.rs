use deku::DekuRead;

// Only one version (0b0000), all future versions will be backwards compatible so no versioning needed

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
pub struct ProfileDescriptor {
    profile_number: Profile, // Not sure if it would be beneficial to make this an enum at some point like in the old impl
    #[deku(pad_bits_before = "7", bits = 1, pad_bytes_after = "1")]
    current_profile: bool,
}

/// A 16-bit value representing a Drive Profile.
///
/// See MMC-6 §5.3.1, Table 92.
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
#[deku(id_type = "u16", endian = "big")]
pub enum Profile {
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
    /// Unknown Profile with a currently reserved Profile Number.
    #[deku(id_pat = "_")]
    Reserved(u16) = 0x0000,
}
