extern crate glfw;
extern crate libc;
extern crate libloading;

use glfw::{Action, Context, Key};
use libretro_sys::{CoreAPI, SystemInfo};
use std::env;
use std::ffi::CStr;
use std::ptr;

fn main() {
    let path = env::current_dir().unwrap();
    println!("Current dir = {}", path.display());

    let lib = libloading::Library::new("assets/cores/nestopia_libretro.dylib").unwrap();
    unsafe {
        //TODO: is this what I want to do??

        //        let core_api = CoreAPI {
        //            retro_api_version: lib.get(b"retro_api_version").unwrap(),
        //            retro_get_system_info: lib.get(b"retro_get_system_info").unwrap(),
        //        };

        let get_version: libloading::Symbol<unsafe extern "C" fn() -> libc::c_uint> =
            lib.get(b"retro_api_version").unwrap();
        let version = get_version();
        println!("libretro api version: {}", version);

        let get_system_info: libloading::Symbol<unsafe extern "C" fn(info: *mut SystemInfo)> =
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

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(300, 300, "My Window", glfw::WindowMode::Windowed)
        .expect("Couldn't create window");

    window.make_current();
    window.set_key_polling(true);

    while !window.should_close() {
        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                _ => {}
            }
        }
    }
}
