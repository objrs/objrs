// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

#![feature(rust_2018_preview, proc_macro_hygiene,)]

extern crate core;
extern crate objrs;
extern crate objrs_frameworks_app_kit;
extern crate objrs_frameworks_core_graphics;
extern crate objrs_frameworks_foundation;
extern crate objrs_frameworks_metal;
extern crate objrs_frameworks_metal_kit;

mod mesh;
mod render;

use objrs::objrs;
use objrs::selector;
use objrs_frameworks_app_kit::{
  NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate, NSMenu, NSMenuItem, NSView,
  NSViewController, NSWindow,
};
use objrs_frameworks_core_graphics::CGRectMake;
use objrs_frameworks_foundation::{nsstring, NSNotification, NSObject, NSProcessInfo};
use objrs_frameworks_metal::MTLCreateSystemDefaultDevice;
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
impl NSApplicationDelegate for AppDelegate {
  #[objrs(selector = "applicationDidFinishLaunching:")]
  fn application_did_finish_launching(&mut self, _notification: &NSNotification) {
    let mut window =
      NSWindow::window_with_content_view_controller(ViewController::new().into_super());
    window.set_title(nsstring!("Objective-Rust Demo"));
    window.make_key_and_order_front(None);
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
  objrs::runtime::autoreleasepool(run_app);
}
