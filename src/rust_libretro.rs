#![feature(globs)]

use libc::c_uint;
use libc::types::os::arch::c95::size_t;

use c_libretro::*;
pub mod c_libretro;




// Set up the automatically configured callbacks
pub static mut retro_video_refresh_cb: Option<retro_video_refresh_t> = None;
unsafe extern "C" fn retro_set_video_refresh(cb: retro_video_refresh_t)
{
	retro_video_refresh_cb = Some(cb);
}

pub static mut retro_audio_sample_cb: Option<retro_audio_sample_t> = None;
unsafe extern "C" fn retro_set_audio_sample(cb: retro_audio_sample)
{
	retro_audio_sample_cb = Some(cb);
}

pub static mut retro_audio_sample_batch_cb: Option<retro_audio_sample_batch_t> = None;
unsafe extern "C" fn retro_set_audio_sample_batch(cb: retro_audio_sample_batch_t)
{
	retro_audio_sample_batch_cb = Some(cb);
}

pub static mut retro_input_poll_cb: Option<retro_audio_sample_batch_t> = None;
unsafe extern "C" fn retro_set_input_poll(cb: retro_audio_sample_batch_t)
{
	retro_input_poll_cb = Some(cb);
}

pub static mut retro_input_state_cb: Option<retro_input_state_t> = None;
unsafe extern "C" fn retro_set_input_state(cb: retro_input_state_t)
{
	retro_input_state_cb = Some(cb);
}

// implement stubs for mandatory extern functions

pub extern "C" fn retro_set_controller_port_device(_port: c_uint, _device: c_uint) {}
pub extern "C" fn retro_reset() {}
pub extern "C" fn retro_serialize_size() -> size_t { 0 }
pub extern "C" fn retro_serialize(_data: *mut u8, _size: size_t) -> u8 { false as u8 }
pub extern "C" fn retro_unserialize(_data: *const u8, _size: size_t) -> u8 { false as u8 }
pub extern "C" fn retro_cheat_reset() {}
pub extern "C" fn retro_cheat_set(_index: c_uint, _enabled: u8, _code: *const u8) {}
pub extern "C" fn retro_load_game_special(_type: c_uint, _info: *const retro_game_info, _num: size_t) -> u8 { false as u8 }
pub extern "C" fn retro_unload_game() {}
pub extern "C" fn retro_get_region() -> c_uint { RETRO_REGION_NTSC }
pub extern "C" fn retro_get_memory_data(_id: c_uint) -> *mut u8 {	core::ptr::null_mut() }
pub extern "C" fn retro_get_memory_size(_id: c_uint) -> size_t { 0 }
pub extern "C" fn retro_load_game(_info: *const u8) -> u8 { true as u8}
