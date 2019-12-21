// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

// #![feature(proc_macro_non_items)]  // Required for nsstring!.
#![no_main]

// extern crate core;
// extern crate libc;
extern crate objrs;

use objrs::objrs;

trait Trait {
  type Foo;
}

impl Trait for NSObject {
  type Foo = usize;
}

#[objrs(class, root_class)]
#[link(name = "Foundation", kind = "framework")]
struct NSObject;

#[objrs(impl)]
#[link(name = "Foundation", kind = "framework")]
impl NSObject {
  #[objrs(selector = "hash")]
  fn hash(&self) -> <Self as Trait>::Foo {}
}

#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSCopying {
}

pub struct Arc<T: ?Sized> {
    ptr: std::ptr::NonNull<T>,
}
pub trait NSCopying2 {
  fn new() -> Option<Arc<Self>>;
}

#[no_mangle]
fn main(_argc: i32, _argv: *const *const u8) -> i32 {
	println!("Running tiny_tests");
  let _class = objrs::class!(r#"NSObject"#);
  let _sel = objrs::selector!(r#"Hello"#);
  return 0;
}
