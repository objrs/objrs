// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

extern crate core;
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use crate::gen::gensym::RandomIdentifier;
use crate::parse::sel_ref_attr::SelRef;
use proc_macro::Diagnostic;
use proc_macro2::TokenStream;
use quote::quote;
use syn::LitByteStr;

pub fn gen_sel_ref(mut sel_ref: SelRef) -> Result<TokenStream, Diagnostic> {
  let random_id = &RandomIdentifier::new();
  let meth_name_export_name =
    ["\x01L_OBJC_METH_VAR_NAME_.__objrs_meth.", random_id, ".", &sel_ref.sel_string].concat();
  let sel_ref_export_name =
    ["\x01L_OBJC_SELECTOR_REFERENCES_.__objrs_sel.", random_id, ".", &sel_ref.sel_string].concat();

  sel_ref.sel_string.push('\x00');
  let len = sel_ref.sel_string.len();
  let selector = LitByteStr::new(sel_ref.sel_string.as_bytes(), sel_ref.sel.span());

  let ident = sel_ref.ident;
  let u8_ty = sel_ref.u8_ty;
  return Ok(quote! {
    #[link_section = "__DATA,__objc_selrefs,literal_pointers,no_dead_strip"]
    #[export_name = #sel_ref_export_name]
    static #ident: &'static [#u8_ty; #len] = {
      #[link_section = "__TEXT,__objc_methname,cstring_literals"]
      #[export_name = #meth_name_export_name]
      static METH_NAME: [#u8_ty; #len] = * #selector;
      &METH_NAME
    };
  });
}

#[cfg(test)]
mod tests {
  use super::*;
  use syn::parse_quote;

  fn default_sel() -> SelRef {
    let sel = "theSel";
    return SelRef {
      sel: parse_quote!(#sel),
      sel_string: String::from(sel),
      ident: parse_quote!(SEL_REF),
      u8_ty: parse_quote!(objrs::__objrs::u8),
    };
  }

  #[test]
  fn sel_ref() {
    let actual = gen_sel_ref(default_sel()).unwrap();

    let expected = quote! {
      #[link_section = "__DATA,__objc_selrefs,literal_pointers,no_dead_strip"]
      #[export_name = "\u{1}L_OBJC_SELECTOR_REFERENCES_.__objrs_sel._default_fake_random_identifier_.theSel"]
      static SEL_REF: &'static [objrs::__objrs::u8; 7usize] = {
        #[link_section = "__TEXT,__objc_methname,cstring_literals"]
        #[export_name = "\u{1}L_OBJC_METH_VAR_NAME_.__objrs_meth._default_fake_random_identifier_.theSel"]
        static METH_NAME: [objrs::__objrs::u8; 7usize] = *b"theSel\0";
        &METH_NAME
      };
    };
    assert_eq!(actual.to_string(), expected.to_string());
  }
}
