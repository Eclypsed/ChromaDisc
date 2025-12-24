/// The number of frames per seconds of audio
pub const FRAMES_PER_SECOND: u8 = 75;

/// The number of frames per minute of audio
pub const FRAMES_PER_MINUTE: u16 = FRAMES_PER_SECOND as u16 * 60;

/// The number of frames in the pregap.
///
/// CDs are designed so that the first frame of playable audio actually occurs at 00:02:00 (2
/// seconds in), meaning there are 150 frames of gap between the first Logical Block Address (LBA)
/// at 00:00:00 and the first Logical Sector Number (LSN) at 00:02:00.
pub const PREGAP_OFFSET: u8 = FRAMES_PER_SECOND * 2;
