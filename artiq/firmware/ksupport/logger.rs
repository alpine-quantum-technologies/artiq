use crate::send;
use core::fmt;
use cstr_core::CStr;
use kernel_proto::*;

#[macro_export]
macro_rules! core_log {
    ($level:expr, $($arg:tt)*) => {{
	$crate::logger::core_log_impl__($level, core::format_args!($($arg)*));
    }};
}

#[doc(hidden)]
pub fn core_log_impl__(level: log::Level, args: fmt::Arguments<'_>) {
    send(&Log { level, args });
}

pub fn core_log_api(level: log::Level, message: *const u8) {
    if message.is_null() {
        return;
    }

    let message = unsafe { CStr::from_ptr(message) };
    match message.to_str() {
        Ok(message) => core_log_impl__(level, core::format_args!("{}", message)),
        Err(error) => {
            core_log_impl__(
                level,
                core::format_args!("{}", unsafe {
                    core::str::from_utf8_unchecked(&message.to_bytes()[..error.valid_up_to()])
                }),
            );
            core_log_impl__(log::Level::Warn, core::format_args!("(invalid utf-8)"))
        }
    };
}
