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
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
  buffer::TokenBuffer, parse_quote, parse_str, punctuated::Punctuated, spanned::Spanned,
  synom::Synom, token::Comma, Attribute, Field, Fields, GenericParam, Ident, ItemStruct, LitStr,
  Path, Visibility,
};

fn link_attribute(attrs: &[Attribute]) -> Option<&Attribute> {
  for attr in attrs {
    let path = &attr.path;
    if path.leading_colon.is_none() && path.segments.len() == 1 && path.segments[0].ident == "link"
    {
      return Some(&attr);
    }
  }
  return None;
}

fn parse_attribute(mut args_parser: AttributeParser) -> Result<(LitStr, Option<Path>), Diagnostic> {
  args_parser.op('=')?;
  let class = args_parser.string()?;
  args_parser.op(',')?;

  args_parser.keyword("super")?;
  args_parser.op('=')?;
  let string_literal = args_parser.string()?;
  args_parser.eof()?;

  let string_value = string_literal.value();
  let super_class;
  if string_value.is_empty() {
    super_class = None;
  } else {
    let result = parse_str::<Path>(&string_value);
    if let Ok(path) = result {
      super_class = Some(path);
    } else {
      return Err(
        string_literal
          .span()
          .unstable()
          .error(format!("`{}` is not a valid type name", string_value)),
      );
    }
  }

  return Ok((class, super_class));
}

pub fn parse_class(
  args_parser: AttributeParser,
  input: TokenStream,
) -> Result<TokenStream, Diagnostic> {
  let (class, super_class) = parse_attribute(args_parser)?;

  let input = TokenBuffer::new2(input);
  let item = match <ItemStruct as Synom>::parse(input.begin()) {
    Ok((st, _)) => st,
    Err(error) => {
      return Err(
        input
          .begin()
          .token_stream()
          .span()
          .unstable()
          .error(format!("failed to parse struct: {}", error.to_string()))
          .note("#[objrs(class ...)] must only be applied to a struct item"),
      );
    }
  };

  let link_attribute = match link_attribute(&item.attrs) {
    Some(attr) => attr,
    None => {
      return Err(
        item
          .span()
          .unstable()
          .error("struct is missing #[link] attribute")
          .note("objrs requires it for external classes")
          .note("objrs does not yet support new class definitions"),
      )
    }
  };

  match item.fields {
    Fields::Named(named) => {
      if !named.named.is_empty() {
        let s;
        if named.named.len() == 1 {
          s = "";
        } else {
          s = "s"
        }
        return Err(
          named
            .span()
            .unstable()
            .error(format!(
              "expected no fields, found {} field{}",
              named.named.len(),
              s
            ))
            .note("objrs does not yet support ivars"),
        );
      }
      named
        .span()
        .unstable()
        .warning("struct does not need `{}`")
        .note("prefer a unit struct")
        .emit();
    }
    Fields::Unnamed(unnamed) => {
      if !unnamed.unnamed.is_empty() {
        let s;
        if unnamed.unnamed.len() == 1 {
          s = "";
        } else {
          s = "s"
        }
        return Err(
          unnamed
            .span()
            .unstable()
            .error(format!(
              "expected no fields, found {} field{}",
              unnamed.unnamed.len(),
              s
            ))
            .note("objrs does not yet support ivars"),
        );
      }
      unnamed
        .span()
        .unstable()
        .warning("struct does not need `()`")
        .note("prefer a unit struct")
        .emit();
    }
    Fields::Unit => (),
  }

  let ident = item.ident;

  let class_name = &class.value();
  let link_name = ["OBJC_CLASS_$_", class_name].concat();
  let link_name = LitStr::new(&link_name, Span::call_site()); // TODO: use Span::def_site().

  let image_info_name = ["\x01L_OBJC_IMAGE_INFO.__objrs.", class_name].concat();
  let image_info_name = LitStr::new(&image_info_name, Span::call_site()); // TODO: use Span::def_site().

  let class_ref_name = ["\x01L_OBJC_CLASSLIST_REFERENCES_$_.__objrs.", class_name].concat();
  let class_ref_name = LitStr::new(&class_ref_name, Span::call_site()); // TODO: use Span::def_site().

  let self_as_class = quote!{
    <#ident as objrs::runtime::__objrs::Class>
  };
  let self_as_nonroot_class = quote!{
    <#ident as objrs::runtime::__objrs::NonRootClass>
  };

  let mut generic_types = vec![];
  let mut generic_lifetimes = vec![];
  let mut generic_idents: Punctuated<&ToTokens, Comma> = Punctuated::new();
  for param in item.generics.params.iter() {
    match param {
      GenericParam::Type(ref generic_type) => {
        generic_types.push(&generic_type.ident);
        generic_idents.push(&generic_type.ident);
      }
      GenericParam::Lifetime(ref generic_lifetime) => {
        generic_lifetimes.push(&generic_lifetime.lifetime);
        generic_idents.push(&generic_lifetime.lifetime);
      }
      GenericParam::Const(ref generic_const) => {
        generic_idents.push(&generic_const.ident);
      }
    }
  }

  let vis = item.vis;

  let generics = &item.generics;
  let where_clause = &generics.where_clause;
  let extern_ident = Ident::new(
    &["__objrs_type_", ident.as_ref()].concat(),
    Span::call_site(),
  ); // TODO: use def_site.
  let mut fields: Punctuated<Field, Comma> = Punctuated::new();
  for generic_type in generic_types {
    fields.push(Field {
      attrs: vec![],
      vis: Visibility::Inherited,
      ident: None,
      colon_token: None,
      ty: parse_quote!(objrs::__objrs::core::marker::PhantomData<#generic_type>),
    });
  }
  for generic_lifetime in generic_lifetimes {
    fields.push(Field {
      attrs: vec![],
      vis: Visibility::Inherited,
      ident: None,
      colon_token: None,
      ty: parse_quote!(objrs::__objrs::core::marker::PhantomData<& #generic_lifetime u8>),
    });
  }
  fields.push(Field {
    attrs: vec![],
    vis: Visibility::Inherited,
    ident: None,
    colon_token: None,
    ty: parse_quote!(#extern_ident),
  });

  let impls;
  let superclass_test;
  if super_class.is_none() {
    impls = quote!{
      impl #generics objrs::runtime::__objrs::RootClass for #ident <#generic_idents> #where_clause {}
    };
    superclass_test = quote!{
      assert_eq!(superclass, objrs::__objrs::core::ptr::null_mut(), "Objective-C class `{}` is not a root class", #self_as_class::CLASS_NAME);
    };
  } else {
    impls = quote!{
      impl #generics objrs::__objrs::core::ops::Deref for #ident <#generic_idents> #where_clause {
        type Target = #super_class;

        #[inline(always)]
        fn deref(&self) -> &Self::Target {
          return unsafe { objrs::__objrs::core::mem::transmute(self) };
        }
      }

      impl #generics objrs::__objrs::core::ops::DerefMut for #ident <#generic_idents> #where_clause {
        #[inline(always)]
        fn deref_mut(&mut self) -> &mut Self::Target {
          return unsafe { objrs::__objrs::core::mem::transmute(self) };
        }
      }

      impl #generics objrs::runtime::__objrs::NonRootClass for #ident <#generic_idents> #where_clause {
        type Super = #super_class;
      }
    };

    superclass_test = quote!{
        assert_ne!(superclass, objrs::__objrs::core::ptr::null_mut(), "Objective-C class `{}`'s superclass `{}` not found in the runtime", #self_as_class::CLASS_NAME, <#self_as_nonroot_class::Super as objrs::runtime::__objrs::Class>::CLASS_NAME);
        let expected_superclass = unsafe { objrs::runtime::objc_getClass(<#self_as_nonroot_class::Super as objrs::runtime::__objrs::Class>::CLASS_NAME_CSTR as *const _ as *const _) };
        assert_eq!(superclass, expected_superclass, "Objective-C class `{}` doesn't have the expected superclass `{}`", #self_as_class::CLASS_NAME, <#self_as_nonroot_class::Super as objrs::runtime::__objrs::Class>::CLASS_NAME);
    };
  }

  let mut class_cstr = class.value();
  class_cstr.push('\0');
  let class_cstr = LitStr::new(&class_cstr, class.span());
  let test = quote!{
    #[cfg(test)]
    #[test]
    fn #ident() {
      extern crate objrs;

      // TODO: improve error messages. They can be improved.
      let class = unsafe { objrs::runtime::objc_getClass(#self_as_class::CLASS_NAME_CSTR as *const _ as *const _) };
      assert_ne!(class, objrs::__objrs::core::ptr::null_mut(), "Objective-C class `{}` not found in the runtime", #self_as_class::CLASS_NAME);

      let superclass = unsafe { objrs::runtime::class_getSuperclass(class) };
      #superclass_test
    }
  };

  let tokens = quote!{
    #link_attribute
    extern "C" {
      // TODO: apply the item's user-provided attributes?
      type #extern_ident;
    }

    #[repr(transparent)]
    #vis struct #ident #generics(#fields) #where_clause;

    impl #generics #ident <#generic_idents> #where_clause {
      #[allow(dead_code)]
      #[doc(hidden)]
      #[inline(always)]
      fn __objrs_class_ref() -> objrs::runtime::Class {
        #[link_section = "__DATA,__objc_imageinfo,regular,no_dead_strip"]
        #[export_name = #image_info_name]
        #[used]
        #[allow(non_upper_case_globals)]
        static IMAGE_INFO: objrs::runtime::objc_image_info = objrs::runtime::objc_image_info::DEFAULT;

        #link_attribute
        extern "C" {
          #[link_name = #link_name]
          static CLASS: objrs::runtime::objc_class;
        }

        #[link_section = "__DATA,__objc_classrefs,regular,no_dead_strip"]
        #[export_name = #class_ref_name]
        #[allow(non_upper_case_globals)]
        static CLASS_REF: objrs::__objrs::SyncHack<objrs::runtime::Class> = objrs::__objrs::SyncHack(unsafe { &CLASS as *const _ as *mut _ });

        // TODO: Revert this hack! It's needed because #[inline(never)] doesn't really do anything
        // for generic functions (inluding non-generic methods for generic types). Also, incremental
        // compilation can wreck havoc with this (it seems to compile things into lots of separate
        // object files, which breaks references to L_* locals).
        #[inline(never)]
        fn class_ref_hack() -> objrs::runtime::Class {
          let ptr = &CLASS_REF.0 as *const _;
          return unsafe { objrs::__objrs::core::ptr::read_volatile(ptr) };
        }
        return class_ref_hack();
        // return unsafe { objrs::__objrs::core::ptr::read_volatile(&CLASS_REF.0 as *const _) };
      }
    }

    unsafe impl #generics objrs::marker::Class for #ident <#generic_idents> #where_clause {}

    impl #generics objrs::runtime::__objrs::Class for #ident <#generic_idents> #where_clause {
      const CLASS_NAME: &'static str = #class;
      const CLASS_NAME_CSTR: &'static str = #class_cstr;
    }

    #impls

    #test
  };

  return Ok(tokens.into());
}
