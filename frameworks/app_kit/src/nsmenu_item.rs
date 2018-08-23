// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

use nsmenu::NSMenu;
use objrs::objrs;
use objrs_frameworks_foundation::{NSObject, NSString};

#[objrs(class, super = NSObject)]
#[link(name = "AppKit", kind = "framework")]
pub struct NSMenuItem;

#[objrs(impl)]
#[link(name = "AppKit", kind = "framework")]
impl NSMenuItem {
  #[objrs(selector = "new")]
  pub fn new() -> objrs::Strong<NSMenuItem> {}

  #[objrs(selector = "alloc")]
  pub fn alloc() -> objrs::Alloc<NSMenuItem> {}

  #[objrs(selector = "initWithTitle:action:keyEquivalent:")]
  pub fn init_with_title_action_key_equivalent(
    self: objrs::Alloc<NSMenuItem>,
    title: &NSString,
    action: objrs::runtime::SEL,
    key_equivalent: &NSString,
  ) -> objrs::Strong<NSMenuItem> {
  }

  #[objrs(selector = "setSubmenu:")]
  pub fn set_submenu(&mut self, menu: objrs::Strong<NSMenu>) {}
}
