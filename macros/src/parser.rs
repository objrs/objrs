// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

extern crate core;
extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::Diagnostic;
use syn::{buffer::Cursor, synom::Synom, Ident, LitStr};

pub struct AttributeParser<'a>(Cursor<'a>);

impl<'a> AttributeParser<'a> {
  pub fn new(cursor: Cursor<'a>) -> AttributeParser<'a> {
    return AttributeParser(cursor);
  }

  pub fn eof(&mut self) -> Result<(), Diagnostic> {
    if let Some((_, next)) = ','.parse(self.0) {
      self.0 = next;
    }

    if let Some((tree, _)) = self.0.token_tree() {
      return Err(
        tree
          .span()
          .unstable()
          .error(format!("expected `)`, found `{}`", tree)),
      );
    }

    return Ok(());
  }

  pub fn string(&mut self) -> Result<LitStr, Diagnostic> {
    return self.expect_one_of(&[StringLiteral]);
  }

  pub fn op(&mut self, op: char) -> Result<(), Diagnostic> {
    return self.expect_one_of(&[op]);
  }

  pub fn keyword(&mut self, keyword: &str) -> Result<Ident, Diagnostic> {
    return self.expect_one_of(&[keyword]);
  }

  pub fn any_keyword(&mut self, keywords: &[&str]) -> Result<Ident, Diagnostic> {
    return self.expect_one_of(keywords);
  }

  fn expect_one_of<T: Parse>(&mut self, allowed_tokens: &[T]) -> Result<T::Type, Diagnostic> {
    // Examples of rustc error messages:
    // "expected identifier, found keyword `XXX`"
    // "expected `{}`, found `{}`"
    // "expected one of `{}`(, (`{}`,)+)? or `{}`, found `{}`"
    // "expected identifier, found reserved identifier `_`""
    if self.0.eof() {
      // TODO: report EOF.
    }

    for expected in allowed_tokens {
      if let Some((value, next)) = expected.parse(self.0) {
        self.0 = next;
        return Ok(value);
      }
    }

    let actual_tree = self.0.token_tree().unwrap().0; // TODO: this is a hack. handle EOF better.

    let expected_message;
    if allowed_tokens.len() == 1 {
      expected_message = allowed_tokens[0].expected_message();
    } else if allowed_tokens.len() == 2 {
      expected_message = format!(
        "one of {} or {}",
        allowed_tokens[0].expected_message(),
        allowed_tokens[1].expected_message()
      );
    } else {
      let mut string = "one of ".to_string();
      for token in &allowed_tokens[..allowed_tokens.len() - 1] {
        string.push_str(&token.expected_message());
        string.push_str(", ");
      }
      string.push_str("or ");
      string.push_str(&allowed_tokens[allowed_tokens.len() - 1].expected_message());
      expected_message = string;
    };

    return Err(actual_tree.span().unstable().error(format!(
      "expected {}, found `{}`",
      expected_message, actual_tree
    )));
  }
}

trait Parse {
  type Type;

  fn parse<'c>(&self, cursor: Cursor<'c>) -> Option<(Self::Type, Cursor<'c>)>;

  fn expected_message(&self) -> String;
}

impl<'a> Parse for &'a str {
  type Type = Ident;

  fn parse<'c>(&self, cursor: Cursor<'c>) -> Option<(Self::Type, Cursor<'c>)> {
    let (term, next) = cursor.term()?;
    if term.as_str() != *self {
      return None;
    }
    return Some((Ident::new(term.as_str(), term.span()), next));
  }

  fn expected_message(&self) -> String {
    return format!("`{}`", self);
  }
}

impl Parse for char {
  type Type = ();

  fn parse<'c>(&self, cursor: Cursor<'c>) -> Option<(Self::Type, Cursor<'c>)> {
    match cursor.op() {
      Some((op, next)) if op.op() == *self => return Some(((), next)),
      _ => return None,
    }
  }

  fn expected_message(&self) -> String {
    return format!("`{}`", self);
  }
}

struct StringLiteral;

impl Parse for StringLiteral {
  type Type = LitStr;

  fn parse<'c>(&self, cursor: Cursor<'c>) -> Option<(Self::Type, Cursor<'c>)> {
    return <Self::Type as Synom>::parse(cursor).ok();
  }

  fn expected_message(&self) -> String {
    return "string literal".to_string();
  }
}
