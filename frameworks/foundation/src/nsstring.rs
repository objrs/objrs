// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

extern crate libc;

extern crate core;
extern crate objrs;

use nsobject;
use objrs::objrs;

// TODO: LLVM's Early CSE pass ("early-cse": http://llvm.org/doxygen/EarlyCSE_8cpp_source.html) is breaking objrs. It optimizes out the static selref variable. Even marking the variable as #[used] doesn't fix it (that causes the variable to not be optimized out, but the LLVM IR still doesn't use it, as it prefers loading the address of the method name instead). Run rustc with ` -C opt-level=0 -C passes=early-cse` to see the impact (use `-C llvm-args=-print-after-all` to verify that it's indeed the Early CSE pass that's causing this).

#[objrs(class = "NSString", super = "nsobject::NSObject")]
#[link(name = "Foundation", kind = "framework")]
pub struct NSString;

#[objrs(impl)]
impl NSString {
  #[objrs(selector = "alloc")]
  pub fn alloc() -> objrs::Alloc<NSString> {}

  // TODO: this is an example of taking a Rust type and mapping it to Objective-C. The wrapper has to be manually created.
  #[inline(always)]
  pub fn init_with_bytes_encoding(
    this: objrs::Alloc<NSString>,
    bytes: &[u8],
    encoding: usize, /* TODO: use a newtype rather than usize */
  ) -> objrs::Strong<NSString> {
    return Self::init_with_bytes_length_encoding(
      this,
      bytes.as_ptr() as *const libc::c_char,
      bytes.len(),
      encoding,
    );
  }

  #[objrs(selector = "initWithBytes:length:encoding:")]
  pub fn init_with_bytes_length_encoding(
    this: objrs::Alloc<NSString>,
    bytes: *const libc::c_char,
    len: usize,
    encoding: usize, /* TODO: use a newtype rather than usize */
  ) -> objrs::Strong<NSString> {
  }

  #[objrs(selector = "string")]
  pub fn string() -> objrs::Strong<NSString> {}

  #[objrs(selector = "stringWithString:")]
  pub fn string_with_string(string: &NSString) -> objrs::Strong<NSString> {}

  #[objrs(selector = "stringWithUTF8String:")]
  pub fn string_with_utf8_string(cstr: &objrs::CStr) -> objrs::Strong<Self> {}

  #[objrs(selector = "length")]
  pub fn length(&self) -> usize {}

  // Raises an Objective-C exception if out of bounds.
  #[objrs(selector = "characterAtIndex:")]
  pub fn character_at_index(&self, index: usize) -> u16 {}

  #[objrs(selector = "hasPrefix:")]
  pub fn has_prefix(&self, prefix: &NSString) -> bool {}

  #[objrs(selector = "hasSuffix:")]
  pub fn has_suffix(&self, suffix: &NSString) -> bool {}

  #[objrs(selector = "isEqualToString:")]
  pub fn is_equal_to_string(&self, string: &NSString) -> bool {}

  #[objrs(selector = "stringByAppendingString:")]
  pub fn string_by_appending_string(&self, string: &NSString) -> objrs::Strong<NSString> {}

  // TODO: this is an example of taking a Rust type and mapping it to Objective-C.
  #[inline(always)]
  pub fn get_file_system_representation(&self, bytes: &mut [u8]) -> Result<(), ()> {
    let success = unsafe {
      self.get_file_system_representation_max_length(
        bytes as *mut _ as *mut libc::c_char,
        bytes.len(),
      )
    };
    if success {
      return Ok(());
    }
    return Err(());
  }

  #[objrs(selector = "getFileSystemRepresentation:maxLength:")]
  pub unsafe fn get_file_system_representation_max_length(
    &self,
    bytes: *mut libc::c_char,
    max_length: usize,
  ) -> bool {
  }

  #[objrs(selector = "UTF8String")]
  pub fn utf8_string(&self) -> &objrs::CStr {}

  pub fn hacky(&self) -> &objrs::CStr {
    return self.utf8_string();
  }
}

pub fn hacky(this: &NSString) -> &objrs::CStr {
  return this.utf8_string();
}

// See https://github.com/opensource-apple/CF/blob/master/CFString.c

#[doc(hidden)]
pub mod __objrs {
  use super::*;

  #[link(name = "CoreFoundation", kind = "framework")]
  extern "C" {
    pub type CFConstantStringClassReference;

    #[link_name = "__CFConstantStringClassReference"]
    #[doc(hidden)]
    pub static CFConstantStringClassReference: CFConstantStringClassReference;
  }

  #[repr(C)]
  #[doc(hidden)]
  pub struct CFConstantString {
    pub isa: &'static CFConstantStringClassReference,
    pub info: u32,
    pub ptr: &'static u8,
    pub length: usize,
  }
  unsafe impl core::marker::Send for CFConstantString {}
  unsafe impl core::marker::Sync for CFConstantString {}
}

// TODO: I don't think this merges duplicate strings (that is, nsstring!("hi"); nsstring!("hi"); will create two separate literals).
pub use objrs_frameworks_foundation_macros::nsstring;

// TODO: add a macro for formatting a string (e.g. nsstring_format!("{}, {}! {}", nsstring!("Hello"), "world", 42))
