mod libretro;

extern crate libc;
use libc::size_t;
use libc::types::common::c95::c_void;

#[repr(C)]
pub struct retro_game_geometry
{
	base_width: libc::c_uint,
	base_height: libc::c_uint,
	max_width: libc::c_uint,
	max_height: libc::c_uint,
	aspect_ratio: f32
}

#[repr(C)]
pub struct retro_system_timing
{
	fps: f64,
	sample_rate: f64
}

#[repr(C)]
pub struct retro_system_av_info
{
	geometry: retro_game_geometry,
	timing: retro_system_timing
}

#[repr(C)]
pub struct retro_system_info
{
	library_name: *const u8, 
	library_version: *const u8,
	valid_extensions: *const u8,
	need_fullpath: bool,                                       
	block_extract: bool     
}

#[repr(C)]
pub struct retro_game_info
{
	path: *const u8,
	data: *const u8,
	size: size_t,
	meta: *const u8
}


