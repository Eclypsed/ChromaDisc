use core::marker::PhantomData;

use crate::{
    core::{
        addressing::{Lba, Span},
        util::{Presence, Present},
        Command, Control, OpCode, OpCodeDef, ReadCommand,
    },
    mmc::device_models::cd::addressing::Msf,
};
use arbitrary_int::u24;
use derive_where::derive_where;
use thiserror::Error;
use zerocopy::{CastError, FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

pub mod c2;
pub mod main_channel;
pub mod sub_channel;

mod private {
    use super::*;

    pub trait SectorRange {
        fn sector_count(&self) -> u32;
    }

    pub trait AddressingModeSeal {
        type AddressingParams: SectorRange;
    }

    impl AddressingModeSeal for Lba {
        type AddressingParams = LbaAddressingParams;
    }

    impl AddressingModeSeal for Msf {
        type AddressingParams = Span<Msf>;
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
}

pub trait ReadCdAddress: private::AddressingModeSeal {}
impl ReadCdAddress for Lba {}
impl ReadCdAddress for Msf {}

pub struct ReadCd<
    A: ReadCdAddress,
    M: main_channel::SectorMode,
    L: main_channel::SelectionOf<M>,
    C: c2::C2ErrorInfo = c2::NoC2,
    S: sub_channel::SubChannelSelection = sub_channel::NoSubChannel,
> {
    digital_audio_play: bool,
    // Interestingly, in the LBA version of this command, byte 1 bit 0 is an obsolete RELADDR flag.
    // However, every reference going back to MMC-1 says this flag should just be 0, so I don't
    // know where it came from but I'm choosing to omit it.
    addressing_params: A::AddressingParams,
    control: Control,
    #[allow(clippy::type_complexity)]
    _marker: PhantomData<fn() -> (M, L, C, S)>,
}

type ReadCdOpcode = OpCode<0xBE>;
type ReadCdMsfOpcode = OpCode<0xB9>;

impl<
        M: main_channel::SectorMode,
        L: main_channel::SelectionOf<M>,
        C: c2::C2ErrorInfo,
        S: sub_channel::SubChannelSelection,
    > Command<ReadCdOpcode> for ReadCd<Lba, M, L, C, S>
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
            (main_channel::MainChannel::<M, L>::SELECTION_VALUE) | (C::C2_SELECTION.value() << 1),
            (S::SUB_CHANNEL_SELECTION.value()),
            self.control.into(),
        ]
    }
}

impl<
        M: main_channel::SectorMode,
        L: main_channel::SelectionOf<M>,
        C: c2::C2ErrorInfo,
        S: sub_channel::SubChannelSelection,
    > Command<ReadCdMsfOpcode> for ReadCd<Msf, M, L, C, S>
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
            (main_channel::MainChannel::<M, L>::SELECTION_VALUE) | (C::C2_SELECTION.value() << 1),
            (S::SUB_CHANNEL_SELECTION.value()),
            self.control.into(),
        ]
    }
}

macro_rules! impl_read_cd_constructor {
    (@impl $sector_mode:ty; [$($dap_param:tt)*]; $dap_value:expr) => {
        impl<
                L: main_channel::SelectionOf<$sector_mode>,
                C: c2::C2ErrorInfo,
                S: sub_channel::SubChannelSelection,
            > ReadCd<Lba, $sector_mode, L, C, S>
        {
            pub fn new(
                $($dap_param)*
                starting_lba: Lba,
                transfer_length: u24,
                control: Control,
            ) -> Self {
                Self {
                    digital_audio_play: $dap_value,
                    addressing_params: private::LbaAddressingParams {
                        starting_lba,
                        transfer_length,
                    },
                    control,
                    _marker: PhantomData,
                }
            }
        }

        impl<
                L: main_channel::SelectionOf<$sector_mode>,
                C: c2::C2ErrorInfo,
                S: sub_channel::SubChannelSelection,
            > ReadCd<Msf, $sector_mode, L, C, S>
        {
            pub fn new($($dap_param)* msf_range: Span<Msf>, control: Control) -> Self {
                Self {
                    digital_audio_play: $dap_value,
                    addressing_params: msf_range,
                    control,
                    _marker: PhantomData,
                }
            }
        }
    };

    // No argument: the field is always `false`.
    ($sector_mode:ty) => {
        impl_read_cd_constructor!(@impl $sector_mode; []; false);
    };

    // With argument: `new` takes `digital_audio_play: bool` as its first parameter.
    ($sector_mode:ty, digital_audio_play) => {
        impl_read_cd_constructor!(
            @impl $sector_mode; [digital_audio_play: bool,]; digital_audio_play
        );
    };
}

// Probably explain here why there is no way to construct an "All sector types" command.
// Either that or in the future add an "All sector types" option that has no Main Channel
// restrictions or useful parsing beyond chunking the raw sectors (since we don't know anything
// about the sectors returned, beyond them being either all CD-DA or all CD Data, without external
// information)

impl_read_cd_constructor!(main_channel::CdDa, digital_audio_play);
impl_read_cd_constructor!(main_channel::Mode1);
impl_read_cd_constructor!(main_channel::Mode2Formless);
impl_read_cd_constructor!(main_channel::Mode2Form1);
impl_read_cd_constructor!(main_channel::Mode2Form2);

#[derive(FromBytes, IntoBytes, Immutable, KnownLayout, Unaligned)]
#[derive_where(Debug, Clone, Copy, PartialEq, Eq;
    main_channel::MainChannel<M, L>,
    <C::C2Presence as Presence>::Output<C::C2Data>,
    <S::SubChannelPresence as Presence>::Output<S::SubChannelData>)]
#[repr(C)]
pub struct Transfer<
    M: main_channel::SectorMode,
    L: main_channel::SelectionOf<M>,
    C: c2::C2ErrorInfo = c2::NoC2,
    S: sub_channel::SubChannelSelection = sub_channel::NoSubChannel,
> {
    main_channel: main_channel::MainChannel<M, L>,
    c2: <C::C2Presence as Presence>::Output<C::C2Data>,
    sub_channel: <S::SubChannelPresence as Presence>::Output<S::SubChannelData>,
}

impl<
        M: main_channel::SectorMode,
        L: main_channel::SelectionOf<M>,
        C: c2::C2ErrorInfo,
        S: sub_channel::SubChannelSelection,
    > Transfer<M, L, C, S>
{
    #[inline]
    pub const fn main_channel(&self) -> &main_channel::MainChannel<M, L> {
        &self.main_channel
    }

    #[inline]
    pub const fn main_channel_mut(&mut self) -> &mut main_channel::MainChannel<M, L> {
        &mut self.main_channel
    }
}

impl<
        M: main_channel::SectorMode,
        L: main_channel::SelectionOf<M>,
        C: c2::C2ErrorInfo<C2Presence = Present>,
        S: sub_channel::SubChannelSelection,
    > Transfer<M, L, C, S>
{
    #[inline]
    pub const fn c2(&self) -> &C::C2Data {
        &self.c2
    }

    #[inline]
    pub const fn c2_mut(&mut self) -> &mut C::C2Data {
        &mut self.c2
    }
}

impl<
        M: main_channel::SectorMode,
        L: main_channel::SelectionOf<M>,
        C: c2::C2ErrorInfo,
        S: sub_channel::SubChannelSelection<SubChannelPresence = Present>,
    > Transfer<M, L, C, S>
{
    #[inline]
    pub const fn sub_channel(&self) -> &S::SubChannelData {
        &self.sub_channel
    }

    #[inline]
    pub const fn sub_channel_mut(&mut self) -> &mut S::SubChannelData {
        &mut self.sub_channel
    }
}

#[derive(Debug, Error)]
pub enum TransfersError {
    #[error("Attempted to parse zero-sized transfer")]
    ZeroSizedTransfer,
}

#[derive(Debug, Clone)]
pub struct Transfers<
    'a,
    M: main_channel::SectorMode,
    L: main_channel::SelectionOf<M>,
    C: c2::C2ErrorInfo,
    S: sub_channel::SubChannelSelection,
> {
    buf: &'a [u8],
    #[allow(clippy::type_complexity)]
    _marker: PhantomData<fn() -> (M, L, C, S)>,
}

impl<
        'a,
        M: main_channel::SectorMode,
        L: main_channel::SelectionOf<M>,
        C: c2::C2ErrorInfo,
        S: sub_channel::SubChannelSelection,
    > Transfers<'a, M, L, C, S>
{
    pub fn new(buf: &'a [u8]) -> Result<Self, TransfersError> {
        if size_of::<Transfer<M, L, C, S>>() == 0 {
            return Err(TransfersError::ZeroSizedTransfer);
        }

        Ok(Self {
            buf,
            _marker: PhantomData,
        })
    }
}

impl<
        'a,
        M: main_channel::SectorMode + 'a,
        L: main_channel::SelectionOf<M> + 'a,
        C: c2::C2ErrorInfo + 'a,
        S: sub_channel::SubChannelSelection + 'a,
    > Iterator for Transfers<'a, M, L, C, S>
{
    type Item = &'a Transfer<M, L, C, S>;

    fn next(&mut self) -> Option<Self::Item> {
        match Transfer::<M, L, C, S>::ref_from_prefix(self.buf) {
            Ok((transfer, rest)) => {
                self.buf = rest;
                Some(transfer)
            }
            Err(CastError::Size(_)) => None,
            // See https://docs.rs/zerocopy/latest/zerocopy/trait.FromBytes.html#method.ref_from_prefix
            Err(CastError::Alignment(_)) => unreachable!("Transfer should be `Unaligned`"),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let num_transfers = self.buf.len() / size_of::<Transfer<M, L, C, S>>();
        (num_transfers, Some(num_transfers))
    }
}

impl<
        'a,
        M: main_channel::SectorMode + 'a,
        L: main_channel::SelectionOf<M> + 'a,
        C: c2::C2ErrorInfo + 'a,
        S: sub_channel::SubChannelSelection + 'a,
    > ExactSizeIterator for Transfers<'a, M, L, C, S>
{
}

impl<
        O: OpCodeDef,
        A: ReadCdAddress,
        M: main_channel::SectorMode,
        L: main_channel::SelectionOf<M>,
        C: c2::C2ErrorInfo,
        S: sub_channel::SubChannelSelection,
    > ReadCommand<O> for ReadCd<A, M, L, C, S>
where
    ReadCd<A, M, L, C, S>: Command<O>,
{
    type Len = u64;
    type Response<'a> = Transfers<'a, M, L, C, S>;
    type Error = TransfersError;

    fn response_len(&self) -> Self::Len {
        (size_of::<Transfer<M, L, C, S>>() as u64)
            * (private::SectorRange::sector_count(&self.addressing_params) as u64)
    }

    fn parse<'a>(&self, buf: &'a [u8]) -> Result<Self::Response<'a>, Self::Error> {
        Transfers::new(buf)
    }
}
