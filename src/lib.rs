/*
	hellorust-libretro
	minimal proof of concept libretro core
    Copyright (C) 2014 Mike Robinson

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

#![crate_type = "dylib"]
#![feature(globs)]
extern crate libc;
extern crate image;
extern crate rlibc;
extern crate rustrt;

use image::GenericImage;
use libc::types::common::c95::c_void;
use libc::c_uint;
use std::mem::transmute;
use std::rt::mutex::{StaticNativeMutex, NATIVE_MUTEX_INIT};
use libretro::*;

pub mod libretro;


static NO_CONTENT: bool = true;
static _RETRO_PIXEL_FORMAT_RGB1555: u32 = 0;
static _RETRO_PIXEL_FORMAT_RGB565: u32 = 2;
static _RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME: c_uint = 18;
static _RETRO_ENVIRONMENT_SET_PIXEL_FORMAT: c_uint = 10;

static SCREEN_WIDTH: c_uint = 320;
static SCREEN_HEIGHT: c_uint = 240;
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
pub extern fn retro_api_version() -> c_uint
{
	println!("hello world: retro_api_version");
	return 1;
}

static mut retro_environment_cb: Option<extern fn (cmd: c_uint, data: *mut u8) -> bool> = None;
#[no_mangle]
pub unsafe extern fn retro_set_environment(cb: extern fn (cmd: c_uint, data: *mut u8) -> bool)
{
	println!("hello world: retro_set_environment");
	retro_environment_cb = Some(cb);

	let no_content: *mut u8 = std::mem::transmute(&NO_CONTENT);
	retro_environment_cb.unwrap()(_RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME, no_content);

	let pixel_format: *mut u8 = std::mem::transmute(&_RETRO_PIXEL_FORMAT_RGB565);
	retro_environment_cb.unwrap()(_RETRO_ENVIRONMENT_SET_PIXEL_FORMAT, pixel_format);
}

static mut frame_buf: *mut c_void = 0i as *mut c_void;

#[no_mangle]
pub extern fn retro_init()
{	

	println!("hello world: retro_init");

	unsafe
	{
	frame_buf = libc::malloc(((SCREEN_WIDTH as uint) * (SCREEN_HEIGHT as uint)) as u64 * std::mem::size_of::<u16>() as u64);
	}


	image_loader();

	rustrt::thread::Thread::spawn(print_message);
	println!("hello world: retro_init done");
}



static WAIT: StaticNativeMutex = NATIVE_MUTEX_INIT;
static QUIT: StaticNativeMutex = NATIVE_MUTEX_INIT;
static mut QUIT_FLAG: bool = false;
static mut QUIT_DONE_FLAG: bool = false;


fn print_message()
{
	loop
	{
		unsafe {
		    let guard = WAIT.lock();
			guard.wait();
		}
		println!("I am running in a different thread!");
		unsafe {
		    let _guard = QUIT.lock();
			if QUIT_FLAG {break};
		}
	}
	unsafe {
		QUIT_DONE_FLAG = true;
	}
}


pub static HELLOPNG: &'static [u8] = include_bin!("test.png");

fn image_loader()
{
	let mut owned_buf = unsafe {std::c_vec::CVec::<u16>::new(frame_buf as *mut u16, SCREEN_WIDTH as uint * SCREEN_HEIGHT as uint)};

	let img = image::load_from_memory(HELLOPNG, image::PNG);
	match img
	{
		Err(e) => { println!("error opening image: {}", e); }
		Ok(someimg) => {
			for ((_, _, p), buf) in someimg.pixels().zip(owned_buf.as_mut_slice().iter_mut())
			{
				let (r, g, b, _) = p.channels();
				*buf = ((r as u16 >> 3) << 11) | ((g as u16 >> 2) << 5) | (b as u16 >> 3);
			}
		}
	}
}


#[no_mangle]
pub unsafe extern fn retro_deinit()
{
	println!("hello world: retro_deinit");
	libc::free(frame_buf);

	{
		let guard = WAIT.lock();
		guard.signal();
	}
	{
	    let _guard = QUIT.lock();
		QUIT_FLAG = true;
	}

	// Rust's native concurrency library is still experimental and incomplete
	// Spinlock for now
	let mut spinlock_quit = false;
	while !spinlock_quit
	{
	    spinlock_quit = QUIT_DONE_FLAG;
	}
	WAIT.destroy();
	QUIT.destroy();
}

struct GState
{
	frame: uint
}


static mut g_state: GState = GState{frame: 0};

#[no_mangle]
pub extern fn retro_run()
{
	let mut state = unsafe {std::c_vec::CVec::<GState>::new(transmute(&g_state), 1)};
	
	let g = &mut state.as_mut_slice()[0];

	g.frame = g.frame + 1;	

	if g.frame % 60 == 0
	{
		unsafe {
			let guard = WAIT.lock();
			guard.signal();
		}
	}

	unsafe
	{
		retro_input_poll_cb.unwrap()();
		retro_video_refresh_cb.unwrap()(frame_buf, SCREEN_WIDTH, SCREEN_HEIGHT, 0);
	}
}


