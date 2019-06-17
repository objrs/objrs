// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

//! Tests `#[objrs(class)]` and `#[objrs(impl)]` on an external class (`NSObject`).

extern crate objrs;
use objrs::objrs;

#[objrs(class, root_class)]
#[link(name = "Foundation", kind = "framework")]
struct NSObject;

#[objrs(impl)]
#[link(name = "Foundation", kind = "framework")]
impl NSObject {
  #[objrs(selector = "hash")]
  fn hash(&self) -> usize {}
}

// TODO: verify the class.

#[cfg(not(test))]
pub fn run_tests() {}
