// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

// A re-implementation of drain_filter. See https://github.com/rust-lang/rust/issues/43244. This
// differs from drain_filter in that it won't run the drain to completion if you drop the iterator.
pub trait DrainExt: IntoIterator {
  fn drain<'a, F: FnMut(&mut <Self as IntoIterator>::Item) -> bool>(
    &'a mut self,
    f: F,
  ) -> Drain<'a, <Self as IntoIterator>::Item, F>;
}

pub struct Drain<'a, T: 'a, F: FnMut(&mut T) -> bool> {
  pred: F,
  end: usize,
  read: usize,
  write: usize,
  vec: &'a mut Vec<T>,
}

impl<'a, T, F: FnMut(&mut T) -> bool> core::ops::Drop for Drain<'a, T, F> {
  fn drop(&mut self) {
    let unclaimed = self.end - self.read;
    unsafe {
      core::ptr::copy(
        self.vec.get_unchecked_mut(self.read) as *mut _,
        self.vec.get_unchecked_mut(self.write) as *mut _,
        unclaimed,
      );
      self.vec.set_len(self.write + unclaimed);
    }
  }
}

impl<'a, T, F: FnMut(&mut T) -> bool> core::iter::Iterator for Drain<'a, T, F> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    let mut iter = self.read;
    while iter != self.end {
      let current = unsafe { self.vec.get_unchecked_mut(iter) };
      if (self.pred)(current) {
        break;
      }

      iter += 1;
    }

    let skip_count = iter - self.read;

    let result;
    if iter != self.end {
      result = Some(unsafe { core::ptr::read(self.vec.get_unchecked_mut(iter) as *mut _) });
      iter += 1;
    } else {
      result = None;
    }

    unsafe {
      core::ptr::copy(
        self.vec.get_unchecked_mut(self.read) as *mut _,
        self.vec.get_unchecked_mut(self.write) as *mut _,
        skip_count,
      )
    };
    self.read = iter;
    self.write += skip_count;

    return result;
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    return (0, Some(self.end - self.read));
  }
}

impl<T> DrainExt for Vec<T> {
  #[inline]
  fn drain<'a, F: FnMut(&mut T) -> bool>(&'a mut self, f: F) -> Drain<'a, T, F> {
    let len = self.len();
    unsafe {
      self.set_len(0);
    }

    return Drain {
      pred: f,
      end: len,
      read: 0,
      write: 0,
      vec: self,
    };
  }
}

#[cfg(test)]
#[test]
fn drain_test() {
  let mut v = vec![1, 2, 3, 4];
  let d = DrainExt::drain(&mut v, |x| *x % 2 == 0).collect::<Vec<_>>();
  assert_eq!(v, vec![1, 3]);
  assert_eq!(d, vec![2, 4]);

  let mut v = vec![2, 4, 6, 8];
  let d = DrainExt::drain(&mut v, |x| *x % 2 == 0).collect::<Vec<_>>();
  assert_eq!(v, vec![]);
  assert_eq!(d, vec![2, 4, 6, 8]);

  let mut v = vec![1, 3, 5, 7];
  let d = DrainExt::drain(&mut v, |x| *x % 2 == 0).collect::<Vec<_>>();
  assert_eq!(v, vec![1, 3, 5, 7]);
  assert_eq!(d, vec![]);

  let mut v = vec!["a".to_string(), "a".to_string(), "b".to_string(), "b".to_string()];
  let d = DrainExt::drain(&mut v, |x| *x == "b").collect::<Vec<_>>();
  assert_eq!(v, vec!["a", "a"]);
  assert_eq!(d, vec!["b", "b"]);

  let mut v = vec![1, 2, 3, 4];
  let mut d = DrainExt::drain(&mut v, |x| *x % 2 == 0);
  assert_eq!(d.size_hint(), (0, Some(4)));
  assert_eq!(d.next(), Some(2));
  assert_eq!(d.size_hint(), (0, Some(2)));
  assert_eq!(d.next(), Some(4));
  assert_eq!(d.size_hint(), (0, Some(0)));
  assert_eq!(d.next(), None);
  assert_eq!(d.size_hint(), (0, Some(0)));

  let mut v = vec![1, 2, 3, 4];
  {
    let mut d = DrainExt::drain(&mut v, |x| *x % 2 == 0);
    assert_eq!(d.next(), Some(2));
  }
  assert_eq!(v, vec![1, 3, 4]);

  let mut v = vec![(), (), (), ()];
  let d = DrainExt::drain(&mut v, |_| false).collect::<Vec<_>>();
  assert_eq!(v, vec![(), (), (), ()]);
  assert_eq!(d, vec![]);

  let mut v = vec![(), (), (), ()];
  let d = DrainExt::drain(&mut v, |_| true).collect::<Vec<_>>();
  assert_eq!(v, vec![]);
  assert_eq!(d, vec![(), (), (), ()]);

  let mut v = vec![(), (), (), ()];
  let mut i = 0;
  let d = DrainExt::drain(&mut v, |_| {
    i += 1;
    return i % 2 != 0;
  })
  .collect::<Vec<_>>();
  assert_eq!(v, vec![(), ()]);
  assert_eq!(d, vec![(), ()]);
}
