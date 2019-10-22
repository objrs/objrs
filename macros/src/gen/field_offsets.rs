// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

extern crate core;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use crate::parse::class_attr::Class;
use crate::util::priv_ident;
use proc_macro2::{Literal, TokenStream, TokenTree};
use quote::quote;

#[allow(unused)] // TODO: remove this.
fn field_offsets(class: &Class) -> TokenStream {
  let iter = class.item.fields.iter();
  if iter.len() == 0 {
    return quote!([]);
  }

  let objrs_root = &class.objrs;
  let base_ref = priv_ident("base_ref");
  let base_u8_ptr = priv_ident("base_u8_ptr");
  let mut offset_tokens = TokenStream::new();
  for (i, field) in iter.enumerate() {
    let field_ident: TokenTree =
      field.ident.clone().map_or_else(|| Literal::usize_unsuffixed(i).into(), |ident| ident.into());

    offset_tokens.extend(quote! {
      (& #base_ref.#field_ident as *const _ as *const #objrs_root::__objrs::u8)
          .offset_from(#base_u8_ptr) as #objrs_root::__objrs::usize,
    });
  }

  // TODO: support generics.
  let ty = &class.item.ident;
  let base = priv_ident("base");
  return quote! {{
    let #base = #objrs_root::__objrs::core::mem::MaybeUninit::<#ty>::uninit();

    unsafe {
      // I really wish Rust had a better, safer way to compute field offsets (in consts).
      let #base_ref = #objrs_root::__objrs::TransmuteHack::<_, &#ty> { from: & #base }.to;
      let #base_u8_ptr = #base_ref as *const _ as *const #objrs_root::__objrs::u8;

      [#offset_tokens]
    }
  }};
}

#[cfg(test)]
mod tests {
  extern crate objrs_test_utils;

  use super::*;
  use crate::parse::class_attr::ClassAttr;
  use objrs_test_utils::assert_tokens_eq;
  use syn::parse_quote;

  fn default_class_attr() -> ClassAttr {
    return ClassAttr {
      class_name: None,
      super_class_name: None,
      super_class: None,
      force_extern: false,
      root_class_name: None,
      objrs: None,
    };
  }

  #[test]
  fn no_fields() {
    let class = Class::new(
      default_class_attr(),
      parse_quote! {
        struct Struct;
      },
    )
    .unwrap();

    let actual = field_offsets(&class);
    let expected = quote!([]);
    assert_tokens_eq!(actual, expected);
  }

  #[test]
  fn named_fields() {
    let class = Class::new(
      default_class_attr(),
      parse_quote! {
        struct Struct {
          foo: u8,
          bar: f32,
          baz: char,
        }
      },
    )
    .unwrap();

    let actual = field_offsets(&class);
    let expected = quote! {{
      let base = objrs::__objrs::core::mem::MaybeUninit::<Struct>::uninit();

      unsafe {
        let base_ref = objrs::__objrs::TransmuteHack::<_, &Struct> { from: &base }.to;
        let base_u8_ptr = base_ref as *const _ as *const objrs::__objrs::u8;

        [
          (&base_ref.foo as *const _ as *const objrs::__objrs::u8).offset_from(base_u8_ptr) as objrs::__objrs::usize,
          (&base_ref.bar as *const _ as *const objrs::__objrs::u8).offset_from(base_u8_ptr) as objrs::__objrs::usize,
          (&base_ref.baz as *const _ as *const objrs::__objrs::u8).offset_from(base_u8_ptr) as objrs::__objrs::usize,
        ]
      }
    }};

    assert_tokens_eq!(actual, expected);
  }

  #[test]
  fn unnamed_fields() {
    let class = Class::new(
      default_class_attr(),
      parse_quote! {
        struct Struct(u8, f32, char);
      },
    )
    .unwrap();

    let actual = field_offsets(&class);
    let expected = quote! {{
      let base = objrs::__objrs::core::mem::MaybeUninit::<Struct>::uninit();

      unsafe {
        let base_ref = objrs::__objrs::TransmuteHack::<_, &Struct> { from: &base }.to;
        let base_u8_ptr = base_ref as *const _ as *const objrs::__objrs::u8;

        [
          (&base_ref.0 as *const _ as *const objrs::__objrs::u8).offset_from(base_u8_ptr) as objrs::__objrs::usize,
          (&base_ref.1 as *const _ as *const objrs::__objrs::u8).offset_from(base_u8_ptr) as objrs::__objrs::usize,
          (&base_ref.2 as *const _ as *const objrs::__objrs::u8).offset_from(base_u8_ptr) as objrs::__objrs::usize,
        ]
      }
    }};

    assert_tokens_eq!(actual, expected);
  }
}
