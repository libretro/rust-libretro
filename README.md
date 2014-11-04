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

Installation
============

rust-libretro cannot easily be distributed as a Cargo dependency because it is
a library for producing libraries. The rust-libretro code needs to access symbols
in your code, and this is not possible across crate boundaries. Additionally,
libretro does not define a guaranteed first entry point for the core, so you
cannot define that entry point in your own code and pass configuration data to
rust-libretro from there.

Future versions of Cargo should make this easier, but for now the simplest
solution is to clone this repo and make your updates directly. 

You can then merge updated versions of only the files in rust_wrapper:

Set up a remote to upstream if you haven't already done so:

`$ git remote add upstream https://github.com/mprobinson/rust-libretro.git`

Fetch changes from upstream:

`$ git fetch upstream`

Tell git to prepare for a partial merge:

`$ git merge --no-ff --no-commit -s ours upstream/master`

Checkout only the changes to files in src/rust_wrapper:

`$ git checkout upstream/master src/rust_wrapper/*`

Merge those changes:

`$ git commit`


Compilation
===========

Rust currently sets all library symbols visible, resulting is very bloated
libraries. If you are linking with GNU ld it is possible to restrict the visible
symbols to those required by the libretro API, using a version script. Cargo
does not support custom rustc parameters, so first run cargo with --verbose to
discover the cargo parameters. Then run rustc manually, passing the version
script in src/visible_symbols.script by appending:

`-C link-args="-Wl,-version-script=src/visible_symbols.script -Wl,-gc-sections"`

This will result in a much more reasonably sized core. The core size may be
futher reduced without loss of function by running `strip -s`.