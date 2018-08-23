// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

use mtkview_delegate::MTKViewDelegate;
use objrs::objrs;

use objrs_frameworks_core_graphics::{CGRect, CGSize};
use objrs_frameworks_metal::{
  MTLClearColor, MTLDeviceId, MTLDrawableId, MTLPixelFormat, MTLRenderPassDescriptor,
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
    device: Option<objrs::Strong<MTLDeviceId>>,
  ) -> objrs::Strong<MTKView> {
  }

  #[objrs(selector = "device")]
  pub fn device(&mut self) -> Option<&mut MTLDeviceId> {}

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
  pub fn current_drawable(&mut self) -> Option<&mut MTLDrawableId> {}

  #[objrs(selector = "drawableSize")]
  pub fn drawable_size(&self) -> CGSize {}
}
