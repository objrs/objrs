// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

use crate::mtlbuffer::MTLBuffer;
use crate::mtlcommand_queue::MTLCommandQueue;
use crate::mtlcompile_options::MTLCompileOptions;
use crate::mtllibrary::MTLLibrary;
use crate::mtlrender_pipeline::{MTLRenderPipelineDescriptor, MTLRenderPipelineState};
use crate::mtlresource::MTLResourceOptions;
use objrs::{objrs, Id};
use objrs_frameworks_foundation::{NSError, NSString};

#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLDevice {
  #[objrs(selector = "newLibraryWithSource:options:error:")]
  fn new_library_with_source_options_error(
    &mut self,
    source: &NSString,
    options: Option<&MTLCompileOptions>,
    error: Option<&mut Option<objrs::Strong<NSError>>>,
  ) -> Option<objrs::Strong<Id<dyn MTLLibrary>>>;

  #[objrs(selector = "newRenderPipelineStateWithDescriptor:error:")]
  fn new_render_pipeline_state_with_descriptor_error(
    &mut self,
    descriptor: &MTLRenderPipelineDescriptor,
    error: Option<&mut Option<objrs::Strong<NSError>>>,
  ) -> Option<objrs::Strong<Id<dyn MTLRenderPipelineState>>>;

  #[objrs(selector = "newCommandQueue")]
  fn new_command_queue(&mut self) -> Option<objrs::Strong<Id<dyn MTLCommandQueue>>>;

  #[objrs(selector = "newBufferWithBytes:length:options:")]
  unsafe fn new_buffer_with_bytes_length_options(
    &mut self,
    pointer: &libc::c_void,
    length: usize,
    options: MTLResourceOptions,
  ) -> Option<objrs::Strong<Id<dyn MTLBuffer>>>;

  #[objrs(selector = "newBufferWithLength:options:")]
  fn new_buffer_with_length_options(
    &mut self,
    length: usize,
    options: MTLResourceOptions,
  ) -> Option<objrs::Strong<Id<dyn MTLBuffer>>>;
}

pub trait MTLDeviceExt: MTLDevice {
  fn new_buffer_with_slice_options<T: Copy>(
    &mut self,
    bytes: &[T],
    options: MTLResourceOptions,
  ) -> Option<objrs::Strong<Id<dyn MTLBuffer>>> {
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

impl<T: MTLDevice + ?Sized> MTLDeviceExt for T {}

#[allow(non_snake_case)]
#[inline(always)]
pub fn MTLCreateSystemDefaultDevice() -> Option<objrs::Strong<Id<dyn MTLDevice>>> {
  #[link(name = "Metal", kind = "framework")]
  extern "C" {
    // TODO: file a feature request to get Option<T> whitelisted for FFI (where sizeof(T) == sizeof(Option<T>) and T is whitelisted for FFI).
    #[allow(improper_ctypes)]
    fn MTLCreateSystemDefaultDevice() -> Option<objrs::Strong<Id<dyn MTLDevice>>>;
  }
  return unsafe { MTLCreateSystemDefaultDevice() };
}
