//! ARTIQ Exception struct declaration.

use core::{marker::PhantomData, slice, str};

/// Exception string value representation.
///
/// This can be a string literal or a string key on the host.
#[derive(Copy, Clone)]
pub enum ExceptionStrValue<'a> {
    HostKey(u32),
    String(Result<&'a str, str::Utf8Error>),
}

/// Exception string representation.
///
/// This can be a string literal or a string key on the host.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExceptionStr<'a> {
    value: usize,
    len: u32,
    s: PhantomData<&'a str>,
}

impl<'a> ExceptionStr<'a> {
    /// Returns the string value.
    pub fn value(&self) -> ExceptionStrValue<'a> {
        if self.len == u32::MAX {
            ExceptionStrValue::HostKey(self.value as u32)
        } else {
            // SAFETY: Length is not `u32::MAX`, so it is a string.
            let s = unsafe { slice::from_raw_parts(self.value as *const u8, self.len as usize) };
            ExceptionStrValue::String(str::from_utf8(s.as_ref()))
        }
    }

    /// Returns the string value, if this is a string.
    pub fn as_str(&self) -> Option<Result<&'a str, core::str::Utf8Error>> {
        match self.value() {
            ExceptionStrValue::String(s) => Some(s),
            _ => None,
        }
    }
}

impl<'a> From<u32> for ExceptionStr<'a> {
    fn from(key: u32) -> Self {
        ExceptionStr {
            value: key as usize,
            len: u32::MAX,
            s: PhantomData,
        }
    }
}

impl<'a> From<&'a str> for ExceptionStr<'a> {
    fn from(s: &'a str) -> Self {
        ExceptionStr {
            value: s.as_ptr() as usize,
            len: s.len() as u32,
            s: PhantomData,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Exception<'a> {
    pub id: u32,
    pub file: ExceptionStr<'a>,
    pub line: u32,
    pub column: u32,
    pub function: ExceptionStr<'a>,
    pub message: ExceptionStr<'a>,
    pub param: [i64; 3],
}

fn fmt_exception_str<'a>(s: ExceptionStr<'a>) -> Result<&'a str, core::fmt::Error> {
    match s.value() {
        ExceptionStrValue::HostKey(_) => Ok("<host string>"),
        ExceptionStrValue::String(s) => s.map_err(|_| core::fmt::Error),
    }
}

impl<'a> core::fmt::Debug for Exception<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Exception {} from {} in {}:{}:{}, message: {}",
            self.id,
            fmt_exception_str(self.function)?,
            fmt_exception_str(self.file)?,
            self.line,
            self.column,
            fmt_exception_str(self.message)?,
        )
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct StackPointerBacktrace {
    pub stack_pointer: usize,
    pub initial_backtrace_size: usize,
    pub current_backtrace_size: usize,
}
