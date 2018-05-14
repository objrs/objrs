// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

#![feature(proc_macro, proc_macro_non_items)]
#![recursion_limit = "256"]

extern crate core;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate proc_macro2;
extern crate syn;

use class::parse_class;
use class_impl::parse_impl;
use parser::AttributeParser;
use proc_macro::Diagnostic;
use proc_macro2::TokenStream;
use selector::parse_selector;
use syn::buffer::TokenBuffer;

mod class;
mod class_impl;
mod gensym;
mod parser;
mod selector;
mod test;

fn shim(args: TokenStream, input: TokenStream) -> Result<TokenStream, Diagnostic> {
  let args_buffer = TokenBuffer::new2(args);

  let mut args_parser = AttributeParser::new(args_buffer.begin());
  let key = args_parser.any_keyword(&["class", "impl", "selector"])?;

  match key.as_ref() {
    "class" => return parse_class(args_parser, input),
    "impl" => return parse_impl(args_parser, input),
    "selector" => return parse_selector(args_parser, input),
    _ => panic!(
      "BUG: unexpected token `{}` as first argument in attribute",
      key
    ),
  };
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
      return proc_macro::TokenStream::empty();
    }
  }
}

// #[proc_macro]
// pub fn objrs_external(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//   let iter = input.into_iter().map(|mut token_tree| {
//     match token_tree {
//       TokenTree::Op(op) if op.op() == ';' && op.spacing() == Spacing::Alone => {
//         token_tree = TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::empty()))
//       }
//       _ => (),
//     }

//     return token_tree;
//   });
//   return iter.collect();
// }

#[proc_macro]
pub fn nothing(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
  return proc_macro::TokenStream::empty();
}

#[proc_macro]
pub fn debug(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  panic!("{:?}", input);
}
