use std::error::Error as StdError;
use std::ffi::{c_char, CStr, CString};
use std::fmt;

use thiserror::Error as ThisError;

// https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/private/defs.h#L64-L67
//
/// Binding to the error type used by Neovim.
#[derive(Clone, ThisError, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Error {
    r#type: ErrorType,
    msg: *mut c_char,
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

// https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/private/defs.h#L27-L31
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
enum ErrorType {
    None = -1,
    Exception,
    #[allow(dead_code)]
    Validation,
}

impl Error {
    pub const fn new() -> Self {
        Self { r#type: ErrorType::None, msg: std::ptr::null_mut() }
    }
}

impl Default for Error {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.msg.is_null() {
            fmt::Debug::fmt(unsafe { CStr::from_ptr(self.msg) }, f)
        } else {
            use ErrorType::*;
            let msg = match self.r#type {
                None => return Ok(()),
                Exception => "exception",
                Validation => "validation",
            };
            write!(f, "{}", msg)
        }
    }
}

impl Error {
    pub(crate) fn from_str<S: Into<String>>(s: S) -> Self {
        let c_string = CString::new(s.into()).unwrap_or_default();
        let ptr = c_string.into_raw() /* TODO: memory leak */;
        Self { r#type: ErrorType::Exception, msg: ptr }
    }

    pub fn from_err<E: StdError>(err: E) -> Self {
        Self::from_str(err.to_string())
    }

    pub fn is_err(&self) -> bool {
        !matches!(self.r#type, ErrorType::None)
    }
}
