// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

// See https://github.com/opensource-apple/objc4/blob/master/runtime/objc-internal.h
// and https://clang.llvm.org/docs/AutomaticReferenceCounting.html

extern crate core;
extern crate libc;

use crate::arc;
use crate::runtime::objc;

#[link(name = "objc")]
extern "C" {
  pub fn objc_autoreleasePoolPush() -> *mut libc::c_void;
  pub fn objc_autoreleasePoolPop(pool: *mut libc::c_void);

  pub fn objc_retain(obj: *mut objc::Id) -> Option<arc::Strong<objc::Id>>;
  pub fn objc_release(obj: Option<arc::Strong<objc::Id>>);
  pub fn objc_autorelease(obj: Option<arc::Strong<objc::Id>>) -> Option<arc::Auto<objc::Id>>;
  pub fn objc_retainAutorelease(obj: *mut objc::Id) -> Option<arc::Auto<objc::Id>>;

  pub fn objc_initWeak(weak: &mut *mut objc::Id, value: *mut objc::Id) -> *mut objc::Id;
  pub fn objc_destroyWeak(weak: &mut *mut objc::Id);
  pub fn objc_loadWeak(weak: &mut *mut objc::Id) -> Option<arc::Auto<objc::Id>>;
  pub fn objc_loadWeakRetained(weak: &mut *mut objc::Id) -> Option<arc::Strong<objc::Id>>;
}
