use super::*;

impl<const MAIN_CHANNEL_SELECTION: u8, C: C2ErrorInfo, S: SubChannelSelection>
    ReadCd<Lba, Mode2Formless<MAIN_CHANNEL_SELECTION>, C, S>
where
    Mode2Formless<MAIN_CHANNEL_SELECTION>: SectorSelection,
{
    pub fn new(starting_lba: Lba, transfer_length: u24, control: Control) -> Self {
        Self {
            _sector_selection: PhantomData,
            digital_audio_play: false,
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
    ReadCd<Msf, Mode2Formless<MAIN_CHANNEL_SELECTION>, C, S>
where
    Mode2Formless<MAIN_CHANNEL_SELECTION>: SectorSelection,
{
    pub fn new(msf_range: Span<Msf>, control: Control) -> Self {
        Self {
            _sector_selection: PhantomData,
            digital_audio_play: false,
            addressing_params: msf_range,
            _c2_marker: PhantomData,
            _sub_channel_marker: PhantomData,
            control,
        }
    }
}

pub struct Mode2Formless<
    const MAIN_CHANNEL_SELECTION: u8 = {
        MainChannel::SYNC | MainChannel::HEADER | MainChannel::USER_DATA
    },
>;
impl_sector_selection!(
    Mode2Formless,
    expected_sector_type: u3::new(0b011),
    field_sizes: {
        sync:       12,
        header:     4,
        sub_header: 0,
        user_data:  2336,
        edc_ecc:    0,
    },
    selections: [
        [],                        // 00h
        [user_data],               // 10h
        [header],                  // 20h
        [header, user_data],       // 30h
        [sync],                    // 80h
        [sync, header],            // A0h
        [sync, header, user_data], // B0h
    ],
);
