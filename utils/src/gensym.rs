// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

extern crate core;

#[link(name = "c")]
extern "C" {
  fn arc4random_buf(buf: *mut u8, nbytes: usize);
}

#[inline(always)]
fn random<T>() -> T {
  let mut val = core::mem::MaybeUninit::uninit();
  unsafe {
    arc4random_buf(val.as_mut_ptr() as *mut u8, core::mem::size_of::<T>());
    return val.assume_init();
  }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct RandomIdentifier([u8; 32]);

impl core::ops::Deref for RandomIdentifier {
  type Target = str;

  fn deref(&self) -> &str {
    return unsafe { core::str::from_utf8_unchecked(&self.0) };
  }
}

impl RandomIdentifier {
  #[inline]
  pub fn new() -> RandomIdentifier {
    let uuid = random::<[u64; 2]>();
    let mask = 0x0f0f0f0f0f0f0f0f;
    let aaaaaaaa = 0x6161616161616161;
    let symbol: [u64; 4] = [
      aaaaaaaa + (uuid[0] & mask),
      aaaaaaaa + ((uuid[0] >> 4) & mask),
      aaaaaaaa + (uuid[1] & mask),
      aaaaaaaa + ((uuid[1] >> 4) & mask),
    ];
    let id = unsafe { core::mem::transmute(symbol) };
    return RandomIdentifier(id);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new() {
    // Make sure we always generate valid ASCII identifiers.
    for _ in 0..100 {
      let ident_str: &str = &RandomIdentifier::new();
      for c in ident_str.bytes() {
        assert!(b'a' <= c && c <= b'p');
      }
    }
  }
}
