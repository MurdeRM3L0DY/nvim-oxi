//! Rust bindings to the C types used by Neovim's API.

mod array;
pub mod conversion;
mod dictionary;
mod error;
mod function;
mod kvec;
mod non_owning;
mod object;
#[cfg(feature = "serde")]
pub mod serde;
mod string;

pub use array::Array;
pub use dictionary::Dictionary;
pub use error::Error;
pub use function::Function;
pub use non_owning::NonOwning;
pub use object::{Object, ObjectKind};
pub use string::String;

pub mod iter {
    //! Iterators over [`Array`](crate::Array)s and
    //! [`Dictionary`](crate::Dictionary)s.

    pub use super::array::ArrayIterator;
    pub use super::dictionary::{DictIter, DictIterMut, DictIterator};
}

// https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/private/defs.h#L69
#[doc(hidden)]
pub type Boolean = bool;

// https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/private/defs.h#L70
#[doc(hidden)]
pub type Integer = i64;

// https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/private/defs.h#L71
#[doc(hidden)]
pub type Float = core::ffi::c_double;

// https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/types.h#L19
#[doc(hidden)]
pub type LuaRef = core::ffi::c_int;

// https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/types.h#L14
#[allow(non_camel_case_types)]
type handle_T = core::ffi::c_int;

// https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/private/defs.h#L84
#[doc(hidden)]
pub type BufHandle = handle_T;

// https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/private/defs.h#L85
#[doc(hidden)]
pub type WinHandle = handle_T;

// https://github.com/neovim/neovim/blob/v0.9.0/src/nvim/api/private/defs.h#L86
#[doc(hidden)]
pub type TabHandle = handle_T;
