// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

use mtllibrary::MTLFunctionId;
use mtlpixel_format::MTLPixelFormat;
use objrs::objrs;
use objrs_frameworks_foundation::NSObject;

#[objrs(class, super = NSObject)]
#[link(name = "Metal", kind = "framework")]
pub struct MTLRenderPipelineDescriptor;

#[objrs(impl)]
#[link(name = "Metal", kind = "framework")]
impl MTLRenderPipelineDescriptor {
  #[objrs(selector = "new")]
  pub fn new() -> objrs::Strong<MTLRenderPipelineDescriptor> {}

  #[objrs(selector = "setRasterSampleCount:")]
  pub fn set_raster_sample_count(&mut self, raster_sample_count: usize) {}

  #[objrs(selector = "setVertexFunction:")]
  pub fn set_vertex_function(&mut self, vertex_function: Option<objrs::Strong<MTLFunctionId>>) {}

  #[objrs(selector = "setFragmentFunction:")]
  pub fn set_fragment_function(&mut self, fragment_function: Option<objrs::Strong<MTLFunctionId>>) {
  }

  #[objrs(selector = "colorAttachments")]
  pub fn color_attachments(&mut self) -> &mut MTLRenderPipelineColorAttachmentDescriptorArray {}
}

#[objrs(class, super = NSObject)]
#[link(name = "Metal", kind = "framework")]
pub struct MTLRenderPipelineColorAttachmentDescriptor;

#[objrs(impl)]
#[link(name = "Metal", kind = "framework")]
impl MTLRenderPipelineColorAttachmentDescriptor {
  #[objrs(selector = "setPixelFormat:")]
  pub fn set_pixel_format(&mut self, pixel_format: MTLPixelFormat) {}
}

#[objrs(class, super = NSObject)]
#[link(name = "Metal", kind = "framework")]
pub struct MTLRenderPipelineColorAttachmentDescriptorArray;

#[objrs(impl)]
#[link(name = "Metal", kind = "framework")]
impl MTLRenderPipelineColorAttachmentDescriptorArray {
  #[objrs(selector = "objectAtIndexedSubscript:")]
  pub fn object_at_indexed_subscript(
    &mut self,
    index: usize,
  ) -> &mut MTLRenderPipelineColorAttachmentDescriptor {
  }
}

#[objrs(protocol, id_ident = MTLRenderPipelineStateId)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLRenderPipelineState {}
