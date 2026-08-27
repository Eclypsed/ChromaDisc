use super::FeatureHeader;
use deku::{ctx::Limit, deku_error, reader::Reader, DekuError, DekuRead, DekuReader};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum DriveSerialNumberDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        #[deku(reader = "read_serial(deku::reader, header.additional_length)")]
        serial_number: String,
    },
}

fn read_serial<R: deku::no_std_io::Read + deku::no_std_io::Seek>(
    reader: &mut Reader<R>,
    len: u8,
) -> Result<String, DekuError> {
    const SPACE: u8 = 0x20;
    Vec::<u8>::from_reader_with_ctx(reader, Limit::new_count(len.into()))?
        .into_iter()
        .skip_while(|b| *b == SPACE)
        .map(|b| {
            if b.is_ascii_graphic() || b == SPACE {
                Ok(b as char)
            } else {
                Err(deku_error!(
                    DekuError::Parse,
                    "Invalid ascii character in Serial Number",
                    "{:02X}",
                    b
                ))
            }
        })
        .collect::<Result<String, DekuError>>()
}
