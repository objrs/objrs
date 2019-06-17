// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

extern crate core;

fn decompress_lzma<'a, T: Copy>(src: &[u8], dst: &'a mut [T]) -> &'a mut [T] {
  const COMPRESSION_LZMA: i32 = 0x306;
  #[link(name = "compression")]
  extern "C" {
    fn compression_decode_buffer(
      dst_buffer: *mut u8,
      dst_size: usize,
      src_buffer: *const u8,
      src_size: usize,
      scratch_buffer: *mut u8,
      algorithm: i32,
    ) -> usize;
  }
  let len;
  unsafe {
    len = compression_decode_buffer(
      dst.as_mut_ptr() as *mut u8,
      dst.len() * core::mem::size_of::<T>(),
      src.as_ptr(),
      src.len(),
      core::ptr::null_mut(),
      COMPRESSION_LZMA,
    ) / core::mem::size_of::<T>();
  }
  return dst.split_at_mut(len).0;
}

pub fn load_triangles() -> [[[f32; 2]; 3]; 16141] {
  // Logo taken from: https://www.rust-lang.org/logos/rust-logo-blk.svg
  // I cleaned it up in Inkscape to make it a single path object. I then triangulated it with:
  // https://github.com/mattdesl/svg-mesh-3d
  // This triangulated mesh was written to a file (in binary form) and then compressed using LZMA.

  // There are 16141 triangles. This decompresses to an array of 96846 i16 values, each value in the
  // range [-32767, 32767]. Each value represents a normalized float in the range [-1.0, 1.0]. A
  // single triangle is defined by 6 values (each point is a 2D (x, y) coordinate).
  static LOGO_LZMA_BYTES: [u8; 67448] = *include_bytes!("logo_triangles.i16.xz");

  let mut triangles_i16: [i16; 96846] = unsafe { core::mem::uninitialized() };
  let triangles_i16 = decompress_lzma(&LOGO_LZMA_BYTES, &mut triangles_i16);
  assert_eq!(triangles_i16.len(), 96846);

  let mut triangles_f32: [f32; 96846] = unsafe { core::mem::uninitialized() };
  for (src, dst) in triangles_i16.iter().zip(triangles_f32.iter_mut()) {
    *dst = *src as f32 * (1.0 / 32767.0);
  }

  return unsafe { core::mem::transmute(triangles_f32) };
}
