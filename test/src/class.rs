// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

//! Tests the `class!` macro.

extern crate objrs;

use objrs::__objrs::runtime;

#[cfg_attr(test, test)]
fn test_class() {
  validate_class(objrs::class!("NSObject"));
}

pub fn validate_class(class: &objrs::Class) {
  let as_id: &objrs::Id = &*class;
  assert!(unsafe { runtime::object_isClass(as_id as *const _) });
}

#[cfg(not(test))]
pub fn run_tests() {
  test_class();
}
