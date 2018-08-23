// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

extern crate objrs;

use nsobject;
use objrs::objrs;

#[objrs(class, super = nsobject::NSObject)]
#[link(name = "Foundation", kind = "framework")]
pub struct NSArray<T: objrs::marker::Class + ?Sized>;

#[objrs(impl)]
#[link(name = "Foundation", kind = "framework")]
impl<T: objrs::marker::Class + ?Sized> NSArray<T> {
  #[objrs(selector = "array")]
  pub fn array() -> objrs::Strong<Self> {}

  #[objrs(selector = "arrayByAddingObject:")]
  pub fn array_by_adding_object(&self, object: objrs::Strong<T>) -> objrs::Strong<Self> {}

  #[objrs(selector = "firstObject")]
  pub fn first_object<'a>(&'a self) -> Option<&'a T> {}
}
