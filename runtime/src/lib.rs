// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

#![feature(global_asm, proc_macro, repr_transparent, untagged_unions, used)]
#![no_std]

mod message;
mod objc;
mod objc_abi;
mod objc_exception;
mod objc_internal;
mod objc_runtime_new;
mod runtime;

pub use message::*;
pub use objc::*;
pub use objc_abi::*;
pub use objc_exception::*;
pub use objc_internal::*;
pub use objc_runtime_new::*;
pub use runtime::*;

extern crate libc;

// pub type c_void = libc::c_void
// pub type c_char = libc::c_char
// pub type c_long_double =
// pub type c_float_complex =
// pub type c_double_complex =
// pub type c_long_double_complex =

#[doc(hidden)]
pub mod __objrs {
  pub trait Class {
    const CLASS_NAME: &'static str;
    const CLASS_NAME_CSTR: &'static str;

    // type CLASS_NAME_TYPE;  // [u8; N];
    // type CLASS_NAME_CSTR_TYPE;  // [u8; N];
    // const CLASS_NAME: CLASS_NAME_TYPE;
    // const CLASS_NAME_CSTR: CLASS_NAME_CSTR_TYPE;
    // const IS_ROOT_CLASS: bool;
  }

  pub trait RootClass {}

  pub trait NonRootClass {
    type Super: Class + ?Sized;
  }
}
