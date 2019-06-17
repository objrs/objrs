// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

// See https://github.com/opensource-apple/objc4/blob/master/runtime/runtime.h

// extern crate core;
extern crate libc;

use crate::runtime::objc;
use crate::CStr;
use core::ptr::NonNull;

#[link(name = "objc")]
extern "C" {
  pub fn objc_getClass(name: &CStr) -> *mut objc::Class;

  pub fn class_getName(class: *const objc::Class) -> Option<&'static CStr>;
  pub fn class_getSuperclass(name: *mut objc::Class) -> *mut objc::Class;
  pub fn class_respondsToSelector(class: *const objc::Class, sel: &objc::Sel) -> bool;
  pub fn class_getClassMethod(class: *mut objc::Class, sel: &objc::Sel) -> *mut objc::Method;
  pub fn class_getInstanceMethod(class: *mut objc::Class, sel: &objc::Sel) -> *mut objc::Method;

  pub fn object_getClass(object: *mut objc::Id) -> *mut objc::Class;
  pub fn object_isClass(object: *const objc::Id) -> bool;

  pub fn method_getNumberOfArguments(method: NonNull<objc::Method>) -> libc::c_uint;
  pub fn method_getTypeEncoding(method: NonNull<objc::Method>) -> Option<&'static CStr>;

  pub fn method_copyReturnType(method: NonNull<objc::Method>) -> NonNull<CStr>;
  pub fn method_getReturnType(method: NonNull<objc::Method>, dst: NonNull<CStr>, dst_len: usize);

  pub fn method_copyArgumentType(
    method: NonNull<objc::Method>,
    index: libc::c_uint,
  ) -> NonNull<CStr>;
  pub fn method_getArgumentType(
    method: NonNull<objc::Method>,
    index: libc::c_uint,
    dst: NonNull<CStr>,
    dst_len: usize,
  );
}
