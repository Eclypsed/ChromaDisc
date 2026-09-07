use super::*;

impl<const MAIN_CHANNEL_SELECTION: u8, C: C2ErrorInfo, S: SubChannelSelection>
    ReadCd<Lba, Mode2Form2<MAIN_CHANNEL_SELECTION>, C, S>
where
    Mode2Form2<MAIN_CHANNEL_SELECTION>: SectorSelection,
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
    ReadCd<Msf, Mode2Form2<MAIN_CHANNEL_SELECTION>, C, S>
where
    Mode2Form2<MAIN_CHANNEL_SELECTION>: SectorSelection,
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

pub struct Mode2Form2<
    const MAIN_CHANNEL_SELECTION: u8 = {
        MainChannel::SYNC
            | MainChannel::SUB_HEADER
            | MainChannel::HEADER
            | MainChannel::USER_DATA
            | MainChannel::EDC_ECC
    },
>;
impl_sector_selection!(
    Mode2Form2,
    expected_sector_type: u3::new(0b101),
    field_sizes: {
        sync:       12,
        header:     4,
        sub_header: 8,
        user_data:  2324,
        edc_ecc:    4, // Optional, filled with 0s if not present
    },
    selections: [
        [],                                             // 00h
        [edc_ecc],                                      // 08h
        [user_data],                                    // 10h
        [user_data, edc_ecc],                           // 18h
        [header],                                       // 20h
        [sub_header],                                   // 40h
        [sub_header, user_data],                        // 50h
        [sub_header, user_data, edc_ecc],               // 58h
        [header, sub_header],                           // 60h
        [header, sub_header, user_data],                // 70h
        [header, sub_header, user_data, edc_ecc],       // 78h
        [sync],                                         // 80h
        [sync, header],                                 // A0h
        [sync, header, sub_header],                     // E0h
        [sync, header, sub_header, user_data],          // F0h
        [sync, header, sub_header, user_data, edc_ecc], // F8h
    ],
);
