// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

use crate::mtlblit_command_encoder::MTLBlitCommandEncoder;
use crate::mtldrawable::MTLDrawable;
use crate::mtlrender_command_encoder::MTLRenderCommandEncoder;
use crate::mtlrender_pass::MTLRenderPassDescriptor;
use objrs::{objrs, Id};

#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLCommandBuffer {
  #[objrs(selector = "blitCommandEncoder")]
  fn blit_command_encoder(&mut self) -> Option<&mut Id<dyn MTLBlitCommandEncoder>>;

  #[objrs(selector = "renderCommandEncoderWithDescriptor:")]
  fn render_command_encoder_with_descriptor<'a>(
    &'a mut self,
    render_pass_descriptor: &mut MTLRenderPassDescriptor,
  ) -> Option<&'a mut Id<dyn MTLRenderCommandEncoder>>;

  #[objrs(selector = "presentDrawable:")]
  fn present_drawable(&mut self, drawable: &mut Id<dyn MTLDrawable>);

  #[objrs(selector = "commit")]
  fn commit(&mut self);

  #[objrs(selector = "waitUntilCompleted")]
  fn wait_until_completed(&mut self);
}
