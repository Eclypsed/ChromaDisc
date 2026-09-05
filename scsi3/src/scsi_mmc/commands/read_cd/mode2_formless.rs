use super::*;

impl<const MAIN_CHANNEL_SELECTION: u8> ReadCd<Lba, Mode2Formless<MAIN_CHANNEL_SELECTION>>
where
    Mode2Formless<MAIN_CHANNEL_SELECTION>: SectorSelection,
{
    pub fn new(
        starting_lba: Lba,
        transfer_length: u24,
        c2_error_info: C2ErrorInfo,
        sub_channel_selection: SubChannelSelection,
        control: Control,
    ) -> Self {
        Self {
            _sector_selection: PhantomData,
            digital_audio_play: false,
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
    for ReadCd<A, Mode2Formless<MAIN_CHANNEL_SELECTION>>
where
    Mode2Formless<MAIN_CHANNEL_SELECTION>: SectorSelection,
    ReadCd<A, Mode2Formless<MAIN_CHANNEL_SELECTION>>: Command<O>,
{
    type Len = u64;
    type Response<'a> = ();
    type Error = ();

    fn response_len(&self) -> Self::Len {
        todo!()
    }

    fn parse<'a>(&self, _buf: &'a [u8]) -> Result<Self::Response<'a>, Self::Error> {
        todo!()
    }
}

pub struct Mode2Formless<const MAIN_CHANNEL_SELECTION: u8>;
impl_sector_selection!(
    Mode2Formless,
    private::SectorType::Mode2Formless,
    [
        MainChannelSelection::NO_FIELDS,                                // 00h
        MainChannelSelection::USER_DATA,                                // 10h
        MainChannelSelection::HEADER,                                   // 20h
        MainChannelSelection::HEADER | MainChannelSelection::USER_DATA, // 30h
        MainChannelSelection::SYNC,                                     // 80h
        MainChannelSelection::SYNC | MainChannelSelection::HEADER,      // A0h
        MainChannelSelection::SYNC | MainChannelSelection::HEADER | MainChannelSelection::USER_DATA, // B0h
    ]
);
