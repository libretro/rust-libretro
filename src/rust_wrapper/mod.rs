use libc::c_uint;
use libc::size_t;
use core::intrinsics::transmute;
use libc::types::common::c95::c_void;
use libc::types::os::arch::c95::c_char;
use core::ptr::null_mut;
use libc::malloc;
use libc::free;
use core::mem::size_of;
use core::mem::uninitialized;
use rust_wrapper::libretro::*;
use core::str::raw::c_str_to_static_slice;
use core::prelude::*;
pub mod libretro;



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

static mut retro_variables: [retro_variable, ..2] =
    [retro_variable {key: 0u as *const c_char, value: 0u as *const c_char}, ..2];

pub enum CoreLogicRate {
    LogicRate60 = 60,
    LogicRate120 = 120,
    LogicRate720 = 720,
}

static FRAME_RATE_KEY: &'static str = "refresh_rate\0";
static LOW_FRAME_RATE_VALUES: &'static str =
    "Display Refresh Rate; 60|30\0";
static MEDIUM_FRAME_RATE_VALUES: &'static str =
    "Display Refresh Rate; 60|120|30\0";
static HIGH_FRAME_RATE_VALUES: &'static str =
    "Display Refresh Rate; 60|72|80|90|102.9|120|144|180|240|24|30|48|51.4|\0";

static mut retro_environment_cb: Option<retro_environment_t> = None;
static mut retro_log_cb: Option<retro_log_printf_t> = None;
#[no_mangle]
pub unsafe extern "C" fn retro_set_environment(cb: retro_environment_t)
{
    use super::{NO_CONTENT, CORE_LOGIC_RATE};
    
    retro_environment_cb = Some(cb);
    
    let no_content: *mut c_void =
        if NO_CONTENT {
            transmute(&NO_CONTENT_FLAG)
        } else {
            transmute(&REQUIRED_CONTENT_FLAG)
        };
    retro_environment_cb.unwrap()(RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME,
                                  no_content);

    let keyptr = FRAME_RATE_KEY.as_ptr() as *const i8;

    retro_variables[0] =
        retro_variable { key: keyptr,
                         value: match CORE_LOGIC_RATE {
                             LogicRate60 => LOW_FRAME_RATE_VALUES,
                             LogicRate120 => MEDIUM_FRAME_RATE_VALUES,
                             LogicRate720 => HIGH_FRAME_RATE_VALUES,
                             }.as_ptr() as *const c_char };
        
    retro_environment_cb.unwrap()(RETRO_ENVIRONMENT_SET_VARIABLES,
                                  retro_variables.as_mut_ptr() as *mut c_void);

    let log_interface = retro_log_callback { log: transmute(0u) };
    retro_environment_cb.unwrap()(RETRO_ENVIRONMENT_GET_LOG_INTERFACE,
                                  transmute(&log_interface));
    retro_log_cb = Some(log_interface.log);
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
        let c_text = malloc_ascii_cstring(text);
        retro_log_cb.unwrap()(level as i32, "%s\n\0".as_ptr() as *const c_char, c_text);
        free(c_text as *mut c_void);
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
      //  fail!("Core option error");
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
            let info: retro_system_av_info = uninitialized();
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
        unsafe {c_str_to_static_slice(get_variable.value) };

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


struct StaticSystemInfo
{
    name: *const c_char,
    version: *const c_char,
    extensions: *const c_char,
}

static mut static_system_info: StaticSystemInfo = StaticSystemInfo
{
    name: 0u8 as *const c_char,
    version: 0u8 as *const c_char,
    extensions: 0u8 as *const c_char,
};

#[no_mangle]
pub unsafe extern "C" fn retro_get_system_info(info: *mut retro_system_info)
{
    use super::{CORE_NAME, CORE_VERSION, VALID_EXTENSIONS};

    static_system_info.name = malloc_ascii_cstring(CORE_NAME);
    static_system_info.version = malloc_ascii_cstring(CORE_VERSION);
    static_system_info.extensions = malloc_ascii_cstring(VALID_EXTENSIONS);
    
    (*info).library_name     = static_system_info.name;
    (*info).library_version  = static_system_info.version;
    (*info).valid_extensions = static_system_info.extensions;
    (*info).need_fullpath    = false as u8;
    (*info).block_extract    = false as u8;
}

unsafe fn malloc_ascii_cstring(src: &str) -> *const c_char
{
    let terminated_max_len = (src.as_bytes().len() + 1) as size_t;
    let dst: *mut c_char = malloc(terminated_max_len) as *mut c_char;
    strip_utf_strlcpy(dst, src.as_bytes(), terminated_max_len);
    dst as *const c_char
}

unsafe fn strip_utf_strlcpy(dst: *mut c_char, src: &[u8], dst_size: size_t)
{
    if dst_size == 0 { return; }
    if dst_size == 1
    {
        *dst = 0i8;
        return;
    }
    
    let mut dst_offset: int = 0;
    let dst_last = dst_size - 1; // reserve space for NULL terminator
    for src_byte in src.iter()
    {
        if (*src_byte & 0x80) == 0
        {
            *dst.offset(dst_offset) = *src_byte as i8;
            dst_offset = dst_offset + 1;
            if dst_offset == dst_last as int { break; }
        }
    }
    *dst.offset(dst_offset) = 0i8;
}


// WARNING
// Don't change without also changing InputState and static asserts
pub enum ControllerButton
{
    PadB = RETRO_DEVICE_ID_JOYPAD_B as int,
    PadY = RETRO_DEVICE_ID_JOYPAD_Y as int,
    PadSelect = RETRO_DEVICE_ID_JOYPAD_SELECT as int,
    PadStart = RETRO_DEVICE_ID_JOYPAD_START as int,
    PadUp = RETRO_DEVICE_ID_JOYPAD_UP as int,
    PadDown = RETRO_DEVICE_ID_JOYPAD_DOWN as int,
    PadLeft = RETRO_DEVICE_ID_JOYPAD_LEFT as int,
    PadRight = RETRO_DEVICE_ID_JOYPAD_RIGHT as int,
    PadA = RETRO_DEVICE_ID_JOYPAD_A as int,
    PadX = RETRO_DEVICE_ID_JOYPAD_X as int,
    PadL = RETRO_DEVICE_ID_JOYPAD_L as int,
    PadR = RETRO_DEVICE_ID_JOYPAD_R as int,
    PadL2 = RETRO_DEVICE_ID_JOYPAD_L2 as int,
    PadR2 = RETRO_DEVICE_ID_JOYPAD_R2 as int,
    PadL3 = RETRO_DEVICE_ID_JOYPAD_L3 as int,
    PadR3 = RETRO_DEVICE_ID_JOYPAD_R3 as int,
}

#[static_assert]
static _I0: bool = (PadB as int == 0);
#[static_assert]
static _I1: bool = (PadY as int == 1);
#[static_assert]
static _I2: bool = (PadSelect as int == 2);
#[static_assert]
static _I3: bool = (PadStart as int == 3);
#[static_assert]
static _I4: bool = (PadUp as int == 4);
#[static_assert]
static _I5: bool = (PadDown as int == 5);
#[static_assert]
static _I6: bool = (PadLeft as int == 6);
#[static_assert]
static _I7: bool = (PadRight as int == 7);
#[static_assert]
static _I8: bool = (PadA as int == 8);
#[static_assert]
static _I9: bool = (PadX as int == 9);
#[static_assert]
static _I10: bool = (PadL as int == 10);
#[static_assert]
static _I11: bool = (PadR as int == 11);
#[static_assert]
static _I12: bool = (PadL2 as int == 12);
#[static_assert]
static _I13: bool = (PadR2 as int == 13);
#[static_assert]
static _I14: bool = (PadL3 as int == 14);
#[static_assert]
static _I15: bool = (PadR3 as int == 15);

pub struct InputState
{
    // WARNING
    // Don't change size without also changing ControllerButton
    // and static asserts
    button: [ButtonState, ..16]
}

pub struct ButtonState
{
    pub pressed: bool,
    pub down: bool,
    pub up: bool
}

impl Index<ControllerButton, ButtonState> for InputState
{
    fn index<'a>(&'a self, index: &ControllerButton) -> &'a ButtonState
    {
        unsafe {self.button.unsafe_get(*index as uint)}
    }
}

impl InputState
{
    pub fn poll(player: u32) -> InputState
    {
        // assert!(player < 16, "Tried to poll input for invalid player number");
        let state: InputState =
        unsafe
        {
            InputState
            {
                button: [
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadB as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadY as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadSelect as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadStart as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadUp as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadDown as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadLeft as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadRight as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadA as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadX as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadL as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadR as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadL2 as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadR2 as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadL3 as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadR3 as u32) != 0, down: false, up: false},
                    ]
            }
        };
        // TODO track state changes and update down and up fields
        state      
    }
}

#[no_mangle]
pub extern fn retro_run()
{
    use super::{AV_SCREEN_WIDTH, AV_SCREEN_HEIGHT, COLOR_DEPTH_32};

//    unsafe {VIDEO_LOCK.lock_noguard();}
    
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
  //          unsafe {VIDEO_LOCK.unlock_noguard();}
  //          unsafe {
  //              let guard = VIDEO_WAIT.lock();
  //              guard.signal();
  //          }
       }
       super::core_run();
    }


    //unsafe {VIDEO_LOCK.lock_noguard();}
    unsafe {
        retro_video_refresh_cb.unwrap()(frame_buf as *const c_void,
                                        AV_SCREEN_WIDTH,
                                        AV_SCREEN_HEIGHT,
                                        (AV_SCREEN_WIDTH *
                                         if COLOR_DEPTH_32 {4} else {2}) as size_t);
    }
    //unsafe {VIDEO_LOCK.unlock_noguard();}
 
}
pub static mut frame_buf: *mut c_void = 0i as *mut c_void;

#[no_mangle]
pub unsafe extern "C" fn retro_init()
{
    use super::{AV_SCREEN_WIDTH, AV_SCREEN_HEIGHT, COLOR_DEPTH_32};
  
    frame_buf = malloc(((AV_SCREEN_WIDTH as uint) *
                        (AV_SCREEN_HEIGHT as uint)) as u64 *
                       if COLOR_DEPTH_32 {size_of::<u32>()}
                       else {size_of::<u16>()} as u64);

    // start video thread
    //Thread::spawn(video_thread);
}

/*
static VIDEO_SHUTDOWN: AtomicBool = INIT_ATOMIC_BOOL;
static VIDEO_LOCK: StaticNativeMutex = NATIVE_MUTEX_INIT;
static VIDEO_WAIT: StaticNativeMutex = NATIVE_MUTEX_INIT;

fn video_thread()
{
    println!("Video thread starts");
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
*/

#[no_mangle]
pub unsafe extern "C" fn retro_deinit()
{
//    VIDEO_SHUTDOWN.store(true, SeqCst);
//    {
//        let guard = VIDEO_WAIT.lock();
//        guard.signal();
//    }
//    VIDEO_LOCK.destroy();
       
    if frame_buf != 0u8 as *mut c_void
        { free(frame_buf); }
    if static_system_info.name != 0u8 as *const c_char
        { free(static_system_info.name as *mut c_void); }
    if static_system_info.version != 0u8 as *const c_char
        { free(static_system_info.version as *mut c_void); }
    if static_system_info.extensions != 0u8 as *const c_char
        { free(static_system_info.extensions as *mut c_void); }
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
pub extern "C" fn retro_get_memory_data(_id: c_uint) -> *mut u8 { null_mut() }
#[no_mangle]
pub extern "C" fn retro_get_memory_size(_id: c_uint) -> size_t { 0 }
#[no_mangle]
pub extern "C" fn retro_load_game(_info: *const u8) -> u8 { true as u8}
#[no_mangle]
pub extern "C" fn retro_api_version() -> c_uint { 1 }

