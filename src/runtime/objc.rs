// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

// See https://github.com/opensource-apple/objc4/blob/master/runtime/objc.h

extern crate core;
extern crate libc;

use crate::CStr;

extern "C" {
  type objc_object;
  type objc_selector;
  type objc_method;

  // This type is just for `Imp`. Alternatively, `unsafe extern "C" fn()` could be used, but Rust's
  // function pointers are nonnullable, and I'm concerned that using it for `Imp` would lead to
  // accidental usage of a nonnullable type when a nullable type is required. Using an extern type
  // avoids those accidents (since the pointer notation (`&` or `*`) is explicit, not implicit, and
  // clearly communicates what is and is not nullable).
  type opaque_function;
}

// TODO: implement the following traits:
// std::panic::UnwindSafe
// core::ops::CoerceUnsized

// This is broken by https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md (will
// be fixed by https://github.com/rust-lang/rfcs/blob/master/text/2027-object_safe_for_dispatch.md).
#[repr(transparent)]
pub struct Id<T = ()>(core::marker::PhantomData<T>, core::cell::UnsafeCell<objc_object>)
where
  T: ?Sized;
unsafe impl<T> crate::marker::Class for Id<T> where T: ?Sized {}

// impl<T: ?Sized> core::ops::Deref for Id<T> {
//   type Target = Id;

//   #[inline(always)]
//   fn deref(&self) -> &Self::Target {
//     return unsafe { core::mem::transmute(self) };
//   }
// }

// impl<T: ?Sized> core::ops::DerefMut for Id<T> {
//   #[inline(always)]
//   fn deref_mut(&mut self) -> &mut Self::Target {
//     return unsafe { core::mem::transmute(self) };
//   }
// }

#[repr(transparent)]
pub struct Class(Id);

impl core::ops::Deref for Class {
  type Target = Id;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    return &self.0;
  }
}

impl core::ops::DerefMut for Class {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    return &mut self.0;
  }
}

#[repr(transparent)]
pub struct Sel(objc_selector);

#[repr(transparent)]
pub struct Imp(opaque_function);

#[repr(transparent)]
pub struct Method(objc_method);

#[link(name = "objc")]
extern "C" {
  pub fn sel_isMapped(sel: &Sel) -> bool;
  pub fn sel_getName(sel: &Sel) -> &'static CStr;
  pub fn sel_registerName(name: &CStr) -> &'static Sel;
  pub fn sel_getUid(name: &CStr) -> &'static Sel;

  pub fn object_getClassName(object: *const Id) -> &'static CStr;
}
