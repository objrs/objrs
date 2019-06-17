// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

extern crate core;
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

use crate::parse::drain_ext::DrainExt;
use proc_macro::Diagnostic;
use proc_macro2::{Delimiter, Span};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{custom_keyword, AttrStyle, Attribute};

// TODO: rename this file, and maybe split some things out into different files.

pub fn take_objrs_attr(attrs: &mut Vec<Attribute>) -> Result<Option<Attribute>, Diagnostic> {
  let objrs_attr;
  {
    let mut iter = DrainExt::drain(attrs, |attr: &mut Attribute| {
      return attr.style == AttrStyle::Outer && attr.path.is_ident("objrs");
    });
    objrs_attr = iter.next();
    if let Some(duplicate_attr) = iter.next() {
      return Err(
        duplicate_attr.span().unstable().error("unexpected secondary objrs attribute found"),
      );
    }
  }

  return Ok(objrs_attr);
}

// pub trait OptionExt {
//   type Type;

//   fn or_else_parse<F>(self, f: F) -> syn::parse::Result<Self>
//   where
//       F: FnOnce() -> syn::parse::Result<Self>, Self: Sized;
// }

// impl<T> OptionExt for Option<T> {
//   type Type = T;

//   fn or_else_parse<F>(self, f: F) -> syn::parse::Result<Option<T>>
//   where
//       F: FnOnce() -> syn::parse::Result<Option<T>> {
//       match self {
//           Some(value) => return Ok(Some(value)),
//           None => return f(),
//       }
//   }
// }

pub trait Key: Parse {
  fn display() -> &'static str;
}

macro_rules! impl_token_key {
  ($ty:ty) => {
    impl Key for $ty {
      fn display() -> &'static str {
        return <$ty as syn::token::Token>::display();
      }
    }
  };
}

macro_rules! impl_custom_key {
  ($ident:ident) => {
    custom_keyword!($ident);
    impl Key for $ident {
      fn display() -> &'static str {
        return concat!("`", stringify!($ident), "`");
      }
    }
  };
}

impl_token_key!(syn::token::Extern);
impl_token_key!(syn::token::Impl);
impl_token_key!(syn::token::Super);
impl_token_key!(syn::token::Type);

impl_custom_key!(assign);
impl_custom_key!(atomic);
impl_custom_key!(category_name);
impl_custom_key!(class);
impl_custom_key!(class_name);
impl_custom_key!(copy);
impl_custom_key!(default);
impl_custom_key!(getter);
impl_custom_key!(id_ident);
impl_custom_key!(instance);
impl_custom_key!(ivar);
impl_custom_key!(name);
impl_custom_key!(no_impl);
impl_custom_key!(nonatomic);
impl_custom_key!(nonnull);
impl_custom_key!(null_resettable);
impl_custom_key!(null_unspecified);
impl_custom_key!(nullable);
impl_custom_key!(objrs);
impl_custom_key!(optional);
impl_custom_key!(property);
impl_custom_key!(protocol);
impl_custom_key!(protocol_name);
impl_custom_key!(readonly);
impl_custom_key!(readwrite);
impl_custom_key!(retain);
impl_custom_key!(root_class);
impl_custom_key!(selector);
impl_custom_key!(setter);
impl_custom_key!(strong);
impl_custom_key!(unsafe_unretained);
impl_custom_key!(weak);

pub trait KeyTuple {
  fn extend(vec: &mut Vec<&'static str>);

  fn parse_or_nop(input: ParseStream) -> Option<Span>;
}

fn step_to(input: ParseStream, target: ParseStream) -> Span {
  let span = input.cursor().span();
  input
    .step(|cursor| {
      let mut cursor = *cursor;
      while cursor != target.cursor() {
        // span = span.join(cursor.span()).unwrap();
        cursor = cursor.token_tree().unwrap().1;
      }
      return Ok(((), cursor));
    })
    .unwrap();
  return span;
}

fn parse_or_nop<K: Key>(input: ParseStream) -> Option<Span> {
  let fork = input.fork();
  if fork.parse::<K>().is_err() {
    return None;
  }
  return Some(step_to(input, &fork));
}

macro_rules! impl_key_tuple {
  ($($ident:ident)*) => {
    impl<$($ident: Key,)*> KeyTuple for ($($ident,)*) {
      fn extend(vec: &mut Vec<&'static str>) {
        vec.extend(&[$(<$ident as Key>::display(),)*]);
      }

      fn parse_or_nop(input: ParseStream) -> Option<Span> {
        return None $(.or_else(|| parse_or_nop::<$ident>(input)))* ;
        // $(
        //   if let Some(span) = parse_or_nop::<$ident>(input) {
        //     return Some(span);
        //   }
        // )*
        // return None;
      }
    }
  }
}

impl_key_tuple!(T0);
impl_key_tuple!(T0 T1);
impl_key_tuple!(T0 T1 T2);
impl_key_tuple!(T0 T1 T2 T3);

pub trait Value {
  fn parse(input: ParseStream, key_span: Span) -> syn::parse::Result<Self>
  where
    Self: Sized;

  fn default() -> Option<Self>
  where
    Self: Sized,
  {
    return None;
  }
}

impl Value for () {
  fn parse(_: ParseStream, _: Span) -> syn::parse::Result<Self> {
    return Ok(());
  }
}

impl Value for Span {
  fn parse(_: ParseStream, key_span: Span) -> syn::parse::Result<Self> {
    return Ok(key_span);
  }
}

impl<T: Value> Value for Option<T> {
  fn parse(input: ParseStream, key_span: Span) -> syn::parse::Result<Self> {
    let value: T = <T as Value>::parse(input, key_span)?;
    return Ok(Some(value));
  }

  fn default() -> Option<Self>
  where
    Self: Sized,
  {
    return Some(None);
  }
}

macro_rules! impl_value_eq_parse {
  ($ty:ty) => {
    impl Value for $ty {
      fn parse(input: ParseStream, _: Span) -> syn::parse::Result<Self>
      where
        Self: Sized,
      {
        let _: syn::token::Eq = input.parse()?;
        return input.parse();
      }
    }
  };
}

impl_value_eq_parse!(syn::Expr);
impl_value_eq_parse!(syn::Ident);
impl_value_eq_parse!(syn::LitStr);
impl_value_eq_parse!(syn::TypePath);

fn first_token(cursor: Cursor) -> String {
  if let Some((ident, _)) = cursor.ident() {
    return format!("`{}`", ident);
  }
  if let Some((punct, _)) = cursor.punct() {
    return format!("`{}`", punct);
  }
  if let Some((literal, _)) = cursor.literal() {
    return format!("`{}`", literal);
  }
  if let Some((lifetime, _)) = cursor.lifetime() {
    return format!("`{}`", lifetime);
  }
  if cursor.group(Delimiter::Parenthesis).is_some() {
    return "`(`".to_string();
  }
  if cursor.group(Delimiter::Brace).is_some() {
    return "`{`".to_string();
  }
  if cursor.group(Delimiter::Bracket).is_some() {
    return "`[`".to_string();
  } else {
    return "`)`".to_string();
  }
}

pub struct KV<'a> {
  input: ParseStream<'a>,
  expected: Vec<&'static str>,
}

impl<'a> KV<'a> {
  pub fn new(input: ParseStream<'a>) -> KV<'a> {
    return KV {
      input: input,
      expected: Vec::new(),
    };
  }

  pub fn parse<K: Key, V: Value>(&mut self) -> syn::parse::Result<V> {
    return self.parse_one_of::<(K,), V>();
  }

  pub fn parse_one_of<K: KeyTuple, V: Value>(&mut self) -> syn::parse::Result<V> {
    if let Some(key_span) = <K as KeyTuple>::parse_or_nop(self.input) {
      self.expected.clear();
      return self.parse_value::<V>(key_span);
    }

    <K as KeyTuple>::extend(&mut self.expected);
    if let Some(default) = <V as Value>::default() {
      return Ok(default);
    }
    return Err(self.error_from_expected());
  }

  pub fn barrier(&mut self) -> syn::parse::Result<()> {
    if self.expected.is_empty() {
      return Ok(());
    }
    return Err(self.error_from_expected());
  }

  pub fn eof(&mut self) -> syn::parse::Result<()> {
    if self.input.is_empty() {
      return Ok(());
    }
    self.expected.push("`)`");
    return Err(self.error_from_expected());
  }

  pub fn error<T: std::fmt::Display>(&self, message: T) -> syn::parse::Error {
    return self.input.error(message);
  }

  fn parse_value<V: Value>(&mut self, key_span: Span) -> syn::parse::Result<V> {
    let value = V::parse(self.input, key_span)?;

    if !self.input.is_empty() {
      let _: syn::token::Comma = self.input.parse()?;
    }

    return Ok(value);
  }

  fn error_from_expected(&mut self) -> syn::parse::Error {
    self.expected.sort();
    let message;
    if self.expected.len() == 1 {
      message = self.expected[0].to_string();
    } else if self.expected.len() == 2 {
      message = format!("one of {} or {}", self.expected[0], self.expected[1]);
    } else {
      let (last, elements) = self.expected.split_last().unwrap();
      message = format!("one of {}, or {}", elements.join(", "), last);
    }
    return self.error(format!("expected {}, found {}", message, first_token(self.input.cursor())));
  }
}
