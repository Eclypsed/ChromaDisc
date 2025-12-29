pub const CHROMADISC_VERSION: &str = "0.0.1";

/// The number of frames per seconds of audio
pub const FRAMES_PER_SECOND: u8 = 75;

/// The number of frames per minute of audio
#[allow(dead_code)]
pub const FRAMES_PER_MINUTE: u16 = FRAMES_PER_SECOND as u16 * 60;

/// The number of frames in the pregap.
///
/// CDs are designed so that the first frame of playable audio actually occurs at 00:02:00 (2
/// seconds in), meaning there are 150 frames of gap between 00:00:00 and the first Logical Block
/// Address (index 0) at 00:02:00.
pub const PREGAP_OFFSET: u8 = FRAMES_PER_SECOND * 2;

/// The maximum number of tracks a CD can hold
#[allow(dead_code)]
pub const MAX_TRACKS: u8 = 99;

/// The track number of the leadout section
#[allow(dead_code)]
pub const LEADOUT_TRACK_NUM: u8 = 0xAA;
