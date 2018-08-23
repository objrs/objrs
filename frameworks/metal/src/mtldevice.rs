// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

use mtlbuffer::MTLBufferId;
use mtlcommand_queue::MTLCommandQueueId;
use mtlcompile_options::MTLCompileOptions;
use mtllibrary::MTLLibraryId;
use mtlrender_pipeline::{MTLRenderPipelineDescriptor, MTLRenderPipelineStateId};
use mtlresource::MTLResourceOptions;
use objrs::objrs;
use objrs_frameworks_foundation::{NSError, NSString};

#[objrs(protocol, id_ident = MTLDeviceId)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLDevice {
  #[objrs(selector = "newLibraryWithSource:options:error:")]
  fn new_library_with_source_options_error(
    &mut self,
    source: &NSString,
    options: Option<&MTLCompileOptions>,
    error: Option<&mut Option<objrs::Strong<NSError>>>,
  ) -> Option<objrs::Strong<MTLLibraryId>>;

  #[objrs(selector = "newRenderPipelineStateWithDescriptor:error:")]
  fn new_render_pipeline_state_with_descriptor_error(
    &mut self,
    descriptor: &MTLRenderPipelineDescriptor,
    error: Option<&mut Option<objrs::Strong<NSError>>>,
  ) -> Option<objrs::Strong<MTLRenderPipelineStateId>>;

  #[objrs(selector = "newCommandQueue")]
  fn new_command_queue(&mut self) -> Option<objrs::Strong<MTLCommandQueueId>>;

  #[objrs(selector = "newBufferWithBytes:length:options:")]
  unsafe fn new_buffer_with_bytes_length_options(
    &mut self,
    pointer: &libc::c_void,
    length: usize,
    options: MTLResourceOptions,
  ) -> Option<objrs::Strong<MTLBufferId>>;

  #[objrs(selector = "newBufferWithLength:options:")]
  fn new_buffer_with_length_options(
    &mut self,
    length: usize,
    options: MTLResourceOptions,
  ) -> Option<objrs::Strong<MTLBufferId>>;
}

impl MTLDeviceId {
  pub fn new_buffer_with_slice_options<T: Copy>(
    &mut self,
    bytes: &[T],
    options: MTLResourceOptions,
  ) -> Option<objrs::Strong<MTLBufferId>> {
    assert!(core::mem::size_of::<T>() > 0);
    unsafe {
      return self.new_buffer_with_bytes_length_options(
        core::mem::transmute(bytes.as_ptr()),
        core::mem::size_of_val(bytes),
        options,
      );
    }
  }
}

#[allow(non_snake_case)]
#[inline(always)]
pub fn MTLCreateSystemDefaultDevice() -> Option<objrs::Strong<MTLDeviceId>> {
  #[link(name = "Metal", kind = "framework")]
  extern "C" {
    // TODO: file a feature request to get Option<T> whitelisted for FFI (where sizeof(T) == sizeof(Option<T>) and T is whitelisted for FFI).
    #[allow(improper_ctypes)]
    fn MTLCreateSystemDefaultDevice() -> Option<objrs::Strong<MTLDeviceId>>;
  }
  return unsafe { MTLCreateSystemDefaultDevice() };
}
