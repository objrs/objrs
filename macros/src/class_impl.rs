// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

extern crate core;
extern crate proc_macro;
extern crate proc_macro2;

use parser::AttributeParser;
use proc_macro::Diagnostic;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{buffer::TokenBuffer, spanned::Spanned, synom::Synom, ItemImpl};

pub fn parse_impl(
  mut args_parser: AttributeParser,
  input: TokenStream,
) -> Result<TokenStream, Diagnostic> {
  args_parser.eof()?;

  let input = TokenBuffer::new2(input);
  let item = match <ItemImpl as Synom>::parse(input.begin()) {
    Ok((item_impl, _)) => item_impl,
    Err(error) => {
      return Err(
        input
          .begin()
          .token_stream()
          .span()
          .unstable()
          .error(format!("failed to parse impl item: {}", error.to_string()))
          .note("#[objrs(impl)] must only be applied to a struct's impl block"),
      );
    }
  };

  let tokens = quote!{
    #item
  };

  return Ok(tokens.into());
}
