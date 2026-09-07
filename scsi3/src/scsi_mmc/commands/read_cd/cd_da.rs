use super::*;

impl<const MAIN_CHANNEL_SELECTION: u8, C: C2ErrorInfo, S: SubChannelSelection>
    ReadCd<Lba, CdDa<MAIN_CHANNEL_SELECTION>, C, S>
where
    CdDa<MAIN_CHANNEL_SELECTION>: SectorSelection,
{
    pub fn new(
        digital_audio_play: bool,
        starting_lba: Lba,
        transfer_length: u24,
        control: Control,
    ) -> Self {
        Self {
            _sector_selection: PhantomData,
            digital_audio_play,
            addressing_params: private::LbaAddressingParams {
                starting_lba,
                transfer_length,
            },
            _c2_marker: PhantomData,
            _sub_channel_marker: PhantomData,
            control,
        }
    }
}

impl<const MAIN_CHANNEL_SELECTION: u8, C: C2ErrorInfo, S: SubChannelSelection>
    ReadCd<Msf, CdDa<MAIN_CHANNEL_SELECTION>, C, S>
where
    CdDa<MAIN_CHANNEL_SELECTION>: SectorSelection,
{
    pub fn new(digital_audio_play: bool, msf_range: Span<Msf>, control: Control) -> Self {
        Self {
            _sector_selection: PhantomData,
            digital_audio_play,
            addressing_params: msf_range,
            _c2_marker: PhantomData,
            _sub_channel_marker: PhantomData,
            control,
        }
    }
}

pub struct CdDa<const MAIN_CHANNEL_SELECTION: u8 = { MainChannel::USER_DATA }>;
impl_sector_selection!(
    CdDa,
    expected_sector_type: u3::new(0b001),
    field_sizes: {
        sync:       0,
        header:     0,
        sub_header: 0,
        user_data:  2352,
        edc_ecc:    0,
    },
    selections: [
        [],          // 00h
        [user_data], // 10h
    ],
);
