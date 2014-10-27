/*
	rust-libretro
        Template for generating libretro cores with rust-libretro

    Copyright (C) 2014 Mike Robinson
    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in
    all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
    THE SOFTWARE.
 */

#![crate_type = "dylib"]
#![feature(globs)]
#![feature(macro_rules)]

extern crate libc;
extern crate rlibc;
extern crate rustrt;

use std::mem::transmute;
use rust_wrapper::*;
pub mod rust_wrapper;

// Static configuration section.
// All values must be set for the core to initialize correctly.
// All strs will be converted to C strings, and any non-ASCII characters will be
// removed.

// Name and version number, for display in the frontend GUI.
static CORE_NAME: &'static str =  "Example Core";
static CORE_VERSION: &'static str = "0.0.1";

// Does the core run without the frontend loading content for it?
const NO_CONTENT: bool = true;

// List of valid extensions for content, separated by pipes. For example:
// static VALID_EXTENSIONS: &'static str = "bin|iso";
// If NO_CONTENT is true then VALID_EXTENSIONS is ignored.
static VALID_EXTENSIONS: &'static str  = "";

// Core screen size in pixels.
// Frontends provide various options for upscaling if this is lower than the
// display resolution. Note that moving objects a non-integer number of pixels
// per frame will result in poor motion quality, and because rust-libretro
// supports adjustable frame rates there is no way to guarantee integer pixel
// movement per frame. To reduce this problem, rust-libretro generates core
// options to increase the internal resolution. The framebuffer will be
// automatically resized, and if you use the built in blitting functions scaling
// will be handled automatically. If you write your own blitting code or similar
// you must take into account the INTERNAL_SCALE_X and INTERNAL_SCALE_Y static
// mut variables. Pixel art purists or those with low performance hardware may
// leave the scale at 1x, and the X and Y axis may be scaled independently so
// scanline shader or CRT users can use horizontal scaling only.
const AV_SCREEN_WIDTH: u32 = 320;
const AV_SCREEN_HEIGHT: u32 = 240;

// Pixel aspect ratio.
// This will usually be 1.0 for square pixels. rust-libretro will automatically
// adjust this as needed to compensate for internal scaling core options.
const AV_PIXEL_ASPECT: f32 = 1.0;

// Libretro is designed around fixed frame rate cores. For maximum compatibility
// with various display refresh rates, rust-libretro uses threaded rendering,
// where a snapshot of video related state is saved after a fixed number of core
// logic updates and used to render a video frame chronously.
//
// Core logic rate is one of three supported values:
// LogicRate60 (60Hz)
// LogicRate120 (120Hz)
// LogicRate720 (720Hz)
//
// rust-libretro automatically generates a core option to allow the user to
// choose a frame rate from a selection of integer divisions of the core
// logic rate. The default is always 60fps for maximum compatibility with common
// 60Hz refresh rate displays. 
//
// Please choose the highest core logic rate possible for your target hardware.
// 720Hz core logic rate has excellent compatibility with all common displays,
// as it is an integer multiple of all common refresh rates, or close enough
// that the frontend can slightly adjust the core speed and resample the audio
// for an exact match. If your core has particularly intensive CPU requirements,
// for example complicated physics simulation, you may need a lower core logic
// rate.
//
// Attempting to simulate transparency with flicker will not work correctly, as
// some frame rates will result in low frequency flashing that will look ugly and
// could pose a risk to photosensitive epileptics. Use alpha blending, or if you
// need only one layer of transparency, dithering may also be acceptable.
//
// Future versions of libretro will include support for automatic configuration
// of the frame rate, support for tuning of the video latency to trade off
// latency with performance, support for polling input at the full core logic
// rate to minimize control latency and jitter, and compatibility of input
// recordings between all frame rates. Choosing a 720Hz core logic rate will give
// you the maximum benefit from these improvements.
const CORE_LOGIC_RATE: CoreLogicRate = LogicRate720;

// Audio sampling rate, in Hertz. The frontend is responsible for resampling
// audio to a rate supported by the hardware, so unusual sampling rates will not
// cause compatibility problems. It may be convenient to use an integer multiple
// of the frame rate.
const AV_SAMPLE_RATE: f64 = 48000.0;

// Should the video format be 32 bit XRGB888?
// This can give increased image quality at the cost of performance and memory
// use. The default is 16 bit RGB565, which is recommended unless you require
// higher image quality.
const COLOR_DEPTH_32: bool = false;

// You must implement several functions that will be automatically called by
// rust-libretro.
// First is core_run(). You can poll input here with
// InputState::poll(playernum) and update the core state accordingly. All state
// change must be deterministic across all platforms, so be careful with
// threading and floating point math. See
// http://randomascii.wordpress.com/2013/07/16/floating-point-determinism/
// for advice on using floats.
pub fn core_run()
{
    // libretro v1 does not include user data pointers, so all state needs
    // to be stored in static muts. So long as you never call core_run() yourself
    // it is safe to convert them to owned data, because the libretro API
    // requires this function to be called from a single thread.
    let g = &mut unsafe {mem_as_mut_slice::<GState>(transmute(&g_state), 1)}[0];

    g.frame = g.frame + 1;
        
    let playernum = 0;
    // InputState::poll returns a struct than can be indexed with the
    // ControllerButton enum.
    let input = InputState::poll(playernum);
   
    
    if (input[PadUp].pressed) && (g.y > 0) {
        g.y = g.y - 32;
    }
    
    if (input[PadDown].pressed) && ((g.y) < ((AV_SCREEN_HEIGHT * 256) - 256)) {
        g.y = g.y + 32;
    }
    
    if (input[PadLeft].pressed) && (g.x > 0) {
        g.x = g.x - 32
    }
    
    if (input[PadRight].pressed) && ((g.x) < ((AV_SCREEN_WIDTH * 256)- 256)) {
       g.x = g.x + 32;
    }
    
}

// This function is periodically called after core_run(). It must save a snapshot
// of all state necessary for rendering video. Depending on the size and
// complexity of the state it may be faster to simply copy all the core state,
// for example using memcpy.
pub fn snapshot_video()
{
    let g = &mut unsafe {mem_as_mut_slice::<GState>(transmute(&g_state), 1)}[0];
    unsafe
    {
        snapshotx = g.x;
        snapshoty = g.y;
    }
}

static mut snapshotx: u32 = 0;
static mut snapshoty: u32 = 0;

// This function renders one frame video in a separate thread. It may only access
// the state saved in snapshot_video(). It must take into account
// INTERNAL_SCALE_X, INTERNAL_SCALE_Y and write to frame_buf. If you use the
// included blitting function this is handled for you. To gain the benefit of
// internal scaling all screen object positions must be stored at sub-pixel
// precision.
pub fn render_video()
{
    image_loader();
    unsafe {write_pixel(snapshotx / 256, snapshoty /256);}
}

// This function returns the size in bytes of the serialized core logic state
// produced by serialize_core_state(). It must not change at runtime, so be
// careful with heap allocation.
pub fn get_serialize_size() -> uint
{
    0
}

// This function saves all core logic state to a known format in a memory buffer.
// It must be possible to restore state with unserialize_core_state() on any
// platform, so serialize to a fixed endianness and take care with pointers.
// It may be simpler to avoid using pointers in the core state and use array
// indices instead. Input state should not be serialized here as rust-libretro
// serializes it automatically. Video state should not be serialized here as
// it would conflict with proposed libretro 2.0 enhancements.
pub fn serialize_core_state()
{
}

// This function restores the core logic state serialized in
// serialize_core_state().
pub fn unserialize_core_state()
{
}

pub static RAWIMAGE: &'static [u8] = include_bin!("rgb565.raw");

fn image_loader()
{
   unsafe {
   	rlibc::memcpy(transmute(frame_buf), transmute(&RAWIMAGE[0]), (AV_SCREEN_WIDTH * AV_SCREEN_HEIGHT * 2) as uint);
   }
}

struct GState
{
    frame: uint,
    x: u32,
    y: u32,
    _phase: f32
}

static mut g_state: GState =
GState
{
   frame: 0,
   x: 0,
   y: 0,
   _phase: 0.0
};

unsafe fn mem_as_mut_slice<T>(base: *mut T, length: uint) -> &'static mut [T] 
{
      transmute(std::raw::Slice {data: base as *const T, len: length})
}

fn write_pixel(x: u32, y: u32)
{
   let buf_slice = unsafe {mem_as_mut_slice(frame_buf as *mut u16, AV_SCREEN_WIDTH as uint * AV_SCREEN_HEIGHT as uint)};
   buf_slice[x as uint + y as uint * AV_SCREEN_WIDTH as uint] = 0xffff;   
}
