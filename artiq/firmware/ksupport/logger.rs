use arrayvec::ArrayString;
use core::fmt;
use cslice::{AsCSlice, CSlice};
use cstr_core::CStr;
use kernel_proto::*;

const HOST_LOG_SERVICE_ID: u32 = 2;
const HOST_LOG_BUFFER_SIZE: usize = 500;

#[macro_export]
macro_rules! core_log {
    ($level:expr, $($arg:tt)*) => {{
	$crate::logger::core_log_impl__($level, core::format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! host_log {
    ($level:expr, $($arg:tt)*) => {{
	$crate::logger::host_log_impl__($level, core::format_args!($($arg)*));
    }};
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

pub fn host_log_api(level: log::Level, message: *const u8) {
    if message.is_null() {
        return;
    }

    let message = unsafe { CStr::from_ptr(message) };
    match message.to_str() {
        Ok(message) => host_log_impl__(level, core::format_args!("{}", message)),
        Err(error) => {
            host_log_impl__(
                level,
                core::format_args!("{}", unsafe {
                    core::str::from_utf8_unchecked(&message.to_bytes()[..error.valid_up_to()])
                }),
            );
            host_log_impl__(log::Level::Warn, core::format_args!("(invalid utf-8)"))
        }
    };
}

#[doc(hidden)]
pub fn core_log_impl__(level: log::Level, args: fmt::Arguments<'_>) {
    crate::send(&Log { level, args });
}

#[doc(hidden)]
pub fn host_log_impl__(level: log::Level, args: fmt::Arguments<'_>) {
    let mut buf = ArrayString::<HOST_LOG_BUFFER_SIZE>::new();
    let bytes = match fmt::write(&mut buf, args) {
        Ok(_) => buf.as_bytes(),
        Err(_) => b"(host log buffer overflow)",
    };

    let tag = &(b"is:n")[..];
    let args = [
        &(level as usize) as *const usize as *const (),
        &(bytes.as_c_slice()) as *const CSlice<u8> as *const (),
    ];
    crate::rpc_send_async(HOST_LOG_SERVICE_ID, &tag.as_c_slice(), args.as_ptr());
}
