use derive_more::Debug;
use zerocopy::{BigEndian, FromBytes, Immutable, IntoBytes, KnownLayout, U16, Unaligned};

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
pub struct ProfileNumber(#[debug("{_0:04X}h")] U16<BigEndian>);

impl ProfileNumber {
    pub const NO_CURRENT_PROFILE: Self = Self::from_u16(0x0000);
    /// Re-writable disk, capable of changing behavior
    pub const NON_REMOVABLE_DISK: Self = Self::from_u16(0x0001);
    /// Re-writable; with removable media
    pub const REMOVABLE_DISK: Self = Self::from_u16(0x0002);
    /// Magneto-Optical disk with sector erase capability
    pub const MO_ERASABLE: Self = Self::from_u16(0x0003);
    /// Optical write once
    pub const OPTICAL_WRITE_ONCE: Self = Self::from_u16(0x0004);
    /// Advance Storage – Magneto-Optical
    pub const AS_MO: Self = Self::from_u16(0x0005);

    // --- 0006h - 0007h Reserved ---

    /// Read only Compact Disc capable
    pub const CD_ROM: Self = Self::from_u16(0x0008);
    /// Write once Compact Disc capable
    pub const CD_R: Self = Self::from_u16(0x0009);
    /// Re-writable Compact Disc capable
    pub const CD_RW: Self = Self::from_u16(0x000A);

    // --- 000Bh - 000Fh Reserved ---

    /// Read only DVD
    pub const DVD_ROM: Self = Self::from_u16(0x0010);
    /// Write once DVD using Sequential recording
    pub const DVD_R_SEQUENTIAL_RECORDING: Self = Self::from_u16(0x0011);
    /// Re-writable DVD
    pub const DVD_RAM: Self = Self::from_u16(0x0012);
    /// Re-recordable DVD using Restricted Overwrite
    pub const DVD_RW_RESTRICTED_OVERWRITE: Self = Self::from_u16(0x0013);
    /// Re-recordable DVD using Sequential recording
    pub const DVD_RW_SEQUENTIAL_RECORDING: Self = Self::from_u16(0x0014);
    /// Dual Layer DVD-R using Sequential recording
    pub const DVD_R_DUAL_LAYER_SEQUENTIAL_RECORDING: Self = Self::from_u16(0x0015);
    /// Dual Layer DVD-R using Layer Jump recording
    pub const DVD_R_DUAL_LAYER_JUMP_RECORDING: Self = Self::from_u16(0x0016);
    /// Dual Layer DVD-RW recording
    pub const DVD_RW_DUAL_LAYER: Self = Self::from_u16(0x0017);
    /// Write once DVD for CSS managed recording
    pub const DVD_DOWNLOAD_DISC_RECORDING: Self = Self::from_u16(0x0018);

    // --- 0019h Reserved ---

    /// DVD+ReWritable
    pub const DVD_PLUS_RW: Self = Self::from_u16(0x001A);
    /// DVD+Recordable
    pub const DVD_PLUS_R: Self = Self::from_u16(0x001B);

    // --- 001Ch - 001Fh Reserved ---

    /// Read only DDCD
    pub const DDCD_ROM: Self = Self::from_u16(0x0020);
    /// Write once DDCD
    pub const DDCD_R: Self = Self::from_u16(0x0021);
    /// Re-Writable DDCD
    pub const DDCD_RW: Self = Self::from_u16(0x0022);

    // --- 0023h - 0029h Reserved ---

    /// DVD+Rewritable Dual Layer
    pub const DVD_PLUS_RW_DUAL_LAYER: Self = Self::from_u16(0x002A);
    /// DVD+Recordable Dual Layer
    pub const DVD_PLUS_R_DUAL_LAYER: Self = Self::from_u16(0x002B);

    // --- 002Ch - 003Fh Reserved ---

    /// Blu-ray Disc ROM
    pub const BD_ROM: Self = Self::from_u16(0x0040);
    /// Blu-ray Disc Recordable – Sequential Recording Mode
    pub const BD_R_SRM: Self = Self::from_u16(0x0041);
    /// Blu-ray Disc Recordable – Random Recording Mode
    pub const BD_R_RRM: Self = Self::from_u16(0x0042);
    /// Blu-ray Disc Rewritable
    pub const BD_RE: Self = Self::from_u16(0x0043);

    // --- 0044h - 004Fh Reserved ---

    /// Read-only HD DVD
    pub const HD_DVD_ROM: Self = Self::from_u16(0x0050);
    /// Write-once HD DVD
    pub const HD_DVD_R: Self = Self::from_u16(0x0051);
    /// Rewritable HD DVD
    pub const HD_DVD_RAM: Self = Self::from_u16(0x0052);
    /// Rewritable HD DVD
    pub const HD_DVD_RW: Self = Self::from_u16(0x0053);

    // --- 0054h - 0057h Reserved ---

    /// Dual Layer Write-once HD DVD
    pub const HD_DVD_R_DUAL_LAYER: Self = Self::from_u16(0x0058);

    // --- 0059h Reserved ---

    /// Dual Layer Rewritable HD DVD
    pub const HD_DVD_RW_DUAL_LAYER: Self = Self::from_u16(0x005A);

    // --- 005Bh - FFFEh Reserved ---

    /// The Drive does not conform to any Profile.
    pub const NON_CONFORMING: Self = Self::from_u16(0xFFFF);

    #[inline]
    pub const fn from_u16(number: u16) -> Self {
        Self(U16::new(number))
    }

    #[inline]
    pub const fn into_u16(self) -> u16 {
        self.0.get()
    }
}
