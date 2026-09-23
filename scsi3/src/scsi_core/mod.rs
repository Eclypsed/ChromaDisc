use arbitrary_int::traits::UnsignedInteger;
use derive_more::{From, Into};
use thiserror::Error;

pub mod addressing;
pub mod util;

mod private {
    pub trait Sealed {}
}

pub(super) type Cdb<const N: usize> = [u8; N];
pub struct OpCode<const N: u8>;

pub trait OpCodeDef: private::Sealed {
    /// OPERATION CODE for valid SCSI commands
    /// ```text
    ///   7   6   5   4   3   2   1   0
    /// +---+---+---+---+---+---+---+---+
    /// | GROUPCODE |    COMMAND CODE   |
    /// +---+---+---+---+---+---+---+---+
    /// ```
    /// See: [SAM-6]
    const OP_CODE: u8;
    type Cdb: AsMut<[u8]> + ?Sized;
}

macro_rules! impl_op_code_def {
    ($len:literal, [$($n:literal),+ $(,)?]) => {
        $(
            impl private::Sealed for OpCode<$n> {}
            impl OpCodeDef for OpCode<$n> {
                const OP_CODE: u8 = $n;
                type Cdb = Cdb<$len>;
            }
        )+
    };
}

// What no const generics in const expressions does to a man...
impl_op_code_def!(
    6,
    [
        // 0b000xxxxx
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D,
        0x1E, 0x1F
    ]
);
impl_op_code_def!(
    10,
    [
        // 0b001xxxxx | 0b010xxxxx
        0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E,
        0x2F, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D,
        0x3E, 0x3F, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x4B, 0x4C,
        0x4D, 0x4E, 0x4F, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x5B,
        0x5C, 0x5D, 0x5E, 0x5F
    ]
);
impl_op_code_def!(
    12,
    [
        // 0b101xxxxx
        0xA0, 0xA1, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xA8, 0xA9, 0xAA, 0xAB, 0xAC, 0xAD, 0xAE,
        0xAF, 0xB0, 0xB1, 0xB2, 0xB3, 0xB4, 0xB5, 0xB6, 0xB7, 0xB8, 0xB9, 0xBA, 0xBB, 0xBC, 0xBD,
        0xBE, 0xBF
    ]
);
impl_op_code_def!(
    16,
    [
        // 0b100xxxxx
        0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x8D, 0x8E,
        0x8F, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9A, 0x9B, 0x9C, 0x9D,
        0x9E, 0x9F
    ]
);

impl private::Sealed for OpCode<0x7E> {}
impl OpCodeDef for OpCode<0x7E> {
    const OP_CODE: u8 = 0x7E;
    type Cdb = [u8];
}

impl private::Sealed for OpCode<0x7F> {}
impl OpCodeDef for OpCode<0x7F> {
    const OP_CODE: u8 = 0x7F;
    type Cdb = [u8];
}

pub trait Command<O: OpCodeDef> {
    fn as_cdb(&self) -> O::Cdb;
}

pub trait ReadCommand<O: OpCodeDef>: Command<O> {
    type Len: UnsignedInteger;
    type Response<'a>; // where Self: 'a <- Maybe do this in the future
    type Error;

    /// The maximum number of bytes the device may transfer for this command
    /// as constructed.
    ///
    /// This is expected size of the response, not a promise about what it will
    /// send. A non-conforming device may return fewer bytes, or attempt more.
    /// Sizing a buffer, capping it, and detecting overrun are the transport's
    /// responsibility.
    fn response_len(&self) -> Self::Len;
    fn parse<'a>(&self, buf: &'a [u8]) -> Result<Self::Response<'a>, Self::Error>;
}

pub trait WriteCommand<O: OpCodeDef>: Command<O> {
    fn write_paramters(&self, buf: &mut [u8]) -> impl Into<usize>;
}

/// CONTROL byte newtype
/// ```text
///   7   6   5   4   3   2   1   0
/// +---+---+---+---+---+---+---+---+
/// |   VS  |  Reserved | N | F | L |
/// +---+---+---+---+---+---+---+---+
/// ```
/// * **VS** - Vendor Specific
/// * **N**  - NACA (Normal Auto Contingent Allegiance)
/// * **F**  - Flag (Obsolete)
/// * **L**  - Link (Obsolete)
///
/// See: [SAM-6]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, From, Into)]
pub struct Control(u8);

#[derive(Debug, Error)]
#[error("Expected at least {EXPECTED_SIZE} bytes of data, received {0}")]
pub struct TruncationError<const EXPECTED_SIZE: usize>(pub usize);
