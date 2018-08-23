// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

extern crate objrs;

use nsstring;
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
