extern crate glfw;
extern crate libc;
extern crate libloading;

use glfw::{Action, Context, Key};
use libretro_sys::{CoreAPI, SystemInfo, EnvironmentFn};
use std::env;
use std::ffi::CStr;
use std::ptr;
use libloading::{Symbol, Library};

pub struct RetroApi<'a> {
    init: Symbol<'a, unsafe extern "C" fn()>,
    api_version: Symbol<'a, unsafe extern "C" fn() -> libc::c_uint>,
    get_system_info: Symbol<'a, unsafe extern "C" fn(info: *mut SystemInfo)>,
    set_environment_callback: Symbol<'a, unsafe extern "C" fn(callback: EnvironmentFn)>,
}

fn main() {
    let path = env::current_dir().unwrap();
    println!("Current dir = {}", path.display());

    let lib = libloading::Library::new("assets/cores/nestopia_libretro.dylib").unwrap();
    let retro_api = init_retro_api(&lib);
    get_version(&retro_api);
    get_system_info(&retro_api);

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

extern "C" fn set_environment_cb(cmd: libc::c_uint, data: *mut libc::c_void) -> bool {
    println!("set_environment called!");
    //TODO: how do I access my window or any other state...????
    false
}

fn init_retro_api(lib: &Library) -> RetroApi {
    unsafe {
        let core_api = RetroApi {
            init: lib.get(b"retro_init").unwrap(),
            api_version: lib.get(b"retro_api_version").unwrap(),
            get_system_info: lib.get(b"retro_get_system_info").unwrap(),
            set_environment_callback: lib.get(b"retro_set_environment").unwrap(),
        };

        (core_api.set_environment_callback)(set_environment_cb);

        (core_api.init)();

        return core_api;
    }
}

fn get_version(retro_api: &RetroApi) {
    unsafe {
        let version = (retro_api.api_version)();
        println!("libretro api version: {}", version);
    }
}

fn get_system_info(retro_api: &RetroApi) {
    unsafe {
        //TODO: how to not do this??
        let mut sys_info = SystemInfo {
            library_name: ptr::null(),
            library_version: ptr::null(),
            valid_extensions: ptr::null(),
            need_fullpath: false,
            block_extract: false,
        };
        let raw_ptr = &mut sys_info as *mut SystemInfo;
        (retro_api.get_system_info)(raw_ptr);

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
