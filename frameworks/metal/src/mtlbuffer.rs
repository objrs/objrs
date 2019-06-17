// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

use objrs::objrs;

#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLBuffer {
  #[objrs(selector = "length")]
  fn length(&self) -> usize;
}
