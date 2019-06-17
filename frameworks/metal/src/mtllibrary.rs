// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

use objrs::{objrs, Id};
use objrs_frameworks_foundation::NSString;

#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLFunction {}

#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLLibrary {
  #[objrs(selector = "newFunctionWithName:")]
  fn new_function_with_name(
    &mut self,
    function_name: &NSString,
  ) -> Option<objrs::Strong<Id<dyn MTLFunction>>>;
}
