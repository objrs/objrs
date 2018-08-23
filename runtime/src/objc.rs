// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

// See https://github.com/opensource-apple/objc4/blob/master/runtime/objc.h

extern crate core;
extern crate libc;

use objc_runtime_new;

// TODO: implement the following traits:
// std::panic::UnwindSafe
// core::ops::CoerceUnsized

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Class(pub *mut objc_runtime_new::objc_class);

impl core::fmt::Pointer for Class {
  fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
    return core::fmt::Pointer::fmt(&self.0, formatter);
  }
}

impl core::fmt::Debug for Class {
  fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
    return core::fmt::Pointer::fmt(&self.0, formatter);
  }
}

impl core::ops::Deref for Class {
  type Target = *mut objc_runtime_new::objc_class;

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

#[derive(Debug)]
#[repr(C)]
pub struct objc_object {
  #[deprecated(
    note = "`isa` was deprecated in Objective-C 2 and may be unavailable in the future"
  )]
  pub isa: Class,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct id(pub *mut objc_object);

impl core::fmt::Pointer for id {
  fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
    return core::fmt::Pointer::fmt(&self.0, formatter);
  }
}

impl core::fmt::Debug for id {
  fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
    return core::fmt::Pointer::fmt(&self.0, formatter);
  }
}

impl core::ops::Deref for id {
  type Target = *mut objc_object;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    return &self.0;
  }
}

impl core::ops::DerefMut for id {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    return &mut self.0;
  }
}

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SEL(pub *const libc::c_char);

impl core::fmt::Pointer for SEL {
  fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
    return core::fmt::Pointer::fmt(&self.0, formatter);
  }
}

impl core::fmt::Debug for SEL {
  fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
    return core::fmt::Pointer::fmt(&self.0, formatter);
  }
}

impl core::ops::Deref for SEL {
  type Target = *const libc::c_char;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    return &self.0;
  }
}

impl core::ops::DerefMut for SEL {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    return &mut self.0;
  }
}

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IMP(pub unsafe extern "C" fn());

impl core::fmt::Pointer for IMP {
  fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
    return core::fmt::Pointer::fmt(&self.0, formatter);
  }
}

impl core::fmt::Debug for IMP {
  fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
    return core::fmt::Pointer::fmt(&self.0, formatter);
  }
}

impl core::ops::Deref for IMP {
  type Target = unsafe extern "C" fn();

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    return &self.0;
  }
}

impl core::ops::DerefMut for IMP {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    return &mut self.0;
  }
}

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Method(pub *mut objc_runtime_new::method_t);

impl core::fmt::Pointer for Method {
  fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
    return core::fmt::Pointer::fmt(&self.0, formatter);
  }
}

impl core::fmt::Debug for Method {
  fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
    return core::fmt::Pointer::fmt(&self.0, formatter);
  }
}

impl core::ops::Deref for Method {
  type Target = *mut objc_runtime_new::method_t;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    return &self.0;
  }
}

impl core::ops::DerefMut for Method {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    return &mut self.0;
  }
}
