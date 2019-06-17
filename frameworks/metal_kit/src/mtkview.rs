// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

use crate::mtkview_delegate::MTKViewDelegate;
use objrs::{objrs, Id};

use objrs_frameworks_core_graphics::{CGRect, CGSize};
use objrs_frameworks_metal::{
  MTLClearColor, MTLDevice, MTLDrawable, MTLPixelFormat, MTLRenderPassDescriptor,
};

#[cfg(target_os = "macos")]
use objrs_frameworks_app_kit::NSView;

#[cfg(target_os = "ios")]
use objrs_frameworks_ui_kit::UIView;

#[cfg(target_os = "macos")]
#[objrs(class, super = NSView)]
#[link(name = "MetalKit", kind = "framework")]
pub struct MTKView;

#[cfg(target_os = "ios")]
#[objrs(class, super = UIView)]
#[link(name = "MetalKit", kind = "framework")]
pub struct MTKView;

#[objrs(impl)]
#[link(name = "MetalKit", kind = "framework")]
impl MTKView {
  #[objrs(selector = "alloc")]
  pub fn alloc() -> objrs::Alloc<MTKView> {}

  #[objrs(selector = "initWithFrame:device:")]
  pub fn init_with_frame_device(
    self: objrs::Alloc<MTKView>,
    frame: CGRect,
    device: Option<objrs::Strong<Id<dyn MTLDevice>>>,
  ) -> objrs::Strong<MTKView> {
  }

  #[objrs(selector = "device")]
  pub fn device(&mut self) -> Option<&mut Id<dyn MTLDevice>> {}

  // TODO: the delegate is weakly held.
  #[objrs(selector = "setDelegate:")]
  pub fn set_delegate<T: MTKViewDelegate + objrs::marker::Class + ?Sized>(
    &mut self,
    delegate: objrs::Strong<T>,
  ) {
  }

  #[objrs(selector = "sampleCount")]
  pub fn sample_count(&self) -> usize {}

  #[objrs(selector = "setSampleCount:")]
  pub fn set_sample_count(&mut self, sample_count: usize) {}

  #[objrs(selector = "setClearColor:")]
  pub fn set_clear_color(&mut self, clear_color: MTLClearColor) {}

  #[objrs(selector = "colorPixelFormat")]
  pub fn color_pixel_format(&self) -> MTLPixelFormat {}

  #[objrs(selector = "currentRenderPassDescriptor")]
  pub fn current_render_pass_descriptor(&mut self) -> Option<&mut MTLRenderPassDescriptor> {}

  // TODO: this really returns a id<CAMetalDrawable>.
  #[objrs(selector = "currentDrawable")]
  pub fn current_drawable(&mut self) -> Option<&mut Id<dyn MTLDrawable>> {}

  #[objrs(selector = "drawableSize")]
  pub fn drawable_size(&self) -> CGSize {}
}
