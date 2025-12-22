use libcdio_sys::{cdio_free_device_list, cdio_get_devices, driver_id_t_DRIVER_LINUX};
use std::ffi::CStr;

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

fn main() {
    let drives = list_drives();

    println!("Connected Drives:");

    for drive in drives {
        println!("{drive}");
    }
}
