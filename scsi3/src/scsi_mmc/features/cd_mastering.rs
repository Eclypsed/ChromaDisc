use super::FeatureHeader;
use arbitrary_int::u24;
use deku::{ctx::Endian, reader::Reader, DekuError, DekuRead, DekuReader};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum CdMasteringDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        #[deku(pad_bits_before = "2", bits = 1)]
        sao: bool,
        #[deku(bits = 1)]
        raw_ms: bool,
        #[deku(bits = 1)]
        raw: bool,
        #[deku(bits = 1)]
        test_write: bool,
        #[deku(bits = 1)]
        cdrw: bool,
        #[deku(bits = 1)]
        r_w: bool,
        #[deku(reader = "read_u24(deku::reader)")]
        maximum_cue_sheet_length: u24,
    },
    #[deku(id = "0b0001")]
    V1 {
        #[deku(pad_bits_before = "1", bits = 1)]
        buf: bool,
        #[deku(bits = 1)]
        sao: bool,
        #[deku(bits = 1)]
        raw_ms: bool,
        #[deku(bits = 1)]
        raw: bool,
        #[deku(bits = 1)]
        test_write: bool,
        #[deku(bits = 1)]
        cdrw: bool,
        #[deku(bits = 1)]
        r_w: bool,
        #[deku(reader = "read_u24(deku::reader)")]
        maximum_cue_sheet_length: u24,
    },
}

fn read_u24<R: deku::no_std_io::Read + deku::no_std_io::Seek>(
    reader: &mut Reader<R>,
) -> Result<u24, DekuError> {
    <[u8; 3]>::from_reader_with_ctx(reader, Endian::Big).map(u24::from_be_bytes)
}
