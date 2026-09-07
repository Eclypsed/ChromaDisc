use std::marker::PhantomData;

use crate::{
    core::{
        addressing::{Lba, Span},
        Command, Control, OpCode, OpCodeDef, ReadCommand,
    },
    mmc::msf::Msf,
};
use arbitrary_int::{u2, u24, u3};
use derive_where::derive_where;
use thiserror::Error;

mod private {
    use super::*;

    pub trait SectorRange {
        fn sector_count(&self) -> u32;
    }

    pub trait AddressingModeSeal {
        type AddressingParams: SectorRange;
    }

    pub struct LbaAddressingParams {
        pub starting_lba: Lba,
        pub transfer_length: u24,
    }

    impl SectorRange for LbaAddressingParams {
        fn sector_count(&self) -> u32 {
            self.transfer_length.into()
        }
    }

    impl SectorRange for Span<Msf> {
        fn sector_count(&self) -> u32 {
            self.end().total_frames() - self.start().total_frames()
        }
    }

    pub trait MainChannelSelectionSeal {
        const MAIN_CHANNEL_SELECTION: u8;
    }

    pub trait SectorType {
        const EXPECTED_SECTOR_TYPE: u3;
        const SYNC_SIZE: u16;
        const SUB_HEADER_SIZE: u16;
        const HEADER_SIZE: u16;
        const USER_DATA_SIZE: u16;
        const EDC_ECC_SIZE: u16;
    }

    pub trait C2Seal {
        const C2_SELECTION: u2;
        const SIZE_BYTES: u16;

        type Data<'a>;

        fn data_from_slice<'a>(bytes: &'a [u8]) -> Self::Data<'a>;
    }

    pub trait SubChannelSeal {
        const SUB_CHANNEL_SELECTION: u3;
        const SIZE_BYTES: u16;

        type Data<'a>;

        fn data_from_slice<'a>(bytes: &'a [u8]) -> Self::Data<'a>;
    }
}

pub trait ReadCdAddress: private::AddressingModeSeal {}

impl private::AddressingModeSeal for Lba {
    type AddressingParams = private::LbaAddressingParams;
}
impl ReadCdAddress for Lba {}

impl private::AddressingModeSeal for Msf {
    type AddressingParams = Span<Msf>;
}
impl ReadCdAddress for Msf {}

pub trait C2ErrorInfo: private::C2Seal {}

pub struct NoC2;
impl private::C2Seal for NoC2 {
    const C2_SELECTION: u2 = u2::new(0b00);
    const SIZE_BYTES: u16 = 0;
    type Data<'a> = ();
    fn data_from_slice<'a>(_: &'a [u8]) -> Self::Data<'a> {}
}
impl C2ErrorInfo for NoC2 {}

pub struct C2Pointers;
impl private::C2Seal for C2Pointers {
    const C2_SELECTION: u2 = u2::new(0b01);
    const SIZE_BYTES: u16 = 294;
    type Data<'a> = &'a [u8; Self::SIZE_BYTES as usize];
    fn data_from_slice<'a>(bytes: &'a [u8]) -> Self::Data<'a> {
        bytes.try_into().unwrap()
    }
}
impl C2ErrorInfo for C2Pointers {}

pub struct BlockC2Pointers;
impl private::C2Seal for BlockC2Pointers {
    const C2_SELECTION: u2 = u2::new(0b10);
    const SIZE_BYTES: u16 = 296;
    type Data<'a> = &'a [u8; Self::SIZE_BYTES as usize];
    fn data_from_slice<'a>(bytes: &'a [u8]) -> Self::Data<'a> {
        bytes.try_into().unwrap()
    }
}
impl C2ErrorInfo for BlockC2Pointers {}

pub trait SubChannelSelection: private::SubChannelSeal {}

pub struct NoSubChannel;
impl private::SubChannelSeal for NoSubChannel {
    const SUB_CHANNEL_SELECTION: u3 = u3::new(0b000);
    const SIZE_BYTES: u16 = 0;
    type Data<'a> = ();
    fn data_from_slice<'a>(_: &'a [u8]) -> Self::Data<'a> {}
}
impl SubChannelSelection for NoSubChannel {}

pub struct RawPW;
impl private::SubChannelSeal for RawPW {
    const SUB_CHANNEL_SELECTION: u3 = u3::new(0b001);
    const SIZE_BYTES: u16 = 96;
    type Data<'a> = &'a [u8; Self::SIZE_BYTES as usize];
    fn data_from_slice<'a>(bytes: &'a [u8]) -> Self::Data<'a> {
        bytes.try_into().unwrap()
    }
}
impl SubChannelSelection for RawPW {}

pub struct FormattedQ;
impl private::SubChannelSeal for FormattedQ {
    const SUB_CHANNEL_SELECTION: u3 = u3::new(0b010);
    const SIZE_BYTES: u16 = 16;
    type Data<'a> = &'a [u8; Self::SIZE_BYTES as usize];
    fn data_from_slice<'a>(bytes: &'a [u8]) -> Self::Data<'a> {
        bytes.try_into().unwrap()
    }
}
impl SubChannelSelection for FormattedQ {}

pub struct CorrectedDeinterleavedRw;
impl private::SubChannelSeal for CorrectedDeinterleavedRw {
    const SUB_CHANNEL_SELECTION: u3 = u3::new(0b100);
    const SIZE_BYTES: u16 = 96;
    type Data<'a> = &'a [u8; Self::SIZE_BYTES as usize];
    fn data_from_slice<'a>(bytes: &'a [u8]) -> Self::Data<'a> {
        bytes.try_into().unwrap()
    }
}
impl SubChannelSelection for CorrectedDeinterleavedRw {}

pub trait SectorSelection: private::SectorType + private::MainChannelSelectionSeal {}

pub struct ReadCd<
    A: ReadCdAddress,
    M: SectorSelection,
    C: C2ErrorInfo = NoC2,
    S: SubChannelSelection = NoSubChannel,
> {
    _sector_selection: PhantomData<M>,
    digital_audio_play: bool,
    // Interestingly, in the LBA version of this command, byte 1 bit 0 is an obsolete RELADDR flag.
    // However, every reference going back to MMC-1 says this flag should just be 0, so I don't
    // know where it came from but I'm choosing to omit it.
    addressing_params: A::AddressingParams,
    _c2_marker: PhantomData<C>,
    _sub_channel_marker: PhantomData<S>,
    control: Control,
}

type ReadCdOpcode = OpCode<0xBE>;
type ReadCdMsfOpcode = OpCode<0xB9>;

impl<M: SectorSelection, C: C2ErrorInfo, S: SubChannelSelection> Command<ReadCdOpcode>
    for ReadCd<Lba, M, C, S>
{
    fn as_cdb(&self) -> <ReadCdOpcode as OpCodeDef>::Cdb {
        let lba_bytes: [u8; 4] = i32::from(self.addressing_params.starting_lba).to_be_bytes();
        let transfer_bytes: [u8; 3] = self.addressing_params.transfer_length.to_be_bytes();

        [
            ReadCdOpcode::OP_CODE,
            (M::EXPECTED_SECTOR_TYPE.value() << 2) | ((self.digital_audio_play as u8) << 1),
            lba_bytes[0],
            lba_bytes[1],
            lba_bytes[2],
            lba_bytes[3],
            transfer_bytes[0],
            transfer_bytes[1],
            transfer_bytes[2],
            (M::MAIN_CHANNEL_SELECTION) | (C::C2_SELECTION.value() << 1),
            (S::SUB_CHANNEL_SELECTION.value()),
            self.control.into(),
        ]
    }
}

impl<M: SectorSelection, C: C2ErrorInfo, S: SubChannelSelection> Command<ReadCdMsfOpcode>
    for ReadCd<Msf, M, C, S>
{
    fn as_cdb(&self) -> <ReadCdMsfOpcode as OpCodeDef>::Cdb {
        [
            ReadCdMsfOpcode::OP_CODE,
            (M::EXPECTED_SECTOR_TYPE.value() << 2) | ((self.digital_audio_play as u8) << 1),
            0,
            self.addressing_params.start().minute().into(),
            self.addressing_params.start().second().into(),
            self.addressing_params.start().frame().into(),
            self.addressing_params.end().minute().into(),
            self.addressing_params.end().second().into(),
            self.addressing_params.end().frame().into(),
            (M::MAIN_CHANNEL_SELECTION) | (C::C2_SELECTION.value() << 1),
            (S::SUB_CHANNEL_SELECTION.value()),
            self.control.into(),
        ]
    }
}

#[derive_where(Debug; C::Data<'a>, S::Data<'a>)]
pub struct Sector<
    'a,
    M: SectorSelection,
    C: C2ErrorInfo = NoC2,
    S: SubChannelSelection = NoSubChannel,
> {
    buf: &'a [u8], // Should always be M::SELECTION_SIZE bytes
    _main_channel_maker: PhantomData<M>,
    pub c2: C::Data<'a>,
    pub sub_channel: S::Data<'a>,
}

impl<'a, M: SectorSelection, C: C2ErrorInfo, S: SubChannelSelection> Sector<'a, M, C, S> {
    const SIZE_BYTES: u16 = main_channel_size::<M>() + C::SIZE_BYTES + S::SIZE_BYTES;
}

#[derive(Debug, Error)]
pub enum SectorsError {
    #[error("Attempted to parse zero-sized sectors")]
    ZeroSizedSector,
    #[error("Trailing bytes encountered, unable to parse into complete sector")]
    PartialSector { trailing: usize },
}

#[derive(Debug)]
pub struct Sectors<'a, M: SectorSelection, C: C2ErrorInfo, S: SubChannelSelection> {
    chunks: core::slice::ChunksExact<'a, u8>, // Chunks should always be Sector::<'_, M, C, S>::SIZE_BYTES
    #[allow(clippy::type_complexity)]
    _marker: PhantomData<fn() -> (M, C, S)>,
}

impl<'a, M: SectorSelection, C: C2ErrorInfo, S: SubChannelSelection> Sectors<'a, M, C, S> {
    pub fn new(buf: &'a [u8]) -> Result<Self, SectorsError> {
        let sector_size = Sector::<'_, M, C, S>::SIZE_BYTES as usize;
        if sector_size == 0 {
            return Err(SectorsError::ZeroSizedSector);
        }

        let trailing = buf.len() % sector_size;
        if trailing != 0 {
            return Err(SectorsError::PartialSector { trailing });
        }

        Ok(Self {
            chunks: buf.chunks_exact(sector_size),
            _marker: PhantomData,
        })
    }
}

impl<'a, M: SectorSelection, C: C2ErrorInfo, S: SubChannelSelection> Iterator
    for Sectors<'a, M, C, S>
{
    type Item = Sector<'a, M, C, S>;

    fn next(&mut self) -> Option<Self::Item> {
        let chunk = self.chunks.next()?;
        let (main, rest) = chunk.split_at(main_channel_size::<M>() as usize);
        let (c2, sub_channel) = rest.split_at(C::SIZE_BYTES as usize);
        Some(Sector {
            buf: main,
            _main_channel_maker: PhantomData,
            c2: C::data_from_slice(c2),
            sub_channel: S::data_from_slice(sub_channel),
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.chunks.size_hint()
    }
}

impl<'a, M: SectorSelection, C: C2ErrorInfo, S: SubChannelSelection> ExactSizeIterator
    for Sectors<'a, M, C, S>
{
}

impl<
        O: OpCodeDef,
        A: ReadCdAddress,
        M: SectorSelection,
        C: C2ErrorInfo,
        S: SubChannelSelection,
    > ReadCommand<O> for ReadCd<A, M, C, S>
where
    ReadCd<A, M, C, S>: Command<O>,
{
    type Len = u64;
    type Response<'a> = Sectors<'a, M, C, S>;
    type Error = SectorsError;

    fn response_len(&self) -> Self::Len {
        (Sector::<'_, M, C, S>::SIZE_BYTES as u64)
            * private::SectorRange::sector_count(&self.addressing_params) as u64
    }

    fn parse<'a>(&self, buf: &'a [u8]) -> Result<Self::Response<'a>, Self::Error> {
        Sectors::new(buf)
    }
}

// Probably explain here why there is no way to construct an "All sector types" command.
// Either that or in the future add an "All sector types" option that has no Main Channel
// restrictions or useful parsing beyond chunking the raw sectors (since we don't know anything
// about the sectors returned, beyond them being either all CD-DA or all CD Data, without external
// information)

pub struct MainChannel;

impl MainChannel {
    pub const NO_FIELDS: u8 = 0;
    pub const SYNC: u8 = 1 << 7;
    pub const SUB_HEADER: u8 = 1 << 6;
    pub const HEADER: u8 = 1 << 5;
    pub const USER_DATA: u8 = 1 << 4;
    pub const EDC_ECC: u8 = 1 << 3;
}

pub const fn main_channel_size<S: SectorSelection>() -> u16 {
    let mut size = 0;
    if S::MAIN_CHANNEL_SELECTION & MainChannel::SYNC != 0 {
        size += S::SYNC_SIZE
    };
    if S::MAIN_CHANNEL_SELECTION & MainChannel::SUB_HEADER != 0 {
        size += S::SUB_HEADER_SIZE
    };
    if S::MAIN_CHANNEL_SELECTION & MainChannel::HEADER != 0 {
        size += S::HEADER_SIZE
    };
    if S::MAIN_CHANNEL_SELECTION & MainChannel::USER_DATA != 0 {
        size += S::USER_DATA_SIZE
    };
    if S::MAIN_CHANNEL_SELECTION & MainChannel::EDC_ECC != 0 {
        size += S::EDC_ECC_SIZE
    };
    size
}

macro_rules! selection_flag {
    () => { 0u8 };
    (sync       $(, $rest:ident)*) => { MainChannel::SYNC       | selection_flag!($($rest),*) };
    (header     $(, $rest:ident)*) => { MainChannel::HEADER     | selection_flag!($($rest),*) };
    (sub_header $(, $rest:ident)*) => { MainChannel::SUB_HEADER | selection_flag!($($rest),*) };
    (user_data  $(, $rest:ident)*) => { MainChannel::USER_DATA  | selection_flag!($($rest),*) };
    (edc_ecc    $(, $rest:ident)*) => { MainChannel::EDC_ECC    | selection_flag!($($rest),*) };
}

macro_rules! field_size {
    (sync,       $sync:expr, $hdr:expr, $sh:expr, $ud:expr, $ecc:expr) => {
        $sync as usize
    };
    (header,     $sync:expr, $hdr:expr, $sh:expr, $ud:expr, $ecc:expr) => {
        $hdr as usize
    };
    (sub_header, $sync:expr, $hdr:expr, $sh:expr, $ud:expr, $ecc:expr) => {
        $sh as usize
    };
    (user_data,  $sync:expr, $hdr:expr, $sh:expr, $ud:expr, $ecc:expr) => {
        $ud as usize
    };
    (edc_ecc,    $sync:expr, $hdr:expr, $sh:expr, $ud:expr, $ecc:expr) => {
        $ecc as usize
    };
}

macro_rules! emit_one_accessor {
    (sync, $off:expr, $sync:expr, $hdr:expr, $sh:expr, $ud:expr, $ecc:expr) => {
        pub fn sync(&self) -> &'a [u8; { $sync as usize }] {
            (&self.buf[$off..$off + $sync as usize]).try_into().unwrap()
        }
    };
    (header, $off:expr, $sync:expr, $hdr:expr, $sh:expr, $ud:expr, $ecc:expr) => {
        pub fn header(&self) -> &'a [u8; { $hdr as usize }] {
            (&self.buf[$off..$off + $hdr as usize]).try_into().unwrap()
        }
    };
    (sub_header, $off:expr, $sync:expr, $hdr:expr, $sh:expr, $ud:expr, $ecc:expr) => {
        pub fn sub_header(&self) -> &'a [u8; { $sh as usize }] {
            (&self.buf[$off..$off + $sh as usize]).try_into().unwrap()
        }
    };
    (user_data, $off:expr, $sync:expr, $hdr:expr, $sh:expr, $ud:expr, $ecc:expr) => {
        pub fn user_data(&self) -> &'a [u8; { $ud as usize }] {
            (&self.buf[$off..$off + $ud as usize]).try_into().unwrap()
        }
    };
    (edc_ecc, $off:expr, $sync:expr, $hdr:expr, $sh:expr, $ud:expr, $ecc:expr) => {
        pub fn edc_ecc(&self) -> &'a [u8; { $ecc as usize }] {
            (&self.buf[$off..$off + $ecc as usize]).try_into().unwrap()
        }
    };
}

macro_rules! emit_field_accessors {
    ($t:ident, $flag:expr, [], $off:expr, $sizes:tt) => {};

    ($t:ident, $flag:expr, [$field:ident $(, $rest:ident)*], $off:expr,
     { sync: $sync:expr, header: $hdr:expr, sub_header: $sh:expr,
       user_data: $ud:expr, edc_ecc: $ecc:expr }) => {
        impl<'a, C: C2ErrorInfo, S: SubChannelSelection>
            Sector<'a, $t<{ $flag }>, C, S>
        {
            emit_one_accessor!($field, $off, $sync, $hdr, $sh, $ud, $ecc);
        }
        emit_field_accessors!(
            $t, $flag, [$($rest),*],
            $off + field_size!($field, $sync, $hdr, $sh, $ud, $ecc),
            { sync: $sync, header: $hdr, sub_header: $sh,
              user_data: $ud, edc_ecc: $ecc }
        );
    };
}

macro_rules! impl_sector_selection {
    (
        $sector_selection:ident,
        expected_sector_type: $sector_type:expr,
        field_sizes: {
            sync:       $sync_size:expr,
            header:     $header_size:expr,
            sub_header: $sub_header_size:expr,
            user_data:  $user_data_size:expr,
            edc_ecc:    $edc_ecc_size:expr $(,)?
        },
        selections: [
            $( [ $($field:ident),* $(,)? ] ),+ $(,)?
        ] $(,)?
    ) => {
        impl<const MAIN_CHANNEL_SELECTION: u8> private::SectorType
            for $sector_selection<MAIN_CHANNEL_SELECTION>
        {
            const EXPECTED_SECTOR_TYPE: u3 = $sector_type;
            const SYNC_SIZE: u16 = $sync_size;
            const SUB_HEADER_SIZE: u16 = $sub_header_size;
            const HEADER_SIZE: u16 = $header_size;
            const USER_DATA_SIZE: u16 = $user_data_size;
            const EDC_ECC_SIZE: u16 = $edc_ecc_size;
        }

        $(
            impl private::MainChannelSelectionSeal
                for $sector_selection<{ selection_flag!($($field),*) }>
            {
                const MAIN_CHANNEL_SELECTION: u8 = selection_flag!($($field),*);
            }

            emit_field_accessors!(
                $sector_selection,
                selection_flag!($($field),*),
                [$($field),*],
                0usize,
                { sync: $sync_size, header: $header_size, sub_header: $sub_header_size,
                  user_data: $user_data_size, edc_ecc: $edc_ecc_size }
            );
        )+

        impl<const MAIN_CHANNEL_SELECTION: u8> SectorSelection
            for $sector_selection<MAIN_CHANNEL_SELECTION>
        where
            $sector_selection<MAIN_CHANNEL_SELECTION>: private::MainChannelSelectionSeal,
        {
        }
    };
}

pub mod cd_da;
pub mod mode1;
pub mod mode2_form1;
pub mod mode2_form2;
pub mod mode2_formless;
