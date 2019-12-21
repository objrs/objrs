// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

#![feature(proc_macro_hygiene)]
#![feature(extern_types)]
extern crate core;

mod mesh;
mod render;

use objrs::objrs;
use objrs::selector;
use macos::AppKit::{
  NSApplication, NSApplicationActivationPolicy, NSApplicationDelegateProto, NSMenu, NSMenuItem, NSView,
  NSViewController, NSWindow,
};
use macos::CoreGraphics::CGRectMake;
use macos::Foundation::{NSString, NSNotification, NSObject, NSProcessInfo};
use macos::Metal::MTLCreateSystemDefaultDevice;
use objrs_frameworks_metal_kit::{MTKView, MTKViewDelegate};

#[objrs(class, super = NSObject)]
struct AppDelegate {
  window: Option<objrs::Strong<NSWindow>>,
}

#[objrs(impl)]
impl AppDelegate {
  #[objrs(selector = "new", no_impl)]
  fn new() -> objrs::Strong<AppDelegate> {}
}

#[objrs(impl)]
impl NSApplicationDelegateProto for AppDelegate {
  #[objrs(selector = "applicationDidFinishLaunching:")]
  fn applicationDidFinishLaunching_ (&mut self, _notification: &NSNotification) {
    let mut window =
      NSWindow::window_with_content_view_controller(ViewController::new().into_super());
    window.set_title(nsstring!("Objective-Rust Demo"));
    window.make_key_and_order_front(0 as *mut _);
    self.window = Some(window);

    let mut app = NSApplication::shared_application();
    let app = unsafe { app.as_mut() };
    app.activate_ignoring_other_apps(true);
  }

  #[objrs(selector = "applicationShouldTerminateAfterLastWindowClosed:")]
  fn application_should_terminate_after_last_window_closed(
    &mut self,
    _application: &NSApplication,
  ) -> bool {
    return true;
  }
}

#[objrs(class, super = NSViewController)]
struct ViewController;

#[objrs(impl)]
impl ViewController {
  #[objrs(selector = "new", no_impl)]
  fn new() -> objrs::Strong<ViewController> {}

  // This isn't actually dead code, as this method is caled from Objective-C. Unfortunately, Rust
  // doesn't know that and warns that this method is never used. So we #[allow(dead_code)].
  #[objrs(selector = "loadView")]
  #[allow(dead_code)]
  fn load_view(&mut self) {
    let frame = CGRectMake(0.0, 0.0, 512.0, 512.0);
    let device = MTLCreateSystemDefaultDevice();
    if device.is_none() {
      println!("This device doesn't support Metal. The demo will not attempt to render anything.");
      self.set_view(NSView::alloc().init_with_frame(frame));
      return;
    }

    let mut view = MTKView::alloc().init_with_frame_device(frame, device);
    let mut delegate = render::Renderer::new();
    let drawable_size = view.drawable_size();
    delegate.mtkview_drawable_size_will_change(&mut view, drawable_size);
    view.set_delegate(delegate);
    self.set_view(view.into_super());
  }

  #[objrs(selector = "setView:", no_impl)]
  fn set_view(&mut self, view: objrs::Strong<NSView>) {}
}

fn build_menu() -> objrs::Strong<NSMenu> {
  let app_name = NSProcessInfo::process_info().process_name();
  let quit_string = nsstring!("Quit ").string_by_appending_string(app_name);

  let quit_button = NSMenuItem::alloc().init_with_title_action_key_equivalent(
    &quit_string,
    selector!("terminate:"),
    nsstring!("q"),
  );
  let mut app_menu = NSMenu::new();
  app_menu.add_item(quit_button);

  let mut app_menu_button = NSMenuItem::new();
  app_menu_button.set_submenu(app_menu);

  let mut menu = NSMenu::new();
  menu.add_item(app_menu_button);
  return menu;
}

fn run_app() {
  let mut app = NSApplication::shared_application();
  let app = unsafe { app.as_mut() };
  app.set_activation_policy(NSApplicationActivationPolicy::REGULAR);

  app.set_delegate(AppDelegate::new());
  app.set_main_menu(build_menu());
  app.run();
}

fn main() {
  objrs::autoreleasepool(run_app);
}
