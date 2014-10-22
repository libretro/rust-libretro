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
//#![no_std]
#![crate_type = "dylib"]
#![feature(globs)]
#![feature(macro_rules)]
//#![feature(lang_items)]
//#![feature(asm)]
extern crate libc;
extern crate rlibc;
extern crate rustrt;
extern crate core;

use libc::types::common::c95::c_void;
use libc::c_uint;
use libc::types::os::arch::c95::size_t;

use rustrt::mutex::{StaticNativeMutex, NATIVE_MUTEX_INIT};
use rust_wrapper::*;
use rust_wrapper::libretro::retro_system_info;
use rust_wrapper::libretro::retro_system_av_info;

use core::prelude::*;
use core::intrinsics::transmute;
use core::mem::size_of;

pub mod rust_wrapper;

static NO_CONTENT: bool = true;
static _RETRO_PIXEL_FORMAT_RGB1555: u32 = 0;
static _RETRO_PIXEL_FORMAT_RGB565: u32 = 2;
static _RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME: c_uint = 18;
static _RETRO_ENVIRONMENT_SET_PIXEL_FORMAT: c_uint = 10;

static SCREEN_WIDTH: c_uint = 320;
static SCREEN_HEIGHT: c_uint = 240;
static FPS: f64 = 120.0;
static SAMPLE_RATE: f64 = 48000.0;

static ASPECT_RATIO: f32 = 1.0;

#[no_mangle]
pub unsafe extern fn retro_get_system_av_info(info: *mut retro_system_av_info)
{
	// println!("hello world: retro_get_system_av_info");
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
	// println!("hello world: retro_get_system_info");
	rlibc::memset(transmute(info), 0, size_of::<retro_system_info>());

	(*info).library_name     = "Hello World\0".as_ptr() as *const i8;  // Rust strings are not null terminated
	(*info).library_version  = "0.0.1\0".as_ptr() as *const i8;        // Null terminate manually
	(*info).valid_extensions = " \0".as_ptr() as *const i8;
	(*info).need_fullpath    = false as u8;
	(*info).block_extract    = false as u8;
}

#[no_mangle]
pub extern fn retro_api_version() -> c_uint
{
	// println!("hello world: retro_api_version");
	return 1;
}

static mut retro_environment_cb: Option<extern fn (cmd: c_uint, data: *mut u8) -> bool> = None;
#[no_mangle]
pub unsafe extern fn retro_set_environment(cb: extern fn (cmd: c_uint, data: *mut u8) -> bool)
{
	// println!("hello world: retro_set_environment");
	retro_environment_cb = Some(cb);

	let no_content: *mut u8 = transmute(&NO_CONTENT);
	retro_environment_cb.unwrap()(_RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME, no_content);

	let pixel_format: *mut u8 = transmute(&_RETRO_PIXEL_FORMAT_RGB565);
	retro_environment_cb.unwrap()(_RETRO_ENVIRONMENT_SET_PIXEL_FORMAT, pixel_format);
}

static mut frame_buf: *mut c_void = 0i as *mut c_void;

#[no_mangle]
pub extern fn retro_init()
{	

	// println!("hello world: retro_init");

	unsafe
	{
	frame_buf = libc::malloc(((SCREEN_WIDTH as uint) * (SCREEN_HEIGHT as uint)) as u64 * size_of::<u16>() as u64);
	// println!("frame_buf: {}", frame_buf);
	}

	image_loader();

	rustrt::thread::Thread::spawn(print_message);
	// println!("hello world: retro_init done");
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
		   {
            let guard = WAIT.lock();
			   guard.wait();
         }
			// println!("I am running in a different thread!");
		   if QUIT_FLAG {break};
   	}
   }
   // println!("QUIT_DONE_FLAG set");
	unsafe {
		QUIT_DONE_FLAG = true;
	}
}


pub static RAWIMAGE: &'static [u8] = include_bin!("rgb565.raw");

fn image_loader()
{
   unsafe {
   	rlibc::memcpy(transmute(frame_buf), transmute(&RAWIMAGE[0]), (SCREEN_WIDTH * SCREEN_HEIGHT * 2) as uint);
   }
}


#[no_mangle]
pub unsafe extern fn retro_deinit()
{
	// println!("hello world: retro_deinit");
	libc::free(frame_buf);

   // Rust's native concurrency library is still experimental and incomplete
	// Spinlock for now
	let mut spinlock_quit = false;
	while !spinlock_quit
	{
   	{
         // println!("Acquiring WAIT lock");
   		let guard = WAIT.lock();
         // println!("Signalling WAIT lock");
   		guard.signal();
   	}
   	{
         // println!("Acquiring QUIT lock");
   	   let _guard = QUIT.lock();
         // println!("Setting QUIT flag");
   		QUIT_FLAG = true;
   	}
	    spinlock_quit = QUIT_DONE_FLAG;
	}
	WAIT.destroy();
	QUIT.destroy();
	// println!("hello world: retro_deinit done");
}

struct GState
{
	frame: uint,
   x: u32,
   y: u32,
   phase: f32
}

static mut g_state: GState =
GState
{
   frame: 0,
   x: 0,
   y: 0,
   phase: 0.0
};

unsafe fn mem_as_mut_slice<T>(base: *mut T, length: uint) -> &'static mut [T] 
{
      transmute(core::raw::Slice {data: base as *const T, len: length})
}

#[no_mangle]
pub extern fn retro_run()
{
   // TODO how to combine this into one statement?
   let g_tmp = unsafe {mem_as_mut_slice::<GState>(transmute(&g_state), 1)};
   let g = &mut g_tmp[0];

   unsafe {retro_input_poll_cb.unwrap()();}
   
   const RETRO_DEVICE_JOYPAD:             libc::c_uint = 1;
   const RETRO_DEVICE_ID_JOYPAD_UP:       libc::c_uint = 4;
   const RETRO_DEVICE_ID_JOYPAD_DOWN:     libc::c_uint = 5;
   const RETRO_DEVICE_ID_JOYPAD_LEFT:     libc::c_uint = 6;
   const RETRO_DEVICE_ID_JOYPAD_RIGHT:    libc::c_uint = 7;
   const RETRO_DEVICE_ID_JOYPAD_A:        libc::c_uint = 8;
   const RETRO_DEVICE_ID_JOYPAD_B:        libc::c_uint = 0;
   
   let up = unsafe {retro_input_state_cb.unwrap()(0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_UP)};
   let right = unsafe {retro_input_state_cb.unwrap()(0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_RIGHT)};
   let down = unsafe {retro_input_state_cb.unwrap()(0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_DOWN)};
   let left = unsafe {retro_input_state_cb.unwrap()(0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_LEFT)};
   let a = unsafe {retro_input_state_cb.unwrap()(0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_A)};
   let _b = unsafe {retro_input_state_cb.unwrap()(0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_B)};

   // trigger an bounds check failure for testing
   // if b == 1 {let mut oob: [u8, ..1]; oob[1]=0;}

   let mut audio_buffer: [u16, ..800] = [0u16, ..800];
   
   if a == 1 { 
      render_audio(&mut audio_buffer, 10000.0, &mut g.phase);
   }

	g.frame = g.frame + 1;
   
   if (up == 1) && (g.y > 0)
   {
      g.y = g.y - 1;
   }

   if (down == 1) && ((g.y) < (SCREEN_HEIGHT - 1))
   {
      g.y = g.y + 1;
   }

   if (left == 1) && (g.x > 0)
   {
      g.x = g.x - 1;
   }

   if (right == 1) && ((g.x) < (SCREEN_WIDTH - 1))
   {
      g.x = g.x + 1;
   }

   image_loader();
   write_pixel(g.x, g.y);

	if g.frame % 60 == 0
	{
		unsafe {
			let guard = WAIT.lock();
			guard.signal();
		}
	}

   unsafe {
      retro_audio_sample_batch_cb.unwrap()(transmute(&audio_buffer), 400);
		retro_video_refresh_cb.unwrap()(frame_buf as *const c_void, SCREEN_WIDTH, SCREEN_HEIGHT, (SCREEN_WIDTH * 2) as size_t);
	}
}

fn render_audio(buffer: &mut[u16, ..800], vol: f32, phase: &mut f32)
{
   for mut i in buffer.iter_mut()
   {
         *phase = *phase + 0.001;
         *i = ((*phase - (*phase).floor()) * vol) as u16;
   }
}

fn write_pixel(x: u32, y: u32)
{
   let buf_slice = unsafe {mem_as_mut_slice(frame_buf as *mut u16, SCREEN_WIDTH as uint * SCREEN_HEIGHT as uint)};
   buf_slice[x as uint + y as uint * SCREEN_WIDTH as uint] = 0xffff;   
}
