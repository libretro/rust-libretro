rust-libretro
=============

WIP libretro wrapper for Rust.

License: MIT

Thread Safety Requirements
==========================
This software is intended soley for producing libretro core libraries, to be
called from a correctly implemented libretro frontend. If you decide to call
any such library from outside its specified environment then you must ensure
thread safety. All calls to the library, with the exception of a call to
retro_audio_callback, must be made from the same thread. Failure to follow this
restriction may result in undefined behavior.

This restriction may be lifted in future versions if I can figure out how to
do it without significantly harming performance.
