// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

// See https://github.com/opensource-apple/objc4/blob/master/runtime/message.h

use objc;

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
  pub receiver: objc::id,
  pub super_class: objc::Class,
}
