// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

extern crate core;
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
use syn::{punctuated::Punctuated, token::Comma, ArgCaptured, Attribute, FnArg, Pat};

pub fn plural_s<T: From<u8> + core::cmp::PartialEq + Copy>(value: T) -> &'static str {
  if value == T::from(1u8) {
    return "";
  } else {
    return "s";
  }
}

pub fn link_attribute(attrs: &[Attribute]) -> Option<&Attribute> {
  for attr in attrs {
    let path = &attr.path;
    if path.leading_colon.is_none() && path.segments.len() == 1 && path.segments[0].ident == "link"
    {
      return Some(&attr);
    }
  }
  return None;
}

pub fn is_instance_method(args: &Punctuated<FnArg, Comma>) -> bool {
  if args.is_empty() {
    return false;
  }

  let is_self_ident = |arg: &ArgCaptured| {
    if let Pat::Ident(ref pat_ident) = arg.pat {
      return pat_ident.ident == "self";
    }
    return false;
  };

  match args[0] {
    FnArg::SelfRef(_) => return true,
    FnArg::SelfValue(_) => return true,
    FnArg::Captured(ref arg) => return is_self_ident(arg),
    _ => return false,
  }
}

#[cfg(procmacro2_semver_exempt)]
pub fn priv_ident(ident: &str) -> proc_macro2::Ident {
  return Ident::new(ident, proc_macro2::Span::def_site());
}

#[cfg(not(procmacro2_semver_exempt))]
pub fn priv_ident(ident: &str) -> proc_macro2::Ident {
  return priv_ident_at(ident, proc_macro2::Span::call_site());
}

#[cfg(procmacro2_semver_exempt)]
pub fn priv_ident_at(ident: &str, span: proc_macro2::Span) -> proc_macro2::Ident {
  let span = span.resolved_at(proc_macro2::Span::def_site());
  return Ident::new(ident, span);
}

#[cfg(not(procmacro2_semver_exempt))]
pub fn priv_ident_at(ident: &str, span: proc_macro2::Span) -> proc_macro2::Ident {
  let span = span.unstable().resolved_at(proc_macro::Span::def_site());
  let ident = proc_macro::Ident::new(ident, span);
  let tree: proc_macro::TokenTree = ident.into();
  let stream: proc_macro::TokenStream = tree.into();
  let stream: proc_macro2::TokenStream = stream.into();
  let tree = stream.into_iter().next().expect("BUG: unexpected EOF, expected a single ident token");
  match tree {
    proc_macro2::TokenTree::Ident(ident) => return ident,
    _ => panic!("BUG: unexpected token tree; expected a single ident token"),
  }
}

fn span_to_span2(span: proc_macro::Span) -> proc_macro2::Span {
  let mut punct = proc_macro::Punct::new('!', proc_macro::Spacing::Alone);
  punct.set_span(span);
  let tree: proc_macro::TokenTree = punct.into();
  let stream: proc_macro::TokenStream = tree.into();
  let stream: proc_macro2::TokenStream = stream.into();
  let tree = stream.into_iter().next().expect("BUG: unexpected EOF, expected a single punct token");
  match tree {
    proc_macro2::TokenTree::Punct(punct) => return punct.span(),
    _ => panic!("BUG: unexpected token tree; expected a single punct token"),
  }
}

pub trait SpanExt {
  fn resolved_at_def_site(self: Self) -> Self;
}

impl SpanExt for proc_macro2::Ident {
  #[cfg(procmacro2_semver_exempt)]
  fn resolved_at_def_site(mut self: Self) -> Self {
    let span = self.span().resolved_at(proc_macro2::Span::def_site());
    self.set_span(span);
    return self;
  }

  #[cfg(not(procmacro2_semver_exempt))]
  fn resolved_at_def_site(mut self: Self) -> Self {
    let span = self.span().unstable().resolved_at(proc_macro::Span::def_site());
    self.set_span(span_to_span2(span));
    return self;
  }
}

impl SpanExt for proc_macro2::Punct {
  #[cfg(procmacro2_semver_exempt)]
  fn resolved_at_def_site(mut self: Self) -> Self {
    let span = self.span().resolved_at(proc_macro2::Span::def_site());
    self.set_span(span);
    return self;
  }

  #[cfg(not(procmacro2_semver_exempt))]
  fn resolved_at_def_site(mut self: Self) -> Self {
    let span = self.span().unstable().resolved_at(proc_macro::Span::def_site());
    self.set_span(span_to_span2(span));
    return self;
  }
}

impl SpanExt for proc_macro2::Literal {
  #[cfg(procmacro2_semver_exempt)]
  fn resolved_at_def_site(mut self: Self) -> Self {
    let span = self.span().resolved_at(proc_macro2::Span::def_site());
    self.set_span(span);
    return self;
  }

  #[cfg(not(procmacro2_semver_exempt))]
  fn resolved_at_def_site(mut self: Self) -> Self {
    let span = self.span().unstable().resolved_at(proc_macro::Span::def_site());
    self.set_span(span_to_span2(span));
    return self;
  }
}

impl SpanExt for proc_macro2::Group {
  #[cfg(procmacro2_semver_exempt)]
  fn resolved_at_def_site(self: Self) -> Self {
    let span = self.span().resolved_at(proc_macro2::Span::def_site());
    let stream = self.stream().resolved_at_def_site();
    let mut group = proc_macro2::Group::new(self.delimiter(), stream);
    group.set_span(span);
    return group;
  }

  #[cfg(not(procmacro2_semver_exempt))]
  fn resolved_at_def_site(self: Self) -> Self {
    let span = self.span().unstable().resolved_at(proc_macro::Span::def_site());
    let stream = self.stream().resolved_at_def_site();
    let mut group = proc_macro2::Group::new(self.delimiter(), stream);
    group.set_span(span_to_span2(span));
    return group;
  }
}

impl SpanExt for proc_macro2::TokenTree {
  fn resolved_at_def_site(self: Self) -> Self {
    match self {
      proc_macro2::TokenTree::Group(group) => {
        return proc_macro2::TokenTree::Group(group.resolved_at_def_site())
      }
      proc_macro2::TokenTree::Ident(ident) => {
        return proc_macro2::TokenTree::Ident(ident.resolved_at_def_site())
      }
      proc_macro2::TokenTree::Punct(punct) => {
        return proc_macro2::TokenTree::Punct(punct.resolved_at_def_site())
      }
      proc_macro2::TokenTree::Literal(literal) => {
        return proc_macro2::TokenTree::Literal(literal.resolved_at_def_site())
      }
    }
  }
}

impl SpanExt for proc_macro2::TokenStream {
  fn resolved_at_def_site(self: Self) -> Self {
    return self.into_iter().map(|tree| tree.resolved_at_def_site()).collect();
  }
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
