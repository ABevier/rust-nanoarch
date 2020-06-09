extern crate glfw;
extern crate libc;
extern crate libloading;

use glfw::{Action, Context, Key};
use libretro_sys::{CoreAPI, SystemInfo, EnvironmentFn, VideoRefreshFn, AudioSampleFn, AudioSampleBatchFn, InputPollFn, InputStateFn};
use std::env;
use std::ffi::CStr;
use std::ptr;
use libloading::{Symbol, Library};
use users::get_current_username;

pub struct RetroApi<'a> {
    init: Symbol<'a, unsafe extern "C" fn()>,
    api_version: Symbol<'a, unsafe extern "C" fn() -> libc::c_uint>,
    get_system_info: Symbol<'a, unsafe extern "C" fn(info: *mut SystemInfo)>,

    set_environment_callback: Symbol<'a, unsafe extern "C" fn(callback: EnvironmentFn)>,
    set_video_refresh_callback: Symbol<'a, unsafe extern "C" fn(callback: VideoRefreshFn)>,
    set_audio_sample_callback: Symbol<'a, unsafe extern "C" fn(callback: AudioSampleFn)>,
    set_audio_sample_batch_callback: Symbol<'a, unsafe extern "C" fn(callback: AudioSampleBatchFn)>,
    set_input_poll_callback: Symbol<'a, unsafe extern "C" fn(callback: InputPollFn)>,
    set_input_state_callback: Symbol<'a, unsafe extern "C" fn(callback: InputStateFn)>,
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
    //TODO: how do I access my window or any other state...????
    match cmd {
        libretro_sys::ENVIRONMENT_GET_USERNAME => {
            println!("ENV: GET USER NAME");
            //TODO: how to cast and set a *void that is **char ??
            match get_current_username() {
                Some(uname) => println!("username: {}", uname.to_str().unwrap()),
                None        => println!("could not get user name"),
            }
        },
        libretro_sys::ENVIRONMENT_GET_LOG_INTERFACE => {
            println!("ENV: GET LOG INTERFACE");
        },
        libretro_sys::ENVIRONMENT_GET_CAN_DUPE => {
            println!("ENV: GET CAN DUPE");
            //TODO: signal true to allow duped frames
        },
        libretro_sys::ENVIRONMENT_SET_PIXEL_FORMAT => {
            println!("ENV: SET PIXEL FORMAT");
        },
        libretro_sys::ENVIRONMENT_GET_SYSTEM_DIRECTORY => {
            println!("ENV: SYSTEM DIRECTORY")
        },
        libretro_sys::ENVIRONMENT_GET_SAVE_DIRECTORY => {
            println!("ENV: SAVE DIRECTORY")
        },
        libretro_sys::ENVIRONMENT_SHUTDOWN => {
            println!("ENV: SHUTDOWN REQUESTED")
        },
        libretro_sys::ENVIRONMENT_GET_VARIABLE => {
            println!("ENV: GET VARIABLE")
        },
        _ => {
            println!("UNKNOWN ENV CMD: {}", cmd)
        }
    }

    false
}

extern "C" fn video_refresh_cb(data: *const libc::c_void, width: libc::c_uint, height: libc::c_uint, pitch: libc::size_t) {
    println!("video_refresh called!");
}

extern "C" fn audio_sample_cb(left: i16, right: i16) {
    println!("audio_sample_called");
}

extern "C" fn audio_sample_batch(data: *const i16, frames: libc::size_t) -> libc::size_t {
    println!("audio_sample_batch_called");
    0
}

extern "C" fn input_poll_cb() {
    println!("input_poll_called")
}

extern "C" fn input_state_cb(port: libc::c_uint, device: libc::c_uint, index: libc::c_uint, id: libc::c_uint) -> i16 {
    println!("input_state called");
    0
}


fn init_retro_api(lib: &Library) -> RetroApi {
    unsafe {
        let core_api = RetroApi {
            init: lib.get(b"retro_init").unwrap(),
            api_version: lib.get(b"retro_api_version").unwrap(),
            get_system_info: lib.get(b"retro_get_system_info").unwrap(),
            set_environment_callback: lib.get(b"retro_set_environment").unwrap(),
            set_video_refresh_callback: lib.get(b"retro_set_video_refresh").unwrap(),
            set_audio_sample_callback: lib.get(b"retro_set_audio_sample").unwrap(),
            set_audio_sample_batch_callback: lib.get(b"retro_set_audio_sample_batch").unwrap(),
            set_input_poll_callback: lib.get(b"retro_set_input_poll").unwrap(),
            set_input_state_callback: lib.get(b"retro_set_input_state").unwrap()
        };

        (core_api.set_environment_callback)(set_environment_cb);
        (core_api.set_video_refresh_callback)(video_refresh_cb);
        (core_api.set_audio_sample_callback)(audio_sample_cb);
        (core_api.set_audio_sample_batch_callback)(audio_sample_batch);
        (core_api.set_input_poll_callback)(input_poll_cb);
        (core_api.set_input_state_callback)(input_state_cb);

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
