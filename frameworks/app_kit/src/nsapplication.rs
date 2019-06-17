// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

use crate::nsapplication_delegate::NSApplicationDelegate;
use crate::nsmenu::NSMenu;
use crate::nsresponder::NSResponder;
use objrs::objrs;

#[link(name = "AppKit", kind = "framework")]
extern "C" {
  pub static NSApp: *mut NSApplication;
}

#[repr(transparent)]
pub struct NSApplicationActivationPolicy(isize);

unsafe impl objrs::marker::Zeroed for NSApplicationActivationPolicy {}
impl objrs::marker::Forgettable for NSApplicationActivationPolicy {}

#[allow(non_upper_case_globals)]
pub const NSApplicationActivationPolicyRegular: NSApplicationActivationPolicy =
  NSApplicationActivationPolicy(0);
#[allow(non_upper_case_globals)]
pub const NSApplicationActivationPolicyAccessory: NSApplicationActivationPolicy =
  NSApplicationActivationPolicy(1);
#[allow(non_upper_case_globals)]
pub const NSApplicationActivationPolicyProhibited: NSApplicationActivationPolicy =
  NSApplicationActivationPolicy(2);

impl NSApplicationActivationPolicy {
  pub const REGULAR: NSApplicationActivationPolicy = NSApplicationActivationPolicyRegular;
  pub const ACCESSORY: NSApplicationActivationPolicy = NSApplicationActivationPolicyAccessory;
  pub const PROHIBITED: NSApplicationActivationPolicy = NSApplicationActivationPolicyProhibited;
}

#[objrs(class, super = NSResponder)]
#[link(name = "AppKit", kind = "framework")]
pub struct NSApplication;

#[objrs(impl)]
#[link(name = "AppKit", kind = "framework")]
impl NSApplication {
  // TODO: this returns an autoreleased value. Is there a better return type we can use?
  #[objrs(selector = "sharedApplication")]
  pub fn shared_application() -> core::ptr::NonNull<NSApplication> {}

  // TODO: what should the return type be here? I think this is returning a fat pointer.
  #[objrs(selector = "delegate")]
  pub fn delegate(&self) -> &dyn NSApplicationDelegate {}

  #[objrs(selector = "setDelegate:")]
  pub fn set_delegate<T: NSApplicationDelegate + objrs::marker::Class + ?Sized>(
    &mut self,
    delegate: objrs::Strong<T>,
  ) {
  }

  #[objrs(selector = "run")]
  pub fn run(&mut self) {}

  #[objrs(selector = "setActivationPolicy:")]
  pub fn set_activation_policy(
    &mut self,
    activation_policy: NSApplicationActivationPolicy,
  ) -> bool {
  }

  #[objrs(selector = "setMainMenu:")]
  pub fn set_main_menu(&mut self, menu: objrs::Strong<NSMenu>) {}

  #[objrs(selector = "activateIgnoringOtherApps:")]
  pub fn activate_ignoring_other_apps(&mut self, ignore_other_apps: bool) {}
}
