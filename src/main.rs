use crate::cdio::{get_track_nums, list_drives};

mod addressing;
mod cdio;
mod constants;

fn main() {
    let drives = list_drives();

    if drives.is_empty() {
        return;
    }

    unsafe {
        get_track_nums(&drives[0]);
    }
}
