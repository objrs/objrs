// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

use crate::mtllibrary::MTLFunction;
use crate::mtlpixel_format::MTLPixelFormat;
use objrs::{objrs, Id};
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
  pub fn set_vertex_function(
    &mut self,
    vertex_function: Option<objrs::Strong<Id<dyn MTLFunction>>>,
  ) {
  }

  #[objrs(selector = "setFragmentFunction:")]
  pub fn set_fragment_function(
    &mut self,
    fragment_function: Option<objrs::Strong<Id<dyn MTLFunction>>>,
  ) {
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

#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLRenderPipelineState {}
