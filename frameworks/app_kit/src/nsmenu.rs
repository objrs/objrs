// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

use nsmenu_item::NSMenuItem;
use objrs::objrs;
use objrs_frameworks_foundation::NSObject;

#[objrs(class, super = NSObject)]
#[link(name = "AppKit", kind = "framework")]
pub struct NSMenu;

#[objrs(impl)]
#[link(name = "AppKit", kind = "framework")]
impl NSMenu {
  #[objrs(selector = "new")]
  pub fn new() -> objrs::Strong<NSMenu> {}

  #[objrs(selector = "addItem:")]
  pub fn add_item(&mut self, item: objrs::Strong<NSMenuItem>) {}
}
