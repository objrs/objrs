// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

use crate::nsresponder::NSResponder;
use crate::nswindow::NSWindow;
use objrs::objrs;
use objrs_frameworks_core_graphics::CGRect;

#[objrs(class, super = NSResponder)]
#[link(name = "AppKit", kind = "framework")]
pub struct NSView;

#[objrs(impl)]
#[link(name = "AppKit", kind = "framework")]
impl NSView {
  #[objrs(selector = "alloc")]
  pub fn alloc() -> objrs::Alloc<NSView> {}

  #[objrs(selector = "initWithFrame:")]
  pub fn init_with_frame(self: objrs::Alloc<NSView>, frame: CGRect) -> objrs::Strong<NSView> {}

  #[objrs(selector = "window")]
  pub fn window(&self) -> &NSWindow {}
}
