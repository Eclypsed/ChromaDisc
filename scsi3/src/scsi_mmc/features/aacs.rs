use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum AacsDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        #[deku(pad_bits_before = "7", bits = 1)]
        binding_nonce_generation: bool,
        binding_nonce_block_count: u8,
        #[deku(pad_bits_before = "4", bits = 4)]
        number_of_agids: u8,
        aacs_version: u8,
    },
    // NOTE: Currently don't have access to a revision with a version 1 byte diagram (somewhere between mmc6r02 and mmc6r02g). Going off of: https://www.t10.org/ftp/t10/document.08/08-375r1.pdf
    #[deku(id = "0b0001")]
    V1 {
        #[deku(pad_bits_before = "5", bits = 1)]
        write_bus_encryption: bool,
        #[deku(bits = 1)]
        bus_encryption: bool,
        #[deku(bits = 1)]
        binding_nonce_generation: bool,
        binding_nonce_block_count: u8,
        #[deku(pad_bits_before = "4", bits = 4)]
        number_of_agids: u8,
        aacs_version: u8,
    },
    #[deku(id = "0b0010")]
    V2 {
        #[deku(pad_bits_before = "3", bits = 1)]
        read_drive_certificate: bool,
        #[deku(bits = 1)]
        rmc: bool, // Idk what this stands for
        #[deku(bits = 1)]
        write_bus_encryption: bool,
        #[deku(bits = 1)]
        bus_encryption: bool,
        #[deku(bits = 1)]
        binding_nonce_generation: bool,
        binding_nonce_block_count: u8,
        #[deku(pad_bits_before = "4", bits = 4)]
        number_of_agids: u8,
        aacs_version: u8,
    },
}
