// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

#![feature(extern_types, repr_transparent)]
#![no_std]

pub mod marker;
#[cfg(test)]
mod test;

extern crate libc;
#[macro_use]
extern crate objrs_macros;

pub extern crate objrs_runtime as runtime;

#[doc(hidden)]
pub use objrs_macros::*;

#[repr(transparent)]
pub struct Alloc<T: runtime::__objrs::Class + ?Sized>(Strong<T>);

// TODO: make this nonnull?
#[repr(transparent)]
pub struct Strong<T: runtime::__objrs::Class + ?Sized>(*mut T);

// See https://github.com/rust-lang/rust/issues/47442
impl<T: runtime::__objrs::Class + ?Sized> core::ops::Drop for Strong<T> {
  #[inline(always)]
  fn drop(&mut self) {
    unsafe {
      runtime::objc_release(self.0 as runtime::id);
    }
  }
}

impl<T: runtime::__objrs::Class + ?Sized> core::ops::Deref for Strong<T> {
  type Target = T;

  #[inline(always)]
  fn deref(&self) -> &T {
    return unsafe { &*self.0 };
  }
}

impl<T: runtime::__objrs::Class + ?Sized> core::ops::DerefMut for Strong<T> {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut T {
    return unsafe { &mut *self.0 };
  }
}

impl<T: runtime::__objrs::Class + ?Sized> core::convert::AsRef<runtime::id> for Strong<T> {
  #[inline(always)]
  fn as_ref(&self) -> &runtime::id {
    return unsafe { core::mem::transmute(self) };
  }
}

impl<T: runtime::__objrs::Class + ?Sized> core::convert::AsMut<runtime::id> for Strong<T> {
  #[inline(always)]
  fn as_mut(&mut self) -> &mut runtime::id {
    return unsafe { core::mem::transmute(self) };
  }
}

#[repr(transparent)]
pub struct Weak<T: runtime::__objrs::Class + ?Sized>(*mut T);

extern "C" {
  type Opaque;
}

#[repr(transparent)]
pub struct CStr(Opaque);

impl CStr {
  pub fn new(string: &str) -> &CStr {
    return Self::with_bytes(string.as_bytes());
  }

  pub fn with_bytes(bytes: &[u8]) -> &CStr {
    assert_eq!(bytes.last(), Some(&b'\0'));
    return unsafe { core::mem::transmute(bytes.as_ptr()) };
  }

  pub fn as_ptr(&self) -> *const libc::c_char {
    return unsafe { core::mem::transmute(self) };
  }
}

#[doc(hidden)]
pub mod __objrs {
  // use super::*;

  pub extern crate core;
  pub extern crate libc;

  #[repr(transparent)]
  pub struct SyncHack<T>(pub T);
  unsafe impl<T> Sync for SyncHack<T> {}

  // If core::mem::transmute ever becomes a const fn, use it instead of this hack.
  // https://github.com/rust-lang/rust/issues/49450
  #[repr(C)]
  pub union TransmuteHack<T: Copy, U: Copy> {
    pub from: T,
    pub to: U,
  }

  #[cfg(test)]
  pub mod test {
    pub use super::super::test::*;
  }
}
