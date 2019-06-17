// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

// See https://github.com/opensource-apple/objc4/blob/master/runtime/objc-exception.h

extern crate libc;

use crate::arc;
use crate::runtime::objc;

#[link(name = "objc")]
extern "C" {
  #[unwind(allowed)]
  pub fn objc_exception_throw(exception: arc::Auto<objc::Id>) -> !;
}
