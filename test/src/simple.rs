// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

//! Simple tests (that are largely redundant) intended for use with `cargo expand` to aid in
//! debugging a macro's code generation.

extern crate objrs;
use objrs::objrs;

#[cfg_attr(test, test)]
fn selector() {
  let _ = objrs::selector!("someSelector");
}

#[cfg_attr(test, test)]
fn class() {
  let _ = objrs::class!("NSObject");
}

#[cfg_attr(test, test)]
fn external_class() {
  #[objrs(class, root_class, extern)]
  struct NSObject;
}

#[cfg_attr(test, test)]
fn external_ivar() {
  #[objrs(class, root_class, extern)]
  struct NSObject {
    #[deprecated]
    isa: *mut objrs::Class,
  }
}

#[objrs(class, root_class, extern)]
struct NSObject;

#[cfg_attr(test, test)]
fn external_impl() {
  #[objrs(impl, extern)]
  impl NSObject {}
}

// #[cfg_attr(test, test)]
// fn external_class_method() {
//   #[objrs(impl, extern)]
//   impl NSObject {
//     #[objrs(selector = "hash")]
//     fn classHash(&self) -> usize {}
//   }
// }

// #[cfg_attr(test, test)]
// fn external_instance_method() {
//   #[objrs(impl, extern)]
//   impl NSObject {
//     #[objrs(selector = "hash")]
//     fn instanceHash(&self) -> usize {}
//   }
// }

#[cfg(not(test))]
pub fn run_tests() {
  selector();
  class();
  external_class();
  external_ivar();
  external_impl();
  // external_class_method();
  // external_instance_method();
}
