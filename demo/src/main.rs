// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

#![feature(
  lang_items, start, libc, used, nonzero, repr_transparent, macros_in_extern, extern_absolute_paths,
  proc_macro, extern_types, proc_macro_non_items
)]
#![no_std]

// extern crate core;
extern crate libc;
extern crate objrs;
extern crate objrs_frameworks_foundation;

use objrs::objrs;
use objrs_frameworks_foundation::*;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
#[lang = "panic_fmt"]
fn panic_fmt() -> ! {
  loop {}
}

extern "C" {
  fn puts(_: *const libc::c_char) -> libc::c_int;
}

#[objrs(class = "NSMutableArray", super = "NSArray<T>")]
#[link(name = "Foundation", kind = "framework")]
pub struct NSMutableArray<T: objrs::marker::Class + ?Sized>;

#[objrs(impl)]
impl<T: objrs::marker::Class + ?Sized> NSMutableArray<T> {
  #[objrs(selector = "array")]
  pub fn array() -> objrs::Strong<Self> {}

  #[objrs(selector = "arrayWithCapacity:")]
  pub fn array_with_capacity(capacity: usize) -> objrs::Strong<Self> {}

  #[objrs(selector = "addObject:")]
  pub fn add_object(&mut self, object: &T) {} // TODO: this should move the object
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
  // fn main() {
  let array = NSMutableArray::<NSString>::array();
  let string = nsstring!("Hello, world!");
  let array = array.array_by_adding_object(&string);
  let first = array.first_object();
  if let Some(string) = first {
    let cstr = string.utf8_string();
    unsafe {
      puts(cstr.as_ptr());
    }
  }

  return 0;
}
