// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

use crate::nsresponder::NSResponder;
use crate::nsview_controller::NSViewController;
use objrs::objrs;
use objrs_frameworks_core_graphics::{CGFloat, CGPoint};
use objrs_frameworks_foundation::NSString;

#[objrs(class, super = NSResponder)]
#[link(name = "AppKit", kind = "framework")]
pub struct NSWindow;

#[objrs(impl)]
#[link(name = "AppKit", kind = "framework")]
impl NSWindow {
  #[objrs(selector = "windowWithContentViewController:")]
  pub fn window_with_content_view_controller(
    view_controller: objrs::Strong<NSViewController>,
  ) -> objrs::Strong<NSWindow> {
  }

  #[objrs(selector = "setTitle:")]
  pub fn set_title(&mut self, title: &NSString) {}

  #[objrs(selector = "makeKeyAndOrderFront:")]
  pub fn make_key_and_order_front(&mut self, sender: *mut objrs::Id) {}

  #[objrs(selector = "mouseLocationOutsideOfEventStream")]
  pub fn mouse_location_outside_of_event_stream(&self) -> CGPoint {}

  #[objrs(selector = "backingScaleFactor")]
  pub fn backing_scale_factor(&self) -> CGFloat {}
}
