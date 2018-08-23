// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

extern crate core;

// For Objective-C property attributes, see https://github.com/llvm-mirror/clang/blob/master/lib/Parse/ParseObjc.cpp

use syn::{
  alt, call, cond_reduce, custom_keyword, do_parse, named, option, parens, punct, syn,
  synom::Synom, Ident, LitStr, Type,
};

pub enum ReadWrite {
  ReadOnly(Ident),
  ReadWrite(Ident),
}

pub enum Nullability {
  Nullable(Ident),
  Nonnull(Ident),
  Unspecified(Ident),
}

pub enum WeakStrong {
  Weak(Ident),
  Strong(Ident),
}

pub enum Atomicity {
  Atomic(Ident),
  Nonatomic(Ident),
}

fn is_weak(weak_strong: &Option<WeakStrong>) -> bool {
  if let Some(WeakStrong::Weak(_)) = weak_strong {
    return true;
  }
  return false;
}

pub struct PropertyAttr {
  pub name: Ident,
  pub ty: Type,
  pub class: Option<Ident>,
  pub read_write: Option<ReadWrite>,
  pub nullability: Option<Nullability>,
  pub weak_strong: Option<WeakStrong>,
  pub atomicity: Option<Atomicity>,
  pub retain: Option<Ident>,
  pub null_resettable: Option<Ident>,
  pub getter: Option<LitStr>,
  pub setter: Option<LitStr>,
}

// Example: property(prop1: i32, readonly, nonatomic, getter = "foo").
// TODO: weak, copy, retain, and strong all require an object type. This should be checked in a
// post-parse validation step.
impl Synom for PropertyAttr {
  named!(parse -> Self, do_parse!(
    custom_keyword!(property) >>
    property: parens!(do_parse!(
      name: syn!(Ident) >>
      punct!(:) >>
      ty: syn!(Type) >>
      class: option!(do_parse!(punct!(,) >> value: custom_keyword!(class) >> (value))) >>
      read_write: option!(do_parse!(
        punct!(,) >>
        value: alt!(
          custom_keyword!(readonly) => { |ident| ReadWrite::ReadOnly(ident) }
          |
          custom_keyword!(readwrite) => { |ident| ReadWrite::ReadWrite(ident) }
        ) >> (value)
      )) >>
      assign: option!(do_parse!(
        punct!(,) >>
        assign: custom_keyword!(assign) >>
        (assign)
      )) >>
      unsafe_unretained: option!(do_parse!(
        punct!(,) >>
        unsafe_unretained: custom_keyword!(unsafe_unretained) >>
        (unsafe_unretained)
      )) >>
      copy: option!(cond_reduce!(assign.is_none() && unsafe_unretained.is_none(), do_parse!(
        punct!(,) >>
        value: custom_keyword!(copy) >>
        (value)
      ))) >>
      weak_strong: option!(cond_reduce!(assign.is_none() && unsafe_unretained.is_none() && copy.is_none(), do_parse!(
        punct!(,) >>
        value: alt!(
          custom_keyword!(weak) => { |ident| WeakStrong::Weak(ident) }
          |
          custom_keyword!(strong) => { |ident| WeakStrong::Strong(ident) }
        ) >> (value)
      ))) >>
      retain: option!(cond_reduce!(assign.is_none() && unsafe_unretained.is_none() && copy.is_none() && !is_weak(&weak_strong), do_parse!(
        punct!(,) >>
        value: custom_keyword!(retain) >>
        (value)
      ))) >>
      nullability: option!(do_parse!(
        punct!(,) >>
        value: alt!(
          custom_keyword!(nullable) => { |ident| Nullability::Nullable(ident) }
          |
          cond_reduce!(!is_weak(&weak_strong), custom_keyword!(nonnull)) => { |ident| Nullability::Nonnull(ident) }
          |
          custom_keyword!(null_unspecified) => { |ident| Nullability::Unspecified(ident) }
        ) >> (value)
      )) >>
      atomicity: option!(do_parse!(
        punct!(,) >>
        value: alt!(
          custom_keyword!(atomic) => { |ident| Atomicity::Atomic(ident) }
          |
          custom_keyword!(nonatomic) => { |ident| Atomicity::Nonatomic(ident) }
        ) >> (value)
      )) >>
      null_resettable: option!(do_parse!(
        punct!(,) >>
        value: custom_keyword!(null_resettable) >>
        (value)
      )) >>
      getter: option!(do_parse!(
        punct!(,) >>
        custom_keyword!(getter) >> punct!(=) >>
        selector: syn!(LitStr) >>
        (selector)
      )) >>
      setter: option!(do_parse!(
        punct!(,) >>
        custom_keyword!(setter) >> punct!(=) >>
        selector: syn!(LitStr) >>
        (selector)
      )) >>
      option!(punct!(,)) >>
      (PropertyAttr {
        name: name,
        ty: ty,
        class: class,
        read_write: read_write,
        nullability: nullability,
        weak_strong: weak_strong,
        atomicity: atomicity,
        retain: retain,
        null_resettable: null_resettable,
        getter: getter,
        setter: setter,
      })
    )) >> (
      property.1
    )
  ));

  fn description() -> Option<&'static str> {
    return Some("objrs property attribute");
  }
}

impl PropertyAttr {
  fn readonly(&self) -> bool {
    if let Some(ReadWrite::ReadOnly(_)) = self.read_write {
      return true;
    }
    return false;
  }

  pub fn getter(&self) -> LitStr {
    if let Some(ref getter) = self.getter {
      return getter.clone();
    }
    return LitStr::new(&self.name.to_string(), self.name.span());
  }

  pub fn setter(&self) -> Option<LitStr> {
    if let Some(ref setter) = self.setter {
      return Some(setter.clone());
    }
    if self.readonly() {
      return None;
    }
    let mut name = self.name.to_string();
    unsafe {
      match name.as_bytes_mut().first_mut() {
        Some(ref mut byte) if b'a' <= **byte && **byte <= b'z' => **byte ^= 0x20,
        _ => (),
      }
    }
    name.push(':');
    let name = &["set", &name].concat();
    return Some(LitStr::new(name, self.name.span()));
  }
}
