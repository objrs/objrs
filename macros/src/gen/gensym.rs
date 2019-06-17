// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

extern crate core;

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
    #[cfg(not(test))]
    use real::new;

    #[cfg(test)]
    use fake::new;

    return RandomIdentifier(new());
  }

  #[cfg(test)]
  pub fn set(id: &'static [u8; 32]) {
    fake::set(id);
  }
}

mod real {
  #[link(name = "c")]
  extern "C" {
    fn arc4random_buf(buf: *mut u8, nbytes: usize);
  }

  #[inline(always)]
  fn random<T>() -> T {
    let mut buf = unsafe { core::mem::uninitialized() };
    unsafe {
      arc4random_buf(&mut buf as *mut _ as *mut u8, core::mem::size_of::<T>());
    }
    return buf;
  }

  #[inline]
  pub fn new() -> [u8; 32] {
    let uuid = random::<[u64; 2]>();
    let mask = 0x0f0f0f0f0f0f0f0f;
    let aaaaaaaa = 0x6161616161616161;
    let symbol: [u64; 4] = [
      aaaaaaaa + (uuid[0] & mask),
      aaaaaaaa + ((uuid[0] >> 4) & mask),
      aaaaaaaa + (uuid[1] & mask),
      aaaaaaaa + ((uuid[1] >> 4) & mask),
    ];
    return unsafe { core::mem::transmute(symbol) };
  }

  #[cfg(test)]
  mod tests {
    use super::new;

    #[test]
    fn test_new() {
      // Make sure we always generate valid ASCII identifiers.
      for _ in 0..100 {
        for c in new().iter() {
          assert!(b'a' <= *c && *c <= b'p');
        }
      }
    }
  }

}

#[cfg(test)]
mod fake {
  use core::cell::Cell;

  static DEFAULT_ID: [u8; 32] = *b"_default_fake_random_identifier_";
  thread_local! {
    static ID: Cell<[u8; 32]> = Cell::new(DEFAULT_ID);
  }

  pub fn new() -> [u8; 32] {
    return ID.with(|global_id| global_id.get());
  }

  pub fn set(id: &[u8; 32]) {
    assert!(id.iter().all(|&c| c.is_ascii_alphanumeric() || c == b'_'));
    ID.with(|global_id| global_id.set(*id));
  }

  mod tests {
    use super::{new, set, DEFAULT_ID};

    #[test]
    fn test_basics() {
      assert_eq!(new(), DEFAULT_ID);

      let new_id = b"hello_world_new_identifier_here_";
      set(new_id);
      assert_eq!(new(), *new_id);
    }

    #[test]
    #[should_panic]
    fn test_non_ascii() {
      set(b"this-is-a-bad-idetifier-zzzzzzzz");
    }
  }
}

#[cfg(test)]
mod tests {
  use super::RandomIdentifier;

  #[test]
  fn test_plumbing() {
    let new_id = b"abcdefghijklmnopqrstuvwxyz012345";
    RandomIdentifier::set(new_id);
    assert_eq!(RandomIdentifier::new().0, *new_id);
  }
}
