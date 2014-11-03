extern crate core;
extern crate libc;
extern crate collections;

use libc::c_uint;
use libc::size_t;
use libc::types::common::c95::c_void;
use libc::types::os::arch::c95::c_char;
use core::prelude::*;
use core::intrinsics::transmute;
use core::atomic::{AtomicBool, SeqCst, INIT_ATOMIC_BOOL};
use collections::*;

use rust_wrapper::libretro::*;
pub use rust_wrapper::input::{InputState, ButtonState, ControllerButton,
                              PadB, PadY, PadSelect, PadStart, PadUp, PadDown,
                              PadLeft, PadRight, PadA, PadX, PadL, PadR,
                              PadL2, PadR2, PadL3, PadR3};
mod input;
mod lang_items;
#[allow(dead_code)] mod libretro;
#[allow(dead_code)] #[path = "rustrt_files/mutex.rs"] mod mutex;
#[allow(dead_code)] #[path = "rustrt_files/thread.rs"] mod thread;
#[allow(dead_code)] #[path = "rustrt_files/stack.rs"] mod stack;
#[allow(dead_code)] #[path = "rustrt_files/stack_overflow.rs"] mod stack_overflow;

macro_rules! CORE_NAME(
    ($name:expr) => (
        static CORE_NAME: &'static str = concat!($name,"\0");
        );
    )

macro_rules! CORE_VERSION(
    ($version:expr) => (
        static CORE_VERSION: &'static str = concat!($version,"\0");
        );
    )

macro_rules! VALID_EXTENSIONS(
    ($ext:expr) => (
        static VALID_EXTENSIONS: &'static str = concat!($ext,"\0");
        );
    )
    
pub struct EnvVar {
    pub key: &'static str,
    pub desc: &'static str,
    pub values: &'static [&'static str]
}



// Set up the automatically configured callbacks
pub static mut retro_video_refresh_cb: Option<retro_video_refresh_t> = None;
#[no_mangle]
pub unsafe extern "C" fn retro_set_video_refresh(cb: retro_video_refresh_t)
{
    retro_video_refresh_cb = Some(cb);
}

pub static mut retro_audio_sample_cb: Option<retro_audio_sample_t> = None;
#[no_mangle]
pub unsafe extern "C" fn retro_set_audio_sample(cb: retro_audio_sample_t)
{
    retro_audio_sample_cb = Some(cb);
}

pub static mut retro_audio_sample_batch_cb: Option<retro_audio_sample_batch_t>
    = None;
#[no_mangle]
pub unsafe extern "C" fn retro_set_audio_sample_batch(cb: retro_audio_sample_batch_t)
{
    retro_audio_sample_batch_cb = Some(cb);
}

pub static mut retro_input_poll_cb: Option<retro_input_poll_t> = None;
#[no_mangle]
pub unsafe extern "C" fn retro_set_input_poll(cb: retro_input_poll_t)
{
    retro_input_poll_cb = Some(cb);
}

pub static mut retro_input_state_cb: Option<retro_input_state_t> = None;
#[no_mangle]
pub unsafe extern "C" fn retro_set_input_state(cb: retro_input_state_t)
{
    retro_input_state_cb = Some(cb);
}

static NO_CONTENT_FLAG: u8  = true as u8;
static REQUIRED_CONTENT_FLAG: u8 = false as u8;

pub enum CoreLogicRate {
    LogicRate60 = 60,
    LogicRate120 = 120,
    LogicRate720 = 720,
}

static FRAME_RATE_KEY: &'static str = "frame_rate\0";
static LOW_FRAME_RATE_VALUES: &'static str =
    "Frame rate; 60|30\0";
static MEDIUM_FRAME_RATE_VALUES: &'static str =
    "Frame rate; 60|120|30\0";
static HIGH_FRAME_RATE_VALUES: &'static str =
    "Frame rate; 60|72|80|90|102.9|120|144|180|240|24|30|48|51.4|\0";

static mut retro_environment_cb: Option<retro_environment_t> = None;
static mut retro_log_cb: Option<retro_log_printf_t> = None;
#[no_mangle]
pub extern "C" fn retro_set_environment(cb: retro_environment_t)
{
    use super::{NO_CONTENT, ENV_VARS, CORE_LOGIC_RATE};
    use collections::slice::{MutableOrdSlice, CloneableVector};

    unsafe {
        retro_environment_cb = Some(cb);

        let log_interface = retro_log_callback { log: transmute(0u) };
        retro_environment_cb.unwrap()(RETRO_ENVIRONMENT_GET_LOG_INTERFACE,
                                      transmute(&log_interface));
        retro_log_cb = Some(log_interface.log);
    
        let no_content: *mut c_void =
            if NO_CONTENT {
                transmute(&NO_CONTENT_FLAG)
            } else {
                transmute(&REQUIRED_CONTENT_FLAG)
            };
        retro_environment_cb.unwrap()(RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME,
                                      no_content);
    }

    // reserve space for automatically implemented + null retro_variables
    let num_vars = ENV_VARS.len() + 2;
    let mut retro_variables = Vec::<retro_variable>::with_capacity(num_vars);
    // Rust needs to hold onto the & references until after the call to C
    let mut keystrings = Vec::<String>::with_capacity(num_vars);
    let mut descstrings = Vec::<String>::with_capacity(num_vars);
    
    // add the automatic env variables
    retro_variables.push(
        retro_variable { key: FRAME_RATE_KEY.as_ptr() as *const i8,
                         value: match CORE_LOGIC_RATE {
                             LogicRate60 => LOW_FRAME_RATE_VALUES,
                             LogicRate120 => MEDIUM_FRAME_RATE_VALUES,
                             LogicRate720 => HIGH_FRAME_RATE_VALUES,
                         }.as_ptr() as *const c_char } );
    keystrings.push(String::from_str(FRAME_RATE_KEY));

    // add the use env variables
    for var in ENV_VARS.iter() {
        for byte in var.key.as_bytes().iter() {
            if *byte == 0u8 { panic!("ENV_VAR key must not contain nulls."); }
        }
        let key = var.key.to_ascii_cstring();

        for byte in var.desc.as_bytes().iter() {
            if *byte == 0u8 { panic!("ENV_VAR desc must not contain nulls."); }
        }
        // desc.len() + semicolon + space + [value.len() + pipe]
        let value_max_len =
            var.desc.len() + 1 + 1 +
            var.values.iter().fold(0, |sum, &x| sum + x.len() + 1);
        let mut value_string = String::with_capacity(value_max_len);

        value_string.push_str(var.desc);
        value_string.push_str("; ");

        for value in var.values.iter() {
            for byte in value.as_bytes().iter() {
                if *byte == 0u8 {
                    panic!("ENV_VAR values must not contain nulls.");
                }
            }
            value_string.push_str(*value);
            value_string.push_str("|");
        }
        let value_cstring = value_string.to_ascii_cstring();
        
        retro_variables.push(
            retro_variable { key: key.as_ptr() as *const c_char,
                             value: value_cstring.as_ptr() as *const c_char } );
        keystrings.push(key);
        descstrings.push(value_cstring);
    }

    retro_variables.push(retro_variable { key: 0u as *const c_char,
                                          value: 0u as *const c_char } );
    keystrings.push(String::from_str(""));

    let mut key_sort= keystrings.clone();
    key_sort.sort();
    let mut key_unique = key_sort.to_vec();
    key_unique.dedup();
    if keystrings.len() != key_unique.len() {
        panic!("Duplicate environment variable keys are forbidden. Are you trying to manually implement an automatic environment variable?");
    }
   
    unsafe {
        retro_environment_cb.unwrap()(
            RETRO_ENVIRONMENT_SET_VARIABLES,
            retro_variables.as_mut_ptr() as *mut c_void);
    }
}

pub enum LogLevel
{
    LogDebug = RETRO_LOG_DEBUG as int,
    LogInfo = RETRO_LOG_INFO as int,
    LogWarn = RETRO_LOG_WARN as int,
    LogError = RETRO_LOG_ERROR as int
}

/// Safely wrapping printf is complicated, so for now only support printing
/// &strs with utf removed.
pub fn retro_log(level: LogLevel, text: &str)
{
    // TODO copy to the stack if text is short
    unsafe {
        let c_text = text.to_ascii_cstring();
        retro_log_cb.unwrap()(level as i32, "%s\n\0".as_ptr() as *const c_char,
                              c_text.as_ptr() as *const c_char);
    }
}

pub fn retro_log_panic(msg: &str, file: &str, line: uint)
{
    // TODO copy to the stack if text is short
    unsafe {
        let c_msg = msg.to_ascii_cstring();
        let c_file = file.to_ascii_cstring();
        retro_log_cb.unwrap()(
            LogError as i32, "\"%s\" at %s line %u\n\0".as_ptr() as *const c_char,
            c_msg.as_ptr() as *const c_char,
            c_file.as_ptr() as *const c_char,
            line);
   }
}

fn set_retro_system_av_info(info: &mut retro_system_av_info, fps: f64)
{
    use super::{AV_SCREEN_WIDTH, AV_SCREEN_HEIGHT, AV_PIXEL_ASPECT,
                AV_SAMPLE_RATE};

    #[static_assert]
    const _A1: bool = AV_SCREEN_HEIGHT > 0;
    #[static_assert]
    const _A2: bool = AV_SCREEN_WIDTH > 0;
    #[static_assert]
    const _A3: bool = AV_PIXEL_ASPECT > 0.0;

    info.timing.fps = fps;
    info.timing.sample_rate = AV_SAMPLE_RATE;
    info.geometry.base_width   = AV_SCREEN_WIDTH;
    info.geometry.base_height  = AV_SCREEN_HEIGHT;
    info.geometry.max_width    = AV_SCREEN_WIDTH;
    info.geometry.max_height   = AV_SCREEN_HEIGHT;
    info.geometry.aspect_ratio = AV_PIXEL_ASPECT;
}


#[no_mangle]
// Silence false warning, because Rust fails to track variable through transmute
#[allow(unused_assignments)]
pub unsafe extern "C" fn retro_get_system_av_info(info: *mut retro_system_av_info)
{
    use super::{COLOR_DEPTH_32, CORE_LOGIC_RATE};

    let frame_mult = get_frame_mult();
    let mut fps = 60.0;
    
    if frame_mult.is_some() {
        fps = CORE_LOGIC_RATE as u32 as f64 /
            frame_mult.unwrap() as f64;
    }
    else {
         panic!("Core option error");
    }

    set_retro_system_av_info(transmute(info), fps);

    let pixel_format: *mut c_void = if COLOR_DEPTH_32 {
        transmute(&RETRO_PIXEL_FORMAT_XRGB8888)
    }
    else {
        transmute(&RETRO_PIXEL_FORMAT_RGB565)
    };
    retro_environment_cb.unwrap()(RETRO_ENVIRONMENT_SET_PIXEL_FORMAT, pixel_format);
}


/// Gets the current frame multiplier.
/// Caches the current value and only runs the more expensive
/// get_environment_frame_mult() if a core option has changed.
fn get_frame_mult() -> Option<u32>
{
    use super::{CORE_LOGIC_RATE};
    static mut cached_frame_mult: Option<u32> = Some(1);
    static mut first_time: bool = true;

    let mut change: u8 = 0;
    unsafe
    {
        retro_environment_cb.unwrap()(
            RETRO_ENVIRONMENT_GET_VARIABLE_UPDATE,
            transmute(&change));
     
        if first_time || change != 0
        {
            first_time = false;
            let new_frame_mult = get_environment_frame_mult();
            if new_frame_mult == cached_frame_mult {
                change = 0;
            }
            else {
                cached_frame_mult = new_frame_mult;
            }
        }

        if change != 0
        {
            let info: retro_system_av_info = core::mem::uninitialized();
            set_retro_system_av_info(transmute(&info), CORE_LOGIC_RATE as u32 as f64 /
                               cached_frame_mult.unwrap() as f64);
            retro_environment_cb.unwrap()(
                RETRO_ENVIRONMENT_SET_SYSTEM_AV_INFO,
                transmute(&info));
        }

        return cached_frame_mult;
    }
}

fn get_environment_frame_mult() -> Option<u32>
{
    use super::CORE_LOGIC_RATE;
        let get_variable =
        retro_variable {key: FRAME_RATE_KEY.as_ptr() as *const i8,
                        value: 0u as *const c_char};
    
    unsafe { retro_environment_cb.unwrap()(RETRO_ENVIRONMENT_GET_VARIABLE,
                                           transmute(&get_variable)); }

    let refresh_rate =
        unsafe {core::str::raw::c_str_to_static_slice(get_variable.value) };

    match CORE_LOGIC_RATE {
        LogicRate60 =>
            match refresh_rate {
                "30" => Some(2u32),
                "60" => Some(1u32),
                _ => None,
            },
        LogicRate120 =>
            match refresh_rate {
                "30" => Some(4u32),
                "60" => Some(2u32),
                "120" => Some(1u32),
                _ => None,
            },
        LogicRate720 =>
            match refresh_rate {
                "24" => Some(30u32),
                "30" => Some(24u32),
                "48" => Some(15u32),
                "51.4" => Some(14u32),
                "60" => Some(12u32),
                "72" => Some(10u32),
                "80" => Some(9u32),
                "90" => Some(8u32),
                "102.9" => Some(7u32),
                "120" => Some(6u32),
                "144" => Some(5u32),
                "180" => Some(4u32),
                "240" => Some(3u32),
                _ => None,
            }
    }
}


#[no_mangle]
pub unsafe extern "C" fn retro_get_system_info(info: *mut retro_system_info)
{
    use super::{CORE_NAME, CORE_VERSION, VALID_EXTENSIONS};

    for retro_string in [CORE_NAME, CORE_VERSION, VALID_EXTENSIONS].iter()
    {
        retro_string.check_valid();
    }
   
    (*info).library_name     = CORE_NAME.as_ptr() as *const i8;
    (*info).library_version  = CORE_VERSION.as_ptr() as *const i8;
    (*info).valid_extensions = VALID_EXTENSIONS.as_ptr() as *const i8;
    (*info).need_fullpath    = false as u8;
    (*info).block_extract    = false as u8;
}

trait RetroString
{
    fn check_valid(self);
    fn to_ascii_cstring(self) -> String;
}

impl<'a> RetroString for &'a str
{
    fn check_valid(self)
    {
        for b in self.as_bytes().iter()
        {
            if (b & 0x80) != 0 {
                panic!("All libretro strings must be ascii.");
            }
        }
        if self.as_bytes()[self.len() - 1] != 0u8 {
            panic!("All libretro strings must be null terminated.");
        }
    }
    fn to_ascii_cstring(self) -> String
    {
        let terminated_max_len = self.as_bytes().len() + 1;
        
        let mut dst = String::with_capacity(terminated_max_len);
        
        for src_byte in self.bytes()
        {
            if (src_byte & 0x80) == 0
            {
                dst.push(src_byte as char);
            }
        }
        dst.push('\0');
        dst
    }    
}



#[no_mangle]
pub extern fn retro_run()
{
    use super::{AV_SCREEN_WIDTH, AV_SCREEN_HEIGHT, COLOR_DEPTH_32};

    unsafe {VIDEO_LOCK.lock_noguard();}
    
    // For now, poll input hardware only once per displayed frame
    // (InputState::poll uses cached values)
    // libretro version 2 will support polling every logic update
    unsafe {retro_input_poll_cb.unwrap()();}
    for i in range(0, get_frame_mult().unwrap()) {
        if i==0 {

            // TODO set the video latency
            // Currently set to maximum possible

            super::snapshot_video();
            super::render_video();
            unsafe {VIDEO_LOCK.unlock_noguard();}
            unsafe {
                let guard = VIDEO_WAIT.lock();
                guard.signal();
            }
       }
       super::core_run();
    }

    unsafe {VIDEO_LOCK.lock_noguard();}
    unsafe {
        retro_video_refresh_cb.unwrap()(frame_buf as *const c_void,
                                        AV_SCREEN_WIDTH,
                                        AV_SCREEN_HEIGHT,
                                        (AV_SCREEN_WIDTH *
                                         if COLOR_DEPTH_32 {4} else {2}) as size_t);
    }
    unsafe {VIDEO_LOCK.unlock_noguard();}
 
}
pub static mut frame_buf: *mut c_void = 0i as *mut c_void;

#[no_mangle]
pub unsafe extern "C" fn retro_init()
{
    use super::{AV_SCREEN_WIDTH, AV_SCREEN_HEIGHT, COLOR_DEPTH_32};
  
    frame_buf = libc::malloc(((AV_SCREEN_WIDTH as uint) *
                              (AV_SCREEN_HEIGHT as uint)) as u64 *
                             if COLOR_DEPTH_32 {core::mem::size_of::<u32>()}
                             else {core::mem::size_of::<u16>()} as u64);

    // start video thread
    thread::Thread::spawn(video_thread);
}


static VIDEO_SHUTDOWN: AtomicBool = INIT_ATOMIC_BOOL;
static VIDEO_LOCK: mutex::StaticNativeMutex = mutex::NATIVE_MUTEX_INIT;
static VIDEO_WAIT: mutex::StaticNativeMutex = mutex::NATIVE_MUTEX_INIT;

fn video_thread()
{
    loop
    {
        unsafe {
            let guard = VIDEO_WAIT.lock();
            guard.wait();
        }
        if VIDEO_SHUTDOWN.load(SeqCst) { break; }
        unsafe {VIDEO_LOCK.lock_noguard();}
        super::render_video();
        unsafe {VIDEO_LOCK.unlock_noguard();}
    }
}


#[no_mangle]
pub unsafe extern "C" fn retro_deinit()
{
    VIDEO_SHUTDOWN.store(true, SeqCst);
    {
        let guard = VIDEO_WAIT.lock();
        guard.signal();
    }
    VIDEO_LOCK.destroy();
    if frame_buf != 0u8 as *mut c_void { libc::free(frame_buf); }
}       


// implement stubs for mandatory extern functions

#[no_mangle]
pub extern "C" fn retro_set_controller_port_device(_port: c_uint, _device: c_uint) {}
#[no_mangle]
pub extern "C" fn retro_reset() {}
#[no_mangle]
pub extern "C" fn retro_serialize_size() -> size_t { 0 }
#[no_mangle]
pub extern "C" fn retro_serialize(_data: *mut u8, _size: size_t) -> u8 { false as u8 }
#[no_mangle]
pub extern "C" fn retro_unserialize(_data: *const u8, _size: size_t) -> u8 { false as u8 }
#[no_mangle]
pub extern "C" fn retro_cheat_reset() {}
#[no_mangle]
pub extern "C" fn retro_cheat_set(_index: c_uint, _enabled: u8, _code: *const u8) {}
#[no_mangle]
pub extern "C" fn retro_load_game_special(_type: c_uint, _info: *const retro_game_info, _num: size_t) -> u8 { false as u8 }
#[no_mangle]
pub extern "C" fn retro_unload_game() {}
#[no_mangle]
pub extern "C" fn retro_get_region() -> c_uint { RETRO_REGION_NTSC }
#[no_mangle]
pub extern "C" fn retro_get_memory_data(_id: c_uint) -> *mut u8 { core::ptr::null_mut() }
#[no_mangle]
pub extern "C" fn retro_get_memory_size(_id: c_uint) -> size_t { 0 }
#[no_mangle]
pub extern "C" fn retro_load_game(_info: *const u8) -> u8 { true as u8}
#[no_mangle]
pub extern "C" fn retro_api_version() -> c_uint { 1 }

