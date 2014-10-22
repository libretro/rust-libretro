extern crate libc;
extern crate core;

use libc::size_t;
use libc::c_uint;
use libc::types::common::c95::c_void;
use core::prelude::*;

#[repr(C)]
pub struct retro_game_geometry
{
	pub base_width: libc::c_uint,
	pub base_height: libc::c_uint,
	pub max_width: libc::c_uint,
	pub max_height: libc::c_uint,
	pub aspect_ratio: f32
}

#[repr(C)]
pub struct retro_system_timing
{
	pub fps: f64,
	pub sample_rate: f64
}

#[repr(C)]
pub struct retro_system_av_info
{
	pub geometry: retro_game_geometry,
	pub timing: retro_system_timing
}

#[repr(C)]
pub struct retro_system_info
{
	pub library_name: *const u8, 
	pub library_version: *const u8,
	pub valid_extensions: *const u8,
	pub need_fullpath: bool,                                       
	pub block_extract: bool     
}

#[repr(C)]
pub struct retro_game_info
{
	pub path: *const u8,
	pub data: *const u8,
	pub size: size_t,
	pub meta: *const u8
}

pub static mut retro_video_refresh_cb: Option<extern fn (data: *mut c_void, width: c_uint, height: c_uint, pitch: c_uint)> = None;
#[no_mangle]
pub unsafe extern fn retro_set_video_refresh(cb: extern fn (data: *mut c_void, width: c_uint, height: c_uint, pitch: c_uint))
{
	// println!("hello world: retro_set_video_refresh");
	retro_video_refresh_cb = Some(cb);
}

pub static mut retro_audio_sample_cb: Option<extern fn (left: i16, right: i16)> = None;
#[no_mangle]
pub unsafe extern fn retro_set_audio_sample(cb: extern fn (left: i16, right: i16))
{
	// println!("hello world: retro_set_audio_sample");
	retro_audio_sample_cb = Some(cb);
}

pub static mut retro_audio_sample_batch_cb: Option<extern fn(data: *mut i16, frames: size_t) -> size_t> = None;
#[no_mangle]
pub unsafe extern fn retro_set_audio_sample_batch(cb: extern fn(data: *mut i16, frames: size_t) -> size_t)
{
	// println!("hello world: retro_set_audio_sample_batch");
	retro_audio_sample_batch_cb = Some(cb);
}

pub static mut retro_input_poll_cb: Option<extern fn()> = None;
#[no_mangle]
pub unsafe extern fn retro_set_input_poll(cb: extern fn())
{
	// println!("hello world: retro_set_input_poll");
	retro_input_poll_cb = Some(cb);
}


pub static mut retro_input_state_cb: Option<extern fn(port: libc::c_uint, device: libc::c_uint, index: libc::c_uint, id: libc::c_uint) -> i16> = None;
#[no_mangle]
pub unsafe extern fn retro_set_input_state(cb: extern fn(port: libc::c_uint, device: libc::c_uint, index: libc::c_uint, id: libc::c_uint) -> i16)
{
	// println!("hello world: retro_set_input_state");
	retro_input_state_cb = Some(cb);
}

#[no_mangle]
pub unsafe extern fn retro_set_controller_port_device(_port: libc::c_uint, _device: libc::c_uint)
{
	// println!("hello world: retro_set_controller_port_device");
}

#[no_mangle]
pub unsafe extern fn retro_reset()
{
	// println!("hello world: retro_reset");
}

#[no_mangle]
pub unsafe extern fn retro_serialize_size() -> size_t
{
	// println!("hello world: retro_serialize_size");
	0
}

#[no_mangle]
pub unsafe extern fn retro_serialize(_data: *mut u8, _size: size_t) -> bool
{
	// println!("hello world: retro_serialize");
	false
}


#[no_mangle]
pub unsafe extern fn retro_unserialize(_data: *mut u8, _size: size_t) -> bool
{
	// println!("hello world: retro_unserialize");
	false
}

#[no_mangle]
pub unsafe extern fn retro_cheat_reset()
{
	// println!("hello world: retro_cheat_reset");
}

#[no_mangle]
pub unsafe extern fn retro_cheat_set(_index: libc::c_uint, _enabled: bool, _code: *mut u8)
{
	// println!("hello world: retro_cheat_reset");
}

#[no_mangle]
pub unsafe extern fn retro_load_game_special(_type: libc::c_uint, _info: *mut retro_game_info, _num: size_t) -> bool
{
	// println!("hello world: retro_load_game_special");
	false
}

#[no_mangle]
pub extern fn retro_unload_game()
{
	// println!("hello world: retro_unload_game");
}

#[no_mangle]
pub extern fn retro_get_region() -> libc::c_uint
{
	// #define RETRO_REGION_NTSC  0
	0
}

#[no_mangle]
pub extern fn retro_get_memory_data(_id: libc::c_uint) -> *mut u8
{
	core::ptr::null_mut()
}

#[no_mangle]
pub extern fn retro_get_memory_size(_id: libc::c_uint) -> size_t
{
	0
}

#[no_mangle]
pub extern fn retro_load_game(_info: *mut u8) -> bool
{
	// println!("hello world: retro_load_game");
	true
}
