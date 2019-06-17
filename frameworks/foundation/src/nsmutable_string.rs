// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

extern crate objrs;

use crate::nsstring;
use objrs::objrs;

#[objrs(class, super = nsstring::NSString)]
#[link(name = "Foundation", kind = "framework")]
pub struct NSMutableString;

#[objrs(impl)]
#[link(name = "Foundation", kind = "framework")]
impl NSMutableString {
  #[objrs(selector = "stringWithCapacity:")]
  pub fn string_with_capacity(capacity: usize) -> objrs::Strong<NSMutableString> {}

  #[objrs(selector = "setString:")]
  pub fn set_string(&mut self, string: &nsstring::NSString) {}
}
