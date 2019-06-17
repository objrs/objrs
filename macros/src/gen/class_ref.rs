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
use crate::parse::class_ref_attr::ClassRef;
use crate::util::priv_ident;
use proc_macro::Diagnostic;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_quote, Ident, LitStr};

fn gen_ref(
  class_ref: ClassRef,
  class_type: &str,
  link_section: &str,
  export_type: &str,
) -> Result<TokenStream, Diagnostic> {
  let random_id = &RandomIdentifier::new();

  let class_str = &class_ref.class.value();
  let link_name = ["OBJC_", class_type, "_$_", class_str].concat();
  let export_name =
    ["\x01L_OBJC_CLASSLIST_", export_type, "_$_.__objrs_class.", random_id, ".", class_str]
      .concat();

  let ident = class_ref.ident;
  let sync_hack_ty = class_ref.sync_hack_ty;
  let class_ty = class_ref.class_ty;
  return Ok(quote! {
    #[link_section = #link_section]
    #[export_name = #export_name]
    static #ident: #sync_hack_ty<&'static #class_ty> = #sync_hack_ty({
      extern "C" {
        #[link_name = #link_name]
        static CLASS: #class_ty;
      }
      unsafe { &CLASS }
    });
  });
}

fn gen_value(
  ref_generator: fn(ClassRef) -> Result<TokenStream, Diagnostic>,
  class_name: &str,
  objrs_root: &Ident,
) -> Result<TokenStream, Diagnostic> {
  let class_ref_ident = priv_ident("CLASS_REF");
  let class_ref = ClassRef {
    class: LitStr::new(class_name, Span::call_site()),
    ident: class_ref_ident.clone(),
    sync_hack_ty: parse_quote!(#objrs_root::__objrs::SyncHack),
    class_ty: parse_quote!(#objrs_root::Class),
  };
  let class_ref = ref_generator(class_ref)?;
  return Ok(quote! {{
    #class_ref
    unsafe { #objrs_root::__objrs::core::ptr::read_volatile(& #class_ref_ident.0 as *const _) }
  }});
}

pub fn gen_class_ref(class_ref: ClassRef) -> Result<TokenStream, Diagnostic> {
  return gen_ref(
    class_ref,
    "CLASS",
    "__DATA,__objc_classrefs,regular,no_dead_strip",
    "REFERENCES",
  );
}

pub fn gen_super_class_ref(class_ref: ClassRef) -> Result<TokenStream, Diagnostic> {
  return gen_ref(class_ref, "CLASS", "__DATA,__objc_superrefs,regular,no_dead_strip", "SUP_REFS");
}

pub fn gen_super_meta_ref(class_ref: ClassRef) -> Result<TokenStream, Diagnostic> {
  return gen_ref(
    class_ref,
    "METACLASS",
    "__DATA,__objc_superrefs,regular,no_dead_strip",
    "SUP_REFS",
  );
}

pub fn gen_class_ref_value(
  class_name: &str,
  objrs_root: &Ident,
) -> Result<TokenStream, Diagnostic> {
  return gen_value(gen_class_ref, class_name, objrs_root);
}

pub fn gen_super_class_ref_value(
  class_name: &str,
  objrs_root: &Ident,
) -> Result<TokenStream, Diagnostic> {
  return gen_value(gen_super_class_ref, class_name, objrs_root);
}

pub fn gen_super_meta_ref_value(
  class_name: &str,
  objrs_root: &Ident,
) -> Result<TokenStream, Diagnostic> {
  return gen_value(gen_super_meta_ref, class_name, objrs_root);
}

#[cfg(test)]
mod tests {
  use super::*;
  use syn::parse_quote;

  fn default_class() -> ClassRef {
    return ClassRef {
      class: parse_quote!("TheClass"),
      ident: parse_quote!(CLASS_REF),
      sync_hack_ty: parse_quote!(objrs::__objrs::SyncHack),
      class_ty: parse_quote!(objrs::Class),
    };
  }

  #[test]
  fn class_ref() {
    let actual = gen_class_ref(default_class()).unwrap();

    let expected = quote! {
      #[link_section = "__DATA,__objc_classrefs,regular,no_dead_strip"]
      #[export_name = "\u{1}L_OBJC_CLASSLIST_REFERENCES_$_.__objrs_class._default_fake_random_identifier_.TheClass"]
      static CLASS_REF: objrs::__objrs::SyncHack<&'static objrs::Class> = objrs::__objrs::SyncHack({
        extern "C" {
          #[link_name = "OBJC_CLASS_$_TheClass"]
          static CLASS: objrs::Class;
        }
        unsafe { &CLASS }
      });
    };
    assert_eq!(actual.to_string(), expected.to_string());
  }

  #[test]
  fn super_class_ref() {
    let actual = gen_super_class_ref(default_class()).unwrap();

    let expected = quote! {
      #[link_section = "__DATA,__objc_superrefs,regular,no_dead_strip"]
      #[export_name = "\u{1}L_OBJC_CLASSLIST_SUP_REFS_$_.__objrs_class._default_fake_random_identifier_.TheClass"]
      static CLASS_REF: objrs::__objrs::SyncHack<&'static objrs::Class> = objrs::__objrs::SyncHack({
        extern "C" {
          #[link_name = "OBJC_CLASS_$_TheClass"]
          static CLASS: objrs::Class;
        }
        unsafe { &CLASS }
      });
    };
    assert_eq!(actual.to_string(), expected.to_string());
  }

  #[test]
  fn super_meta_ref() {
    let actual = gen_super_meta_ref(default_class()).unwrap();

    let expected = quote! {
      #[link_section = "__DATA,__objc_superrefs,regular,no_dead_strip"]
      #[export_name = "\u{1}L_OBJC_CLASSLIST_SUP_REFS_$_.__objrs_class._default_fake_random_identifier_.TheClass"]
      static CLASS_REF: objrs::__objrs::SyncHack<&'static objrs::Class> = objrs::__objrs::SyncHack({
        extern "C" {
          #[link_name = "OBJC_METACLASS_$_TheClass"]
          static CLASS: objrs::Class;
        }
        unsafe { &CLASS }
      });
    };
    assert_eq!(actual.to_string(), expected.to_string());
  }
}
