use std::fmt::Display;

use deku::{DekuRead, DekuWrite};
use derive_more::{Debug, From, Into};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, From, Into, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct ProfileNumber(#[debug("{_0:04X}h")] u16);

impl ProfileNumber {
    pub const NO_CURRENT_PROFILE: Self = Self(0x0000);
    /// Re-writable disk, capable of changing behavior
    pub const NON_REMOVABLE_DISK: Self = Self(0x0001);
    /// Re-writable; with removable media
    pub const REMOVABLE_DISK: Self = Self(0x0002);
    /// Magneto-Optical disk with sector erase capability
    pub const MO_ERASABLE: Self = Self(0x0003);
    /// Optical write once
    pub const OPTICAL_WRITE_ONCE: Self = Self(0x0004);
    /// Advance Storage – Magneto-Optical
    pub const AS_MO: Self = Self(0x0005);

    // --- 0006h - 0007h Reserved ---

    /// Read only Compact Disc capable
    pub const CD_ROM: Self = Self(0x0008);
    /// Write once Compact Disc capable
    pub const CD_R: Self = Self(0x0009);
    /// Re-writable Compact Disc capable
    pub const CD_RW: Self = Self(0x000A);

    // --- 000Bh - 000Fh Reserved ---

    /// Read only DVD
    pub const DVD_ROM: Self = Self(0x0010);
    /// Write once DVD using Sequential recording
    pub const DVD_R_SEQUENTIAL_RECORDING: Self = Self(0x0011);
    /// Re-writable DVD
    pub const DVD_RAM: Self = Self(0x0012);
    /// Re-recordable DVD using Restricted Overwrite
    pub const DVD_RW_RESTRICTED_OVERWRITE: Self = Self(0x0013);
    /// Re-recordable DVD using Sequential recording
    pub const DVD_RW_SEQUENTIAL_RECORDING: Self = Self(0x0014);
    /// Dual Layer DVD-R using Sequential recording
    pub const DVD_R_DUAL_LAYER_SEQUENTIAL_RECORDING: Self = Self(0x0015);
    /// Dual Layer DVD-R using Layer Jump recording
    pub const DVD_R_DUAL_LAYER_JUMP_RECORDING: Self = Self(0x0016);
    /// Dual Layer DVD-RW recording
    pub const DVD_RW_DUAL_LAYER: Self = Self(0x0017);
    /// Write once DVD for CSS managed recording
    pub const DVD_DOWNLOAD_DISC_RECORDING: Self = Self(0x0018);

    // --- 0019h Reserved ---

    /// DVD+ReWritable
    pub const DVD_PLUS_RW: Self = Self(0x001A);
    /// DVD+Recordable
    pub const DVD_PLUS_R: Self = Self(0x001B);

    // --- 001Ch - 001Fh Reserved ---

    /// Read only DDCD
    pub const DDCD_ROM: Self = Self(0x0020);
    /// Write once DDCD
    pub const DDCD_R: Self = Self(0x0021);
    /// Re-Writable DDCD
    pub const DDCD_RW: Self = Self(0x0022);

    // --- 0023h - 0029h Reserved ---

    /// DVD+Rewritable Dual Layer
    pub const DVD_PLUS_RW_DUAL_LAYER: Self = Self(0x002A);
    /// DVD+Recordable Dual Layer
    pub const DVD_PLUS_R_DUAL_LAYER: Self = Self(0x002B);

    // --- 002Ch - 003Fh Reserved ---

    /// Blu-ray Disc ROM
    pub const BD_ROM: Self = Self(0x0040);
    /// Blu-ray Disc Recordable – Sequential Recording Mode
    pub const BD_R_SRM: Self = Self(0x0041);
    /// Blu-ray Disc Recordable – Random Recording Mode
    pub const BD_R_RRM: Self = Self(0x0042);
    /// Blu-ray Disc Rewritable
    pub const BD_RE: Self = Self(0x0043);

    // --- 0044h - 004Fh Reserved ---

    /// Read-only HD DVD
    pub const HD_DVD_ROM: Self = Self(0x0050);
    /// Write-once HD DVD
    pub const HD_DVD_R: Self = Self(0x0051);
    /// Rewritable HD DVD
    pub const HD_DVD_RAM: Self = Self(0x0052);
    /// Rewritable HD DVD
    pub const HD_DVD_RW: Self = Self(0x0053);

    // --- 0054h - 0057h Reserved ---

    /// Dual Layer Write-once HD DVD
    pub const HD_DVD_R_DUAL_LAYER: Self = Self(0x0058);

    // --- 0059h Reserved ---

    /// Dual Layer Rewritable HD DVD
    pub const HD_DVD_RW_DUAL_LAYER: Self = Self(0x005A);

    // --- 005Bh - FFFEh Reserved ---

    /// The Drive does not conform to any Profile.
    pub const NON_CONFORMING: Self = Self(0xFFFF);
}

impl Display for ProfileNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({:04X}h)",
            match *self {
                Self::NO_CURRENT_PROFILE => "No Current Profile",
                Self::NON_REMOVABLE_DISK => "Non-removeable Disk",
                Self::REMOVABLE_DISK => "Removable Disk",
                _ => todo!(),
            },
            self.0
        )
    }
}
