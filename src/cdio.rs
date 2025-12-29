use libcdio_sys::{
    CDIO_INVALID_LSN, cdio_destroy, cdio_free_device_list, cdio_get_devices,
    cdio_get_first_track_num, cdio_get_num_tracks, cdio_get_track_lsn, cdio_open,
    cdio_track_enums_CDIO_CDROM_LEADOUT_TRACK, driver_id_t_DRIVER_LINUX,
};
use std::ffi::{CStr, CString};

#[allow(dead_code)]
pub fn list_drives() -> Vec<String> {
    let mut drives: Vec<String> = Vec::new();

    unsafe {
        let devlist = cdio_get_devices(driver_id_t_DRIVER_LINUX);

        if devlist.is_null() {
            return drives;
        }

        let mut i = 0;
        loop {
            let dev = *devlist.add(i);
            if dev.is_null() {
                break;
            }

            let cstr = CStr::from_ptr(dev);
            drives.push(cstr.to_string_lossy().into_owned());
            i += 1;
        }

        cdio_free_device_list(devlist);
    }

    drives
}

#[allow(dead_code)]
pub unsafe fn get_track_nums(device: &str) {
    unsafe {
        let device_name = CString::new(device).expect("CString::new failed");
        let p_cdio = cdio_open(device_name.as_ptr(), driver_id_t_DRIVER_LINUX);
        let first_track_num = cdio_get_first_track_num(p_cdio);
        let num_tracks = cdio_get_num_tracks(p_cdio);

        println!(
            "CD-ROM Track List ({} - {})",
            first_track_num,
            first_track_num + num_tracks - 1
        );

        for i in 0..=num_tracks {
            let lsn = cdio_get_track_lsn(p_cdio, i);
            if lsn != CDIO_INVALID_LSN {
                println!("{:3}: {:06}", i, lsn);
            }
        }

        let leadout_track: u8 = cdio_track_enums_CDIO_CDROM_LEADOUT_TRACK
            .try_into()
            .unwrap();
        let leadout_lsn = cdio_get_track_lsn(p_cdio, leadout_track);
        println!("{:3X}: {:06} leadout", leadout_track, leadout_lsn);

        cdio_destroy(p_cdio);
    }
}
