// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

// TODO: Use span's resolved_at/located_at to use def_site() span with line/column information of user code.

#![feature(proc_macro_diagnostic, proc_macro_span)]
#![recursion_limit = "512"]

extern crate core;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use class::{parse_class, ClassAttr};
use class_impl::{parse_impl, ImplAttr};
use proc_macro::Diagnostic;
use proc_macro2::TokenStream;
use protocol::{parse_protocol, ProtocolAttr};
use quote::quote;
use syn::{
  alt, buffer::Cursor, buffer::TokenBuffer, call, custom_keyword, do_parse, named, syn,
  synom::PResult, synom::Synom,
};

mod class;
mod class_impl;
mod gensym;
mod ivar;
mod property;
mod protocol;
mod selector;
mod test;
mod util;

fn consume_all_tokens(_: Cursor) -> PResult<()> {
  return Ok(((), Cursor::empty()));
}

enum ObjrsAttr {
  Impl(ImplAttr),
  Class(ClassAttr),
  Protocol(ProtocolAttr),
  Selector,
  Ivar,
}

impl Synom for ObjrsAttr {
  named!(parse -> Self, do_parse!(
    attr: alt!(
      syn!(ImplAttr) =>  { ObjrsAttr::Impl }
      |
      syn!(ClassAttr) =>  { ObjrsAttr::Class }
      |
      syn!(ProtocolAttr) =>  { ObjrsAttr::Protocol }
      |
      do_parse!(custom_keyword!(selector) >> call!(consume_all_tokens) >> ()) =>  { |_| ObjrsAttr::Selector }
      |
      do_parse!(custom_keyword!(ivar) >> call!(consume_all_tokens) >> ()) =>  { |_| ObjrsAttr::Ivar }
    ) >> (attr)
  ));

  fn description() -> Option<&'static str> {
    return Some("objrs attribute");
  }
}

fn parse<T: Synom>(cursor: Cursor) -> Result<T, Diagnostic> {
  let (value, cursor) =
    <T as Synom>::parse(cursor).map_err(|err| cursor.span().unstable().error(err.to_string()))?;
  if cursor.eof() {
    return Ok(value);
  } else {
    return Err(cursor.span().unstable().error("tokens were not parsed"));
  }
}

fn shim(args: TokenStream, input: TokenStream) -> Result<TokenStream, Diagnostic> {
  let args_buffer = TokenBuffer::new2(args);
  let cursor = args_buffer.begin();
  match parse(cursor)? {
    ObjrsAttr::Impl(attr) => return parse_impl(attr, input),
    ObjrsAttr::Class(attr) => return parse_class(attr, input),
    ObjrsAttr::Protocol(attr) => return parse_protocol(attr, input),
    ObjrsAttr::Selector => {
      return Err(
        cursor
          .span()
          .unstable()
          .error("attribute must be enclosed in an impl block with a #[objrs(impl)] attribute"),
      )
    }
    ObjrsAttr::Ivar => {
      return Err(
        cursor
          .span()
          .unstable()
          .error("attribute must be enclosed in a struct item with a #[objrs(class)] attribute"),
      )
    }
  }
}

#[proc_macro_attribute]
pub fn objrs(
  args: proc_macro::TokenStream,
  input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  match shim(args.into(), input.into()) {
    Ok(stream) => return stream.into(),
    Err(diagnostic) => {
      diagnostic.emit();
      return proc_macro::TokenStream::new();
    }
  }
}

#[proc_macro]
pub fn selector(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  match selector::parse_selector_ref(input.into()) {
    Ok(stream) => return stream.into(),
    Err(diagnostic) => {
      diagnostic.emit();
      // Return an empty SEL. This should help other diagnostic messages.
      return quote!{{
        extern crate objrs as __objrs_root;
        __objrs_root::runtime::SEL(0 as *const _)
      }}.into();
    }
  }
}

// #[proc_macro]
// pub fn objrs_external(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//   let iter = input.into_iter().map(|mut token_tree| {
//     match token_tree {
//       TokenTree::Op(op) if op.op() == ';' && op.spacing() == Spacing::Alone => {
//         token_tree = TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::new()))
//       }
//       _ => (),
//     }

//     return token_tree;
//   });
//   return iter.collect();
// }

// #[proc_macro_attribute]
// pub fn hack(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//   use quote::quote;
//   let pub_foo = proc_macro2::Ident::new("Foo", proc_macro2::Span::call_site());
//   let priv_foo: proc_macro::TokenTree = proc_macro::Ident::new("Foo", proc_macro::Span::def_site()).into();//proc_macro2::Ident::new("Foo", proc_macro2::Span::def_site());
//   let priv_foo: proc_macro::TokenStream = priv_foo.into();
//   let priv_foo: proc_macro2::TokenStream = priv_foo.into();
//   let foo = quote!{
//     pub struct #priv_foo{
//       x: u32,
//     }
//     impl core::ops::Deref for #pub_foo {
//       type Target = #priv_foo;

//       #[inline(always)]
//       fn deref(&self) -> &Self::Target {
//         return unsafe { core::mem::transmute(self) };
//       }
//     }
//   };
//   let input: TokenStream = input.into();
//   return quote!{
//     #input
//     #foo
//   }.into();
// }

// #[proc_macro]
// pub fn nothing(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
//   return proc_macro::TokenStream::new();
// }

// #[proc_macro]
// pub fn debug(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//   panic!("{:?}", input);
// }
