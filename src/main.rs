extern crate libc as libc;
extern crate libloading as lib;

use libretro_sys::{CoreAPI, SystemInfo};
use std::env;
use std::ffi::CStr;
use std::ptr;

fn main() {
    let path = env::current_dir().unwrap();
    println!("Current dir = {}", path.display());

    let lib = lib::Library::new("assets/cores/nestopia_libretro.dylib").unwrap();
    unsafe {
        //TODO: is this what I want to do??

        //        let core_api = CoreAPI {
        //            retro_api_version: lib.get(b"retro_api_version").unwrap(),
        //            retro_get_system_info: lib.get(b"retro_get_system_info").unwrap(),
        //        };

        let get_version: lib::Symbol<unsafe extern "C" fn() -> libc::c_uint> =
            lib.get(b"retro_api_version").unwrap();
        let version = get_version();
        println!("libretro api version: {}", version);

        let get_system_info: lib::Symbol<unsafe extern "C" fn(info: *mut SystemInfo)> =
            lib.get(b"retro_get_system_info").unwrap();

        //TODO: how to not do this??
        let mut sys_info = SystemInfo {
            library_name: ptr::null(),
            library_version: ptr::null(),
            valid_extensions: ptr::null(),
            need_fullpath: false,
            block_extract: false,
        };
        let raw_ptr = &mut sys_info as *mut SystemInfo;
        get_system_info(raw_ptr);

        println!("System info:");
        println!(
            "  library_name: {}",
            CStr::from_ptr(sys_info.library_name).to_str().unwrap()
        );
        println!(
            "  library_version: {}",
            CStr::from_ptr(sys_info.library_version).to_str().unwrap()
        );
        println!(
            "  valid_extensions: {}",
            CStr::from_ptr(sys_info.valid_extensions).to_str().unwrap()
        );
        println!("  need_fullpath:{}", sys_info.need_fullpath);
    }
}
