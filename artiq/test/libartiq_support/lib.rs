#![feature(libc, panic_unwind, rustc_private, c_unwind)]
#![crate_name = "artiq_support"]
#![crate_type = "cdylib"]

extern crate libc;
extern crate std as core;
extern crate unwind;

// Note: this does *not* match the cslice crate!
// ARTIQ Python has the slice length field fixed at 32 bits, even on 64-bit platforms.
mod cslice {
    use core::marker::PhantomData;

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct CSlice<'a, T> {
        base: *const T,
        len: u32,
        marker: PhantomData<&'a ()>,
    }
}

#[path = "."]
pub mod eh {
    #[path = "../../firmware/libeh/dwarf.rs"]
    pub mod dwarf;
    #[path = "../../firmware/libeh/eh_artiq.rs"]
    pub mod eh_artiq;
}
#[path = "../../firmware/ksupport/eh_artiq.rs"]
pub mod eh_artiq;

use std::process;

fn terminate(
    exceptions: &'static [Option<eh_artiq::Exception<'static>>],
    _stack_pointers: &'static [eh_artiq::StackPointerBacktrace],
    _backtrace: &'static mut [(usize, usize)],
) -> ! {
    println!("{}", exceptions.len());
    for exception in exceptions.iter() {
        let exception = exception.as_ref().unwrap();
        println!(
            "Uncaught {}: {} ({}, {}, {})",
            exception.id,
            exception.message.as_str().unwrap().unwrap(),
            exception.param[0],
            exception.param[1],
            exception.param[2]
        );
        println!(
            "at {}:{}:{}",
            exception.file.as_str().unwrap().unwrap(),
            exception.line,
            exception.column
        );
    }
    process::exit(1);
}

#[export_name = "now"]
pub static mut NOW: i64 = 0;
