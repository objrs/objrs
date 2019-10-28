// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

// TODO: remove this when the refactoring is done.
#![allow(unused)]

extern crate core;
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

use crate::parse::class_attr::Class;
use crate::span_ext::SpanExt;
use crate::util::priv_ident;
use proc_macro2::{Literal, Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned, ToTokens};
use syn::{
  parse2, parse_quote, punctuated::Punctuated, spanned::Spanned, token::Comma, Attribute, Field,
  Fields, GenericParam, Ident, ItemStruct, LitByteStr, LitStr, Type, Visibility,
};

pub fn root_metaclass_ident(class_name: &str) -> Ident {
  return Ident::new(&["__objrs_rootmeta_", class_name].concat(), Span::call_site());
}

pub fn super_metaclass_ident(class_name: &str) -> Ident {
  return Ident::new(&["__objrs_supermeta_", class_name].concat(), Span::call_site());
}

pub fn super_class_ident(class_name: &str) -> Ident {
  return Ident::new(&["__objrs_super_", class_name].concat(), Span::call_site());
}

pub fn ivar_list_ident(class_name: &str) -> Ident {
  return Ident::new(&["__objrs_ivars_", class_name].concat(), Span::call_site());
}

fn gen_statics(class: &Class) -> TokenStream {
  let objrs_root = &class.objrs;

  let ident_string = &class.item.ident.to_string();
  let root_metaclass_ident = root_metaclass_ident(ident_string);
  let super_class_ident = super_class_ident(ident_string);
  let super_metaclass_ident = super_metaclass_ident(ident_string);

  let super_class_link_name;
  let super_metaclass_link_name;
  if let Some(ref super_class_name) = class.super_class_name {
    let super_class_name = &super_class_name.value();
    super_class_link_name = ["OBJC_CLASS_$_", super_class_name].concat();
    super_metaclass_link_name = ["OBJC_METACLASS_$_", super_class_name].concat();
  } else {
    super_class_link_name = String::new();
    super_metaclass_link_name = ["OBJC_CLASS_$_", &class.class_name.value()].concat();
  }

  let root_metaclass_link_name = ["OBJC_METACLASS_$_", &class.root_class_name.value()].concat();

  let mut statics = quote! {
    extern "C" {
      #[doc(hidden)]
      #[link_name = #root_metaclass_link_name]
      static #root_metaclass_ident: #objrs_root::Class;

      #[doc(hidden)]
      #[link_name = #super_metaclass_link_name]
      static #super_metaclass_ident: #objrs_root::__objrs::runtime::objc_class;
    }
  };

  if class.super_class_name.is_some() {
    statics.extend(quote! {
      extern "C" {
        #[doc(hidden)]
        #[link_name = #super_class_link_name]
        static #super_class_ident: #objrs_root::Class;
      }
    });
  } else {
    statics.extend(quote! {
      #[doc(hidden)]
      const #super_class_ident: () = ();
    });
  }

  return statics;
}

fn requires_cxx_construct(class: &Class) -> TokenStream {
  if class.item.fields.is_empty() {
    return quote!(false);
  }

  for ivar_attr in class.ivar_attrs.iter() {
    if ivar_attr.default.is_some() {
      return quote!(true);
    }
  }

  let objrs_root = &class.objrs;
  let mut iter = class.item.fields.iter();
  let mut tokens = TokenStream::new();
  if let Some(field) = iter.next() {
    let field_ty = &field.ty;
    tokens = quote!(<#field_ty as #objrs_root::__objrs::RequiresCxxConstruct>::VALUE);
  }
  for field in iter {
    let field_ty = &field.ty;
    tokens.extend(quote!(|| <#field_ty as #objrs_root::__objrs::RequiresCxxConstruct>::VALUE));
  }
  return tokens;
}

fn requires_cxx_destruct(class: &Class) -> TokenStream {
  if class.item.fields.is_empty() {
    return quote!(false);
  }

  let objrs_root = &class.objrs;
  let mut iter = class.item.fields.iter();
  let mut tokens = TokenStream::new();
  if let Some(field) = iter.next() {
    let field_ty = &field.ty;
    tokens = quote!(<#field_ty as #objrs_root::__objrs::RequiresCxxDestruct>::VALUE);
  }
  for field in iter {
    let field_ty = &field.ty;
    tokens.extend(quote!(|| <#field_ty as #objrs_root::__objrs::RequiresCxxDestruct>::VALUE));
  }
  return tokens;
}

// TODO: make this not pub
pub fn pub_item_struct_and_deref_impls(class: &Class) -> TokenStream {
  let objrs_root = &class.objrs;
  let this_field = priv_ident("__objrs_field_this");
  let generics_field = priv_ident("__objrs_field_generics");

  let pub_ident = &class.item.ident;
  let generics = &class.item.generics;
  let (impl_generics, ty_generics, where_clause) = class.item.generics.split_for_impl();
  let super_ty = class.super_class.clone().unwrap_or_else(|| parse_quote!(#objrs_root::Id));

  let mut phantom_generics = TokenStream::new();
  for ty in class.item.generics.type_params() {
    let ident = &ty.ident;
    phantom_generics.extend(quote!(#ident,));
  }
  for lifetime in class.item.generics.lifetimes() {
    let lifetime = &lifetime.lifetime;
    phantom_generics.extend(quote!(& #lifetime (),));
  }

  let mut attrs = TokenStream::new();
  for attr in class.item.attrs.iter() {
    attr.to_tokens(&mut attrs);
  }
  let vis = &class.item.vis;
  let struct_token = &class.item.struct_token;
  return quote! {
    #attrs
    #[repr(transparent)]
    #vis #struct_token #pub_ident #generics #where_clause {
      #[doc(hidden)]
      #generics_field: #objrs_root::__objrs::core::marker::PhantomData<(#phantom_generics)>,
      #[doc(hidden)]
      #this_field: #super_ty,
    }

    impl #impl_generics #objrs_root::__objrs::core::ops::Deref for #pub_ident #ty_generics #where_clause {
      type Target = #super_ty;

      #[inline(always)]
      fn deref(&self) -> &Self::Target {
        return &self.#this_field;
      }
    }

    impl #impl_generics #objrs_root::__objrs::core::ops::DerefMut for #pub_ident #ty_generics #where_clause {
      #[inline(always)]
      fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.#this_field;
      }
    }
  };
}

fn fields_item(class: &Class) -> TokenStream {
  let objrs_root = &class.objrs;

  let mut fields = class.item.fields.clone();
  for field in fields.iter_mut() {
    let ty = &field.ty;
    let span = field.ty.span();
    let head = quote_spanned!(span => #objrs_root::__objrs::Field<);
    let tail = quote_spanned!(span => >);
    field.ty = Type::Verbatim(quote!(#head #ty #tail));
  }

  let (impl_generics, ty_generics, where_clause) = class.item.generics.split_for_impl();
  let priv_item = ItemStruct {
    attrs: Vec::new(),
    vis: class.item.vis.clone(),
    struct_token: class.item.struct_token,
    ident: class.item.ident.clone().resolved_at_def_site(),
    generics: class.item.generics.clone(),
    fields: fields,
    semi_token: class.item.semi_token,
  };

  let offset_name_prefix: &str = &["OBJC_IVAR_$_", &class.class_name.value(), "."].concat();
  let mut fields_init = TokenStream::new();
  for (i, (field, attr)) in class.item.fields.iter().zip(class.ivar_attrs.iter()).enumerate() {
    let offset_export_name;
    match (&attr.name, &field.ident) {
      (Some(name), _) => offset_export_name = [offset_name_prefix, &name.value()].concat(),
      (None, Some(ident)) => offset_export_name = [offset_name_prefix, &ident.to_string()].concat(),
      (None, None) => offset_export_name = format!("{}{}", offset_name_prefix, i),
    }

    let ident = &field.ident;
    let colon = &field.colon_token;
    fields_init.extend(quote! {
      #ident #colon {
        extern "C" {
          #[link_name = #offset_export_name]
          static IVAR_OFFSET: #objrs_root::__objrs::usize;
        }
        unsafe { #objrs_root::__objrs::Field::with_offset(&IVAR_OFFSET) }
      },
    });
  }

  match class.item.fields {
    Fields::Named(_) => fields_init = quote!({ #fields_init }),
    Fields::Unnamed(_) => fields_init = quote!((#fields_init)),
    Fields::Unit => (),
  }

  let pub_ty_ident = &class.item.ident;
  let priv_ty_ident = &priv_item.ident;

  return quote! {
    #[doc(hidden)]
    #priv_item

    impl #impl_generics #objrs_root::__objrs::Fields for #pub_ty_ident #ty_generics #where_clause {
      type Type = #priv_ty_ident #ty_generics;

      #[inline(always)]
      fn from_ref(&self) -> #objrs_root::__objrs::ThisAndFields<Self, Self::Type> {
        return <Self as #objrs_root::__objrs::Fields>::from_ptr(self as *const _ as *mut _);
      }

      #[inline(always)]
      fn from_ptr(this: *mut Self) -> #objrs_root::__objrs::ThisAndFields<Self, Self::Type> {
        #[allow(deprecated)]
        return #objrs_root::__objrs::ThisAndFields {
          this: this,
          fields: #priv_ty_ident #fields_init,
        };
      }
    }
  };
}

fn pub_impls(class: &Class) -> TokenStream {
  let objrs_root = &class.objrs;
  let ident = &class.item.ident;
  let (impl_generics, ty_generics, where_clause) = class.item.generics.split_for_impl();
  let mut impls = quote! {
    unsafe impl #impl_generics #objrs_root::marker::Class for #ident #ty_generics #where_clause {}
  };

  if let Some(super_class) = &class.super_class {
    impls.extend(quote! {
      unsafe impl #impl_generics #objrs_root::marker::NonRootClass for #ident #ty_generics #where_clause {
        type Super = #super_class;
      }
    });
  } else {
    impls.extend(quote! {
      unsafe impl #impl_generics #objrs_root::marker::RootClass for #ident #ty_generics #where_clause {}
    });
  }

  return impls;
}

#[cfg(test)]
mod tests {
  extern crate objrs_test_utils;

  use super::*;
  use crate::parse::class_attr::ClassAttr;
  use objrs_test_utils::assert_tokens_eq;
  use syn::{parse2, ItemStruct};

  fn make_class(tokens: TokenStream) -> Class {
    let mut item = parse2::<ItemStruct>(tokens).unwrap();
    let objrs_attr = item.attrs.remove(0);
    assert!(objrs_attr.path.is_ident("objrs"));
    let class_attr = objrs_attr.parse_args::<ClassAttr>().unwrap();
    return Class::new(class_attr, item.into_token_stream()).unwrap();
  }

  #[test]
  fn test_cxx_no_fields() {
    let class = make_class(quote! {
      #[objrs(class, super = NSObject)]
      struct Foo;
    });

    let actual = requires_cxx_construct(&class);
    let expected = quote!(false);
    assert_tokens_eq!(actual, expected);

    let actual = requires_cxx_destruct(&class);
    assert_tokens_eq!(actual, expected);
  }

  #[test]
  fn test_cxx_with_fields() {
    let class = make_class(quote! {
      #[objrs(class, super = NSObject)]
      struct Foo {
        x: f32,
        y: u32,
        #[objrs(ivar, name = "nop")]
        z: char,
      }
    });

    let actual = requires_cxx_construct(&class);
    let expected = quote! {
      <f32 as objrs::__objrs::RequiresCxxConstruct>::VALUE ||
      <u32 as objrs::__objrs::RequiresCxxConstruct>::VALUE ||
      <char as objrs::__objrs::RequiresCxxConstruct>::VALUE
    };
    assert_tokens_eq!(actual, expected);

    let actual = requires_cxx_destruct(&class);
    let expected = quote! {
      <f32 as objrs::__objrs::RequiresCxxDestruct>::VALUE ||
      <u32 as objrs::__objrs::RequiresCxxDestruct>::VALUE ||
      <char as objrs::__objrs::RequiresCxxDestruct>::VALUE
    };
    assert_tokens_eq!(actual, expected);
  }

  #[test]
  fn test_cxx_with_fields_with_custom_default() {
    let class = make_class(quote! {
      #[objrs(class, super = NSObject)]
      struct Foo {
        x: f32,
        #[objrs(ivar, default = 42)]
        y: u32,
        z: char,
      }
    });

    let actual = requires_cxx_construct(&class);
    let expected = quote!(true);
    assert_tokens_eq!(actual, expected);
  }

  #[test]
  fn test_gen_statics() {
    let class = make_class(quote! {
      #[objrs(class, name = "ClassName", super(name = "SuperName", type = SuperTy), root_class = "RootName")]
      struct ClassTy;
    });

    let actual = gen_statics(&class);
    let expected = quote! {
      extern "C" {
        #[doc(hidden)]
        #[link_name = "OBJC_METACLASS_$_RootName"]
        static __objrs_rootmeta_ClassTy: objrs::Class;

        #[doc(hidden)]
        #[link_name = "OBJC_METACLASS_$_SuperName"]
        static __objrs_supermeta_ClassTy: objrs::__objrs::runtime::objc_class;
      }

      extern "C" {
        #[doc(hidden)]
        #[link_name = "OBJC_CLASS_$_SuperName"]
        static __objrs_super_ClassTy: objrs::Class;
      }
    };
    assert_tokens_eq!(actual, expected);
  }

  #[test]
  fn test_gen_statics_root_class() {
    let class = make_class(quote! {
      #[objrs(class, name = "ClassName", root_class)]
      struct ClassTy;
    });

    let actual = gen_statics(&class);
    let expected = quote! {
      extern "C" {
        #[doc(hidden)]
        #[link_name = "OBJC_METACLASS_$_ClassName"]
        static __objrs_rootmeta_ClassTy: objrs::Class;

        #[doc(hidden)]
        #[link_name = "OBJC_CLASS_$_ClassName"]
        static __objrs_supermeta_ClassTy: objrs::__objrs::runtime::objc_class;
      }

      #[doc(hidden)]
      const __objrs_super_ClassTy: () = ();
    };
    assert_tokens_eq!(actual, expected);
  }

  #[test]
  fn test_pub_item_struct_and_deref_impls() {
    let class = make_class(quote! {
      #[objrs(class, super = SuperTy)]
      pub struct ClassTy<T> where T: objrs::marker::Class {
        foo: u8,
        bar: u16,
      }
    });

    let actual = pub_item_struct_and_deref_impls(&class);
    let expected = quote! {
      #[repr(transparent)]
      pub struct ClassTy<T> where T: objrs::marker::Class {
        #[doc(hidden)]
        __objrs_field_generics: objrs::__objrs::core::marker::PhantomData<(T,)>,
        #[doc(hidden)]
        __objrs_field_this: SuperTy,
      }

      impl<T> objrs::__objrs::core::ops::Deref for ClassTy<T> where T: objrs::marker::Class {
        type Target = SuperTy;

        #[inline(always)]
        fn deref(&self) -> &Self::Target {
          return &self.__objrs_field_this;
        }
      }

      impl<T> objrs::__objrs::core::ops::DerefMut for ClassTy<T> where T: objrs::marker::Class {
        #[inline(always)]
        fn deref_mut(&mut self) -> &mut Self::Target {
          return &mut self.__objrs_field_this;
        }
      }
    };
    assert_tokens_eq!(actual, expected);
  }

  #[test]
  fn test_pub_item_struct_and_deref_impls_root_class() {
    let class = make_class(quote! {
      #[objrs(class, name = "ClassName", root_class)]
      #[doc = "Keep this"]
      struct ClassTy;
    });

    let actual = pub_item_struct_and_deref_impls(&class);
    let expected = quote! {
      #[doc = "Keep this"]
      #[repr(transparent)]
      struct ClassTy {
        #[doc(hidden)]
        __objrs_field_generics: objrs::__objrs::core::marker::PhantomData<()>,
        #[doc(hidden)]
        __objrs_field_this: objrs::Id,
      }

      impl objrs::__objrs::core::ops::Deref for ClassTy {
        type Target = objrs::Id;

        #[inline(always)]
        fn deref(&self) -> &Self::Target {
          return &self.__objrs_field_this;
        }
      }

      impl objrs::__objrs::core::ops::DerefMut for ClassTy {
        #[inline(always)]
        fn deref_mut(&mut self) -> &mut Self::Target {
          return &mut self.__objrs_field_this;
        }
      }
    };
    assert_tokens_eq!(actual, expected);
  }

  #[test]
  fn test_fields_item_unit() {
    let class = make_class(quote! {
      #[objrs(class, name = "ClassName", root_class)]
      #[doc = "Remove this"]
      struct ClassTy;
    });

    let actual = fields_item(&class);
    let expected = quote! {
      #[doc(hidden)]
      struct ClassTy;

      impl objrs::__objrs::Fields for ClassTy {
        type Type = ClassTy;

        #[inline(always)]
        fn from_ref(&self) -> objrs::__objrs::ThisAndFields<Self, Self::Type> {
          return <Self as objrs::__objrs::Fields>::from_ptr(self as *const _ as *mut _);
        }

        #[inline(always)]
        fn from_ptr(this: *mut Self) -> objrs::__objrs::ThisAndFields<Self, Self::Type> {
          #[allow(deprecated)]
          return objrs::__objrs::ThisAndFields {
            this: this,
            fields: ClassTy,
          };
        }
      }
    };
    assert_tokens_eq!(actual, expected);
  }

  #[test]
  fn test_fields_item_tuple() {
    let class = make_class(quote! {
      #[objrs(class, name = "ClassName", root_class)]
      struct ClassTy<T: objrs::marker::Class>(u8, T);
    });

    let actual = fields_item(&class);
    let expected = quote! {
      #[doc(hidden)]
      struct ClassTy<T: objrs::marker::Class>(objrs::__objrs::Field<u8>, objrs::__objrs::Field<T>);

      impl<T: objrs::marker::Class> objrs::__objrs::Fields for ClassTy<T> {
        type Type = ClassTy<T>;

        #[inline(always)]
        fn from_ref(&self) -> objrs::__objrs::ThisAndFields<Self, Self::Type> {
          return <Self as objrs::__objrs::Fields>::from_ptr(self as *const _ as *mut _);
        }

        #[inline(always)]
        fn from_ptr(this: *mut Self) -> objrs::__objrs::ThisAndFields<Self, Self::Type> {
          #[allow(deprecated)]
          return objrs::__objrs::ThisAndFields {
            this: this,
            fields: ClassTy(
              {
                extern "C" {
                  #[link_name = "OBJC_IVAR_$_ClassName.0"]
                  static IVAR_OFFSET: objrs::__objrs::usize;
                }
                unsafe { objrs::__objrs::Field::with_offset(&IVAR_OFFSET) }
              },
              {
                extern "C" {
                  #[link_name = "OBJC_IVAR_$_ClassName.1"]
                  static IVAR_OFFSET: objrs::__objrs::usize;
                }
                unsafe { objrs::__objrs::Field::with_offset(&IVAR_OFFSET) }
              },
            ),
          };
        }
      }
    };
    assert_tokens_eq!(actual, expected);
  }

  #[test]
  fn test_fields_item_struct() {
    let class = make_class(quote! {
      #[objrs(class, name = "ClassName", root_class)]
      struct ClassTy<T> where T: objrs::marker::Class {
        value: T,
      }
    });

    let actual = fields_item(&class);
    let expected = quote! {
      #[doc(hidden)]
      struct ClassTy<T> where T: objrs::marker::Class {
        value: objrs::__objrs::Field<T>,
      }

      impl<T> objrs::__objrs::Fields for ClassTy<T> where T: objrs::marker::Class {
        type Type = ClassTy<T>;

        #[inline(always)]
        fn from_ref(&self) -> objrs::__objrs::ThisAndFields<Self, Self::Type> {
          return <Self as objrs::__objrs::Fields>::from_ptr(self as *const _ as *mut _);
        }

        #[inline(always)]
        fn from_ptr(this: *mut Self) -> objrs::__objrs::ThisAndFields<Self, Self::Type> {
          #[allow(deprecated)]
          return objrs::__objrs::ThisAndFields {
            this: this,
            fields: ClassTy {
              value: {
                extern "C" {
                  #[link_name = "OBJC_IVAR_$_ClassName.value"]
                  static IVAR_OFFSET: objrs::__objrs::usize;
                }
                unsafe { objrs::__objrs::Field::with_offset(&IVAR_OFFSET) }
              },
            },
          };
        }
      }
    };
    assert_tokens_eq!(actual, expected);
  }

  #[test]
  fn test_pub_impls() {
    let class = make_class(quote! {
      #[objrs(class, super = SuperTy)]
      struct ClassTy<T> where T: objrs::marker::Class {
        value: T,
      }
    });

    let actual = pub_impls(&class);
    let expected = quote! {
      unsafe impl<T> objrs::marker::Class for ClassTy<T> where T: objrs::marker::Class {}
      unsafe impl<T> objrs::marker::NonRootClass for ClassTy<T> where T: objrs::marker::Class {
        type Super = SuperTy;
      }
    };
    assert_tokens_eq!(actual, expected);
  }

  #[test]
  fn test_pub_impls_root_class() {
    let class = make_class(quote! {
      #[objrs(class, root_class)]
      struct ClassTy<T> where T: objrs::marker::Class {
        value: T,
      }
    });

    let actual = pub_impls(&class);
    let expected = quote! {
      unsafe impl<T> objrs::marker::Class for ClassTy<T> where T: objrs::marker::Class {}
      unsafe impl<T> objrs::marker::RootClass for ClassTy<T> where T: objrs::marker::Class {}
    };
    assert_tokens_eq!(actual, expected);
  }
}
