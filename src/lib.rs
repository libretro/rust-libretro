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
#![feature(macro_rules)]

extern crate libc;
extern crate rlibc;
extern crate rustrt;

use std::mem::size_of;
use std::mem::transmute;
use libc::types::common::c95::c_void;
use libc::c_uint;
use libc::types::os::arch::c95::size_t;
use rustrt::mutex::{StaticNativeMutex, NATIVE_MUTEX_INIT};

use rust_wrapper::*;
pub mod rust_wrapper;

// Static configuration section.
// All values must be set for the core to initialize correctly.
// All strs will be converted to C strings,
// and any non-ASCII characters removed.

// Name and version number, for display in the frontend GUI.
static CORE_NAME: &'static str =  "Example Core";
static CORE_VERSION: &'static str = "0.0.1";

// Does the core run without the frontend loading content for it?
const NO_CONTENT: bool = true;

// List of valid extensions for content, separated by pipes. For example:
// static VALID_EXTENSIONS: &'static str = "bin|iso";
// If NO_CONTENT is true then VALID_EXTENSIONS is ignored.
static VALID_EXTENSIONS: &'static str  = "";

// Initial screen size in pixels.
const AV_SCREEN_WIDTH: u32 = 320;
const AV_SCREEN_HEIGHT: u32 = 240;

// Maximum screen size in pixels.
// Resizing up to this size is possible without the frontend needing to
// reinitialize the video driver.
const AV_MAX_SCREEN_WIDTH: u32 = 320;  
const AV_MAX_SCREEN_HEIGHT: u32 = 240;

// Pixel aspect ratio.
// This will usually be 1.0 for square pixels.
const AV_PIXEL_ASPECT: f32 = 1.0;

// Libretro is designed around fixed frame rate cores. To allow for maximum
// compatibility with various display refresh rates, rust-libretro uses a
// threaded rendering architecture, where a snapshot of video related state is
// saved periodically and used to render a video frame asychronously.
//
// Core logic rate is one of three supported values:
// LogicRate60 (60Hz)
// LogicRate120 (120Hz)
// LogicRate720 (720Hz)
//
// rust-libretro automatically generates a core option to allow the user to
// choose a frame rate from a selection of integer divisions of the core
// logic rate. The default is always 60Hz for maximum compatibility with common
// 60Hz refresh rate displays. 
//
// Please choose the highest core logic rate possible for your target hardware.
// 720Hz core logic rate has excellent compatibility with all common displays,
// as it is an integer multiple of all common refresh rates, or close enough
// that the frontend can slightly adjust the core speed and resample the audio
// for an exact match. If your core has particularly intensive CPU requirements,
// for example complicated physics simulation, you may require a lower core
// logic rate.
//
// Future versions of libretro will include support for automatic configuration
// of the frame rate, support for tuning of the video latency to trade off
// latency with performance, support for polling input at the full core logic
// rate to minimize control latency and jitter, and compatibility of input
// recordings between all frame rates. Chosing a 720Hz core logic rate will give
// you the maximum benefit from these improvements.
const CORE_LOGIC_RATE: CoreLogicRate = LogicRate720;

// Audio sampling rate, in Hertz. The frontend is responsible for resampling
// audio to a rate supported by the hardware, so unusual sampling rates will not
// cause compatibility problems. It may be convenient to use an integer multiple
// of the frame rate.
const AV_SAMPLE_RATE: f64 = 48000.0;

// All AV_* initialization values may be replaced at runtime, using the
// reset_system_av_info function. This requires an expensive reinitialization
// so frequent changes are not recommended.
// TODO: implement reset_system_av_info

// Should the video format be 32 bit XRGB888?
// This can give increased image quality at the cost of performance and memory
// use. The default is 16 bit RGB565, which is recommended unless higher image
// quality is required.
const COLOR_DEPTH_32: bool = false;


static mut frame_buf: *mut c_void = 0i as *mut c_void;

#[no_mangle]
pub extern fn retro_init()
{	

	// println!("hello world: retro_init");

	unsafe
	{
	frame_buf = libc::malloc(((AV_SCREEN_WIDTH as uint) * (AV_SCREEN_HEIGHT as uint)) as u64 * size_of::<u16>() as u64);
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
   	rlibc::memcpy(transmute(frame_buf), transmute(&RAWIMAGE[0]), (AV_SCREEN_WIDTH * AV_SCREEN_HEIGHT * 2) as uint);
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
      transmute(std::raw::Slice {data: base as *const T, len: length})
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

   if (down == 1) && ((g.y) < (AV_SCREEN_HEIGHT - 1))
   {
      g.y = g.y + 1;
   }

   if (left == 1) && (g.x > 0)
   {
      g.x = g.x - 1;
   }

   if (right == 1) && ((g.x) < (AV_SCREEN_WIDTH - 1))
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
		retro_video_refresh_cb.unwrap()(frame_buf as *const c_void, AV_SCREEN_WIDTH, AV_SCREEN_HEIGHT, (AV_SCREEN_WIDTH * 2) as size_t);
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
   let buf_slice = unsafe {mem_as_mut_slice(frame_buf as *mut u16, AV_SCREEN_WIDTH as uint * AV_SCREEN_HEIGHT as uint)};
   buf_slice[x as uint + y as uint * AV_SCREEN_WIDTH as uint] = 0xffff;   
}
