use super::private::{MainChannelSelectionSeal, SectorRange};
use super::*;

impl<const MAIN_CHANNEL_SELECTION: u8> ReadCd<Lba, CdDa<MAIN_CHANNEL_SELECTION>>
where
    CdDa<MAIN_CHANNEL_SELECTION>: SectorSelection,
{
    pub fn new(
        digital_audio_play: bool,
        starting_lba: Lba,
        transfer_length: u24,
        c2_error_info: C2ErrorInfo,
        sub_channel_selection: SubChannelSelection,
        control: Control,
    ) -> Self {
        Self {
            _sector_selection: PhantomData,
            digital_audio_play,
            addressing_params: private::LbaAddressingParams {
                starting_lba,
                transfer_length,
            },
            c2_error_info,
            sub_channel_selection,
            control,
        }
    }
}

impl<O: OpCodeDef, A: ReadCdAddress, const MAIN_CHANNEL_SELECTION: u8> ReadCommand<O>
    for ReadCd<A, CdDa<MAIN_CHANNEL_SELECTION>>
where
    CdDa<MAIN_CHANNEL_SELECTION>: SectorSelection,
    ReadCd<A, CdDa<MAIN_CHANNEL_SELECTION>>: Command<O>,
{
    type Len = u64;
    type Response<'a> = ();
    type Error = ();

    fn response_len(&self) -> Self::Len {
        let mut sector_size: u64 = 0;
        if CdDa::<MAIN_CHANNEL_SELECTION>::user_data() {
            sector_size += 2352;
        }

        sector_size * self.addressing_params.sector_count() as u64
    }

    fn parse<'a>(&self, _buf: &'a [u8]) -> Result<Self::Response<'a>, Self::Error> {
        todo!()
    }
}

pub struct CdDa<const MAIN_CHANNEL_SELECTION: u8>;
impl_sector_selection!(
    CdDa,
    private::SectorType::CdDa,
    [
        MainChannelSelection::NO_FIELDS, // 00h
        MainChannelSelection::USER_DATA  // 10h
    ]
);
