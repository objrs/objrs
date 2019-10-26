// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

extern crate core;
extern crate objrs_utils;
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

#[cfg(test)]
extern crate objrs_test_utils;

use crate::span_ext::SpanExt;
use syn::{punctuated::Punctuated, token::Comma, AttrStyle, Attribute, FnArg, Ident, Pat, PatType};

#[cfg(not(test))]
pub use objrs_utils::RandomIdentifier;

#[cfg(test)]
pub type RandomIdentifier = objrs_test_utils::FakeRandomIdentifier;

pub fn link_attribute(attrs: &[Attribute]) -> Option<&Attribute> {
  for attr in attrs {
    match attr.style {
      AttrStyle::Outer if attr.path.is_ident("link") => return Some(&attr),
      _ => continue,
    }
  }
  return None;
}

pub fn is_instance_method(args: &Punctuated<FnArg, Comma>) -> bool {
  if args.is_empty() {
    return false;
  }

  let is_self_ident = |pat_ty: &PatType| {
    if let Pat::Ident(ref pat_ident) = *pat_ty.pat {
      return pat_ident.ident == "self";
    }
    return false;
  };

  match args[0] {
    FnArg::Receiver(_) => return true,
    FnArg::Typed(ref pat_ty) => return is_self_ident(pat_ty),
  }
}

pub fn priv_ident(ident: &str) -> proc_macro2::Ident {
  return priv_ident_at(ident, proc_macro2::Span::call_site());
}

pub fn priv_ident_at(ident: &str, span: proc_macro2::Span) -> proc_macro2::Ident {
  return Ident::new(ident, span.resolved_at_def_site());
}

// A re-implementation of drain_filter. See https://github.com/rust-lang/rust/issues/43244. Once
// that's stabalized this can be deleted. But until then, I'll implement it myself. There's too much
// nightly feature creep in objrs. This differs from drain_filter in that it won't run the drain to
// completion if you drop the iterator.
pub trait DrainExt: IntoIterator {
  fn drain<'a, F: FnMut(&mut <Self as IntoIterator>::Item) -> bool>(
    &'a mut self,
    f: F,
  ) -> Drain<'a, <Self as IntoIterator>::Item, F>;
}

impl<T> DrainExt for Vec<T> {
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

pub struct Drain<'a, T: 'a, F: FnMut(&mut T) -> bool> {
  pred: F,
  end: usize,
  read: usize,
  write: usize,
  vec: &'a mut Vec<T>,
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

impl<'a, T, F: FnMut(&mut T) -> bool> core::ops::Drop for Drain<'a, T, F> {
  fn drop(&mut self) {
    let unclaimed = self.end - self.read;
    unsafe {
      core::ptr::copy(
        self.vec.get_unchecked_mut(self.read) as *mut _,
        self.vec.get_unchecked_mut(self.write) as *mut _,
        unclaimed,
      )
    };
    unsafe { self.vec.set_len(self.write + unclaimed) };
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
}
