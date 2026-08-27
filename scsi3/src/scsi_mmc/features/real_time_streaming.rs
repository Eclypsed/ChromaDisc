use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum RealTimeStreamingDescriptor {
    #[deku(id = "0b0000")]
    V0,
    #[deku(id = "0b0001")]
    V1 {
        #[deku(pad_bits_before = "3", bits = 1, pad_bytes_after = "3")]
        stream_writing: bool,
    },
    #[deku(id = "0b0010")]
    V2 {
        #[deku(pad_bits_before = "4", bits = 1)]
        set_cd_speed: bool,
        #[deku(bits = 1)]
        mode_page_2a: bool,
        #[deku(bits = 1)]
        write_speed_performace_descriptor: bool,
        #[deku(bits = 1, pad_bytes_after = "3")]
        stream_writing: bool,
    },
    // I have no idea why v3 and v4 have the same byte layout
    #[deku(id = "0b0011")]
    V3 {
        #[deku(pad_bits_before = "3", bits = 1)]
        read_buffer_capacity_block: bool,
        #[deku(bits = 1)]
        set_cd_speed: bool,
        #[deku(bits = 1)]
        mode_page_2a: bool,
        #[deku(bits = 1)]
        write_speed_performace_descriptor: bool,
        #[deku(bits = 1, pad_bytes_after = "3")]
        stream_writing: bool,
    },
    // I have no idea why v3 and v4 have the same byte layout
    #[deku(id = "0b0100")]
    V4 {
        #[deku(pad_bits_before = "3", bits = 1)]
        read_buffer_capacity_block: bool,
        #[deku(bits = 1)]
        set_cd_speed: bool,
        #[deku(bits = 1)]
        mode_page_2a: bool,
        #[deku(bits = 1)]
        write_speed_performace_descriptor: bool,
        #[deku(bits = 1, pad_bytes_after = "3")]
        stream_writing: bool,
    },
    #[deku(id = "0b0101")]
    V5 {
        #[deku(pad_bits_before = "2", bits = 1)]
        set_minimum_performance: bool,
        #[deku(bits = 1)]
        read_buffer_capacity_block: bool,
        #[deku(bits = 1)]
        set_cd_speed: bool,
        #[deku(bits = 1)]
        mode_page_2a: bool,
        #[deku(bits = 1)]
        write_speed_performace_descriptor: bool,
        #[deku(bits = 1, pad_bytes_after = "3")]
        stream_writing: bool,
    },
}
