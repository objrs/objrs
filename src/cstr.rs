// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

extern crate core;
extern crate libc;

extern "C" {
  type OpaqueCharacterType;
}

#[repr(transparent)]
pub struct CStr(OpaqueCharacterType);

impl CStr {
  pub unsafe fn new(string: &str) -> &CStr {
    return Self::with_bytes(string.as_bytes());
  }

  pub unsafe fn with_bytes(bytes: &[u8]) -> &CStr {
    assert_eq!(bytes.last(), Some(&b'\0'));
    return core::mem::transmute(bytes.as_ptr());
  }

  pub fn len(&self) -> usize {
    return unsafe { libc::strlen(core::mem::transmute(self)) };
  }

  #[inline(always)]
  pub fn as_ptr(&self) -> *const libc::c_char {
    return unsafe { core::mem::transmute(self) };
  }
}
