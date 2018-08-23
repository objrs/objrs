// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

use mtlbuffer::MTLBufferId;
use mtlrender_pipeline::MTLRenderPipelineStateId;
use objrs::objrs;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct MTLPrimitiveType(usize);

// pub const MTLPrimitiveTypePoint: MTLPrimitiveType = MTLPrimitiveType(0);
// pub const MTLPrimitiveTypeLine: MTLPrimitiveType = MTLPrimitiveType(1);
// pub const MTLPrimitiveTypeLineStrip: MTLPrimitiveType = MTLPrimitiveType(2);
// pub const MTLPrimitiveTypeTriangle: MTLPrimitiveType = MTLPrimitiveType(3);
// pub const MTLPrimitiveTypeTriangleStrip: MTLPrimitiveType = MTLPrimitiveType(4);

impl MTLPrimitiveType {
  pub const POINT: MTLPrimitiveType = MTLPrimitiveType(0);
  pub const LINE: MTLPrimitiveType = MTLPrimitiveType(1);
  pub const LINE_STRIP: MTLPrimitiveType = MTLPrimitiveType(2);
  pub const TRIANGLE: MTLPrimitiveType = MTLPrimitiveType(3);
  pub const TRIANGLE_STRIP: MTLPrimitiveType = MTLPrimitiveType(4);
}

unsafe impl objrs::marker::Zeroed for MTLPrimitiveType {}
impl objrs::marker::Forgettable for MTLPrimitiveType {}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(C)]
#[allow(non_snake_case)]
pub struct MTLViewport {
  pub originX: f64,
  pub originY: f64,
  pub width: f64,
  pub height: f64,
  pub znear: f64,
  pub zfar: f64,
}

unsafe impl objrs::marker::Zeroed for MTLViewport {}
impl objrs::marker::Forgettable for MTLViewport {}

#[objrs(protocol, id_ident = MTLRenderCommandEncoderId)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLRenderCommandEncoder: objrs::marker::Class {
  #[objrs(selector = "setViewport:")]
  fn set_viewport(&mut self, viewport: MTLViewport);

  #[objrs(selector = "setRenderPipelineState:")]
  fn set_render_pipeline_state(&mut self, state: &mut MTLRenderPipelineStateId);

  #[objrs(selector = "setVertexBuffer:offset:atIndex:")]
  unsafe fn set_vertex_buffer_at_index(
    &mut self,
    vertex_buffer: Option<&mut MTLBufferId>,
    offset: usize,
    index: usize,
  );

  #[objrs(selector = "setVertexBytes:length:atIndex:")]
  unsafe fn set_vertex_bytes_at_index(
    &mut self,
    vertex_bytes: &libc::c_void,
    length: usize,
    index: usize,
  );

  #[objrs(selector = "drawPrimitives:vertexStart:vertexCount:")]
  unsafe fn draw_primitives_vertex_start_vertex_count(
    &mut self,
    primitive_type: MTLPrimitiveType,
    vertex_start: usize,
    vertex_count: usize,
  );

  // TODO: this should be in MTLCommandEncoder, and the render encoder should specify itself as a superset of MTLCommandEncoder. objrs just doesn't have a good way of handling this yet due to the id_ident hack.
  #[objrs(selector = "endEncoding")]
  fn end_encoding(&mut self);
}

impl MTLRenderCommandEncoderId {
  pub fn set_vertex_slice<T: Copy>(&mut self, vertex_slice: &[T], index: usize) {
    assert!(core::mem::size_of::<T>() > 0);
    unsafe {
      return self.set_vertex_bytes_at_index(
        core::mem::transmute(vertex_slice.as_ptr()),
        core::mem::size_of_val(vertex_slice),
        index,
      );
    }
  }

  pub fn set_vertex_struct<T: Copy>(&mut self, vertex_struct: &T, index: usize) {
    assert!(core::mem::size_of::<T>() > 0);
    unsafe {
      return self.set_vertex_bytes_at_index(
        core::mem::transmute(vertex_struct),
        core::mem::size_of::<T>(),
        index,
      );
    }
  }
}
