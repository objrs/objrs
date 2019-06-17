// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

mod message;
mod objc;
mod objc_abi;
mod objc_exception;
mod objc_internal;
mod objc_runtime_new;
mod runtime;

pub use crate::runtime::message::*;
pub use crate::runtime::objc::*;
pub use crate::runtime::objc_abi::*;
pub use crate::runtime::objc_exception::*;
pub use crate::runtime::objc_internal::*;
pub use crate::runtime::objc_runtime_new::*;
pub use crate::runtime::runtime::*;

#[doc(hidden)]
pub mod __objrs {
  pub unsafe trait Class {
    const CLASS_NAME: &'static str;
    const CLASS_NAME_CSTR: &'static str;

    const HAS_IVARS: bool;
    const IS_ROOT_CLASS: bool;
    const REQUIRES_CXX_CONSTRUCT: bool;
    const REQUIRES_CXX_DESTRUCT: bool;

    type FIELDS;

    const INSTANCE_START: usize;
    const INSTANCE_SIZE: usize;

    // type CLASS_NAME_TYPE;  // [u8; N];
    // type CLASS_NAME_CSTR_TYPE;  // [u8; N];
    // const CLASS_NAME: CLASS_NAME_TYPE;
    // const CLASS_NAME_CSTR: CLASS_NAME_CSTR_TYPE;

    extern "C" fn cxx_construct(this: *mut Self, _: usize) -> *mut Self;
    extern "C" fn cxx_destruct(this: *mut Self, _: usize);
  }

  pub unsafe trait RootClass: Class {}

  pub unsafe trait NonRootClass: Class {
    type Super: Class + ?Sized;
  }
}
