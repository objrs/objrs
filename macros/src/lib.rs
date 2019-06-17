// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

#![feature(proc_macro_diagnostic, proc_macro_span, proc_macro_def_site)]
#![recursion_limit = "512"]

extern crate core;
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use crate::class::transform_class;
use crate::class_impl::transform_impl;
use crate::gen::class_ref::gen_class_ref;
use crate::gen::gen_protocol::transform_protocol;
use crate::gen::sel_ref::gen_sel_ref;
use crate::parse::class_attr::{Class, ClassAttr};
use crate::parse::class_ref_attr::ClassRef;
use crate::parse::impl_attr::ImplAttr;
use crate::parse::protocol_attr::{Protocol, ProtocolAttr};
use crate::parse::sel_ref_attr::SelRef;
use proc_macro::Diagnostic;
use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::parse2;

mod class;
mod class_impl;
mod gen;
mod parse;
mod selector;
mod span_ext;
mod util;

enum ObjrsAttr {
  Impl(ImplAttr),
  Class(ClassAttr),
  Protocol(ProtocolAttr),
}

impl ObjrsAttr {
  fn transform(self, input: TokenStream) -> Result<TokenStream, Diagnostic> {
    match self {
      ObjrsAttr::Impl(attr) => return transform_impl(attr, input),
      ObjrsAttr::Class(attr) => return transform_class(Class::new(input, attr)?),
      ObjrsAttr::Protocol(attr) => return transform_protocol(Protocol::new(input, attr)?),
    }
  }
}

impl Parse for ObjrsAttr {
  fn parse(input: ParseStream) -> syn::parse::Result<Self> {
    use crate::parse::attr::{class, ivar, protocol, selector};
    use syn::token::Impl;

    let lookahead = input.lookahead1();
    if lookahead.peek(Impl) {
      return input.parse().map(ObjrsAttr::Impl);
    } else if lookahead.peek(class) {
      return input.parse().map(ObjrsAttr::Class);
    } else if lookahead.peek(protocol) {
      return input.parse().map(ObjrsAttr::Protocol);
    } else if lookahead.peek(selector) {
      return Err(syn::parse::Error::new(
        input.cursor().span(),
        "attribute must be enclosed in an impl block with a #[objrs(impl)] attribute",
      ));
    } else if lookahead.peek(ivar) {
      return Err(syn::parse::Error::new(
        input.cursor().span(),
        "attribute must be enclosed in a struct item with a #[objrs(class)] attribute",
      ));
    }
    return Err(lookahead.error());
  }
}

fn parse_err_to_diagnostic(err: syn::parse::Error) -> Diagnostic {
  return err.span().unstable().error(err.to_string());
}

#[proc_macro_attribute]
pub fn objrs(
  args: proc_macro::TokenStream,
  input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  let args: TokenStream = args.into();
  let input: TokenStream = input.into();
  let result = parse2::<ObjrsAttr>(args).map_err(parse_err_to_diagnostic);
  match result.and_then(|attr| attr.transform(input)) {
    Ok(stream) => return stream.into(),
    Err(diagnostic) => {
      diagnostic.emit();
      return proc_macro::TokenStream::new();
    }
  }
}

fn diagnostic_to_pair(diagnostic: Diagnostic) -> (Diagnostic, TokenStream) {
  return (diagnostic, TokenStream::new());
}

#[proc_macro_attribute]
pub fn sel_ref(
  args: proc_macro::TokenStream,
  input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  let args: TokenStream = args.into();
  let input: TokenStream = input.into();
  let result =
    SelRef::new(args, input).and_then(|sel_ref| gen_sel_ref(sel_ref).map_err(diagnostic_to_pair));
  match result {
    Ok(stream) => return stream.into(),
    Err((diagnostic, input)) => {
      diagnostic.emit();
      return input.into();
    }
  }
}

#[proc_macro_attribute]
pub fn class_ref(
  args: proc_macro::TokenStream,
  input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  let args: TokenStream = args.into();
  let input: TokenStream = input.into();
  let result = ClassRef::new(args, input)
    .and_then(|class_ref| gen_class_ref(class_ref).map_err(diagnostic_to_pair));
  match result {
    Ok(stream) => return stream.into(),
    Err((diagnostic, input)) => {
      diagnostic.emit();
      return input.into();
    }
  }
}

// TODO: decide whether or not to stablize this. If so, maybe parse the #[objrs()] macros and
// transform them to make sure they explicitly say `extern` so users don't have to repeat that.
// #[proc_macro]
// pub fn objrs_external(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//   let iter = input.into_iter().map(|mut token_tree| {
//     match token_tree {
//       TokenTree::Op(op) if op.op() == ';' && op.spacing() == Spacing::Alone => {
//         token_tree = TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::new()))
//       }
//       _ => (),
//     }
//
//     return token_tree;
//   });
//   return iter.collect();
// }
