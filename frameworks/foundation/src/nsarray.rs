// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

extern crate objrs;

use crate::nsobject;
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
