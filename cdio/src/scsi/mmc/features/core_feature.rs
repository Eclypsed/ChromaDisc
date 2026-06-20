// Need to model different versions

use deku::DekuRead;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "version", ctx = "version: u8")]
pub enum CoreDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        physical_interface_standard: PhysicalInterfaceStandard,
    },
    #[deku(id = "0b0001")]
    V1 {
        physical_interface_standard: PhysicalInterfaceStandard,
        #[deku(pad_bits_before = "7", bits = 1, pad_bytes_after = "3")]
        device_busy_event: bool,
    },
    #[deku(id = "0b0010")]
    V2 {
        physical_interface_standard: PhysicalInterfaceStandard,
        #[deku(pad_bits_before = "6", bits = 1)]
        inq2: bool,
        #[deku(bits = 1, pad_bytes_after = "3")]
        device_busy_event: bool,
    },
}

/// A 32-bit value representing what physical interface the drive is using.
///
/// See MMC-6 §5.3.2, Table 97.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
#[deku(id_type = "u32", endian = "big")]
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
    #[deku(id_pat = "0x00010000..=0x0001FFFF")]
    IncitsDefined(u32),
    #[deku(id_pat = "0x00020000..=0x0002FFFF")]
    SffDefined(u32),
    #[deku(id_pat = "0x00030000..=0x0003FFFF")]
    IeeDefined(u32),
    VendorUnique = 0x0000FFFF,
    #[deku(id_pat = "_")]
    Reserved(u32),
}
