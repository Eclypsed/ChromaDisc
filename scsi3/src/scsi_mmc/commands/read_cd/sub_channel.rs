use arbitrary_int::u3;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

use crate::core::util::{Absent, Presence, Present};

mod private {

    use super::*;

    pub trait SubChannelMarker {
        const SUB_CHANNEL_SELECTION: u3;
        type SubChannelData: FromBytes + IntoBytes + Immutable + KnownLayout + Unaligned;
        type SubChannelPresence: Presence;
    }
}

pub trait SubChannelSelection: private::SubChannelMarker + 'static {}

pub struct NoSubChannel;
impl private::SubChannelMarker for NoSubChannel {
    const SUB_CHANNEL_SELECTION: u3 = u3::new(0b000);
    type SubChannelData = ();
    type SubChannelPresence = Absent;
}
impl SubChannelSelection for NoSubChannel {}

pub struct RawPw;
impl private::SubChannelMarker for RawPw {
    const SUB_CHANNEL_SELECTION: u3 = u3::new(0b001);
    type SubChannelData = [u8; 96];
    type SubChannelPresence = Present;
}
impl SubChannelSelection for RawPw {}

pub struct FormattedQ;
impl private::SubChannelMarker for FormattedQ {
    const SUB_CHANNEL_SELECTION: u3 = u3::new(0b010);
    type SubChannelData = [u8; 16];
    type SubChannelPresence = Present;
}
impl SubChannelSelection for FormattedQ {}

pub struct CorrectedDeinterleavedRw;
impl private::SubChannelMarker for CorrectedDeinterleavedRw {
    const SUB_CHANNEL_SELECTION: u3 = u3::new(0b100);
    type SubChannelData = [u8; 96];
    type SubChannelPresence = Present;
}
impl SubChannelSelection for CorrectedDeinterleavedRw {}
