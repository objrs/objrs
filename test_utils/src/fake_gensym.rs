// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

extern crate core;

use core::cell::Cell;

thread_local! {
  static ID: Cell<[u8; 32]> = Cell::new(*b"_default_fake_random_identifier_");
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct FakeRandomIdentifier([u8; 32]);

impl core::ops::Deref for FakeRandomIdentifier {
  type Target = str;

  fn deref(&self) -> &str {
    return unsafe { core::str::from_utf8_unchecked(&self.0) };
  }
}

impl std::fmt::Debug for FakeRandomIdentifier {
  fn fmt(&self, fmt: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
    return core::fmt::Debug::fmt(&**self, fmt);
  }
}

impl FakeRandomIdentifier {
  #[inline]
  pub fn new() -> FakeRandomIdentifier {
    return FakeRandomIdentifier(ID.with(|global_id| global_id.get()));
  }

  #[cfg(test)]
  pub fn set(id: &'static [u8; 32]) {
    assert!(id.iter().all(|&c| c.is_ascii_alphanumeric() || c == b'_'));
    ID.with(|global_id| global_id.set(*id));
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basics() {
    assert_eq!(&*FakeRandomIdentifier::new(), "_default_fake_random_identifier_");

    FakeRandomIdentifier::set(b"hello_world_new_identifier_here_");
    assert_eq!(&*FakeRandomIdentifier::new(), "hello_world_new_identifier_here_");
  }

  #[test]
  #[should_panic]
  fn test_bad_identifier() {
    FakeRandomIdentifier::set(b"this-is-a-bad-idetifier-zzzzzzzz");
  }
}
