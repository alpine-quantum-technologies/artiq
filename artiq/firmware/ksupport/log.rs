use arrayvec::ArrayString;
use core::fmt::{self, Write};
use cstr_core::CStr;

#[macro_export]
macro_rules! core_log {
    ($($arg:tt)*) => {{
	$crate::log::core_log_impl__(core::format_args!($($arg)*));
    }};
}

mod glue {
    extern "C" {
        pub fn core_log(fmt: *const u8, args: ...);
    }
}

const LOG_BUFFER_SIZE: usize = 500;

#[doc(hidden)]
pub fn core_log_impl__(args: fmt::Arguments<'_>) {
    let mut buf = ArrayString::<LOG_BUFFER_SIZE>::new();
    let bytes = match fmt::write(&mut buf, args).map(|_| core::write!(&mut buf, "\n\0")) {
        Ok(_) => buf.as_bytes(),
        Err(_) => b"(core_log buffer overflow)\n\0",
    };

    let cstr = CStr::from_bytes_with_nul(bytes).unwrap();
    unsafe { glue::core_log(cstr.as_ptr()) }
}
