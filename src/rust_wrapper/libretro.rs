/* Copyright (C) 2010-2014 The RetroArch team
 *
 * ---------------------------------------------------------------------------------------
 * The following license statement only applies to this libretro API header (libretro.rs).
 * ---------------------------------------------------------------------------------------
 *
 * Permission is hereby granted, free of charge,
 * to any person obtaining a copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software,
 * and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
 * INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 * IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
 * WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

extern crate libc;
use libc::types::common::c95::c_void;
use libc::types::os::arch::c95::size_t;
use libc::types::os::arch::c95::c_char;
use libc::c_uint;
use libc::c_int;

/* Used for checking API/ABI mismatches that can break libretro 
 * implementations.
 * It is not incremented for compatible changes to the API.
 */
pub const RETRO_API_VERSION:         c_uint = 1;

/*
 * Libretro's fundamental device abstractions.
 *
 * Libretro's input system consists of some standardized device types,
 * such as a joypad (with/without analog), mouse, keyboard, lightgun 
 * and a pointer.
 *
 * The functionality of these devices are fixed, and individual cores 
 * map their own concept of a controller to libretro's abstractions.
 * This makes it possible for frontends to map the abstract types to a 
 * real input device, and not having to worry about binding input 
 * correctly to arbitrary controller layouts.
 */

pub const RETRO_DEVICE_TYPE_SHIFT:         c_uint = 8;
pub const RETRO_DEVICE_MASK:               c_uint = ((1 << RETRO_DEVICE_TYPE_SHIFT as uint) - 1);
macro_rules! RETRO_DEVICE_SUBCLASS(
   ($base:expr, $id:expr) => (
      (((id + 1) << RETRO_DEVICE_TYPE_SHIFT) | base)
   );
)

/* Input disabled. */
pub const RETRO_DEVICE_NONE:         c_uint = 0;

/* The JOYPAD is called RetroPad. It is essentially a Super Nintendo 
 * controller, but with additional L2/R2/L3/R3 buttons, similar to a 
 * PS1 DualShock. */
pub const RETRO_DEVICE_JOYPAD:       c_uint = 1;

/* The mouse is a simple mouse, similar to Super Nintendo's mouse.
 * X and Y coordinates are reported relatively to last poll (poll callback).
 * It is up to the libretro implementation to keep track of where the mouse 
 * pointer is supposed to be on the screen.
 * The frontend must make sure not to interfere with its own hardware 
 * mouse pointer.
 */
pub const RETRO_DEVICE_MOUSE:        c_uint = 2;

/* KEYBOARD device lets one poll for raw key pressed.
 * It is poll based, so input callback will return with the current 
 * pressed state.
 * For event/text based keyboard input, see
 * RETRO_ENVIRONMENT_SET_KEYBOARD_CALLBACK.
 */
pub const RETRO_DEVICE_KEYBOARD:     c_uint = 3;

/* Lightgun X/Y coordinates are reported relatively to last poll,
 * similar to mouse. */
pub const RETRO_DEVICE_LIGHTGUN:     c_uint = 4;

/* The ANALOG device is an extension to JOYPAD (RetroPad).
 * Similar to DualShock it adds two analog sticks.
 * This is treated as a separate device type as it returns values in the 
 * full analog range of [-0x8000, 0x7fff]. Positive X axis is right.
 * Positive Y axis is down.
 * Only use ANALOG type when polling for analog values of the axes.
 */
pub const RETRO_DEVICE_ANALOG:       c_uint = 5;

/* Abstracts the concept of a pointing mechanism, e.g. touch.
 * This allows libretro to query in absolute coordinates where on the 
 * screen a mouse (or something similar) is being placed.
 * For a touch centric device, coordinates reported are the coordinates
 * of the press.
 *
 * Coordinates in X and Y are reported as:
 * [-0x7fff, 0x7fff]: -0x7fff corresponds to the far left/top of the screen,
 * and 0x7fff corresponds to the far right/bottom of the screen.
 * The "screen" is here defined as area that is passed to the frontend and 
 * later displayed on the monitor.
 *
 * The frontend is free to scale/resize this screen as it sees fit, however,
 * (X, Y) = (-0x7fff, -0x7fff) will correspond to the top-left pixel of the 
 * game image, etc.
 *
 * To check if the pointer coordinates are valid (e.g. a touch display 
 * actually being touched), PRESSED returns 1 or 0.
 *
 * If using a mouse on a desktop, PRESSED will usually correspond to the 
 * left mouse button, but this is a frontend decision.
 * PRESSED will only return 1 if the pointer is inside the game screen.
 *
 * For multi-touch, the index variable can be used to successively query 
 * more presses.
 * If index = 0 returns true for _PRESSED, coordinates can be extracted
 * with _X, _Y for index = 0. One can then query _PRESSED, _X, _Y with 
 * index = 1, and so on.
 * Eventually _PRESSED will return false for an index. No further presses 
 * are registered at this point. */
pub const RETRO_DEVICE_POINTER:      c_uint = 6;

/* Buttons for the RetroPad (JOYPAD).
 * The placement of these is equivalent to placements on the 
 * Super Nintendo controller.
 * L2/R2/L3/R3 buttons correspond to the PS1 DualShock. */
pub const RETRO_DEVICE_ID_JOYPAD_B:         c_uint = 0;
pub const RETRO_DEVICE_ID_JOYPAD_Y:         c_uint = 1;
pub const RETRO_DEVICE_ID_JOYPAD_SELECT:    c_uint = 2;
pub const RETRO_DEVICE_ID_JOYPAD_START:     c_uint = 3;
pub const RETRO_DEVICE_ID_JOYPAD_UP:        c_uint = 4;
pub const RETRO_DEVICE_ID_JOYPAD_DOWN:      c_uint = 5;
pub const RETRO_DEVICE_ID_JOYPAD_LEFT:      c_uint = 6;
pub const RETRO_DEVICE_ID_JOYPAD_RIGHT:     c_uint = 7;
pub const RETRO_DEVICE_ID_JOYPAD_A:         c_uint = 8;
pub const RETRO_DEVICE_ID_JOYPAD_X:         c_uint = 9;
pub const RETRO_DEVICE_ID_JOYPAD_L:        c_uint = 10;
pub const RETRO_DEVICE_ID_JOYPAD_R:        c_uint = 11;
pub const RETRO_DEVICE_ID_JOYPAD_L2:       c_uint = 12;
pub const RETRO_DEVICE_ID_JOYPAD_R2:       c_uint = 13;
pub const RETRO_DEVICE_ID_JOYPAD_L3:       c_uint = 14;
pub const RETRO_DEVICE_ID_JOYPAD_R3:       c_uint = 15;

/* Index / Id values for ANALOG device. */
pub const RETRO_DEVICE_INDEX_ANALOG_LEFT:   c_uint = 0;
pub const RETRO_DEVICE_INDEX_ANALOG_RIGHT:  c_uint = 1;
pub const RETRO_DEVICE_ID_ANALOG_X:         c_uint = 0;
pub const RETRO_DEVICE_ID_ANALOG_Y:         c_uint = 1;

/* Id values for MOUSE. */
pub const RETRO_DEVICE_ID_MOUSE_X:          c_uint = 0;
pub const RETRO_DEVICE_ID_MOUSE_Y:          c_uint = 1;
pub const RETRO_DEVICE_ID_MOUSE_LEFT:       c_uint = 2;
pub const RETRO_DEVICE_ID_MOUSE_RIGHT:      c_uint = 3;
pub const RETRO_DEVICE_ID_MOUSE_WHEELUP:    c_uint = 4;
pub const RETRO_DEVICE_ID_MOUSE_WHEELDOWN:  c_uint = 5;
pub const RETRO_DEVICE_ID_MOUSE_MIDDLE:     c_uint = 6;

/* Id values for LIGHTGUN types. */
pub const RETRO_DEVICE_ID_LIGHTGUN_X:        c_uint = 0;
pub const RETRO_DEVICE_ID_LIGHTGUN_Y:        c_uint = 1;
pub const RETRO_DEVICE_ID_LIGHTGUN_TRIGGER:  c_uint = 2;
pub const RETRO_DEVICE_ID_LIGHTGUN_CURSOR:   c_uint = 3;
pub const RETRO_DEVICE_ID_LIGHTGUN_TURBO:    c_uint = 4;
pub const RETRO_DEVICE_ID_LIGHTGUN_PAUSE:    c_uint = 5;
pub const RETRO_DEVICE_ID_LIGHTGUN_START:    c_uint = 6;

/* Id values for POINTER. */
pub const RETRO_DEVICE_ID_POINTER_X:         c_uint = 0;
pub const RETRO_DEVICE_ID_POINTER_Y:         c_uint = 1;
pub const RETRO_DEVICE_ID_POINTER_PRESSED:   c_uint = 2;

/* Returned from retro_get_region(). */
pub const RETRO_REGION_NTSC:  c_uint = 0;
pub const RETRO_REGION_PAL:   c_uint = 1;

/* Rust enums are tagged unions and are not compatible with
 * C enums. All enums are converted to lists of constants. */
// Id values for LANGUAGE
// enum values from retro_language:
pub const RETRO_LANGUAGE_ENGLISH:                c_int = 0;
pub const RETRO_LANGUAGE_JAPANESE:               c_int = 1;
pub const RETRO_LANGUAGE_FRENCH:                 c_int = 2;
pub const RETRO_LANGUAGE_SPANISH:                c_int = 3;
pub const RETRO_LANGUAGE_GERMAN:                 c_int = 4;
pub const RETRO_LANGUAGE_ITALIAN:                c_int = 5;
pub const RETRO_LANGUAGE_DUTCH:                  c_int = 6;
pub const RETRO_LANGUAGE_PORTUGUESE:             c_int = 7;
pub const RETRO_LANGUAGE_RUSSIAN:                c_int = 8;
pub const RETRO_LANGUAGE_KOREAN:                 c_int = 9;
pub const RETRO_LANGUAGE_CHINESE_TRADITIONAL:   c_int = 10;
pub const RETRO_LANGUAGE_CHINESE_SIMPLIFIED:    c_int = 11;
pub const RETRO_LANGUAGE_LAST:                  c_int = 12;

/* Passed to retro_get_memory_data/size().
 * If the memory type doesn't apply to the 
 * implementation NULL/0 can be returned.
 */
pub const RETRO_MEMORY_MASK:        c_uint = 0xff;

/* Regular save RAM. This RAM is usually found on a game cartridge,
 * backed up by a battery.
 * If save game data is too complex for a single memory buffer,
 * the SAVE_DIRECTORY (preferably) or SYSTEM_DIRECTORY environment
 * callback can be used. */
pub const RETRO_MEMORY_SAVE_RAM:    c_uint = 0;

/* Some games have a built-in clock to keep track of time.
 * This memory is usually just a couple of bytes to keep track of time.
 */
pub const RETRO_MEMORY_RTC:         c_uint = 1;

/* System ram lets a frontend peek into a game systems main RAM. */
pub const RETRO_MEMORY_SYSTEM_RAM:  c_uint = 2;

/* Video ram lets a frontend peek into a game systems video RAM (VRAM). */
pub const RETRO_MEMORY_VIDEO_RAM:   c_uint = 3;

/* Keysyms used for ID in input state callback when polling RETRO_KEYBOARD. */
// enum values from retro_key:
pub const RETROK_UNKNOWN:        c_int = 0;
pub const RETROK_FIRST:          c_int = 0;
pub const RETROK_BACKSPACE:      c_int = 8;
pub const RETROK_TAB:            c_int = 9;
pub const RETROK_CLEAR:          c_int = 12;
pub const RETROK_RETURN:         c_int = 13;
pub const RETROK_PAUSE:          c_int = 19;
pub const RETROK_ESCAPE:         c_int = 27;
pub const RETROK_SPACE:          c_int = 32;
pub const RETROK_EXCLAIM:        c_int = 33;
pub const RETROK_QUOTEDBL:       c_int = 34;
pub const RETROK_HASH:           c_int = 35;
pub const RETROK_DOLLAR:         c_int = 36;
pub const RETROK_AMPERSAND:      c_int = 38;
pub const RETROK_QUOTE:          c_int = 39;
pub const RETROK_LEFTPAREN:      c_int = 40;
pub const RETROK_RIGHTPAREN:     c_int = 41;
pub const RETROK_ASTERISK:       c_int = 42;
pub const RETROK_PLUS:           c_int = 43;
pub const RETROK_COMMA:          c_int = 44;
pub const RETROK_MINUS:          c_int = 45;
pub const RETROK_PERIOD:         c_int = 46;
pub const RETROK_SLASH:          c_int = 47;
pub const RETROK_0:              c_int = 48;
pub const RETROK_1:              c_int = 49;
pub const RETROK_2:              c_int = 50;
pub const RETROK_3:              c_int = 51;
pub const RETROK_4:              c_int = 52;
pub const RETROK_5:              c_int = 53;
pub const RETROK_6:              c_int = 54;
pub const RETROK_7:              c_int = 55;
pub const RETROK_8:              c_int = 56;
pub const RETROK_9:              c_int = 57;
pub const RETROK_COLON:          c_int = 58;
pub const RETROK_SEMICOLON:      c_int = 59;
pub const RETROK_LESS:           c_int = 60;
pub const RETROK_EQUALS:         c_int = 61;
pub const RETROK_GREATER:        c_int = 62;
pub const RETROK_QUESTION:       c_int = 63;
pub const RETROK_AT:             c_int = 64;
pub const RETROK_LEFTBRACKET:    c_int = 91;
pub const RETROK_BACKSLASH:      c_int = 92;
pub const RETROK_RIGHTBRACKET:   c_int = 93;
pub const RETROK_CARET:          c_int = 94;
pub const RETROK_UNDERSCORE:     c_int = 95;
pub const RETROK_BACKQUOTE:      c_int = 96;
#[allow(non_upper_case_globals)]
pub const RETROK_a:              c_int = 97;
#[allow(non_upper_case_globals)]
pub const RETROK_b:              c_int = 98;
#[allow(non_upper_case_globals)]
pub const RETROK_c:              c_int = 99;
#[allow(non_upper_case_globals)]
pub const RETROK_d:              c_int = 100;
#[allow(non_upper_case_globals)]
pub const RETROK_e:              c_int = 101;
#[allow(non_upper_case_globals)]
pub const RETROK_f:              c_int = 102;
#[allow(non_upper_case_globals)]
pub const RETROK_g:              c_int = 103;
#[allow(non_upper_case_globals)]
pub const RETROK_h:              c_int = 104;
#[allow(non_upper_case_globals)]
pub const RETROK_i:              c_int = 105;
#[allow(non_upper_case_globals)]
pub const RETROK_j:              c_int = 106;
#[allow(non_upper_case_globals)]
pub const RETROK_k:              c_int = 107;
#[allow(non_upper_case_globals)]
pub const RETROK_l:              c_int = 108;
#[allow(non_upper_case_globals)]
pub const RETROK_m:              c_int = 109;
#[allow(non_upper_case_globals)]
pub const RETROK_n:              c_int = 110;
#[allow(non_upper_case_globals)]
pub const RETROK_o:              c_int = 111;
#[allow(non_upper_case_globals)]
pub const RETROK_p:              c_int = 112;
#[allow(non_upper_case_globals)]
pub const RETROK_q:              c_int = 113;
#[allow(non_upper_case_globals)]
pub const RETROK_r:              c_int = 114;
#[allow(non_upper_case_globals)]
pub const RETROK_s:              c_int = 115;
#[allow(non_upper_case_globals)]
pub const RETROK_t:              c_int = 116;
#[allow(non_upper_case_globals)]
pub const RETROK_u:              c_int = 117;
#[allow(non_upper_case_globals)]
pub const RETROK_v:              c_int = 118;
#[allow(non_upper_case_globals)]
pub const RETROK_w:              c_int = 119;
#[allow(non_upper_case_globals)]
pub const RETROK_x:              c_int = 120;
#[allow(non_upper_case_globals)]
pub const RETROK_y:              c_int = 121;
#[allow(non_upper_case_globals)]
pub const RETROK_z:              c_int = 122;
pub const RETROK_DELETE:         c_int = 127;

pub const RETROK_KP0:            c_int = 256;
pub const RETROK_KP1:            c_int = 257;
pub const RETROK_KP2:            c_int = 258;
pub const RETROK_KP3:            c_int = 259;
pub const RETROK_KP4:            c_int = 260;
pub const RETROK_KP5:            c_int = 261;
pub const RETROK_KP6:            c_int = 262;
pub const RETROK_KP7:            c_int = 263;
pub const RETROK_KP8:            c_int = 264;
pub const RETROK_KP9:            c_int = 265;
pub const RETROK_KP_PERIOD:      c_int = 266;
pub const RETROK_KP_DIVIDE:      c_int = 267;
pub const RETROK_KP_MULTIPLY:    c_int = 268;
pub const RETROK_KP_MINUS:       c_int = 269;
pub const RETROK_KP_PLUS:        c_int = 270;
pub const RETROK_KP_ENTER:       c_int = 271;
pub const RETROK_KP_EQUALS:      c_int = 272;

pub const RETROK_UP:             c_int = 273;
pub const RETROK_DOWN:           c_int = 274;
pub const RETROK_RIGHT:          c_int = 275;
pub const RETROK_LEFT:           c_int = 276;
pub const RETROK_INSERT:         c_int = 277;
pub const RETROK_HOME:           c_int = 278;
pub const RETROK_END:            c_int = 279;
pub const RETROK_PAGEUP:         c_int = 280;
pub const RETROK_PAGEDOWN:       c_int = 281;

pub const RETROK_F1:             c_int = 282;
pub const RETROK_F2:             c_int = 283;
pub const RETROK_F3:             c_int = 284;
pub const RETROK_F4:             c_int = 285;
pub const RETROK_F5:             c_int = 286;
pub const RETROK_F6:             c_int = 287;
pub const RETROK_F7:             c_int = 288;
pub const RETROK_F8:             c_int = 289;
pub const RETROK_F9:             c_int = 290;
pub const RETROK_F10:            c_int = 291;
pub const RETROK_F11:            c_int = 292;
pub const RETROK_F12:            c_int = 293;
pub const RETROK_F13:            c_int = 294;
pub const RETROK_F14:            c_int = 295;
pub const RETROK_F15:            c_int = 296;

pub const RETROK_NUMLOCK:        c_int = 300;
pub const RETROK_CAPSLOCK:       c_int = 301;
pub const RETROK_SCROLLOCK:      c_int = 302;
pub const RETROK_RSHIFT:         c_int = 303;
pub const RETROK_LSHIFT:         c_int = 304;
pub const RETROK_RCTRL:          c_int = 305;
pub const RETROK_LCTRL:          c_int = 306;
pub const RETROK_RALT:           c_int = 307;
pub const RETROK_LALT:           c_int = 308;
pub const RETROK_RMETA:          c_int = 309;
pub const RETROK_LMETA:          c_int = 310;
pub const RETROK_LSUPER:         c_int = 311;
pub const RETROK_RSUPER:         c_int = 312;
pub const RETROK_MODE:           c_int = 313;
pub const RETROK_COMPOSE:        c_int = 314;

pub const RETROK_HELP:           c_int = 315;
pub const RETROK_PRINT:          c_int = 316;
pub const RETROK_SYSREQ:         c_int = 317;
pub const RETROK_BREAK:          c_int = 318;
pub const RETROK_MENU:           c_int = 319;
pub const RETROK_POWER:          c_int = 320;
pub const RETROK_EURO:           c_int = 321;
pub const RETROK_UNDO:           c_int = 322;

pub const RETROK_LAST:           c_int = 323;

// enum values from retro_mod:
pub const RETROKMOD_NONE:       c_int = 0x0000;

pub const RETROKMOD_SHIFT:      c_int = 0x01;
pub const RETROKMOD_CTRL:       c_int = 0x02;
pub const RETROKMOD_ALT:        c_int = 0x04;
pub const RETROKMOD_META:       c_int = 0x08;

pub const RETROKMOD_NUMLOCK:    c_int = 0x10;
pub const RETROKMOD_CAPSLOCK:   c_int = 0x20;
pub const RETROKMOD_SCROLLOCK:  c_int = 0x40;

/* If set, this call is not part of the public libretro API yet. It can 
 * change or be removed at any time. */
pub const RETRO_ENVIRONMENT_EXPERIMENTAL: c_uint = 0x10000;
/* Environment callback to be used internally in frontend. */
pub const RETRO_ENVIRONMENT_PRIVATE: c_uint = 0x20000;

/* Environment commands. */
pub const RETRO_ENVIRONMENT_SET_ROTATION:  c_uint = 1;
                                           /* *const c_uint --
                                            * Sets screen rotation of graphics.
                                            * Is only implemented if rotation can be accelerated by hardware.
                                            * Valid values are 0, 1, 2, 3, which rotates screen by 0, 90, 180, 
                                            * 270 degrees counter-clockwise respectively.
                                            */
pub const RETRO_ENVIRONMENT_GET_OVERSCAN:  c_uint = 2;
                                           /* *mut u8 --
                                            * Boolean value whether or not the implementation should use overscan, 
                                            * or crop away overscan.
                                            */
pub const RETRO_ENVIRONMENT_GET_CAN_DUPE:  c_uint = 3;
                                           /* *mut u8 --
                                            * Boolean value whether or not frontend supports frame duping,
                                            * passing NULL to video frame callback.
                                            */

                                           /* Environ 4, 5 are no longer supported (GET_VARIABLE / SET_VARIABLES), 
                                            * and reserved to avoid possible ABI clash.
                                            */

pub const RETRO_ENVIRONMENT_SET_MESSAGE:   c_uint = 6;
                                           /* *const retro_message --
                                            * Sets a message to be displayed in implementation-specific manner 
                                            * for a certain amount of 'frames'.
                                            * Should not be used for trivial messages, which should simply be 
                                            * logged via RETRO_ENVIRONMENT_GET_LOG_INTERFACE (or as a 
                                            * fallback, stderr).
                                            */
pub const RETRO_ENVIRONMENT_SHUTDOWN:      c_uint = 7;
                                           /* N/A (NULL) --
                                            * Requests the frontend to shutdown.
                                            * Should only be used if game has a specific
                                            * way to shutdown the game from a menu item or similar.
                                            */
pub const RETRO_ENVIRONMENT_SET_PERFORMANCE_LEVEL: c_uint = 8;
                                           /* *const c_uint --
                                            * Gives a hint to the frontend how demanding this implementation
                                            * is on a system. E.g. reporting a level of 2 means
                                            * this implementation should run decently on all frontends
                                            * of level 2 and up.
                                            *
                                            * It can be used by the frontend to potentially warn
                                            * about too demanding implementations.
                                            *
                                            * The levels are "floating".
                                            *
                                            * This function can be called on a per-game basis,
                                            * as certain games an implementation can play might be
                                            * particularly demanding.
                                            * If called, it should be called in retro_load_game().
                                            */
pub const RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY: c_uint = 9;
                                           /* *const *mut c_char --
                                            * Returns the "system" directory of the frontend.
                                            * This directory can be used to store system specific 
                                            * content such as BIOSes, configuration data, etc.
                                            * The returned value can be NULL.
                                            * If so, no such directory is defined,
                                            * and it's up to the implementation to find a suitable directory.
                                            *
                                            * NOTE: Some cores used this folder also for "save" data such as 
                                            * memory cards, etc, for lack of a better place to put it.
                                            * This is now discouraged, and if possible, cores should try to 
                                            * use the new GET_SAVE_DIRECTORY.
                                            */
pub const RETRO_ENVIRONMENT_SET_PIXEL_FORMAT: c_uint = 10;
                                           /* *const c_int (enum retro_pixel_format) --
                                            * Sets the internal pixel format used by the implementation.
                                            * The default pixel format is RETRO_PIXEL_FORMAT_0RGB1555.
                                            * This pixel format however, is deprecated (see enum retro_pixel_format).
                                            * If the call returns false, the frontend does not support this pixel 
                                            * format.
                                            *
                                            * This function should be called inside retro_load_game() or 
                                            * retro_get_system_av_info().
                                            */
pub const RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS: c_uint = 11;
                                           /* *const retro_input_descriptor --
                                            * Sets an array of retro_input_descriptors.
                                            * It is up to the frontend to present this in a usable way.
                                            * The array is terminated by retro_input_descriptor::description 
                                            * being set to NULL.
                                            * This function can be called at any time, but it is recommended 
                                            * to call it as early as possible.
                                            */
pub const RETRO_ENVIRONMENT_SET_KEYBOARD_CALLBACK: c_uint = 12;
                                           /* *const retro_keyboard_callback --
                                            * Sets a callback function used to notify core about keyboard events.
                                            */
pub const RETRO_ENVIRONMENT_SET_DISK_CONTROL_INTERFACE: c_uint = 13;
                                           /* *const retro_disk_control_callback --
                                            * Sets an interface which frontend can use to eject and insert 
                                            * disk images.
                                            * This is used for games which consist of multiple images and 
                                            * must be manually swapped out by the user (e.g. PSX).
                                            */
pub const RETRO_ENVIRONMENT_SET_HW_RENDER: c_uint = 14;
                                           /* *mut retro_hw_render_callback --
                                            * Sets an interface to let a libretro core render with 
                                            * hardware acceleration.
                                            * Should be called in retro_load_game().
                                            * If successful, libretro cores will be able to render to a 
                                            * frontend-provided framebuffer.
                                            * The size of this framebuffer will be at least as large as 
                                            * max_width/max_height provided in get_av_info().
                                            * If HW rendering is used, pass only RETRO_HW_FRAME_BUFFER_VALID or 
                                            * NULL to retro_video_refresh_t.
                                            */
pub const RETRO_ENVIRONMENT_GET_VARIABLE: c_uint = 15;
                                           /* *mut retro_variable --
                                            * Interface to acquire user-defined information from environment
                                            * that cannot feasibly be supported in a multi-system way.
                                            * 'key' should be set to a key which has already been set by 
                                            * SET_VARIABLES.
                                            * 'data' will be set to a value or NULL.
                                            */
pub const RETRO_ENVIRONMENT_SET_VARIABLES: c_uint = 16;
                                           /* *const retro_variable --
                                            * Allows an implementation to signal the environment
                                            * which variables it might want to check for later using 
                                            * GET_VARIABLE.
                                            * This allows the frontend to present these variables to 
                                            * a user dynamically.
                                            * This should be called as early as possible (ideally in 
                                            * retro_set_environment).
                                            *
                                            * 'data' points to an array of retro_variable structs 
                                            * terminated by a { NULL, NULL } element.
                                            * retro_variable::key should be namespaced to not collide 
                                            * with other implementations' keys. E.g. A core called 
                                            * 'foo' should use keys named as 'foo_option'.
                                            * retro_variable::value should contain a human readable 
                                            * description of the key as well as a '|' delimited list 
                                            * of expected values.
                                            *
                                            * The number of possible options should be very limited, 
                                            * i.e. it should be feasible to cycle through options 
                                            * without a keyboard.
                                            *
                                            * First entry should be treated as a default.
                                            *
                                            * Example entry:
                                            * { "foo_option", "Speed hack coprocessor X; false|true" }
                                            *
                                            * Text before first ';' is description. This ';' must be 
                                            * followed by a space, and followed by a list of possible 
                                            * values split up with '|'.
                                            *
                                            * Only strings are operated on. The possible values will 
                                            * generally be displayed and stored as-is by the frontend.
                                            */
pub const RETRO_ENVIRONMENT_GET_VARIABLE_UPDATE: c_uint = 17;
                                           /* *mut u8 --
                                            * Result is set to true if some variables are updated by
                                            * frontend since last call to RETRO_ENVIRONMENT_GET_VARIABLE.
                                            * Variables should be queried with GET_VARIABLE.
                                            */
pub const RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME: c_uint = 18;
                                           /* *const u8 --
                                            * If true, the libretro implementation supports calls to 
                                            * retro_load_game() with NULL as argument.
                                            * Used by cores which can run without particular game data.
                                            * This should be called within retro_set_environment() only.
                                            */
pub const RETRO_ENVIRONMENT_GET_LIBRETRO_PATH: c_uint = 19;
                                           /* *const *mut c_char --
                                            * Retrieves the absolute path from where this libretro 
                                            * implementation was loaded.
                                            * NULL is returned if the libretro was loaded statically 
                                            * (i.e. linked statically to frontend), or if the path cannot be 
                                            * determined.
                                            * Mostly useful in cooperation with SET_SUPPORT_NO_GAME as assets can 
                                            * be loaded without ugly hacks.
                                            */
                                           
                                           /* Environment 20 was an obsolete version of SET_AUDIO_CALLBACK. 
                                            * It was not used by any known core at the time,
                                            * and was removed from the API. */
pub const RETRO_ENVIRONMENT_SET_AUDIO_CALLBACK: c_uint = 22;
                                           /* *const retro_audio_callback --
                                            * Sets an interface which is used to notify a libretro core about audio 
                                            * being available for writing.
                                            * The callback can be called from any thread, so a core using this must 
                                            * have a thread safe audio implementation.
                                            * It is intended for games where audio and video are completely 
                                            * asynchronous and audio can be generated on the fly.
                                            * This interface is not recommended for use with emulators which have 
                                            * highly synchronous audio.
                                            *
                                            * The callback only notifies about writability; the libretro core still 
                                            * has to call the normal audio callbacks
                                            * to write audio. The audio callbacks must be called from within the 
                                            * notification callback.
                                            * The amount of audio data to write is up to the implementation.
                                            * Generally, the audio callback will be called continously in a loop.
                                            *
                                            * Due to thread safety guarantees and lack of sync between audio and 
                                            * video, a frontend  can selectively disallow this interface based on 
                                            * internal configuration. A core using this interface must also 
                                            * implement the "normal" audio interface.
                                            *
                                            * A libretro core using SET_AUDIO_CALLBACK should also make use of 
                                            * SET_FRAME_TIME_CALLBACK.
                                            */
pub const RETRO_ENVIRONMENT_SET_FRAME_TIME_CALLBACK: c_uint = 21;
                                           /* *const retro_frame_time_callback --
                                            * Lets the core know how much time has passed since last 
                                            * invocation of retro_run().
                                            * The frontend can tamper with the timing to fake fast-forward, 
                                            * slow-motion, frame stepping, etc.
                                            * In this case the delta time will use the reference value 
                                            * in frame_time_callback..
                                            */
pub const RETRO_ENVIRONMENT_GET_RUMBLE_INTERFACE: c_uint = 23;
                                           /* *mut retro_rumble_interface -
                                            * Gets an interface which is used by a libretro core to set 
                                            * state of rumble motors in controllers.
                                            * A strong and weak motor is supported, and they can be 
                                            * controlled indepedently.
                                            */
pub const RETRO_ENVIRONMENT_GET_INPUT_DEVICE_CAPABILITIES: c_uint = 24;
                                           /* *mut uint64 --
                                            * Gets a bitmask telling which device type are expected to be 
                                            * handled properly in a call to retro_input_state_t.
                                            * Devices which are not handled or recognized always return 
                                            * 0 in retro_input_state_t.
                                            * Example bitmask: caps = (1 << RETRO_DEVICE_JOYPAD) | (1 << RETRO_DEVICE_ANALOG).
                                            * Should only be called in retro_run().
                                            */
pub const RETRO_ENVIRONMENT_GET_SENSOR_INTERFACE: c_uint = (25 | RETRO_ENVIRONMENT_EXPERIMENTAL);
                                           /* *mut retro_sensor_interface --
                                            * Gets access to the sensor interface.
                                            * The purpose of this interface is to allow
                                            * setting state related to sensors such as polling rate, 
                                            * enabling/disable it entirely, etc.
                                            * Reading sensor state is done via the normal 
                                            * input_state_callback API.
                                            */
pub const RETRO_ENVIRONMENT_GET_CAMERA_INTERFACE: c_uint = (26 | RETRO_ENVIRONMENT_EXPERIMENTAL);
                                           /* *mut retro_camera_callback --
                                            * Gets an interface to a video camera driver.
                                            * A libretro core can use this interface to get access to a 
                                            * video camera.
                                            * New video frames are delivered in a callback in same 
                                            * thread as retro_run().
                                            *
                                            * GET_CAMERA_INTERFACE should be called in retro_load_game().
                                            *
                                            * Depending on the camera implementation used, camera frames 
                                            * will be delivered as a raw framebuffer,
                                            * or as an OpenGL texture directly.
                                            *
                                            * The core has to tell the frontend here which types of 
                                            * buffers can be handled properly.
                                            * An OpenGL texture can only be handled when using a 
                                            * libretro GL core (SET_HW_RENDER).
                                            * It is recommended to use a libretro GL core when 
                                            * using camera interface.
                                            *
                                            * The camera is not started automatically. The retrieved start/stop 
                                            * functions must be used to explicitly
                                            * start and stop the camera driver.
                                            */
pub const RETRO_ENVIRONMENT_GET_LOG_INTERFACE: c_uint = 27;
                                           /* *mut retro_log_callback --
                                            * Gets an interface for logging. This is useful for 
                                            * logging in a cross-platform way
                                            * as certain platforms cannot use use stderr for logging. 
                                            * It also allows the frontend to
                                            * show logging information in a more suitable way.
                                            * If this interface is not used, libretro cores should 
                                            * log to stderr as desired.
                                            */
pub const RETRO_ENVIRONMENT_GET_PERF_INTERFACE: c_uint = 28;
                                           /* *mut retro_perf_callback --
                                            * Gets an interface for performance counters. This is useful 
                                            * for performance logging in a cross-platform way and for detecting 
                                            * architecture-specific features, such as SIMD support.
                                            */
pub const RETRO_ENVIRONMENT_GET_LOCATION_INTERFACE: c_uint = 29;
                                           /* *mut retro_location_callback --
                                            * Gets access to the location interface.
                                            * The purpose of this interface is to be able to retrieve 
                                            * location-based information from the host device,
                                            * such as current latitude / longitude.
                                            */
pub const RETRO_ENVIRONMENT_GET_CONTENT_DIRECTORY: c_uint = 30;
                                           /* *const *mut c_char --
                                            * Returns the "content" directory of the frontend.
                                            * This directory can be used to store specific assets that the 
                                            * core relies upon, such as art assets,
                                            * input data, etc etc.
                                            * The returned value can be NULL.
                                            * If so, no such directory is defined,
                                            * and it's up to the implementation to find a suitable directory.
                                            */
pub const RETRO_ENVIRONMENT_GET_SAVE_DIRECTORY: c_uint = 31;
                                           /* *const *mut c_char --
                                            * Returns the "save" directory of the frontend.
                                            * This directory can be used to store SRAM, memory cards, 
                                            * high scores, etc, if the libretro core
                                            * cannot use the regular memory interface (retro_get_memory_data()).
                                            *
                                            * NOTE: libretro cores used to check GET_SYSTEM_DIRECTORY for 
                                            * similar things before.
                                            * They should still check GET_SYSTEM_DIRECTORY if they want to 
                                            * be backwards compatible.
                                            * The path here can be NULL. It should only be non-NULL if the 
                                            * frontend user has set a specific save path.
                                            */
pub const RETRO_ENVIRONMENT_SET_SYSTEM_AV_INFO: c_uint = 32;
                                           /* *const retro_system_av_info --
                                            * Sets a new av_info structure. This can only be called from 
                                            * within retro_run().
                                            * This should *only* be used if the core is completely altering the 
                                            * internal resolutions, aspect ratios, timings, sampling rate, etc.
                                            * Calling this can require a full reinitialization of video/audio 
                                            * drivers in the frontend,
                                            *
                                            * so it is important to call it very sparingly, and usually only with 
                                            * the users explicit consent.
                                            * An eventual driver reinitialize will happen so that video and 
                                            * audio callbacks
                                            * happening after this call within the same retro_run() call will 
                                            * target the newly initialized driver.
                                            *
                                            * This callback makes it possible to support configurable resolutions 
                                            * in games, which can be useful to
                                            * avoid setting the "worst case" in max_width/max_height.
                                            *
                                            * ***HIGHLY RECOMMENDED*** Do not call this callback every time 
                                            * resolution changes in an emulator core if it's
                                            * expected to be a temporary change, for the reasons of possible 
                                            * driver reinitialization.
                                            * This call is not a free pass for not trying to provide 
                                            * correct values in retro_get_system_av_info(). If you need to change 
                                            * things like aspect ratio or nominal width/height, 
                                            * use RETRO_ENVIRONMENT_SET_GEOMETRY, which is a softer variant 
                                            * of SET_SYSTEM_AV_INFO.
                                            *
                                            * If this returns false, the frontend does not acknowledge a 
                                            * changed av_info struct.
                                            */
pub const RETRO_ENVIRONMENT_SET_PROC_ADDRESS_CALLBACK: c_uint = 33;
                                           /* *const retro_get_proc_address_interface --
                                            * Allows a libretro core to announce support for the 
                                            * get_proc_address() interface.
                                            * This interface allows for a standard way to extend libretro where 
                                            * use of environment calls are too indirect,
                                            * e.g. for cases where the frontend wants to call directly into the core.
                                            *
                                            * If a core wants to expose this interface, SET_PROC_ADDRESS_CALLBACK 
                                            * **MUST** be called from within retro_set_environment().
                                            */
pub const RETRO_ENVIRONMENT_SET_SUBSYSTEM_INFO: c_uint = 34;
                                           /* *const retro_subsystem_info --
                                            * This environment call introduces the concept of libretro "subsystems".
                                            * A subsystem is a variant of a libretro core which supports 
                                            * different kinds of games.
                                            * The purpose of this is to support e.g. emulators which might 
                                            * have special needs, e.g. Super Nintendo's Super GameBoy, Sufami Turbo.
                                            * It can also be used to pick among subsystems in an explicit way 
                                            * if the libretro implementation is a multi-system emulator itself.
                                            *
                                            * Loading a game via a subsystem is done with retro_load_game_special(),
                                            * and this environment call allows a libretro core to expose which 
                                            * subsystems are supported for use with retro_load_game_special().
                                            * A core passes an array of retro_game_special_info which is terminated 
                                            * with a zeroed out retro_game_special_info struct.
                                            *
                                            * If a core wants to use this functionality, SET_SUBSYSTEM_INFO
                                            * **MUST** be called from within retro_set_environment().
                                            */
pub const RETRO_ENVIRONMENT_SET_CONTROLLER_INFO: c_uint = 35;
                                           /* *const retro_controller_info --
                                            * This environment call lets a libretro core tell the frontend 
                                            * which controller types are recognized in calls to 
                                            * retro_set_controller_port_device().
                                            *
                                            * Some emulators such as Super Nintendo
                                            * support multiple lightgun types which must be specifically 
                                            * selected from.
                                            * It is therefore sometimes necessary for a frontend to be able 
                                            * to tell the core about a special kind of input device which is 
                                            * not covered by the libretro input API.
                                            *
                                            * In order for a frontend to understand the workings of an input device,
                                            * it must be a specialized type
                                            * of the generic device types already defined in the libretro API.
                                            *
                                            * Which devices are supported can vary per input port.
                                            * The core must pass an array of const struct retro_controller_info which 
                                            * is terminated with a blanked out struct. Each element of the struct 
                                            * corresponds to an ascending port index to 
                                            * retro_set_controller_port_device().
                                            * Even if special device types are set in the libretro core, 
                                            * libretro should only poll input based on the base input device types.
                                            */
pub const RETRO_ENVIRONMENT_SET_MEMORY_MAPS: c_uint = (36 | RETRO_ENVIRONMENT_EXPERIMENTAL);
                                           /* *const retro_memory_map --
                                            * This environment call lets a libretro core tell the frontend 
                                            * about the memory maps this core emulates.
                                            * This can be used to implement, for example, cheats in a core-agnostic way.
                                            *
                                            * Should only be used by emulators; it doesn't make much sense for 
                                            * anything else.
                                            * It is recommended to expose all relevant pointers through 
                                            * retro_get_memory_* as well.
                                            *
                                            * Can be called from retro_init and retro_load_game.
                                            */
pub const RETRO_ENVIRONMENT_SET_GEOMETRY: c_uint = 37;
                                           /* *const retro_game_geometry --
                                            * This environment call is similar to SET_SYSTEM_AV_INFO for changing 
                                            * video parameters, but provides a guarantee that drivers will not be 
                                            * reinitialized.
                                            * This can only be called from within retro_run().
                                            *
                                            * The purpose of this call is to allow a core to alter nominal 
                                            * width/heights as well as aspect ratios on-the-fly, which can be 
                                            * useful for some emulators to change in run-time.
                                            *
                                            * max_width/max_height arguments are ignored and cannot be changed
                                            * with this call as this could potentially require a reinitialization or a 
                                            * non-constant time operation.
                                            * If max_width/max_height are to be changed, SET_SYSTEM_AV_INFO is required.
                                            *
                                            * A frontend must guarantee that this environment call completes in 
                                            * constant time.
                                            */
pub const RETRO_ENVIRONMENT_GET_USERNAME: c_uint = 38;
                                           /* *const *mut c_char
                                            * Returns the specified username of the frontend, if specified by the user.
                                            * This username can be used as a nickname for a core that has online facilities 
                                            * or any other mode where personalization of the user is desirable.
                                            * The returned value can be NULL.
                                            * If this environ callback is used by a core that requires a valid username, 
                                            * a default username should be specified by the core.
                                            */
pub const RETRO_ENVIRONMENT_GET_LANGUAGE: c_uint = 39;
                                           /* *mut c_uint --
                                            * Returns the specified language of the frontend, if specified by the user.
                                            * It can be used by the core for localization purposes.
                                            */

pub const RETRO_MEMDESC_CONST:     c_uint = (1 << 0);   /* The frontend will never change this memory area once retro_load_game has returned. */
pub const RETRO_MEMDESC_BIGENDIAN: c_uint = (1 << 1);   /* The memory area contains big endian data. Default is little endian. */
pub const RETRO_MEMDESC_ALIGN_2:   c_uint = (1 << 16);  /* All memory access in this area is aligned to their own size, or 2, whichever is smaller. */
pub const RETRO_MEMDESC_ALIGN_4:   c_uint = (2 << 16);
pub const RETRO_MEMDESC_ALIGN_8:   c_uint = (3 << 16);
pub const RETRO_MEMDESC_MINSIZE_2: c_uint = (1 << 24);  /* All memory in this region is accessed at least 2 bytes at the time. */
pub const RETRO_MEMDESC_MINSIZE_4: c_uint = (2 << 24);
pub const RETRO_MEMDESC_MINSIZE_8: c_uint = (3 << 24);
#[repr(C)]
pub struct retro_memory_descriptor
{
   flags: u64,

   /* Pointer to the start of the relevant ROM or RAM chip.
    * It's strongly recommended to use 'offset' if possible, rather than 
    * doing math on the pointer.
    *
    * If the same byte is mapped my multiple descriptors, their descriptors 
    * must have the same pointer.
    * If 'start' does not point to the first byte in the pointer, put the 
    * difference in 'offset' instead.
    *
    * May be NULL if there's nothing usable here (e.g. hardware registers and 
    * open bus). No flags should be set if the pointer is NULL.
    * It's recommended to minimize the number of descriptors if possible,
    * but not mandatory. */
   pub ptr: *mut c_void,
   pub offset: size_t,

   /* This is the location in the emulated address space 
    * where the mapping starts. */
   pub start: size_t,

   /* Which bits must be same as in 'start' for this mapping to apply.
    * The first memory descriptor to claim a certain byte is the one 
    * that applies.
    * A bit which is set in 'start' must also be set in this.
    * Can be zero, in which case each byte is assumed mapped exactly once. 
    * In this case, 'len' must be a power of two. */
   pub select: size_t,

   /* If this is nonzero, the set bits are assumed not connected to the 
    * memory chip's address pins. */
   pub disconnect: size_t,

   /* This one tells the size of the current memory area.
    * If, after start+disconnect are applied, the address is higher than 
    * this, the highest bit of the address is cleared.
    *
    * If the address is still too high, the next highest bit is cleared.
    * Can be zero, in which case it's assumed to be infinite (as limited 
    * by 'select' and 'disconnect'). */
   pub len: size_t,

   /* To go from emulated address to physical address, the following 
    * order applies:
    * Subtract 'start', pick off 'disconnect', apply 'len', add 'offset'.
    *
    * The address space name must consist of only a-zA-Z0-9_-, 
    * should be as short as feasible (maximum length is 8 plus the NUL),
    * and may not be any other address space plus one or more 0-9A-F 
    * at the end.
    * However, multiple memory descriptors for the same address space is 
    * allowed, and the address space name can be empty. NULL is treated 
    * as empty.
    *
    * Address space names are case sensitive, but avoid lowercase if possible.
    * The same pointer may exist in multiple address spaces.
    *
    * Examples:
    * blank+blank - valid (multiple things may be mapped in the same namespace)
    * 'Sp'+'Sp' - valid (multiple things may be mapped in the same namespace)
    * 'A'+'B' - valid (neither is a prefix of each other)
    * 'S'+blank - valid ('S' is not in 0-9A-F)
    * 'a'+blank - valid ('a' is not in 0-9A-F)
    * 'a'+'A' - valid (neither is a prefix of each other)
    * 'AR'+blank - valid ('R' is not in 0-9A-F)
    * 'ARB'+blank - valid (the B can't be part of the address either, because 
    * there is no namespace 'AR')
    * blank+'B' - not valid, because it's ambigous which address space B1234 
    * would refer to.
    * The length can't be used for that purpose; the frontend may want 
    * to append arbitrary data to an address, without a separator. */
   pub addrspace: *const c_char,
}

/* The frontend may use the largest value of 'start'+'select' in a 
 * certain namespace to infer the size of the address space.
 *
 * If the address space is larger than that, a mapping with .ptr=NULL 
 * should be at the end of the array, with .select set to all ones for 
 * as long as the address space is big.
 *
 * Sample descriptors (minus .ptr, and RETRO_MEMFLAG_ on the flags):
 * SNES WRAM:
 * .start=0x7E0000, .len=0x20000
 * (Note that this must be mapped before the ROM in most cases; some of the 
 * ROM mappers 
 * try to claim $7E0000, or at least $7E8000.)
 * SNES SPC700 RAM:
 * .addrspace="S", .len=0x10000
 * SNES WRAM mirrors:
 * .flags=MIRROR, .start=0x000000, .select=0xC0E000, .len=0x2000
 * .flags=MIRROR, .start=0x800000, .select=0xC0E000, .len=0x2000
 * SNES WRAM mirrors, alternate equivalent descriptor:
 * .flags=MIRROR, .select=0x40E000, .disconnect=~0x1FFF
 * (Various similar constructions can be created by combining parts of 
 * the above two.)
 * SNES LoROM (512KB, mirrored a couple of times):
 * .flags=CONST, .start=0x008000, .select=0x408000, .disconnect=0x8000, .len=512*1024
 * .flags=CONST, .start=0x400000, .select=0x400000, .disconnect=0x8000, .len=512*1024
 * SNES HiROM (4MB):
 * .flags=CONST,                 .start=0x400000, .select=0x400000, .len=4*1024*1024
 * .flags=CONST, .offset=0x8000, .start=0x008000, .select=0x408000, .len=4*1024*1024
 * SNES ExHiROM (8MB):
 * .flags=CONST, .offset=0,                  .start=0xC00000, .select=0xC00000, .len=4*1024*1024
 * .flags=CONST, .offset=4*1024*1024,        .start=0x400000, .select=0xC00000, .len=4*1024*1024
 * .flags=CONST, .offset=0x8000,             .start=0x808000, .select=0xC08000, .len=4*1024*1024
 * .flags=CONST, .offset=4*1024*1024+0x8000, .start=0x008000, .select=0xC08000, .len=4*1024*1024
 * Clarify the size of the address space:
 * .ptr=NULL, .select=0xFFFFFF
 * .len can be implied by .select in many of them, but was included for clarity.
 */

#[repr(C)]
pub struct retro_memory_map
{
   pub descriptors: *const retro_memory_descriptor,
   pub num_descriptors: c_uint,
}

#[repr(C)]
pub struct retro_controller_description
{
   /* Human-readable description of the controller. Even if using a generic 
    * input device type, this can be set to the particular device type the 
    * core uses. */
   pub desc: *const c_void,

   /* Device type passed to retro_set_controller_port_device(). If the device 
    * type is a sub-class of a generic input device type, use the 
    * RETRO_DEVICE_SUBCLASS macro to create an ID.
    *
    * E.g. RETRO_DEVICE_SUBCLASS(RETRO_DEVICE_JOYPAD, 1). */
   pub id: c_uint,
}

#[repr(C)]
pub struct retro_controller_info
{
   pub types: *const retro_controller_description,
   pub num_types: c_uint,
}

#[repr(C)]
pub struct retro_subsystem_memory_info
{
   /* The extension associated with a memory type, e.g. "psram". */
   pub extension: *const c_char,

   /* The memory type for retro_get_memory(). This should be at 
    * least 0x100 to avoid conflict with standardized 
    * libretro memory types. */
   pub mem_type: c_uint, /* NOTE renamed to avoid keyword conflict */
}

#[repr(C)]
pub struct retro_subsystem_rom_info
{
   /* Describes what the content is (SGB BIOS, GB ROM, etc). */
   pub desc: *const c_char,

   /* Same definition as retro_get_system_info(). */
   pub valid_extensions: *const c_char,

   /* Same definition as retro_get_system_info(). */
   /* TODO use c_bool if Rust introduces it */
   pub need_fullpath: u8,

   /* Same definition as retro_get_system_info(). */
   pub block_extract: u8,

   /* This is set if the content is required to load a game. 
    * If this is set to false, a zeroed-out retro_game_info can be passed. */
   pub required: u8,

   /* Content can have multiple associated persistent 
    * memory types (retro_get_memory()). */
   pub memory: *const retro_subsystem_memory_info,
   pub num_memory: c_uint,
}

#[repr(C)]
pub struct retro_subsystem_info
{
   /* Human-readable string of the subsystem type, e.g. "Super GameBoy" */
   pub desc: *const c_char,

   /* A computer friendly short string identifier for the subsystem type.
    * This name must be [a-z].
    * E.g. if desc is "Super GameBoy", this can be "sgb".
    * This identifier can be used for command-line interfaces, etc.
    */
   pub ident: *const c_char,

   /* Infos for each content file. The first entry is assumed to be the 
    * "most significant" content for frontend purposes.
    * E.g. with Super GameBoy, the first content should be the GameBoy ROM, 
    * as it is the most "significant" content to a user.
    * If a frontend creates new file paths based on the content used 
    * (e.g. savestates), it should use the path for the first ROM to do so. */
   pub roms: *const retro_subsystem_rom_info,

   /* Number of content files associated with a subsystem. */
   pub num_roms: c_uint,
   
   /* The type passed to retro_load_game_special(). */
   pub id: c_uint,
}

#[allow(non_camel_case_types)]
pub type retro_proc_address_t = extern "C" fn();

/* libretro API extension functions:
 * (None here so far).
 *
 * Get a symbol from a libretro core.
 * Cores should only return symbols which are actual 
 * extensions to the libretro API.
 *
 * Frontends should not use this to obtain symbols to standard 
 * libretro entry points (static linking or dlsym).
 *
 * The symbol name must be equal to the function name, 
 * e.g. if void retro_foo(void); exists, the symbol must be called "retro_foo".
 * The returned function pointer must be cast to the corresponding type.
 */
#[allow(non_camel_case_types)]
pub type retro_get_proc_address_t = extern "C" fn(sym: *const c_char)
                                     -> retro_proc_address_t;

#[repr(C)]
pub struct retro_get_proc_address_interface
{
   pub get_proc_address: retro_get_proc_address_t,
}

// enum values from retro_log_level:
pub const RETRO_LOG_DEBUG: c_int = 0;
pub const RETRO_LOG_INFO: c_int = 1;
pub const RETRO_LOG_WARN: c_int = 2;
pub const RETRO_LOG_ERROR: c_int = 3;

/* Logging function. Takes log level argument as well. */
// level is one of RETRO_LOG_* consts
#[allow(non_camel_case_types)]
pub type retro_log_printf_t = extern "C" fn(level: c_int,
                                             fmt: *const c_char,
                                             ...);
#[repr(C)]
pub struct retro_log_callback
{
   pub log: retro_log_printf_t,
}

/* Performance related functions */

/* ID values for SIMD CPU features */
pub const RETRO_SIMD_SSE:      c_uint = (1 << 0);
pub const RETRO_SIMD_SSE2:     c_uint = (1 << 1);
pub const RETRO_SIMD_VMX:      c_uint = (1 << 2);
pub const RETRO_SIMD_VMX128:   c_uint = (1 << 3);
pub const RETRO_SIMD_AVX:      c_uint = (1 << 4);
pub const RETRO_SIMD_NEON:     c_uint = (1 << 5);
pub const RETRO_SIMD_SSE3:     c_uint = (1 << 6);
pub const RETRO_SIMD_SSSE3:    c_uint = (1 << 7);
pub const RETRO_SIMD_MMX:      c_uint = (1 << 8);
pub const RETRO_SIMD_MMXEXT:   c_uint = (1 << 9);
pub const RETRO_SIMD_SSE4:     c_uint = (1 << 10);
pub const RETRO_SIMD_SSE42:    c_uint = (1 << 11);
pub const RETRO_SIMD_AVX2:     c_uint = (1 << 12);
pub const RETRO_SIMD_VFPU:     c_uint = (1 << 13);
pub const RETRO_SIMD_PS:       c_uint = (1 << 14);
pub const RETRO_SIMD_AES:      c_uint = (1 << 15);

#[allow(non_camel_case_types)]
pub type retro_perf_tick_t = u64;
#[allow(non_camel_case_types)]
pub type retro_time_t = i64;

#[repr(C)]
pub struct retro_perf_counter
{
   pub ident: *const c_char,
   pub start: retro_perf_tick_t,
   pub total: retro_perf_tick_t,
   pub call_cnt: retro_perf_tick_t,

   pub registered: u8,
}

/* Returns current time in microseconds.
 * Tries to use the most accurate timer available.
 */
#[allow(non_camel_case_types)]
pub type retro_perf_get_time_usec_t = extern "C" fn() -> retro_time_t;

/* A simple counter. Usually nanoseconds, but can also be CPU cycles.
 * Can be used directly if desired (when creating a more sophisticated 
 * performance counter system).
 * */
#[allow(non_camel_case_types)]
pub type retro_perf_get_counter_t = extern "C" fn() -> retro_perf_tick_t;

/* Returns a bit-mask of detected CPU features (RETRO_SIMD_*). */
#[allow(non_camel_case_types)]
pub type retro_get_cpu_features_t = extern "C" fn() -> u64;

/* Asks frontend to log and/or display the state of performance counters.
 * Performance counters can always be poked into manually as well.
 */
#[allow(non_camel_case_types)]
pub type retro_perf_log_t = extern "C" fn();

/* Register a performance counter.
 * ident field must be set with a discrete value and other values in 
 * retro_perf_counter must be 0.
 * Registering can be called multiple times. To avoid calling to 
 * frontend redundantly, you can check registered field first. */
#[allow(non_camel_case_types)]
pub type retro_perf_register_t = extern "C" fn(counter: *mut retro_perf_counter);

/* Starts a registered counter. */
#[allow(non_camel_case_types)]
pub type retro_perf_start_t = extern "C" fn(counter: *mut retro_perf_counter);

/* Stops a registered counter. */
#[allow(non_camel_case_types)]
pub type retro_perf_stop_t = extern "C" fn(counter: *mut retro_perf_counter);

/* TODO: Port example code to Rust
 * For convenience it can be useful to wrap register, start and stop in macros.
 * E.g.:
 * #ifdef LOG_PERFORMANCE
 * #define RETRO_PERFORMANCE_INIT(perf_cb, name) static struct retro_perf_counter name = {#name}; if (!name.registered) perf_cb.perf_register(&(name))
 * #define RETRO_PERFORMANCE_START(perf_cb, name) perf_cb.perf_start(&(name))
 * #define RETRO_PERFORMANCE_STOP(perf_cb, name) perf_cb.perf_stop(&(name))
 * #else
 * ... Blank macros ...
 * #endif
 *
 * These can then be used mid-functions around code snippets.
 *
 * extern struct retro_perf_callback perf_cb;  * Somewhere in the core.
 *
 * void do_some_heavy_work(void)
 * {
 *    RETRO_PERFORMANCE_INIT(cb, work_1;
 *    RETRO_PERFORMANCE_START(cb, work_1);
 *    heavy_work_1();
 *    RETRO_PERFORMANCE_STOP(cb, work_1);
 *
 *    RETRO_PERFORMANCE_INIT(cb, work_2);
 *    RETRO_PERFORMANCE_START(cb, work_2);
 *    heavy_work_2();
 *    RETRO_PERFORMANCE_STOP(cb, work_2);
 * }
 *
 * void retro_deinit(void)
 * {
 *    perf_cb.perf_log();  * Log all perf counters here for example.
 * }
 */

#[repr(C)]
pub struct retro_perf_callback
{
   pub get_time_usec:            retro_perf_get_time_usec_t,
   pub get_cpu_features:         retro_get_cpu_features_t,

   pub get_perf_counter:         retro_perf_get_counter_t,
   pub perf_register:            retro_perf_register_t,
   pub perf_start:               retro_perf_start_t,
   pub perf_stop:                retro_perf_stop_t,
   pub perf_log:                 retro_perf_log_t,
}

/* FIXME: Document the sensor API and work out behavior.
 * It will be marked as experimental until then.
 */
// enum values from retro_sensor_action:
pub const RETRO_SENSOR_ACCELEROMETER_ENABLE: c_uint = 0;
pub const RETRO_SENSOR_ACCELEROMETER_DISABLE: c_uint = 1;

/* Id values for SENSOR types. */
pub const RETRO_SENSOR_ACCELEROMETER_X: c_uint = 0;
pub const RETRO_SENSOR_ACCELEROMETER_Y: c_uint = 1;
pub const RETRO_SENSOR_ACCELEROMETER_Z: c_uint = 2;

// action is one of RETRO_SENSOR_ACCELEROMETER_* consts
#[allow(non_camel_case_types)]
pub type retro_set_sensor_state_t = extern "C" fn(port: c_uint,
                                                   action: c_int,
                                                   rate: c_uint);

#[allow(non_camel_case_types)]
pub type retro_sensor_get_input_t = extern "C" fn(port: c_uint,
                                                   id: c_uint) 
                                                   -> ::libc::c_float;
#[allow(dead_code)]
#[repr(C)]
struct retro_sensor_interface
{
   set_sensor_state: retro_set_sensor_state_t,
   get_sensor_input: retro_sensor_get_input_t,
}

// enum values from retro_camera_buffer:
pub const RETRO_CAMERA_BUFFER_OPENGL_TEXTURE: c_int = 0;
pub const RETRO_CAMERA_BUFFER_RAW_FRAMEBUFFER: c_int = 1;

/* Starts the camera driver. Can only be called in retro_run(). */
#[allow(non_camel_case_types)]
pub type retro_camera_start_t = extern "C" fn() -> u8;

/* Stops the camera driver. Can only be called in retro_run(). */
#[allow(non_camel_case_types)]
pub type retro_camera_stop_t = extern "C" fn();

/* Callback which signals when the camera driver is initialized 
 * and/or deinitialized.
 * retro_camera_start_t can be called in initialized callback.
 */
#[allow(non_camel_case_types)]
pub type retro_camera_lifetime_status_t = extern "C" fn();

/* A callback for raw framebuffer data. buffer points to an XRGB8888 buffer.
 * Width, height and pitch are similar to retro_video_refresh_t.
 * First pixel is top-left origin.
 */
#[allow(non_camel_case_types)]
pub type retro_camera_frame_raw_framebuffer_t = extern "C" fn
                                                (buffer: *const c_uint,
                                                 width: c_uint,
                                                 height: c_uint,
                                                 pitch: size_t);

/* A callback for when OpenGL textures are used.
 *
 * texture_id is a texture owned by camera driver.
 * Its state or content should be considered immutable, except for things like 
 * texture filtering and clamping.
 *
 * texture_target is the texture target for the GL texture.
 * These can include e.g. GL_TEXTURE_2D, GL_TEXTURE_RECTANGLE, and possibly 
 * more depending on extensions.
 *
 * affine points to a packed 3x3 column-major matrix used to apply an affine 
 * transform to texture coordinates. (affine_matrix * vec3(coord_x, coord_y, 1.0))
 * After transform, normalized texture coord (0, 0) should be bottom-left 
 * and (1, 1) should be top-right (or (width, height) for RECTANGLE).
 *
 * GL-specific typedefs are avoided here to avoid relying on gl.h in 
 * the API definition.
 */
#[allow(non_camel_case_types)]
pub type retro_camera_frame_opengl_texture_t = extern "C" fn
                                              (texture_id: c_uint,
                                               texture_target: c_uint,
                                               affine: *const ::libc::c_float);

#[allow(dead_code)]
#[repr(C)]
struct retro_camera_callback
{
   /* Set by libretro core. 
    * Example bitmask: caps = (1 << RETRO_CAMERA_BUFFER_OPENGL_TEXTURE) | (1 << RETRO_CAMERA_BUFFER_RAW_FRAMEBUFFER).
    */
   pub caps: ::libc::c_ulong,

   pub width: c_uint, /* Desired resolution for camera. Is only used as a hint. */
   pub height: c_uint,
   pub start: retro_camera_start_t, /* Set by frontend. */
   pub stop: retro_camera_stop_t, /* Set by frontend. */

   /* Set by libretro core if raw framebuffer callbacks will be used. */
   pub frame_raw_framebuffer: retro_camera_frame_raw_framebuffer_t,
   /* Set by libretro core if OpenGL texture callbacks will be used. */
   pub frame_opengl_texture: retro_camera_frame_opengl_texture_t,

   /* Set by libretro core. Called after camera driver is initialized and 
    * ready to be started.
    * Can be NULL, in which this callback is not called.
    */
   pub initialized: retro_camera_lifetime_status_t,

   /* Set by libretro core. Called right before camera driver is 
    * deinitialized.
    * Can be NULL, in which this callback is not called.
    */
   pub deinitialized: retro_camera_lifetime_status_t,
}

/* Sets the interval of time and/or distance at which to update/poll 
 * location-based data.
 *
 * To ensure compatibility with all location-based implementations,
 * values for both interval_ms and interval_distance should be provided.
 *
 * interval_ms is the interval expressed in milliseconds.
 * interval_distance is the distance interval expressed in meters.
 */
#[allow(non_camel_case_types)]
pub type retro_location_set_interval_t = extern "C" fn(interval_ms: c_uint,
                                                        interval_distance: c_uint);

/* Start location services. The device will start listening for changes to the
 * current location at regular intervals (which are defined with 
 * retro_location_set_interval_t). */
#[allow(non_camel_case_types)]
pub type retro_location_start_t = extern "C" fn() -> u8;

/* Stop location services. The device will stop listening for changes 
 * to the current location. */
#[allow(non_camel_case_types)]
pub type retro_location_stop_t = extern "C" fn();

/* Get the position of the current location. Will set parameters to 
 * 0 if no new  location update has happened since the last time. */
#[allow(non_camel_case_types)]
pub type retro_location_get_position_t = extern "C" fn
                                         (lat: *mut ::libc::c_double,
                                          lon: *mut ::libc::c_double,
                                          horiz_accuracy: *mut ::libc::c_double,
                                          vert_accuracy: *mut ::libc::c_double) -> u8;

/* Callback which signals when the location driver is initialized 
 * and/or deinitialized.
 * retro_location_start_t can be called in initialized callback.
 */
#[allow(non_camel_case_types)]
pub type retro_location_lifetime_status_t = extern "C" fn();

#[allow(dead_code)]
#[repr(C)]
struct retro_location_callback
{
   pub start: retro_location_start_t,
   pub stop: retro_location_stop_t,
   pub get_position: retro_location_get_position_t,
   pub set_interval: retro_location_set_interval_t,

   pub initialized: retro_location_lifetime_status_t,
   pub deinitialized: retro_location_lifetime_status_t,
}

// enum values from retro_rumble_effect:
pub const RETRO_RUMBLE_STRONG: c_int = 0;
pub const RETRO_RUMBLE_WEAK: c_int = 1;

/* Sets rumble state for joypad plugged in port 'port'. 
 * Rumble effects are controlled independently,
 * and setting e.g. strong rumble does not override weak rumble.
 * Strength has a range of [0, 0xffff].
 *
 * Returns true if rumble state request was honored. 
 * Calling this before first retro_run() is likely to return false. */
// effect is one of RETRO_RUMBLE_* consts
#[allow(non_camel_case_types)]
pub type retro_set_rumble_state_t = extern "C" fn(port: c_uint,
                                                    effect: c_int,
                                                    strength: u16)
                                                    -> u8;

#[allow(dead_code)]
#[repr(C)]
struct retro_rumble_interface
{
   pub set_rumble_state: retro_set_rumble_state_t,
}

/* Notifies libretro that audio data should be written. */
#[allow(non_camel_case_types)]
pub type retro_audio_callback_t = extern "C" fn();

/* True: Audio driver in frontend is active, and callback is 
 * expected to be called regularily.
 * False: Audio driver in frontend is paused or inactive. 
 * Audio callback will not be called until set_state has been 
 * called with true.
 * Initial state is false (inactive).
 */
#[allow(non_camel_case_types)]
pub type retro_audio_set_state_callback_t = extern "C" fn(enabled: u8);

#[allow(dead_code)]
#[repr(C)]
struct retro_audio_callback
{
   pub callback: retro_audio_callback_t,
   pub set_state: retro_audio_set_state_callback_t,
}

/* Notifies a libretro core of time spent since last invocation 
 * of retro_run() in microseconds.
 *
 * It will be called right before retro_run() every frame.
 * The frontend can tamper with timing to support cases like 
 * fast-forward, slow-motion and framestepping.
 *
 * In those scenarios the reference frame time value will be used. */
#[allow(non_camel_case_types)]
pub type retro_usec_t = ::libc::c_long;
#[allow(non_camel_case_types)]
pub type retro_frame_time_callback_t = extern "C" fn(usec: retro_usec_t);

#[allow(dead_code)]
#[repr(C)]
struct retro_frame_time_callback
{
   pub callback: retro_frame_time_callback_t,
   /* Represents the time of one frame. It is computed as 
    * 1000000 / fps, but the implementation will resolve the 
    * rounding to ensure that framestepping, etc is exact. */
   pub reference: retro_usec_t,
}

/* Pass this to retro_video_refresh_t if rendering to hardware.
 * Passing NULL to retro_video_refresh_t is still a frame dupe as normal.
 * */
pub const RETRO_HW_FRAME_BUFFER_VALID: *const c_void = -1i as *const c_void;

/* Invalidates the current HW context.
 * Any GL state is lost, and must not be deinitialized explicitly.
 * If explicit deinitialization is desired by the libretro core,
 * it should implement context_destroy callback.
 * If called, all GPU resources must be reinitialized.
 * Usually called when frontend reinits video driver.
 * Also called first time video driver is initialized, 
 * allowing libretro core to initialize resources.
 */
#[allow(non_camel_case_types)]
pub type retro_hw_context_reset_t = extern "C" fn();

/* Gets current framebuffer which is to be rendered to.
 * Could change every frame potentially.
 */
#[allow(non_camel_case_types)]
pub type retro_hw_get_current_framebuffer_t = extern "C" fn() -> 
                                               ::libc::types::os::arch::c99::uintptr_t;

/* Get a symbol from HW context. */
#[allow(non_camel_case_types)]
pub type retro_hw_get_proc_address_t = extern "C" fn(sym: *const c_char)
                                        -> retro_proc_address_t;

// enum values for retro_hw_context_type:
pub const RETRO_HW_CONTEXT_NONE:             c_uint = 0;
   /* OpenGL 2.x. Driver can choose to use latest compatibility context. */
pub const RETRO_HW_CONTEXT_OPENGL:           c_uint = 1; 
   /* OpenGL ES 2.0. */
pub const   RETRO_HW_CONTEXT_OPENGLES2:        c_uint = 2;
   /* Modern desktop core GL context. Use version_major/
    * version_minor fields to set GL version. */
pub const   RETRO_HW_CONTEXT_OPENGL_CORE:      c_uint = 3;
   /* OpenGL ES 3.0 */
pub const   RETRO_HW_CONTEXT_OPENGLES3:        c_uint = 4;
   /* OpenGL ES 3.1+. Set version_major/version_minor. For GLES2 and GLES3,
    * use the corresponding enums directly. */
pub const   RETRO_HW_CONTEXT_OPENGLES_VERSION: c_uint = 5;

#[allow(dead_code)]
#[repr(C)]
struct retro_hw_render_callback
{
   /* Which API to use. Set by libretro core. */
   // context_type is one of RETRO_HW_CONTEXT_* consts
   pub context_type: c_int,

   /* Called when a context has been created or when it has been reset.
    * An OpenGL context is only valid after context_reset() has been called.
    *
    * When context_reset is called, OpenGL resources in the libretro 
    * implementation are guaranteed to be invalid.
    *
    * It is possible that context_reset is called multiple times during an 
    * application lifecycle.
    * If context_reset is called without any notification (context_destroy),
    * the OpenGL context was lost and resources should just be recreated
    * without any attempt to "free" old resources.
    */
   pub context_reset: retro_hw_context_reset_t,

   /* Set by frontend. */
   pub get_current_framebuffer: retro_hw_get_current_framebuffer_t,

   /* Set by frontend. */
   pub get_proc_address: retro_hw_get_proc_address_t,

   /* Set if render buffers should have depth component attached. */
   pub depth: u8,

   /* Set if stencil buffers should be attached. */
   pub stencil: u8,

   /* If depth and stencil are true, a packed 24/8 buffer will be added. 
    * Only attaching stencil is invalid and will be ignored. */

   /* Use conventional bottom-left origin convention. If false, 
    * standard libretro top-left origin semantics are used. */
   pub bottom_left_origin: u8,
   
   /* Major version number for core GL context or GLES 3.1+. */
   pub version_major: c_uint,

   /* Minor version number for core GL context or GLES 3.1+. */
   pub version_minor: c_uint,

   /* If this is true, the frontend will go very far to avoid 
    * resetting context in scenarios like toggling fullscreen, etc.
    */
   pub cache_context: u8,

   /* The reset callback might still be called in extreme situations 
    * such as if the context is lost beyond recovery.
    *
    * For optimal stability, set this to false, and allow context to be 
    * reset at any time.
    */
   
   /* A callback to be called before the context is destroyed in a 
    * controlled way by the frontend. */
   pub context_destroy: retro_hw_context_reset_t,

   /* OpenGL resources can be deinitialized cleanly at this step.
    * context_destroy can be set to NULL, in which resources will 
    * just be destroyed without any notification.
    *
    * Even when context_destroy is non-NULL, it is possible that 
    * context_reset is called without any destroy notification.
    * This happens if context is lost by external factors (such as 
    * notified by GL_ARB_robustness).
    *
    * In this case, the context is assumed to be already dead,
    * and the libretro implementation must not try to free any OpenGL 
    * resources in the subsequent context_reset.
    */

   /* Creates a debug context. */
   pub debug_context: u8,
}

/* Callback type passed in RETRO_ENVIRONMENT_SET_KEYBOARD_CALLBACK. 
 * Called by the frontend in response to keyboard events.
 * down is set if the key is being pressed, or false if it is being released.
 * keycode is the RETROK value of the char.
 * character is the text character of the pressed key. (UTF-32).
 * key_modifiers is a set of RETROKMOD values or'ed together.
 *
 * The pressed/keycode state can be indepedent of the character.
 * It is also possible that multiple characters are generated from a 
 * single keypress.
 * Keycode events should be treated separately from character events.
 * However, when possible, the frontend should try to synchronize these.
 * If only a character is posted, keycode should be RETROK_UNKNOWN.
 *
 * Similarily if only a keycode event is generated with no corresponding 
 * character, character should be 0.
 */
#[allow(non_camel_case_types)]
pub type retro_keyboard_event_t = extern "C" fn
                                  (down: u8,
                                   keycode: c_uint,
                                   character: u32,
                                   key_modifiers: u16);

#[allow(dead_code)]
#[repr(C)]
struct retro_keyboard_callback
{
   pub callback: retro_keyboard_event_t,
}

/* Callbacks for RETRO_ENVIRONMENT_SET_DISK_CONTROL_INTERFACE.
 * Should be set for implementations which can swap out multiple disk 
 * images in runtime.
 *
 * If the implementation can do this automatically, it should strive to do so.
 * However, there are cases where the user must manually do so.
 *
 * Overview: To swap a disk image, eject the disk image with 
 * set_eject_state(true).
 * Set the disk index with set_image_index(index). Insert the disk again 
 * with set_eject_state(false).
 */

/* If ejected is true, "ejects" the virtual disk tray.
 * When ejected, the disk image index can be set.
 */

#[allow(non_camel_case_types)]
pub type retro_set_eject_state_t = extern "C" fn(ejected: u8) -> u8;

/* Gets current eject state. The initial state is 'not ejected'. */
#[allow(non_camel_case_types)]
pub type retro_get_eject_state_t = extern "C" fn() -> u8;

/* Gets current disk index. First disk is index 0.
 * If return value is >= get_num_images(), no disk is currently inserted.
 */
#[allow(non_camel_case_types)]
pub type retro_get_image_index_t = extern "C" fn() -> c_uint;

/* Sets image index. Can only be called when disk is ejected.
 * The implementation supports setting "no disk" by using an 
 * index >= get_num_images().
 */
#[allow(non_camel_case_types)]
pub type retro_set_image_index_t = extern "C" fn(index: c_uint) -> u8;

/* Gets total number of images which are available to use. */
#[allow(non_camel_case_types)]
pub type retro_get_num_images_t = extern "C" fn() -> c_uint;

/* Replaces the disk image associated with index.
 * Arguments to pass in info have same requirements as retro_load_game().
 * Virtual disk tray must be ejected when calling this.
 *
 * Replacing a disk image with info = NULL will remove the disk image 
 * from the internal list.
 * As a result, calls to get_image_index() can change.
 *
 * E.g. replace_image_index(1, NULL), and previous get_image_index() 
 * returned 4 before.
 * Index 1 will be removed, and the new index is 3.
 */
#[allow(non_camel_case_types)]
pub type retro_replace_image_index_t = extern "C" fn
                                       (index: c_uint,
                                        info: *const retro_game_info) -> u8;

/* Adds a new valid index (get_num_images()) to the internal disk list.
 * This will increment subsequent return values from get_num_images() by 1.
 * This image index cannot be used until a disk image has been set 
 * with replace_image_index. */
#[allow(non_camel_case_types)]
pub type retro_add_image_index_t = extern "C" fn() -> u8;

#[allow(dead_code)]
#[repr(C)]
pub struct retro_disk_control_callback
{
   pub set_eject_state: retro_set_eject_state_t,
   pub get_eject_state: retro_get_eject_state_t,

   pub get_image_index: retro_get_image_index_t,
   pub set_image_index: retro_set_image_index_t,
   pub get_num_images: retro_get_num_images_t,

   pub replace_image_index: retro_replace_image_index_t,
   pub add_image_index: retro_add_image_index_t,
}

// values from enum retro_pixel_format
/* 0RGB1555, native endian.
* 0 bit must be set to 0.
* This pixel format is default for compatibility concerns only.
* If a 15/16-bit pixel format is desired, consider using RGB565. */
pub const RETRO_PIXEL_FORMAT_0RGB1555: c_uint = 0;

/* XRGB8888, native endian.
* X bits are ignored. */
pub const RETRO_PIXEL_FORMAT_XRGB8888: c_uint = 1;

/* RGB565, native endian.
* This pixel format is the recommended format to use if a 15/16-bit
* format is desired as it is the pixel format that is typically 
* available on a wide range of low-power devices.
*
* It is also natively supported in APIs like OpenGL ES. */
pub const RETRO_PIXEL_FORMAT_RGB565: c_uint = 2;

#[allow(dead_code)]
#[repr(C)]
pub struct retro_message
{
   pub msg: *const c_char,  /* Message to be displayed. */
   pub frames: c_uint,      /* Duration in frames of message. */
}

/* Describes how the libretro implementation maps a libretro input bind
 * to its internal input system through a human readable string.
 * This string can be used to better let a user configure input. */
#[allow(dead_code)]
#[repr(C)]
pub struct retro_input_descriptor
{
   /* Associates given parameters with a description. */
   pub port: c_uint,
   pub device: c_uint,
   pub index: c_uint,
   pub id: c_uint,

   /* Human readable description for parameters.
    * The pointer must remain valid until
    * retro_unload_game() is called. */
   pub description: *const c_char,
}

#[allow(dead_code)]
#[repr(C)]
pub struct retro_system_info
{
   /* All pointers are owned by libretro implementation, and pointers must 
    * remain valid until retro_deinit() is called. */

   pub library_name: *const c_char,     /* Descriptive name of library. Should not 
                                                   * contain any version numbers, etc. */
   pub library_version: *const c_char,  /* Descriptive version of core. */

   pub valid_extensions: *const c_char, /* A string listing probably content 
                                                  * extensions the core will be able to 
                                                  * load, separated with pipe.
                                                  * I.e. "bin|rom|iso".
                                                  * Typically used for a GUI to filter 
                                                  * out extensions. */

   /* If true, retro_load_game() is guaranteed to provide a valid pathname 
    * in retro_game_info::path.
    * ::data and ::size are both invalid.
    *
    * If false, ::data and ::size are guaranteed to be valid, but ::path 
    * might not be valid.
    *
    * This is typically set to true for libretro implementations that must 
    * load from file.
    * Implementations should strive for setting this to false, as it allows 
    * the frontend to perform patching, etc. */
   pub need_fullpath: u8,                      

   /* If true, the frontend is not allowed to extract any archives before 
    * loading the real content.
    * Necessary for certain libretro implementations that load games 
    * from zipped archives. */
   pub block_extract: u8,
}

#[allow(dead_code)]
#[repr(C)]
pub struct retro_game_geometry
{
   pub base_width:  c_uint,   /* Nominal video width of game. */
   pub base_height: c_uint,   /* Nominal video height of game. */
   pub max_width:   c_uint,   /* Maximum possible width of game. */
   pub max_height:  c_uint,   /* Maximum possible height of game. */

   pub aspect_ratio: f32,  /* Nominal aspect ratio of game. If
                            * aspect_ratio is <= 0.0, an aspect ratio
                            * of base_width / base_height is assumed.
                            * A frontend could override this setting,
                            * if desired. */
}

#[allow(dead_code)]
#[repr(C)]
pub struct retro_system_timing
{
   pub fps: f64,             /* FPS of video content. */
   pub sample_rate: f64,     /* Sampling rate of audio. */
}

#[allow(dead_code)]
#[repr(C)]
pub struct retro_system_av_info
{
   pub geometry: retro_game_geometry,
   pub timing: retro_system_timing,
}

#[allow(dead_code)]
#[repr(C)]
pub struct retro_variable
{
   /* Variable to query in RETRO_ENVIRONMENT_GET_VARIABLE.
    * If NULL, obtains the complete environment string if more 
    * complex parsing is necessary.
    * The environment string is formatted as key-value pairs 
    * delimited by semicolons as so:
    * "key1=value1;key2=value2;..."
    */
   pub key: *const c_char,
   
   /* Value to be obtained. If key does not exist, it is set to NULL. */
   pub value: *const c_char,
}

#[allow(dead_code)]
#[repr(C)]
pub struct retro_game_info
{
   pub path: *const c_char,     /* Path to game, UTF-8 encoded.
                                  * Usually used as a reference.
                                  * May be NULL if rom was loaded from stdin
                                  * or similar. 
                                  * retro_system_info::need_fullpath guaranteed 
                                  * that this path is valid. */
   pub data: *const c_char,     /* Memory buffer of loaded game. Will be NULL 
                                  * if need_fullpath was set. */
   pub size: size_t,             /* Size of memory buffer. */
   pub meta: *const c_char,     /* String of implementation specific meta-data. */
}

/* Callbacks */

/* Environment callback. Gives implementations a way of performing 
 * uncommon tasks. Extensible. */
#[allow(non_camel_case_types)]
pub type retro_environment_t = extern "C" fn (cmd: c_uint,
                                                data: *mut c_void) -> u8;

/* Render a frame. Pixel format is 15-bit 0RGB1555 native endian 
 * unless changed (see RETRO_ENVIRONMENT_SET_PIXEL_FORMAT).
 *
 * Width and height specify dimensions of buffer.
 * Pitch specifices length in bytes between two lines in buffer.
 *
 * For performance reasons, it is highly recommended to have a frame 
 * that is packed in memory, i.e. pitch == width * byte_per_pixel.
 * Certain graphic APIs, such as OpenGL ES, do not like textures 
 * that are not packed in memory.
 */
#[allow(non_camel_case_types)]
pub type retro_video_refresh_t = extern "C" fn(data: *const c_void,
                                                 width: c_uint,
                                                 height: c_uint,
                                                 pitch: size_t);

/* Renders a single audio frame. Should only be used if implementation 
 * generates a single sample at a time.
 * Format is signed 16-bit native endian.
 */
#[allow(non_camel_case_types)]
pub type retro_audio_sample_t = extern "C" fn(left: i16, right: i16);

/* Renders multiple audio frames in one go.
 *
 * One frame is defined as a sample of left and right channels, interleaved.
 * I.e. int16_t buf[4] = { l, r, l, r }; would be 2 frames.
 * Only one of the audio callbacks must ever be used.
 */
#[allow(non_camel_case_types)]
pub type retro_audio_sample_batch_t = extern "C" fn(data: *const i16,
                                                     frames: size_t)
                                                     -> size_t;

/* Polls input. */
#[allow(non_camel_case_types)]
pub type retro_input_poll_t = extern "C" fn();

/* Queries for input for player 'port'. device will be masked with 
 * RETRO_DEVICE_MASK.
 *
 * Specialization of devices such as RETRO_DEVICE_JOYPAD_MULTITAP that 
 * have been set with retro_set_controller_port_device()
 * will still use the higher level RETRO_DEVICE_JOYPAD to request input.
 */
#[allow(non_camel_case_types)]
pub type retro_input_state_t = extern "C" fn(port: c_uint,
                                              device: c_uint,
                                              index: c_uint,
                                              id: c_uint)
                                              -> i16;

/* Rust does not use function prototypes. Original C prototypes are
 * included for reference. */
/* Sets callbacks. retro_set_environment() is guaranteed to be called 
 * before retro_init().
 *
 * The rest of the set_* functions are guaranteed to have been called 
 * before the first call to retro_run() is made. */
// void retro_set_environment(retro_environment_t);
// void retro_set_video_refresh(retro_video_refresh_t);
// void retro_set_audio_sample(retro_audio_sample_t);
// void retro_set_audio_sample_batch(retro_audio_sample_batch_t);
// void retro_set_input_poll(retro_input_poll_t);
// void retro_set_input_state(retro_input_state_t);

/* Library global initialization/deinitialization. */
// void retro_init(void);
// void retro_deinit(void);

/* Must return RETRO_API_VERSION. Used to validate ABI compatibility
 * when the API is revised. */
// unsigned retro_api_version(void);

/* Gets statically known system info. Pointers provided in *info 
 * must be statically allocated.
 * Can be called at any time, even before retro_init(). */
// void retro_get_system_info(struct retro_system_info *info);

/* Gets information about system audio/video timings and geometry.
 * Can be called only after retro_load_game() has successfully completed.
 * NOTE: The implementation of this function might not initialize every 
 * variable if needed.
 * E.g. geom.aspect_ratio might not be initialized if core doesn't 
 * desire a particular aspect ratio. */
// void retro_get_system_av_info(struct retro_system_av_info *info);

/* Sets device to be used for player 'port'.
 * By default, RETRO_DEVICE_JOYPAD is assumed to be plugged into all 
 * available ports.
 * Setting a particular device type is not a guarantee that libretro cores 
 * will only poll input based on that particular device type. It is only a 
 * hint to the libretro core when a core cannot automatically detect the 
 * appropriate input device type on its own. It is also relevant when a 
 * core can change its behavior depending on device type. */
// void retro_set_controller_port_device(unsigned port, unsigned device);

/* Resets the current game. */
// void retro_reset(void);

/* Runs the game for one video frame.
 * During retro_run(), input_poll callback must be called at least once.
 * 
 * If a frame is not rendered for reasons where a game "dropped" a frame,
 * this still counts as a frame, and retro_run() should explicitly dupe 
 * a frame if GET_CAN_DUPE returns true.
 * In this case, the video callback can take a NULL argument for data.
 */
// void retro_run(void);

/* Returns the amount of data the implementation requires to serialize 
 * internal state (save states).
 * Between calls to retro_load_game() and retro_unload_game(), the 
 * returned size is never allowed to be larger than a previous returned 
 * value, to ensure that the frontend can allocate a save state buffer once.
 */
// size_t retro_serialize_size(void);

/* Serializes internal state. If failed, or size is lower than
 * retro_serialize_size(), it should return false, true otherwise. */
// bool retro_serialize(void *data, size_t size);
// bool retro_unserialize(const void *data, size_t size);

// void retro_cheat_reset(void);
// void retro_cheat_set(unsigned index, bool enabled, const char *code);

/* Loads a game. */
// bool retro_load_game(const struct retro_game_info *game);

/* Loads a "special" kind of game. Should not be used,
 * except in extreme cases. */
// bool retro_load_game_special(
//   unsigned game_type,
//   const struct retro_game_info *info, size_t num_info
// );

/* Unloads a currently loaded game. */
// void retro_unload_game(void);

/* Gets region of game. */
// unsigned retro_get_region(void);

/* Gets region of memory. */
// void *retro_get_memory_data(unsigned id);
// size_t retro_get_memory_size(unsigned id);
