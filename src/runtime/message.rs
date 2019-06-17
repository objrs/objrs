// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

// See https://github.com/opensource-apple/objc4/blob/master/runtime/message.h

use crate::runtime::objc;

// TODO: these need #[unwind] since FFI code may throw an Objective-C exception. We probably need
// #[unwind] on the Rust (msg_recv) method implementations too. Make sure that the casted function
// pointers also allow unwinding.
#[link(name = "objc")]
extern "C" {
  pub fn objc_msgSend();
  pub fn objc_msgSendSuper();
  #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
  pub fn objc_msgSend_fpret();
  #[cfg(target_arch = "x86_64")]
  pub fn objc_msgSend_fp2ret();
  #[cfg(not(target_arch = "aarch64"))]
  pub fn objc_msgSend_stret();
  #[cfg(not(target_arch = "aarch64"))]
  pub fn objc_msgSendSuper_stret();
  pub fn method_invoke();
  #[cfg(not(target_arch = "aarch64"))]
  pub fn method_invoke_stret();
  pub fn _objc_msgForward();
  #[cfg(not(target_arch = "aarch64"))]
  pub fn _objc_msgForward_stret();
}

#[repr(C)]
pub struct objc_super {
  pub receiver: core::ptr::NonNull<objc::Id>,
  pub super_class: core::ptr::NonNull<objc::Class>,
}
