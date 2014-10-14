#![crate_type = "dylib"]
extern crate libc;

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
	library_name: *const i8, 
	library_version: *const i8,
	valid_extensions: *const i8,
	need_fullpath: bool,                                       
	block_extract: bool     
}

#[no_mangle]
pub extern fn retro_get_system_av_info(info: &mut retro_system_av_info)
{
	println!("Set system AV info");
	info.timing.fps = 120.0;
	info.timing.sample_rate = 44100.0;
	info.geometry.base_width   = 240;
	info.geometry.base_height  = 320;
	info.geometry.max_width    = 240;
	info.geometry.max_height   = 320;
	info.geometry.aspect_ratio = 3.0/4.0;
}

#[no_mangle]
pub unsafe extern fn retro_get_system_info(info: &mut retro_system_info)
{
	println!("Set system info");
	info.library_name     = "Hello World".to_c_str().as_ptr();
	info.library_version  = "0.0.1".to_c_str().as_ptr();
	info.valid_extensions = "".to_c_str().as_ptr();
	info.need_fullpath    = false;
	info.block_extract    = false;
}

#[no_mangle]
pub extern fn retro_api_version() -> uint
{
	println!("Set API version");
	return 1;
}


static mut retro_environment_cb: Option<extern fn (cmd: uint, data: *mut u8)->bool> = None;

#[no_mangle]
pub unsafe extern fn retro_set_environment(cb: extern fn (cmd: uint, data: *mut u8)->bool)
{
	println!("Set environment callback");
	retro_environment_cb = Some(cb);
}

static mut retro_video_refresh_cb: Option<extern fn (data: *mut u8, width: uint, height: uint, pitch: uint)> = None;

#[no_mangle]
pub unsafe extern fn retro_set_video_refresh(cb: extern fn (data: *mut u8, width: uint, height: uint, pitch: uint))
{
	println!("Set video refresh callback");
	retro_video_refresh_cb = Some(cb);
}



#[no_mangle]
pub extern fn retro_init()
{
	println!("Core init");

	unsafe
	{
		let no_content: *mut u8 = std::mem::transmute(&true);
		//#define RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME 18
		retro_environment_cb.unwrap()(18, no_content);
	}
}
#[no_mangle]
pub extern fn retro_load_game(_info: *mut u8) -> bool
{
	println!("Load game");
	true
}

#[no_mangle]
pub extern fn retro_deinit()
{
	println!("Core deinit");
}

#[no_mangle]
pub extern fn retro_run()
{
	println!("Retro run");
}


