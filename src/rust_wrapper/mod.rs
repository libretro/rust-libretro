extern crate core;
use libc::c_uint;
use libc::types::os::arch::c95::size_t;
use std::mem::transmute;
use libc::types::common::c95::c_void;
use rust_wrapper::libretro::*;
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

static mut retro_environment_cb: Option<retro_environment_t> = None;
#[no_mangle]
pub unsafe extern "C" fn retro_set_environment(cb: retro_environment_t)
{
    use super::NO_CONTENT;
    
    retro_environment_cb = Some(cb);
    
    let no_content: *mut c_void =
        if NO_CONTENT {
            transmute(&NO_CONTENT_FLAG)
        } else {
            transmute(&REQUIRED_CONTENT_FLAG)
        };
    retro_environment_cb.unwrap()(RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME, no_content);
    
    let pixel_format: *mut c_void = transmute(&RETRO_PIXEL_FORMAT_RGB565);
    retro_environment_cb.unwrap()(RETRO_ENVIRONMENT_SET_PIXEL_FORMAT, pixel_format);
}


#[no_mangle]
pub unsafe extern "C" fn retro_get_system_av_info(info: *mut retro_system_av_info)
{
    use super::{AV_SCREEN_WIDTH, AV_SCREEN_HEIGHT, AV_MAX_SCREEN_WIDTH,
                AV_MAX_SCREEN_HEIGHT, AV_PIXEL_ASPECT, AV_FRAME_RATE,
                AV_SAMPLE_RATE};
    #[static_assert]
    static _A1: bool = AV_FRAME_RATE > 0.0;
    #[static_assert]
    static _A2: bool = AV_SCREEN_HEIGHT <= AV_MAX_SCREEN_HEIGHT;
    #[static_assert]
    static _A3: bool = AV_SCREEN_WIDTH <= AV_MAX_SCREEN_WIDTH;
    #[static_assert]
    static _A4: bool = AV_SCREEN_HEIGHT > 0;
    #[static_assert]
    static _A5: bool = AV_SCREEN_WIDTH > 0;
    #[static_assert]
    static _A6: bool = AV_FRAME_RATE > 0.0;
    #[static_assert]
    static _A7: bool = AV_PIXEL_ASPECT > 0.0;
 
    (*info).timing.fps = AV_FRAME_RATE;
    (*info).timing.sample_rate = AV_SAMPLE_RATE;
    (*info).geometry.base_width   = AV_SCREEN_WIDTH;
    (*info).geometry.base_height  = AV_SCREEN_HEIGHT;
    (*info).geometry.max_width    = AV_MAX_SCREEN_WIDTH;
    (*info).geometry.max_height   = AV_MAX_SCREEN_HEIGHT;
    (*info).geometry.aspect_ratio = AV_PIXEL_ASPECT;
}

#[no_mangle]
pub unsafe extern "C" fn retro_get_system_info(info: *mut retro_system_info)
{
    use super::{CORE_NAME, CORE_VERSION, VALID_EXTENSIONS};

    // TODO: is this memset really necessary?
    // rlibc::memset(transmute(info), 0, size_of::<retro_system_info>());

    (*info).library_name     = "Hello World\0".as_ptr() as *const i8;  // Rust strings are not null terminated
    (*info).library_version  = "0.0.1\0".as_ptr() as *const i8;        // Null terminate manually
    (*info).valid_extensions = " \0".as_ptr() as *const i8;
    (*info).need_fullpath    = false as u8;
    (*info).block_extract    = false as u8;
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
pub extern "C" fn retro_get_memory_data(_id: c_uint) -> *mut u8 {	core::ptr::null_mut() }
#[no_mangle]
pub extern "C" fn retro_get_memory_size(_id: c_uint) -> size_t { 0 }
#[no_mangle]
pub extern "C" fn retro_load_game(_info: *const u8) -> u8 { true as u8}
#[no_mangle]
pub extern "C" fn retro_api_version() -> c_uint { 1 }

