// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

// TODO: Use span's resolved_at/located_at to use def_site() span with line/column information of user code.

#![feature(proc_macro_diagnostic, proc_macro_span, proc_macro_def_site)]
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
use proc_macro2::{Span, TokenStream};
use protocol::{parse_protocol, ProtocolAttr};
use quote::quote;
use syn::parse2;
use syn::parse::{ParseStream, Parse};

mod class;
mod class_impl;
mod gensym;
mod ivar;
mod property;
mod protocol;
mod selector;
mod test;
mod util;

enum ObjrsAttr {
  Impl(ImplAttr),
  Class(ClassAttr),
  Protocol(ProtocolAttr),
  Selector(Span),
  Ivar(Span),
}

impl Parse for ObjrsAttr {
  fn parse(input: ParseStream) -> syn::parse::Result<Self> {
    use syn::token::Impl;
    use util::{ivar, selector, class, protocol};

    let lookahead = input.lookahead1();
    if lookahead.peek(Impl) {
      return input.parse().map(ObjrsAttr::Impl);
    } else if lookahead.peek(class) {
      return input.parse().map(ObjrsAttr::Class);
    } else if lookahead.peek(protocol) {
      return input.parse().map(ObjrsAttr::Protocol);
    } else if lookahead.peek(selector) {
      return Ok(ObjrsAttr::Selector(input.cursor().span()));
    } else if lookahead.peek(ivar) {
      return Ok(ObjrsAttr::Ivar(input.cursor().span()));
    }
    return Err(lookahead.error());
  }
}

fn to_diagnostic(err: syn::parse::Error) -> Diagnostic {
  return err.span().unstable().error(err.to_string());
}

fn shim(args: TokenStream, input: TokenStream) -> Result<TokenStream, Diagnostic> {
  match parse2(args).map_err(to_diagnostic)? {
    ObjrsAttr::Impl(attr) => return parse_impl(attr, input),
    ObjrsAttr::Class(attr) => return parse_class(attr, input),
    ObjrsAttr::Protocol(attr) => return parse_protocol(attr, input),
    ObjrsAttr::Selector(span) => {
      return Err(
        span
          .unstable()
          .error("attribute must be enclosed in an impl block with a #[objrs(impl)] attribute"),
      )
    }
    ObjrsAttr::Ivar(span) => {
      return Err(
        span
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
