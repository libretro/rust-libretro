hellorust-libretro
==================

Test libretro-core in Rust

Pre-alpha WIP, do not use this as a reference for good code.

License: GPL3+, with the exception of src/libretro.rs which is MIT

Thread Safety Requirements
==========================
This software is intended soley for producing libretro core libraries, to be
called from a correctly implemented libretro frontend. If you decide to call
any such library from outside its specified environment then you must ensure
thread safety. All calls to the library, with the exception of a call to
retro_audio_callback, must be made from the same thread. Failure to follow this
restriction may result in undefined behavior.