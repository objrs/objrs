// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

use crate::mtlbuffer::MTLBuffer;
use objrs::{objrs, Id};

#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLBlitCommandEncoder: objrs::marker::Class {
  #[objrs(selector = "copyFromBuffer:sourceOffset:toBuffer:destinationOffset:size:")]
  unsafe fn copy_from_buffer_source_offset_to_buffer_destination_offset_size(
    &mut self,
    source_buffer: &Id<dyn MTLBuffer>,
    source_offset: usize,
    destination_buffer: &mut Id<dyn MTLBuffer>,
    destination_offset: usize,
    size: usize,
  );

  // TODO: this should be in MTLCommandEncoder, and the blit encoder should specify itself as a superset of MTLCommandEncoder. objrs just doesn't have a good way of handling this yet due to the id_ident hack.
  #[objrs(selector = "endEncoding")]
  fn end_encoding(&mut self);
}

pub trait MTLBlitCommandEncoderExt: MTLBlitCommandEncoder {
  fn copy_from_buffer_to_buffer(
    &mut self,
    source_buffer: &Id<dyn MTLBuffer>,
    destination_buffer: &mut Id<dyn MTLBuffer>,
  ) {
    let size = source_buffer.length();
    assert_eq!(size, destination_buffer.length());

    unsafe {
      return self.copy_from_buffer_source_offset_to_buffer_destination_offset_size(
        source_buffer,
        0,
        destination_buffer,
        0,
        size,
      );
    }
  }
}

impl<T: MTLBlitCommandEncoder + ?Sized> MTLBlitCommandEncoderExt for T {}
