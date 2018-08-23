// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

use mtlbuffer::{MTLBuffer, MTLBufferId};
use objrs::objrs;

#[objrs(protocol, id_ident = MTLBlitCommandEncoderId)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLBlitCommandEncoder: objrs::marker::Class {
  #[objrs(selector = "copyFromBuffer:sourceOffset:toBuffer:destinationOffset:size:")]
  unsafe fn copy_from_buffer_source_offset_to_buffer_destination_offset_size(
    &mut self,
    source_buffer: &MTLBufferId,
    source_offset: usize,
    destination_buffer: &mut MTLBufferId,
    destination_offset: usize,
    size: usize,
  );

  // TODO: this should be in MTLCommandEncoder, and the blit encoder should specify itself as a superset of MTLCommandEncoder. objrs just doesn't have a good way of handling this yet due to the id_ident hack.
  #[objrs(selector = "endEncoding")]
  fn end_encoding(&mut self);
}

impl MTLBlitCommandEncoderId {
  pub fn copy_from_buffer_to_buffer(
    &mut self,
    source_buffer: &MTLBufferId,
    destination_buffer: &mut MTLBufferId,
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
