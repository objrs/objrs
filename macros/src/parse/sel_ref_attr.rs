// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

extern crate proc_macro2;
extern crate syn;

use proc_macro::Diagnostic;
use proc_macro2::TokenStream;
use syn::{parse2, Ident, ItemStatic, LitStr, Type};

// TODO: move this somewhere else.
/// Validates the selector and returns its string value and the number of arguments (excluding self)
/// it takes.
pub fn validate_selector(sel: &LitStr) -> Result<(String, usize), Diagnostic> {
  let string = sel.value();
  let mut allow_digit = false;
  let mut expected_arg_count = 0;
  let mut ends_with_colon = false;
  for byte in string.bytes() {
    if !byte.is_ascii() {
      return Err(
        sel
          .span()
          .unstable()
          .error(format!("selector contains non-ASCII byte `0x{:02x?}`", byte))
          .note("Objective-C only permits ASCII characters in selector names"),
      );
    }
    if byte.is_ascii_control() {
      return Err(
        sel
          .span()
          .unstable()
          .error(format!("selector contains ASCII control character `0x{:02x?}`", byte))
          .note("Objective-C does not permit ASCII control characters in selector names"),
      );
    }
    if byte.is_ascii_whitespace() {
      return Err(
        sel
          .span()
          .unstable()
          .error(format!(
            "selector contains whitespace character `{}` (ASCII value `0x{:02x?}`)",
            byte as char, byte
          ))
          .note("Objective-C does not permit whitespace in selector names"),
      );
    }
    if byte != b':' && byte != b'_' && byte != b'$' && byte.is_ascii_punctuation() {
      let error_msg = format!(
        "selector contains punctuation character `{}` (ASCII value `0x{:02x?}`)",
        byte as char, byte
      );
      let note_msg =
        "Objective-C does not permit punctuation (except for `:`, `_`, and `$`) in selector names";
      return Err(sel.span().unstable().error(error_msg).note(note_msg));
    }
    if !allow_digit && byte.is_ascii_digit() {
      return Err(
        sel
          .span()
          .unstable()
          .error(format!(
            "selector contains digit `{}` (ASCII value `0x{:02x?}`) in an illegal location",
            byte as char, byte
          ))
          .note("digits must be preceded by an alphabetic, `_`, or `$` character"),
      );
    }
    let is_colon = byte == b':';
    allow_digit = !is_colon;
    expected_arg_count += is_colon as usize;
    ends_with_colon = is_colon;
  }

  if expected_arg_count > 0 && !ends_with_colon {
    return Err(
      sel
        .span()
        .unstable()
        .error("selector expects arguments but doesn't end with `:`")
        .note("the last character in a selector that takes arguments must be `:`"),
    );
  }

  return Ok((string, expected_arg_count));
}

fn parse_and_validate_selector(args: TokenStream) -> Result<(LitStr, String, usize), Diagnostic> {
  let sel;
  match parse2::<LitStr>(args) {
    Ok(value) => {
      sel = value;
    }
    Err(err) => {
      return Err(err.span().unstable().error(err.to_string()).note(
        "selector! requires a single string literal. Examples: selector!(\"terminate:\"); \
         nsstring!(\"length\"); nsstring!(r#\"doFoo:withBar:\"#);",
      ));
    }
  }

  let (string, args) = validate_selector(&sel)?;
  return Ok((sel, string, args));
}

pub struct SelRef {
  pub sel: LitStr,
  pub sel_string: String,
  pub ident: Ident,
  pub u8_ty: Box<Type>,
}

impl SelRef {
  pub fn new(args: TokenStream, input: TokenStream) -> Result<SelRef, (Diagnostic, TokenStream)> {
    let sel;
    let sel_string;
    match parse_and_validate_selector(args) {
      Ok((lit, string, _)) => {
        sel = lit;
        sel_string = string;
      }
      Err(diagnostic) => {
        return Err((diagnostic, input));
      }
    }

    let item = parse2::<ItemStatic>(input).expect("BUG: failed to parse static item");

    let ty_ref = if let Type::Reference(ty_ref) = *item.ty { Some(ty_ref) } else { None };
    let ty_ref = ty_ref.unwrap();
    let array = if let Type::Array(array) = *ty_ref.elem { Some(array) } else { None };
    let array = array.unwrap();
    let u8_ty = array.elem;

    return Ok(SelRef {
      sel: sel,
      sel_string: sel_string,
      ident: item.ident,
      u8_ty: u8_ty,
    });
  }
}
