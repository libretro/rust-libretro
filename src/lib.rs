#![crate_type = "dylib"]
extern crate libc;
extern crate rlibc;
use libc::size_t;

#[repr(C)]
pub struct retro_game_geometry
{
	base_width: uint,
	base_height: uint,
	max_width: uint,
	max_height: uint,
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


static NO_CONTENT: bool = true;
static SCREEN_WIDTH: uint = 240;
static SCREEN_HEIGHT: uint = 320;
static FPS: f64 = 120.0;
static SAMPLE_RATE: f64 = 44100.0;
static ASPECT_RATIO: f32 = 1.0;



#[no_mangle]
pub unsafe extern fn retro_get_system_av_info(info: *mut retro_system_av_info)
{
	println!("hello world: retro_get_system_av_info");
	(*info).timing.fps = FPS;
	(*info).timing.sample_rate = SAMPLE_RATE;
	(*info).geometry.base_width   = SCREEN_WIDTH;
	(*info).geometry.base_height  = SCREEN_HEIGHT;
	(*info).geometry.max_width    = SCREEN_WIDTH;
	(*info).geometry.max_height   = SCREEN_HEIGHT;
	(*info).geometry.aspect_ratio = ASPECT_RATIO;
}

#[no_mangle]
pub unsafe extern fn retro_get_system_info(info: *mut retro_system_info)
{
	println!("hello world: retro_get_system_info");
	rlibc::memset(std::mem::transmute(info), 0, std::mem::size_of::<retro_system_info>());

	(*info).library_name     = "Hello World\0".as_ptr();  // Rust strings are not null terminated
	(*info).library_version  = "0.0.1\0".as_ptr();        // Null terminate manually
	(*info).valid_extensions = "\0".as_ptr();
	(*info).need_fullpath    = false;
	(*info).block_extract    = false;
}

#[no_mangle]
pub extern fn retro_api_version() -> uint
{
	println!("hello world: retro_api_version");
	return 1;
}


static mut retro_environment_cb: Option<extern fn (cmd: uint, data: *mut u8) -> bool> = None;
#[no_mangle]
pub unsafe extern fn retro_set_environment(cb: extern fn (cmd: uint, data: *mut u8) -> bool)
{
	println!("hello world: retro_set_environment");
	retro_environment_cb = Some(cb);
	let no_content: *mut u8 = std::mem::transmute(&NO_CONTENT);
	//#define RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME 18
	retro_environment_cb.unwrap()(18, no_content);
}

static mut retro_video_refresh_cb: Option<extern fn (data: *mut libc::types::common::c95::c_void, width: uint, height: uint, pitch: uint)> = None;
#[no_mangle]
pub unsafe extern fn retro_set_video_refresh(cb: extern fn (data: *mut libc::types::common::c95::c_void, width: uint, height: uint, pitch: uint))
{
	println!("hello world: retro_set_video_refresh");
	retro_video_refresh_cb = Some(cb);
}

static mut retro_audio_sample_cb: Option<extern fn (left: i16, right: i16)> = None;
#[no_mangle]
pub unsafe extern fn retro_set_audio_sample(cb: extern fn (left: i16, right: i16))
{
	println!("hello world: retro_set_audio_sample");
	retro_audio_sample_cb = Some(cb);
}

static mut retro_audio_sample_batch_cb: Option<extern fn(data: *mut i16, frames: size_t) -> size_t> = None;
#[no_mangle]
pub unsafe extern fn retro_set_audio_sample_batch(cb: extern fn(data: *mut i16, frames: size_t) -> size_t)
{
	println!("hello world: retro_set_audio_sample_batch");
	retro_audio_sample_batch_cb = Some(cb);
}

static mut retro_input_poll_cb: Option<extern fn()> = None;
#[no_mangle]
pub unsafe extern fn retro_set_input_poll(cb: extern fn())
{
	println!("hello world: retro_set_input_poll");
	retro_input_poll_cb = Some(cb);
}


static mut retro_input_state_cb: Option<extern fn(port: uint, device: uint, index: uint, id: uint) -> i16> = None;
#[no_mangle]
pub unsafe extern fn retro_set_input_state(cb: extern fn(port: uint, device: uint, index: uint, id: uint) -> i16)
{
	println!("hello world: retro_set_input_state");
	retro_input_state_cb = Some(cb);
}

#[no_mangle]
pub unsafe extern fn retro_set_controller_port_device(_port: uint, _device: uint)
{
	println!("hello world: retro_set_controller_port_device");
}

#[no_mangle]
pub unsafe extern fn retro_reset()
{
	println!("hello world: retro_reset");
}

#[no_mangle]
pub unsafe extern fn retro_serialize_size() -> size_t
{
	println!("hello world: retro_serialize_size");
	0
}

#[no_mangle]
pub unsafe extern fn retro_serialize(_data: *mut u8, _size: size_t) -> bool
{
	println!("hello world: retro_serialize");
	false
}


#[no_mangle]
pub unsafe extern fn retro_unserialize(_data: *mut u8, _size: size_t) -> bool
{
	println!("hello world: retro_unserialize");
	false
}

#[no_mangle]
pub unsafe extern fn retro_cheat_reset()
{
	println!("hello world: retro_cheat_reset");
}

#[no_mangle]
pub unsafe extern fn retro_cheat_set(_index: uint, _enabled: bool, _code: *mut u8)
{
	println!("hello world: retro_cheat_reset");
}

#[no_mangle]
pub unsafe extern fn retro_load_game_special(_type: uint, _info: *mut retro_game_info, _num: size_t) -> bool
{
	println!("hello world: retro_load_game_special");
	false
}

#[no_mangle]
pub extern fn retro_unload_game()
{
	println!("hello world: retro_unload_game");
}

#[no_mangle]
pub extern fn retro_get_region() -> uint
{
	// #define RETRO_REGION_NTSC  0
	0
}

#[no_mangle]
pub extern fn retro_get_memory_data(_id: uint) -> *mut u8
{
	std::ptr::null_mut()
}

#[no_mangle]
pub extern fn retro_get_memory_size(_id: uint) -> size_t
{
	0
}



static mut frame_buf: Option<*mut libc::types::common::c95::c_void> = None;

#[no_mangle]
pub extern fn retro_init()
{
	unsafe
	{
	frame_buf = Some(libc::malloc((std::mem::size_of::<u16>() * SCREEN_WIDTH * SCREEN_HEIGHT) as u64));
	}
	println!("hello world: retro_init");
}

#[no_mangle]
pub extern fn retro_load_game(_info: *mut u8) -> bool
{
	println!("hello world: retro_load_game");
	true
}

#[no_mangle]
pub extern fn retro_deinit()
{
	println!("hello world: retro_deinit");
}

#[no_mangle]
pub extern fn retro_run()
{
	println!("hello world: retro_run");
	unsafe
	{
		retro_video_refresh_cb.unwrap()(frame_buf.unwrap(), SCREEN_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH);
	}
}


