// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

use nsresponder::NSResponder;
use nsview_controller::NSViewController;
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
  pub fn make_key_and_order_front(&mut self, sender: Option<objrs::runtime::id>) {}

  #[objrs(selector = "mouseLocationOutsideOfEventStream")]
  pub fn mouse_location_outside_of_event_stream(&self) -> CGPoint {}

  #[objrs(selector = "backingScaleFactor")]
  pub fn backing_scale_factor(&self) -> CGFloat {}
}
