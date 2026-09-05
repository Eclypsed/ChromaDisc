use super::*;

impl<const MAIN_CHANNEL_SELECTION: u8> ReadCd<Lba, Mode2Form2<MAIN_CHANNEL_SELECTION>>
where
    Mode2Form2<MAIN_CHANNEL_SELECTION>: SectorSelection,
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
    for ReadCd<A, Mode2Form2<MAIN_CHANNEL_SELECTION>>
where
    Mode2Form2<MAIN_CHANNEL_SELECTION>: SectorSelection,
    ReadCd<A, Mode2Form2<MAIN_CHANNEL_SELECTION>>: Command<O>,
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

pub struct Mode2Form2<const MAIN_CHANNEL_SELECTION: u8>;
impl_sector_selection!(
    Mode2Form2,
    private::SectorType::Mode2Form2,
    [
        MainChannelSelection::NO_FIELDS, // 00h
        MainChannelSelection::EDC_ECC,   // 08h
        MainChannelSelection::USER_DATA, // 10h
        MainChannelSelection::USER_DATA | MainChannelSelection::EDC_ECC, // 18h
        MainChannelSelection::HEADER,    // 20h
        MainChannelSelection::SUB_HEADER, // 40h
        MainChannelSelection::SUB_HEADER | MainChannelSelection::USER_DATA, // 50h
        MainChannelSelection::SUB_HEADER // 58h
            | MainChannelSelection::USER_DATA
            | MainChannelSelection::EDC_ECC,
        MainChannelSelection::SUB_HEADER | MainChannelSelection::HEADER, // 60h
        MainChannelSelection::SUB_HEADER // 70h
            | MainChannelSelection::HEADER
            | MainChannelSelection::USER_DATA,
        MainChannelSelection::SUB_HEADER // 78h
            | MainChannelSelection::HEADER
            | MainChannelSelection::USER_DATA
            | MainChannelSelection::EDC_ECC,
        MainChannelSelection::SYNC,                                // 80h
        MainChannelSelection::SYNC | MainChannelSelection::HEADER, // A0h
        MainChannelSelection::SYNC // E0h
            | MainChannelSelection::SUB_HEADER
            | MainChannelSelection::HEADER,
        MainChannelSelection::SYNC // F0h
            | MainChannelSelection::SUB_HEADER
            | MainChannelSelection::HEADER
            | MainChannelSelection::USER_DATA,
        MainChannelSelection::SYNC // F8h
            | MainChannelSelection::SUB_HEADER
            | MainChannelSelection::HEADER
            | MainChannelSelection::USER_DATA
            | MainChannelSelection::EDC_ECC,
    ]
);
