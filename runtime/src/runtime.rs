// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

// See https://github.com/opensource-apple/objc4/blob/master/runtime/runtime.h

// extern crate core;
extern crate libc;

use objc;

#[link(name = "objc")]
extern "C" {
  pub fn objc_getClass(name: *const libc::c_char) -> objc::Class;

  pub fn class_getSuperclass(name: objc::Class) -> objc::Class;
  pub fn class_respondsToSelector(class: objc::Class, sel: objc::SEL) -> bool;
  pub fn class_getClassMethod(class: objc::Class, sel: objc::SEL) -> objc::Method;
  pub fn class_getInstanceMethod(class: objc::Class, sel: objc::SEL) -> objc::Method;

  pub fn method_getNumberOfArguments(method: objc::Method) -> libc::c_uint;
  pub fn method_getTypeEncoding(method: objc::Method) -> *const libc::c_char;

  pub fn method_copyReturnType(method: objc::Method) -> *mut libc::c_char;
  pub fn method_getReturnType(method: objc::Method, dst: *mut libc::c_char, dst_len: usize);

  pub fn method_copyArgumentType(method: objc::Method, index: libc::c_uint) -> *mut libc::c_char;
  pub fn method_getArgumentType(
    method: objc::Method,
    index: libc::c_uint,
    dst: *mut libc::c_char,
    dst_len: usize,
  );

  pub fn sel_registerName(name: *const libc::c_char) -> objc::SEL;
  pub fn sel_getName(sel: objc::SEL) -> *const libc::c_char;
}
