// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

// For Objective-C property attributes, see https://github.com/llvm-mirror/clang/blob/master/lib/Parse/ParseObjc.cpp

use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitStr, Type};

pub enum ReadWrite {
  ReadOnly(Span),
  ReadWrite(Span),
}

pub enum Nullability {
  Nullable(Span),
  Nonnull(Span),
  Unspecified(Span),
}

pub enum WeakStrong {
  Weak(Span),
  Strong(Span),
}

pub enum Atomicity {
  Atomic(Span),
  Nonatomic(Span),
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
  pub class: Option<Span>,
  pub read_write: Option<ReadWrite>,
  pub nullability: Option<Nullability>,
  pub weak_strong: Option<WeakStrong>,
  pub atomicity: Option<Atomicity>,
  pub retain: Option<Span>,
  pub null_resettable: Option<Span>,
  pub getter: Option<LitStr>,
  pub setter: Option<LitStr>,
}

// Example: property(prop1: i32, readonly, nonatomic, getter = "foo").
// TODO: weak, copy, retain, and strong all require an object type. This should be checked in a
// post-parse validation step.
impl Parse for PropertyAttr {
  fn parse(input: ParseStream) -> syn::parse::Result<Self> {
    use crate::parse::attr::{
      assign, atomic, class, copy, getter, nonatomic, nonnull, null_resettable, null_unspecified,
      nullable, readonly, readwrite, retain, setter, strong, unsafe_unretained, weak, KV,
    };
    use syn::parenthesized;
    use syn::token::{Colon, Comma, Paren};

    let content;
    let _: Paren = parenthesized!(content in input);
    let input = &content;

    let name: Ident = input.parse()?;
    let _: Colon = input.parse()?;
    let ty: Type = input.parse()?;

    if input.is_empty() {
      return Ok(PropertyAttr {
        name: name,
        ty: ty,
        class: None,
        read_write: None,
        nullability: None,
        weak_strong: None,
        atomicity: None,
        retain: None,
        null_resettable: None,
        getter: None,
        setter: None,
      });
    }

    let _: Comma = input.parse()?;

    let mut kv = KV::new(input);

    let class: Option<Span> = kv.parse::<class, _>()?;

    // use util::OptionExt;
    // let read_write = kv.parse::<readonly, Option<Span>>()?.map(ReadWrite::ReadOnly)
    //   .or_else_parse(|| kv.parse::<readwrite, _>().map(ReadWrite::ReadWrite))?;
    let readonly: Option<Span> = kv.parse::<readonly, _>()?;
    let readwrite: Option<Span> =
      if readonly.is_some() { None } else { kv.parse::<readwrite, _>()? };
    let read_write = readonly.map(ReadWrite::ReadOnly).or(readwrite.map(ReadWrite::ReadWrite));

    // let readonly: Option<Span> = kv.parse::<readonly, _>()?;
    // let readwrite: Option<Span> = if readonly.is_some() { None } else { kv.parse::<readwrite, _>()? };
    // let read_write;
    // if let Some(span) = readonly {
    //   readwrite = Some(ReadWrite::ReadOnly(span));
    // } else if let Some(span) = readwrite {
    //   readwrite = Some(ReadWrite::ReadWrite(span));
    // } else {
    //   read_write = None;
    // }

    let assign: Option<Span> = kv.parse::<assign, _>()?;

    let unsafe_unretained: Option<Span> = kv.parse::<unsafe_unretained, _>()?;

    let copy: Option<Span>;
    if assign.is_none() && unsafe_unretained.is_none() {
      copy = kv.parse::<copy, _>()?;
    } else {
      copy = None;
    }

    let weak_strong;
    if assign.is_none() && unsafe_unretained.is_none() && copy.is_none() {
      let weak: Option<Span> = kv.parse::<weak, _>()?;
      let strong: Option<Span> = if weak.is_some() { None } else { kv.parse::<strong, _>()? };
      weak_strong = weak.map(WeakStrong::Weak).or(strong.map(WeakStrong::Strong));
    // if weak.is_some() {
    //   weak_strong = Some(WeakStrong::Weak(ident));
    // } else if strong.is_some() {
    //   weak_strong = Some(WeakStrong::Strong(ident));
    // } else {
    //   weak_strong = None;
    // }
    } else {
      weak_strong = None;
    }

    let retain: Option<Span>;
    if assign.is_none() && unsafe_unretained.is_none() && copy.is_none() && !is_weak(&weak_strong) {
      retain = kv.parse::<retain, _>()?;
    } else {
      retain = None;
    }

    let nullable: Option<Span> = kv.parse::<nullable, _>()?;
    let nonnull: Option<Span>;
    if nullable.is_none() && !is_weak(&weak_strong) {
      nonnull = kv.parse::<nonnull, _>()?;
    } else {
      nonnull = None;
    }
    let null_unspecified: Option<Span>;
    if nullable.is_none() && nonnull.is_none() {
      null_unspecified = kv.parse::<null_unspecified, _>()?;
    } else {
      null_unspecified = None;
    }
    let nullability = nullable
      .map(Nullability::Nullable)
      .or(nonnull.map(Nullability::Nonnull))
      .or(null_unspecified.map(Nullability::Unspecified));
    // if nullable.is_some() {
    //   nullability = Some(Nullability::Nullable(ident));
    // } else if nonnull.is_some() {
    //   nullability = Some(Nullability::Nonnull(ident));
    // } else if null_unspecified.is_some()() {
    //   nullability = Some(Nullability::Unspecified(ident));
    // } else {
    //   nullability = None;
    // }

    let atomic: Option<Span> = kv.parse::<atomic, _>()?;
    let nonatomic: Option<Span> = if atomic.is_some() { None } else { kv.parse::<nonatomic, _>()? };
    let atomicity = atomic.map(Atomicity::Atomic).or(nonatomic.map(Atomicity::Nonatomic));
    // if atomic.is_some() {
    //   atomicity = Some(Atomicity::Atomic(ident));
    // } else if nonatomic.is_some() {
    //   atomicity = Some(Atomicity::Nonatomic(ident));
    // } else {
    //   atomicity = None;
    // }

    let null_resettable: Option<Span> = kv.parse::<null_resettable, _>()?;

    let getter: Option<LitStr> = kv.parse::<getter, _>()?;
    let setter: Option<LitStr> = kv.parse::<setter, _>()?;

    kv.eof()?;
    return Ok(PropertyAttr {
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
    });
  }
}

impl crate::parse::attr::Value for PropertyAttr {
  fn parse(input: ParseStream, _: Span) -> syn::parse::Result<Self>
  where
    Self: Sized,
  {
    use syn::parenthesized;

    let content;
    let _: syn::token::Paren = parenthesized!(content in input);
    return content.parse();
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
