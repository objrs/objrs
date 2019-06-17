// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

use crate::nsmenu::NSMenu;
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
    action: &objrs::Sel,
    key_equivalent: &NSString,
  ) -> objrs::Strong<NSMenuItem> {
  }

  #[objrs(selector = "setSubmenu:")]
  pub fn set_submenu(&mut self, menu: objrs::Strong<NSMenu>) {}
}
