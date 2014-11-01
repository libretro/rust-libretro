extern crate core;
use core::prelude::*;
use core::fmt::FormatError;
use core::fmt::FormatWriter;
use super::retro_log_panic;

#[lang = "stack_exhausted"]
extern fn stack_exhausted()
{
    unsafe {
        core::intrinsics::abort();
    }
}

#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt(args: &core::fmt::Arguments,
                    file: &str,
                    line: uint) -> !
{
    struct PanicWriter
    {
        buffer: [u8, ..1024],
        offset: uint
    }
    
    impl core::fmt::FormatWriter for PanicWriter
    {
        fn write(&mut self, bytes: &[u8]) -> Result<(), core::fmt::FormatError>
        {
            let buf_len = self.buffer.len();
            let partial_buf = self.buffer.slice_mut(self.offset, buf_len);
            core::slice::bytes::copy_memory(partial_buf, bytes);
            self.offset = self.offset + bytes.len();
            Ok(())
        }
    }

    let mut panic_writer = PanicWriter {buffer: [0u8, ..1024], offset: 0};
    let _ = write!(&mut panic_writer, "{}", args);

    let panic_str = core::str::from_utf8(panic_writer.buffer);
    retro_log_panic(panic_str.unwrap(), file, line);
    unsafe {
        core::intrinsics::abort();
    }
}
