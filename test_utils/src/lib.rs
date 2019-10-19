// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Item;

mod diff;
mod fake_gensym;
mod fmt_tokens;

pub use diff::Diff;
pub use fake_gensym::FakeRandomIdentifier;
pub use fmt_tokens::FormatTokens;

pub struct TokenEqHelper(Item);

impl TokenEqHelper {
  pub fn new<T: ToTokens>(value: &T) -> TokenEqHelper {
    return TokenEqHelper(Item::Verbatim(<T as ToTokens>::to_token_stream(value)));
  }
}

impl PartialEq for TokenEqHelper {
  fn eq(&self, other: &TokenEqHelper) -> bool {
    return self.0 == other.0;
  }

  fn ne(&self, other: &TokenEqHelper) -> bool {
    return self.0 != other.0;
  }
}

impl ToTokens for TokenEqHelper {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    return self.0.to_tokens(tokens);
  }

  fn to_token_stream(&self) -> TokenStream {
    return self.0.to_token_stream();
  }
  fn into_token_stream(self) -> TokenStream
  where
    Self: Sized,
  {
    return self.0.into_token_stream();
  }
}

impl Eq for TokenEqHelper {}

#[macro_export]
macro_rules! assert_tokens_eq {
  ($actual:expr, $expected:expr) => {{
    let actual = $crate::TokenEqHelper::new(&$actual);
    let expected = $crate::TokenEqHelper::new(&$expected);
    if actual != expected {
      let actual_str = format!("{}", $crate::FormatTokens(actual));
      let expected_str = format!("{}", $crate::FormatTokens(expected));
      let diff =
        $crate::Diff::new(stringify!($expected), &expected_str, stringify!($actual), &actual_str);
      panic!("\n{}\n", diff);
    }
  }};
}
