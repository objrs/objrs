// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

use objrs::objrs;

use crate::nsapplication::NSApplication;
use objrs_frameworks_foundation::NSNotification;

#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSApplicationDelegate {
  #[objrs(selector = "applicationDidFinishLaunching:", optional)]
  fn application_did_finish_launching(&mut self, notification: &NSNotification);

  #[objrs(selector = "applicationShouldTerminateAfterLastWindowClosed:", optional)]
  fn application_should_terminate_after_last_window_closed(
    &mut self,
    application: &NSApplication,
  ) -> bool;
}
