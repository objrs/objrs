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
use crate::util::priv_ident;

use proc_macro2::{Literal, Span, TokenStream, TokenTree};
use quote::quote;
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
      static #super_class_ident: () = ();
    });
  }

  return statics;
}

fn requires_cxx_construct(class: &Class) -> TokenStream {
  for ivar_attr in class.ivar_attrs.iter() {
    if ivar_attr.default.is_some() {
      return quote!(true);
    }
  }

  let objrs_root = &class.objrs;
  let mut tokens = quote!(false);
  for field in class.item.fields.iter() {
    let field_ty = &field.ty;
    tokens.extend(quote!(|| <#field_ty as #objrs_root::__objrs::RequiresCxxConstruct>::VALUE));
  }
  return tokens;
}

fn requires_cxx_destruct(class: &Class) -> TokenStream {
  let objrs_root = &class.objrs;
  let mut tokens = quote!(false);
  for field in class.item.fields.iter() {
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

  let vis = &class.item.vis;
  let struct_token = &class.item.struct_token;
  return quote! {
    // TODO: apply the item's user-provided attributes?
    #[repr(transparent)]
    #vis #struct_token #pub_ident #generics {
      #generics_field: #objrs_root::__objrs::core::marker::PhantomData<(#phantom_generics)>,
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
  let _vis = &class.item.vis;
  let _struct_token = &class.item.struct_token;
  return quote! {
    // #vis #struct_token #pub_ident {
    // }
  };
  //   let mut item_fields = item.clone();
  //   item_fields.ident = priv_ident.clone();
  //   item_fields.generics = Default::default();

  //   match item_fields.fields {
  //     Fields::Named(_) => fields_init = quote!({ #fields_init }),
  //     Fields::Unnamed(_) => fields_init = quote!((#fields_init)),
  //     Fields::Unit => (),
  //   }

  //   for field in item_fields.fields.iter_mut() {
  //     let new_ty;
  //     {
  //       let ty = &field.ty;
  //       let span = field.ty.span();
  //       let head = quote_spanned!(span => #objrs_root::__objrs::Field<);
  //       let tail = quote_spanned!(span => >);
  //       new_ty = parse_quote!(#head #ty #tail);
  //     }
  //     field.ty = new_ty;
  //   }
}

// pub fn transform_class(class: Class) -> Result<TokenStream, Diagnostic> {
//   let pub_ident = &item.ident;
//   let priv_ident = priv_ident_at(&pub_ident.to_string(), pub_ident.span());
//   let mut original_item = item.clone();
//   original_item.attrs.push(parse_quote!(#[allow(non_camel_case_types)]));
//   original_item.attrs.push(parse_quote!(#[allow(dead_code)]));
//   let original_item_ident = ["__objrs_original_", &pub_ident.to_string()].concat();
//   original_item.ident = priv_ident_at(&original_item_ident, pub_ident.span());
//   let original_item_ident = &original_item.ident;
//   original_item.generics = Default::default();
//   for field in original_item.fields.iter_mut() {
//     strip_ivar_attr(field)?;
//   }

//   let class_name;
//   if let Some(name) = attr.class_name {
//     class_name = name;
//   } else {
//     class_name = LitStr::new(&item.ident.to_string(), item.ident.span());
//   }
//   let class_name_str = &class_name.value();
//   let class_name_cstr = LitStr::new(&[&class_name_str, "\0"].concat(), class_name.span());

//   let super_class = attr.super_class;
//   let super_class_name = attr.super_class_name.or_else(|| {
//     let super_class_ident = super_class
//       .as_ref()
//       .and_then(|type_path| type_path.path.segments.last().map(|p| p.value().ident.clone()));
//     return super_class_ident.map(|ident| LitStr::new(&ident.to_string(), ident.span())); // TODO: use def_site.
//   });
//   let is_root_class = super_class.is_none();

//   let root_metaclass_ident = root_metaclass_ident(class_name_str);
//   let super_metaclass_ident = super_metaclass_ident(class_name_str);
//   let objrs_root = crate::util::priv_ident("__objrs_root");

//   // TODO: Add a unit test that checks ivar names and types (same as the unit test to check selector
//   // argument/return types).

//   let mut generic_idents: Punctuated<&ToTokens, Comma> = Punctuated::new();
//   for param in item.generics.params.iter() {
//     match param {
//       GenericParam::Type(ref generic_type) => {
//         generic_idents.push(&generic_type.ident);
//       }
//       GenericParam::Lifetime(ref generic_lifetime) => {
//         generic_idents.push(&generic_lifetime.lifetime);
//       }
//       GenericParam::Const(ref generic_const) => {
//         generic_idents.push(&generic_const.ident);
//       }
//     }
//   }

//   let native_ty = quote!(#objrs_root::__objrs);
//   let link_attr = link_attribute(&item.attrs);
//   let force_extern = attr.force_extern || link_attr.is_some();
//   let offset_name_prefix = ["OBJC_IVAR_$_", class_name_str, "."].concat();
//   let offset_name_prefix = offset_name_prefix.as_ref();
//   let ivar_list;
//   let has_ivars;
//   let mut cxx_construct = TokenStream::new();
//   let mut cxx_destruct = TokenStream::new();
//   let mut prev_unpadded_size_of_ident = crate::util::priv_ident("UNPADDED_SIZE_OF_START");
//   let mut unpadded_size_of = quote! {
//     const FIELD_END_START: #native_ty::usize = 0;
//     const #prev_unpadded_size_of_ident: #native_ty::usize = 0;
//   };

//   let ivar_name_prefix =
//     &["\x01L_OBJC_METH_VAR_NAME_.__objrs_ivar.", class_name_str, "::"].concat();
//   let ivar_type_prefix =
//     &["\x01L_OBJC_METH_VAR_TYPE_.__objrs_ivar.", class_name_str, "::"].concat();

//   let mut fields_init = TokenStream::new();
//   let mut field_count: usize = 0;
//   let mut field_tokens = quote!();
//   for (i, field) in item.fields.iter_mut().enumerate() {
//     field_count += 1;
//     let field_ident: TokenTree =
//       field.ident.clone().map_or_else(|| Literal::usize_unsuffixed(i).into(), |ident| ident.into());

//     let attr = parse_ivar(field, force_extern)?;
//     let ivar_ident_str: &str =
//       &attr.name.map_or_else(|| field_ident.to_string(), |lit| lit.value());

//     let field_end_ident = crate::util::priv_ident(&format!("FIELD_END_{}", i));
//     let unpadded_size_of_ident = crate::util::priv_ident(&format!("UNPADDED_SIZE_OF_{}", i));
//     let field_ty = &field.ty;
//     unpadded_size_of.extend(quote!{
//       const #field_end_ident: #native_ty::usize = #objrs_root::__objrs::core::mem::size_of::<#field_ty>() + unsafe { #objrs_root::__objrs::TransmuteHack::<_, #native_ty::usize> { from: &#objrs_root::__objrs::TransmuteHack::<#native_ty::usize, &#original_item_ident> { from: 0 }.to.#field_ident }.to };
//       const #unpadded_size_of_ident: #native_ty::usize = (((#prev_unpadded_size_of_ident > #field_end_ident) as #native_ty::usize) * #prev_unpadded_size_of_ident) | (((#prev_unpadded_size_of_ident <= #field_end_ident) as #native_ty::usize) * #field_end_ident);
//     });
//     prev_unpadded_size_of_ident = unpadded_size_of_ident;

//     let field_colon = field.ident.as_ref().map_or_else(TokenStream::new, |ident| quote!(#ident:));
//     let offset_export_name = [offset_name_prefix, ivar_ident_str].concat();
//     fields_init.extend(quote!{
//       #field_colon {
//       #link_attr
//       extern "C" {
//         #[link_name = #offset_export_name]
//         static IVAR_OFFSET: #native_ty::usize;
//       }
//       #objrs_root::__objrs::Field(unsafe { #objrs_root::__objrs::core::mem::transmute::<&#native_ty::usize, *mut #field_ty>(&IVAR_OFFSET) })
//     },});

//     if force_extern {
//       continue;
//     }

//     let offset = quote! {{
//       // Objective-C marks this as .private_extern in the assembly. I don't think we have a way to
//       // do that in Rust.
//       #[link_section = "__DATA,__objc_ivar"]
//       #[export_name = #offset_export_name]
//       // TODO: support generics.
//       // This is equivalent to the C code `(size_t)&((T *)0)->field`. This is UB in Rust since it
//       // creates a null reference, but I haven't been able to come up with a non-UB alternative.
//       // TODO: do something like https://internals.rust-lang.org/t/discussion-on-offset-of/7440 to avoid going through a deref.
//       static IVAR_OFFSET: #native_ty::usize = <#pub_ident as #objrs_root::__objrs::runtime::__objrs::Class>::INSTANCE_START + unsafe { #objrs_root::__objrs::TransmuteHack::<_, #native_ty::usize> { from: &#objrs_root::__objrs::TransmuteHack::<#native_ty::usize, &#original_item_ident> { from: 0 }.to.#field_ident }.to };
//       &IVAR_OFFSET as *const _ as *mut _
//     }};

//     let ivar_name_export_name = [ivar_name_prefix, ivar_ident_str].concat();
//     let ivar_name = [ivar_ident_str, "\x00"].concat();
//     let ivar_name = ivar_name.as_bytes();
//     let ivar_name_len = ivar_name.len();
//     let ivar_name = LitByteStr::new(ivar_name, Span::call_site()); // TODO: use def_site().
//     let name = quote! {{
//       #[link_section = "__TEXT,__objc_methname,cstring_literals"]
//       #[export_name = #ivar_name_export_name]
//       static IVAR_NAME: [#native_ty::u8; #ivar_name_len] = *#ivar_name;
//       &IVAR_NAME as *const #native_ty::u8
//     }};

//     if let Some(default) = attr.default {
//       cxx_construct.extend(quote!{
//         #objrs_root::__objrs::Field::construct_with_value(&mut this_and_fields.fields.#field_ident, this_as_u8, #default);
//       });
//     } else {
//       cxx_construct.extend(quote!{
//         if <#field_ty as #objrs_root::__objrs::RequiresCxxConstruct>::VALUE {
//           #objrs_root::__objrs::Field::construct(&mut this_and_fields.fields.#field_ident, this_as_u8);
//         }
//       });
//     }
//     cxx_destruct.extend(quote!{
//       if <#field_ty as #objrs_root::__objrs::RequiresCxxDestruct>::VALUE && #objrs_root::__objrs::core::mem::needs_drop::<#field_ty>() {
//         #objrs_root::__objrs::Field::destruct(&mut this_and_fields.fields.#field_ident, this_as_u8);
//       }
//     });

//     let ivar_type_export_name = [ivar_type_prefix, ivar_ident_str].concat();
//     let encoded_type = quote! {{
//       #[link_section = "__TEXT,__objc_methtype,cstring_literals"]
//       #[export_name = #ivar_type_export_name]
//       static IVAR_TYPE: #objrs_root::__objrs::Packed2<<#field_ty as #objrs_root::__objrs::TypeEncodingHack>::Type, #native_ty::u8> = #objrs_root::__objrs::Packed2(<#field_ty as #objrs_root::__objrs::TypeEncodingHack>::BYTES, b'\x00');
//       unsafe { #objrs_root::__objrs::TransmuteHack::<_, *const #native_ty::u8> { from: &IVAR_TYPE }.to }
//     }};

//     // TODO: the following is incorrect and incomplete. It builds the weak ivar layout counting
//     // the number of fields. Instead, the layout should be based on the number of bytes and the
//     // offset of each field. That really complicates things. I think we need const fn in order to
//     // do this properly.
//     // quote!{{
//     //   const WEAK_IVAR_LAYOUT: [u8; 1] = [0x00];  // TODO: replace with a proper value.
//     //   const OLD_LEN: usize = 1;  // TODO: replace with real value.

//     //   const IS_WEAK: bool = <#field_ty as #objrs_root::__objrs::IsWeak>::VALUE;
//     //   const LAST_BYTE: u8 = WEAK_IVAR_LAYOUT[OLD_LEN - 1];  // TODO: replace with real value.
//     //   const WEAKS: u8 = LAST_BYTE & 0x0f;
//     //   const SKIPS: u8 = LAST_BYTE >> 4;
//     //   const NEW_WEAKS: u8 = WEAKS + IS_WEAK as u8;
//     //   const NEW_SKIPS: u8 = SKIPS + !IS_WEAK as u8;
//     //   const PUSH: bool = NEW_WEAKS | NEW_SKIPS > 0x0f || (!IS_WEAK && WEAKS > 0);
//     //   const MASK: u8 = 0xff * PUSH as u8;
//     //   const NEW_LAST_BYTE: u8 = ((IS_WEAK as u8) & MASK) |
//     //                             ((((NEW_SKIPS & 0x0f) << 4) | (NEW_WEAKS & 0x0f)) & !MASK);
//     //   const TRUNCATED_LEN: usize = OLD_LEN - !PUSH as usize;

//     //   unsafe { #objrs_root::__objrs::TransmuteHack::<_, [u8; TRUNCATED_LEN + 1]> { from: #objrs_root::__objrs::Packed2(#objrs_root::__objrs::TransmuteHack::<_, [u8; TRUNCATED_LEN]> { from: WEAK_IVAR_LAYOUT }.to, NEW_LAST_BYTE) }.to }
//     // }};
//     // TODO: convert WEAK_IVAR_LAYOUT to nil if no ivars are weak.
//     // TODO: truncate the trailing SKIPS in the array.

//     field_tokens.extend(quote! {
//       #objrs_root::__objrs::runtime::ivar_t {
//         offset: #offset,
//         name: #name as *const _,
//         encoded_type: #encoded_type as *const _,
//         alignment_raw: #objrs_root::__objrs::align_log_2::<#field_ty>(),
//         size: #objrs_root::__objrs::core::mem::size_of::<#field_ty>() as #native_ty::u32,
//       },
//     });
//   }

//   if !force_extern {
//     let ivar_list_ident = ivar_list_ident(class_name_str);

//     has_ivars = field_count > 0;
//     if has_ivars {
//       let ivar_list_export_name = ["\x01l_OBJC_$_INSTANCE_VARIABLES_", class_name_str].concat();
//       ivar_list = Some(quote! {
//         #[doc(hidden)]
//         #[repr(C)]
//         struct #ivar_list_ident {
//           entsize_and_flags: #native_ty::u32,
//           count: #native_ty::u32,
//           ivars: [#objrs_root::__objrs::runtime::ivar_t; #field_count],
//         }
//         #[doc(hidden)]
//         #[link_section = "__DATA,__objc_const"]
//         #[export_name = #ivar_list_export_name]
//         static #ivar_list_ident: #ivar_list_ident = {
//           unsafe impl #objrs_root::__objrs::core::marker::Sync for #ivar_list_ident {}
//           #ivar_list_ident {
//             entsize_and_flags: #objrs_root::__objrs::core::mem::size_of::<#objrs_root::__objrs::runtime::ivar_t>() as #native_ty::u32,
//             count: #field_count as #native_ty::u32,
//             ivars: {
//               #original_item
//               [#field_tokens]
//             },
//           }
//         };
//       });
//     } else {
//       ivar_list = Some(quote! {
//         #[doc(hidden)]
//         static #ivar_list_ident: () = ();
//       });
//     }
//   } else {
//     has_ivars = false;
//     ivar_list = None;
//   }

//   let mut item_fields = item.clone();
//   item_fields.ident = priv_ident.clone();
//   item_fields.generics = Default::default();

//   match item_fields.fields {
//     Fields::Named(_) => fields_init = quote!({ #fields_init }),
//     Fields::Unnamed(_) => fields_init = quote!((#fields_init)),
//     Fields::Unit => (),
//   }

//   for field in item_fields.fields.iter_mut() {
//     let new_ty;
//     {
//       let ty = &field.ty;
//       let span = field.ty.span();
//       let head = quote_spanned!(span => #objrs_root::__objrs::Field<);
//       let tail = quote_spanned!(span => >);
//       new_ty = parse_quote!(#head #ty #tail);
//     }
//     field.ty = new_ty;
//   }

//   let vis = &item.vis;

//   let generics = &item.generics;
//   let where_clause = &generics.where_clause;

//   let impls;
//   let super_instance_end;
//   // let superclass_test;
//   if super_class.is_none() {
//     impls = quote! {
//       unsafe impl #generics #objrs_root::__objrs::runtime::__objrs::RootClass for #pub_ident <#generic_idents> #where_clause {}
//       unsafe impl #generics #objrs_root::marker::RootClass for #pub_ident <#generic_idents> #where_clause {}
//     };
//     super_instance_end = quote!(0);
//   // superclass_test = quote!{
//   //   assert_eq!(superclass, #objrs_root::__objrs::core::ptr::null_mut(), "Objective-C class `{}` is not a root class", #self_as_class::CLASS_NAME);
//   // };
//   } else {
//     impls = quote! {
//       unsafe impl #generics #objrs_root::__objrs::runtime::__objrs::NonRootClass for #pub_ident <#generic_idents> #where_clause {
//         type Super = #super_class;
//       }

//       unsafe impl #generics #objrs_root::marker::NonRootClass for #pub_ident <#generic_idents> #where_clause {
//         type Super = #super_class;
//       }
//     };
//     super_instance_end = quote! {
//       <#super_class as #objrs_root::__objrs::runtime::__objrs::Class>::INSTANCE_SIZE
//     };

//     // superclass_test = quote!{
//     //     assert_ne!(superclass, #objrs_root::__objrs::core::ptr::null_mut(), "Objective-C class `{}`'s superclass `{}` not found in the runtime", #self_as_class::CLASS_NAME, <#self_as_nonroot_class::Super as #objrs_root::__objrs::runtime::__objrs::Class>::CLASS_NAME);
//     //     let expected_superclass = unsafe { #objrs_root::__objrs::runtime::objc_getClass(<#self_as_nonroot_class::Super as #objrs_root::__objrs::runtime::__objrs::Class>::CLASS_NAME_CSTR as *const _ as *const _) };
//     //     assert_eq!(superclass, expected_superclass, "Objective-C class `{}` doesn't have the expected superclass `{}`", #self_as_class::CLASS_NAME, <#self_as_nonroot_class::Super as #objrs_root::__objrs::runtime::__objrs::Class>::CLASS_NAME);
//     // };
//   }

//   // let test = quote!{
//   //   #[cfg(test)]
//   //   #[test]
//   //   fn #ident() {
//   //     // TODO: improve error messages. They can be improved.
//   //     let class = unsafe { #objrs_root::__objrs::runtime::objc_getClass(#self_as_class::CLASS_NAME_CSTR as *const _ as *const _) };
//   //     assert_ne!(class, #objrs_root::__objrs::core::ptr::null_mut(), "Objective-C class `{}` not found in the runtime", #self_as_class::CLASS_NAME);

//   //     let superclass = unsafe { #objrs_root::__objrs::runtime::class_getSuperclass(class) };
//   //     #superclass_test
//   //   }
//   // };

//   let cxx_construct_export_name = ["\x01-[", class_name_str, " .cxx_construct]"].concat();
//   let cxx_destruct_export_name = ["\x01-[", class_name_str, " .cxx_destruct]"].concat();
//   let pub_objrs = attr.objrs.unwrap_or_else(|| crate::util::priv_ident("objrs"));

//   let tokens = quote! {
//     extern crate #pub_objrs as #objrs_root;

//     // TODO: The ivars feature is incomplete. We need an internal-only type that has all the
//     // requested fields. And
//     #ivar_list

//     #item_fields

//     impl #generics #objrs_root::__objrs::Fields for #pub_ident <#generic_idents> #where_clause {
//       type Type = #priv_ident;

//       #[inline(always)]
//       fn from_ref(&self) -> #objrs_root::__objrs::ThisAndFields<Self, Self::Type> {
//         return <Self as #objrs_root::__objrs::Fields>::from_ptr(self as *const _ as *mut _);
//       }

//       #[inline(always)]
//       fn from_ptr(this: *mut Self) -> #objrs_root::__objrs::ThisAndFields<Self, Self::Type> {
//         #[allow(deprecated)]
//         return #objrs_root::__objrs::ThisAndFields {
//           this: this,
//           fields: #priv_ident #fields_init,
//         };
//       }
//     }

//     unsafe impl #generics #objrs_root::__objrs::runtime::__objrs::Class for #pub_ident <#generic_idents> #where_clause {
//       const CLASS_NAME: &'static #native_ty::str = #class_name;
//       const CLASS_NAME_CSTR: &'static #native_ty::str = #class_name_cstr;
//       const HAS_IVARS: #native_ty::bool = #has_ivars;
//       const IS_ROOT_CLASS: #native_ty::bool = #is_root_class;
//       const REQUIRES_CXX_CONSTRUCT: #native_ty::bool = #requires_cxx_construct;
//       const REQUIRES_CXX_DESTRUCT: #native_ty::bool = #requires_cxx_destruct;
//       const INSTANCE_START: #native_ty::usize = {
//         const ALIGN: #native_ty::usize = {
//           #original_item
//           #objrs_root::__objrs::core::mem::align_of::<#original_item_ident>()
//         };
//         ((#super_instance_end + (ALIGN - 1)) / ALIGN) * ALIGN
//       };
//       #[allow(deprecated)]
//       const INSTANCE_SIZE: #native_ty::usize = {
//         #original_item
//         // core::mem::size_of includes trailing padding. Objective-C omits the trailing padding
//         // here. We emulate this behavior by finding the field with the maximum offset and adding
//         // the field size to the field offset, thus omitting any trailing padding.
//         #unpadded_size_of
//         <Self as #objrs_root::__objrs::runtime::__objrs::Class>::INSTANCE_START + #prev_unpadded_size_of_ident
//       };
//       type FIELDS = #priv_ident;

//       #[export_name = #cxx_construct_export_name]
//       extern "C" fn cxx_construct(this: *mut Self, _: #native_ty::usize) -> *mut Self {
//         let this_as_u8 = this as *mut #native_ty::u8;
//         let mut this_and_fields = <Self as #objrs_root::__objrs::Fields>::from_ptr(this);
//         unsafe {
//           #cxx_construct
//         }
//         return this;
//       }

//       #[export_name = #cxx_destruct_export_name]
//       extern "C" fn cxx_destruct(this: *mut Self, _: #native_ty::usize) {
//         let this_as_u8 = this as *mut #native_ty::u8;
//         let mut this_and_fields = <Self as #objrs_root::__objrs::Fields>::from_ptr(this);
//         unsafe {
//           #cxx_destruct
//         }
//       }
//     }

//     unsafe impl #generics #objrs_root::marker::Class for #pub_ident <#generic_idents> #where_clause {}

//     #impls

//     #statics
//   };

//   return Ok(tokens.into());
// }

// // TODO: require ivars to implement the Default trait.
