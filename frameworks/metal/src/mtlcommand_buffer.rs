// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

use mtlblit_command_encoder::MTLBlitCommandEncoderId;
use mtldrawable::MTLDrawableId;
use mtlrender_command_encoder::MTLRenderCommandEncoderId;
use mtlrender_pass::MTLRenderPassDescriptor;
use objrs::objrs;

#[objrs(protocol, id_ident = MTLCommandBufferId)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLCommandBuffer {
  #[objrs(selector = "blitCommandEncoder")]
  fn blit_command_encoder(&mut self) -> Option<&mut MTLBlitCommandEncoderId>;

  #[objrs(selector = "renderCommandEncoderWithDescriptor:")]
  fn render_command_encoder_with_descriptor<'a>(
    &'a mut self,
    render_pass_descriptor: &mut MTLRenderPassDescriptor,
  ) -> Option<&'a mut MTLRenderCommandEncoderId>;

  #[objrs(selector = "presentDrawable:")]
  fn present_drawable(&mut self, drawable: &mut MTLDrawableId);

  #[objrs(selector = "commit")]
  fn commit(&mut self);

  #[objrs(selector = "waitUntilCompleted")]
  fn wait_until_completed(&mut self);
}
