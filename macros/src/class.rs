// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

extern crate core;
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

use ivar::IvarAttr;
use proc_macro::Diagnostic;
use proc_macro2::{Literal, Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned, ToTokens};
use syn::{
  alt, buffer::TokenBuffer, call, custom_keyword, do_parse, keyword, named, option, parens, parse2,
  parse_quote, punct, punctuated::Punctuated, spanned::Spanned, syn, synom::Synom, token::Comma,
  Attribute, Field, Fields, GenericParam, Ident, ItemStruct, LitByteStr, LitStr, Type, TypePath,
  Visibility,
};
use util;
use util::{link_attribute, priv_ident_at, DrainExt};

pub struct ClassAttr {
  class_name: Option<LitStr>,
  super_class_name: Option<LitStr>, // Only needed due to the lack of associated extern statics.
  super_class: Option<TypePath>,
  force_extern: bool,
  root_class_name: Option<LitStr>, // Only needed due to the lack of associated extern statics.
}

// #[objrs(class,
//         [name = "ExportName",]
//         root_class
//         [, extern][,])]
// #[objrs(class,
//         [name = "ExportName",]
//         super = NSObject
//         [, root_class = "ExportName"]
//         [, extern][,])]
// #[objrs(class,
//         [name = "ExportName",]
//         super(name = "ExportName",
//               type = NSObject[,])
//         [, root_class = "ExportName"]
//         [, extern][,])]
impl Synom for ClassAttr {
  named!(parse -> Self, do_parse!(
    custom_keyword!(class) >>
    punct!(,) >>
    name: option!(do_parse!(custom_keyword!(name) >> punct!(=) >> name: syn!(LitStr) >> (name))) >>
    super_info: alt!(
      do_parse!(keyword!(super) >>
        super_class: alt!(
          do_parse!(punct!(=) >> super_ty: syn!(TypePath) >> ((None, super_ty)))
          |
          parens!(do_parse!(custom_keyword!(name) >> punct!(=) >> super_name: syn!(LitStr) >> punct!(,) >>
                            keyword!(type) >> punct!(=) >> super_ty: syn!(TypePath) >> option!(punct!(,)) >>
                            (Some(super_name), super_ty))) => { |group| group.1 }
        ) >> root_class: option!(do_parse!(custom_keyword!(root_class) >> punct!(=) >> root_name: syn!(LitStr) >> (root_name)))
        >> ((super_class.0, Some(super_class.1), root_class))
      )
      |
      custom_keyword!(root_class) => { |_| (None, None, None) }
      ) >>
    force_extern: option!(do_parse!(punct!(,) >> keyword!(extern) >> (()))) >>
    option!(punct!(,)) >>
    (ClassAttr {
      class_name: name,
      super_class_name: super_info.0,
      super_class: super_info.1,
      force_extern: force_extern.is_some(),
      root_class_name: super_info.2,
    })
  ));

  fn description() -> Option<&'static str> {
    return Some("objrs class attribute");
  }
}

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

fn strip_ivar_attr(field: &mut Field) -> Result<Option<Attribute>, Diagnostic> {
  let mut iter = DrainExt::drain(&mut field.attrs, |attr: &mut Attribute| {
    let path = &attr.path;
    let segments = &path.segments;
    return path.leading_colon.is_none() && segments.len() == 1 && segments[0].ident == "objrs";
  });
  let ivar_attr = iter.next();
  if let Some(duplicate_attr) = iter.next() {
    return Err(duplicate_attr.span().unstable().error("unexpected secondary objrs attribute"));
  }
  return Ok(ivar_attr);
}

fn parse_ivar(field: &mut Field, force_extern: bool) -> Result<IvarAttr, Diagnostic> {
  let ivar_attr: IvarAttr;
  match strip_ivar_attr(field)? {
    Some(attr) => {
      let span = attr.span();
      ivar_attr = parse2(attr.tts).map_err(|e| span.unstable().error(e.to_string()))?;
    }
    None => return Ok(IvarAttr::default()),
  }

  if force_extern {
    if let Some(ref default) = ivar_attr.default {
      return Err(
        default.span().unstable().error("extern classes cannot specify default values for ivars"),
      );
    }
  }

  return Ok(ivar_attr);
}

pub fn parse_class(attr: ClassAttr, input: TokenStream) -> Result<TokenStream, Diagnostic> {
  let input = TokenBuffer::new2(input);
  let mut item = match <ItemStruct as Synom>::parse(input.begin()) {
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

  let pub_ident = &item.ident;
  let priv_ident = priv_ident_at(&pub_ident.to_string(), pub_ident.span());
  let mut original_item = item.clone();
  original_item.attrs.push(parse_quote!(#[allow(non_camel_case_types)]));
  original_item.attrs.push(parse_quote!(#[allow(dead_code)]));
  let original_item_ident = ["__objrs_original_", &pub_ident.to_string()].concat();
  original_item.ident = priv_ident_at(&original_item_ident, pub_ident.span());
  let original_item_ident = &original_item.ident;
  original_item.generics = Default::default();
  for field in original_item.fields.iter_mut() {
    strip_ivar_attr(field)?;
  }

  let class_name;
  if let Some(name) = attr.class_name {
    class_name = name;
  } else {
    class_name = LitStr::new(&item.ident.to_string(), item.ident.span());
  }
  let class_name_str = &class_name.value();
  let class_name_cstr = LitStr::new(&[&class_name_str, "\0"].concat(), class_name.span());

  let super_class = attr.super_class;
  let super_class_name = attr.super_class_name.or_else(|| {
    let super_class_ident = super_class
      .as_ref()
      .and_then(|type_path| type_path.path.segments.last().map(|p| p.value().ident.clone()));
    return super_class_ident.map(|ident| LitStr::new(&ident.to_string(), ident.span())); // TODO: use def_site.
  });
  let is_root_class = super_class.is_none();

  let root_metaclass_link;
  let root_metaclass_link_name;
  if let Some(ref name) = attr.root_class_name {
    root_metaclass_link_name =
      LitStr::new(&["OBJC_METACLASS_$_", &name.value()].concat(), name.span());
    root_metaclass_link = TokenStream::new();
  } else {
    root_metaclass_link_name = LitStr::new("OBJC_METACLASS_$_NSObject", Span::call_site());
    root_metaclass_link = quote!(#[link(name = "Foundation", kind = "framework")]);
  };

  let root_metaclass_ident = root_metaclass_ident(class_name_str);
  let super_metaclass_ident = super_metaclass_ident(class_name_str);
  // TODO: use __objrs_root instead of objrs.
  let mut statics = quote!{
    #root_metaclass_link
    extern "C" {
      #[doc(hidden)]
      #[link_name = #root_metaclass_link_name]
      static #root_metaclass_ident: objrs::runtime::objc_class;
    }
  };

  let super_class_ident = super_class_ident(class_name_str);
  let super_metaclass_link_name;
  if let Some(ref super_class_name) = super_class_name {
    let super_class_name = &super_class_name.value();
    super_metaclass_link_name = ["OBJC_METACLASS_$_", super_class_name].concat();
    // LitStr::new(&["OBJC_METACLASS_$_", super_class_name].concat(), super_class_name.span()); // TODO: use def_site

    let super_class_link_name = ["OBJC_CLASS_$_", super_class_name].concat(); //LitStr::new(&["OBJC_CLASS_$_", super_class_name].concat(), super_class_name.span()); // TODO: use def_site
    statics.extend(quote!{
      extern "C" {
        #[doc(hidden)]
        // TODO: use __objrs_root instead of objrs.
        #[link_name = #super_class_link_name]
        static #super_class_ident: objrs::runtime::objc_class;
      }
    });
  } else {
    super_metaclass_link_name = ["OBJC_CLASS_$_", class_name_str].concat(); //LitStr::new(&class_export_name, class_impl.class_name.span()); // TODO: use def_site
    statics.extend(quote!{
      #[doc(hidden)]
      static #super_class_ident: () = ();
    });
  }
  // TODO: use __objrs_root instead of objrs.
  statics.extend(quote!{
    extern "C" {
      #[doc(hidden)]
      #[link_name = #super_metaclass_link_name]
      static #super_metaclass_ident: objrs::runtime::objc_class;
    }
  });

  // TODO: Add a unit test that checks ivar names and types (same as the unit test to check selector
  // argument/return types).

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

  let link_attr = link_attribute(&item.attrs);
  let force_extern = attr.force_extern || link_attr.is_some();
  let offset_name_prefix = ["OBJC_IVAR_$_", class_name_str, "."].concat();
  let offset_name_prefix = offset_name_prefix.as_ref();
  let ivar_list;
  let has_ivars;
  let mut requires_cxx_construct = TokenStream::new();
  let mut requires_cxx_destruct = TokenStream::new();
  let mut cxx_construct = TokenStream::new();
  let mut cxx_destruct = TokenStream::new();
  let mut prev_unpadded_size_of_ident = util::priv_ident("UNPADDED_SIZE_OF_START");
  let mut unpadded_size_of = quote!{
    const FIELD_END_START: usize = 0;
    const #prev_unpadded_size_of_ident: usize = 0;
  };

  let ivar_name_prefix =
    &["\x01L_OBJC_METH_VAR_NAME_.__objrs_ivar.", class_name_str, "::"].concat();
  let ivar_type_prefix =
    &["\x01L_OBJC_METH_VAR_TYPE_.__objrs_ivar.", class_name_str, "::"].concat();

  let mut fields_init = TokenStream::new();
  let mut field_count: usize = 0;
  let mut field_tokens = quote!();
  let mut force_cxx_construct: bool = false;
  for (i, field) in item.fields.iter_mut().enumerate() {
    field_count += 1;
    let field_ident: TokenTree =
      field.ident.clone().map_or_else(|| Literal::usize_unsuffixed(i).into(), |ident| ident.into());

    let attr = parse_ivar(field, force_extern)?;
    let ivar_ident_str: &str =
      &attr.name.map_or_else(|| field_ident.to_string(), |lit| lit.value());

    let field_end_ident = util::priv_ident(&format!("FIELD_END_{}", i));
    let unpadded_size_of_ident = util::priv_ident(&format!("UNPADDED_SIZE_OF_{}", i));
    let field_ty = &field.ty;
    unpadded_size_of.extend(quote!{
      const #field_end_ident: usize = __objrs_root::__objrs::core::mem::size_of::<#field_ty>() + unsafe { __objrs_root::__objrs::TransmuteHack::<_, usize> { from: &__objrs_root::__objrs::TransmuteHack::<usize, &#original_item_ident> { from: 0 }.to.#field_ident }.to };
      const #unpadded_size_of_ident: usize = (((#prev_unpadded_size_of_ident > #field_end_ident) as usize) * #prev_unpadded_size_of_ident) | (((#prev_unpadded_size_of_ident <= #field_end_ident) as usize) * #field_end_ident);
    });
    prev_unpadded_size_of_ident = unpadded_size_of_ident;

    let field_colon = field.ident.as_ref().map_or_else(TokenStream::new, |ident| quote!(#ident:));
    let offset_export_name = [offset_name_prefix, ivar_ident_str].concat();
    fields_init.extend(quote!{
      #field_colon {
      #link_attr
      extern "C" {
        #[link_name = #offset_export_name]
        static IVAR_OFFSET: usize;
      }
      __objrs_root::__objrs::Field(unsafe { __objrs_root::__objrs::core::mem::transmute::<&usize, *mut #field_ty>(&IVAR_OFFSET) })
    },});

    if force_extern {
      continue;
    }

    let offset_export_name = [offset_name_prefix, ivar_ident_str].concat();
    let offset = quote!{{
      // Objective-C marks this as .private_extern in the assembly. I don't think we have a way to
      // do that in Rust.
      #[link_section = "__DATA,__objc_ivar"]
      #[export_name = #offset_export_name]
      // TODO: support generics.
      // This is equivalent to the C code `(size_t)&((T *)0)->field`. This is UB in Rust since it
      // creates a null reference, but I haven't been able to come up with a non-UB alternative.
      static IVAR_OFFSET: usize = <#pub_ident as __objrs_root::runtime::__objrs::Class>::INSTANCE_START + unsafe { __objrs_root::__objrs::TransmuteHack::<_, usize> { from: &__objrs_root::__objrs::TransmuteHack::<usize, &#original_item_ident> { from: 0 }.to.#field_ident }.to };
      &IVAR_OFFSET as *const _ as *mut _
    }};

    let ivar_name_export_name = [ivar_name_prefix, ivar_ident_str].concat();
    let ivar_name = [ivar_ident_str, "\x00"].concat();
    let ivar_name = ivar_name.as_bytes();
    let ivar_name_len = ivar_name.len();
    let ivar_name = LitByteStr::new(ivar_name, Span::call_site()); // TODO: use def_site().
    let name = quote!{{
      #[link_section = "__TEXT,__objc_methname,cstring_literals"]
      #[export_name = #ivar_name_export_name]
      static IVAR_NAME: [u8; #ivar_name_len] = *#ivar_name;
      &IVAR_NAME as *const u8
    }};

    if !force_cxx_construct {
      if attr.default.is_some() {
        force_cxx_construct = true;
        requires_cxx_construct = quote!(true);
      } else {
        requires_cxx_construct.extend(quote!{
          <#field_ty as __objrs_root::__objrs::RequiresCxxConstruct>::VALUE ||
        });
      }
    }
    requires_cxx_destruct.extend(quote!{
      <#field_ty as __objrs_root::__objrs::RequiresCxxDestruct>::VALUE ||
    });

    if let Some(default) = attr.default {
      cxx_construct.extend(quote!{
        __objrs_root::__objrs::Field::construct_with_value(&mut this_and_fields.fields.#field_ident, this_as_u8, #default);
      });
    } else {
      cxx_construct.extend(quote!{
        if <#field_ty as __objrs_root::__objrs::RequiresCxxConstruct>::VALUE {
          __objrs_root::__objrs::Field::construct(&mut this_and_fields.fields.#field_ident, this_as_u8);
        }
      });
    }
    cxx_destruct.extend(quote!{
      if <#field_ty as __objrs_root::__objrs::RequiresCxxDestruct>::VALUE && __objrs_root::__objrs::core::mem::needs_drop::<#field_ty>() {
        __objrs_root::__objrs::Field::destruct(&mut this_and_fields.fields.#field_ident, this_as_u8);
      }
    });

    let ivar_type_export_name = [ivar_type_prefix, ivar_ident_str].concat();
    let encoded_type = quote!{{
      #[link_section = "__TEXT,__objc_methtype,cstring_literals"]
      #[export_name = #ivar_type_export_name]
      static IVAR_TYPE: __objrs_root::__objrs::Packed2<<#field_ty as __objrs_root::__objrs::TypeEncodingHack>::Type, u8> = __objrs_root::__objrs::Packed2(<#field_ty as __objrs_root::__objrs::TypeEncodingHack>::BYTES, b'\x00');
      unsafe { __objrs_root::__objrs::TransmuteHack::<_, *const u8> { from: &IVAR_TYPE }.to }
    }};

    let alignment_raw = quote!{{
      const ALIGN: usize = __objrs_root::__objrs::core::mem::align_of::<#field_ty>();
      (ALIGN >= 0x00000002) as u32 + (ALIGN >= 0x00000004) as u32 + (ALIGN >= 0x00000008) as u32 +
      (ALIGN >= 0x00000010) as u32 + (ALIGN >= 0x00000020) as u32 + (ALIGN >= 0x00000040) as u32 +
      (ALIGN >= 0x00000080) as u32 + (ALIGN >= 0x00000100) as u32 + (ALIGN >= 0x00000200) as u32 +
      (ALIGN >= 0x00000400) as u32 + (ALIGN >= 0x00000800) as u32 + (ALIGN >= 0x00001000) as u32 +
      (ALIGN >= 0x00002000) as u32 + (ALIGN >= 0x00004000) as u32 + (ALIGN >= 0x00008000) as u32 +
      (ALIGN >= 0x00010000) as u32 + (ALIGN >= 0x00020000) as u32 + (ALIGN >= 0x00040000) as u32 +
      (ALIGN >= 0x00080000) as u32 + (ALIGN >= 0x00100000) as u32 + (ALIGN >= 0x00200000) as u32 +
      (ALIGN >= 0x00400000) as u32 + (ALIGN >= 0x00800000) as u32 + (ALIGN >= 0x01000000) as u32 +
      (ALIGN >= 0x02000000) as u32 + (ALIGN >= 0x04000000) as u32 + (ALIGN >= 0x08000000) as u32 +
      (ALIGN >= 0x10000000) as u32 + (ALIGN >= 0x20000000) as u32 + (ALIGN >= 0x40000000) as u32 +
      (ALIGN >= 0x80000000) as u32
    }};

    // TODO: the following is incorrect and incomplete. It builds the weak ivar layout counting
    // the number of fields. Instead, the layout should be based on the number of bytes and the
    // offset of each field. That really complicates things. I think we need const fn in order to
    // do this properly.
    // quote!{{
    //   const WEAK_IVAR_LAYOUT: [u8; 1] = [0x00];  // TODO: replace with a proper value.
    //   const OLD_LEN: usize = 1;  // TODO: replace with real value.

    //   const IS_WEAK: bool = <#field_ty as __objrs_root::__objrs::IsWeak>::VALUE;
    //   const LAST_BYTE: u8 = WEAK_IVAR_LAYOUT[OLD_LEN - 1];  // TODO: replace with real value.
    //   const WEAKS: u8 = LAST_BYTE & 0x0f;
    //   const SKIPS: u8 = LAST_BYTE >> 4;
    //   const NEW_WEAKS: u8 = WEAKS + IS_WEAK as u8;
    //   const NEW_SKIPS: u8 = SKIPS + !IS_WEAK as u8;
    //   const PUSH: bool = NEW_WEAKS | NEW_SKIPS > 0x0f || (!IS_WEAK && WEAKS > 0);
    //   const MASK: u8 = 0xff * PUSH as u8;
    //   const NEW_LAST_BYTE: u8 = ((IS_WEAK as u8) & MASK) |
    //                             ((((NEW_SKIPS & 0x0f) << 4) | (NEW_WEAKS & 0x0f)) & !MASK);
    //   const TRUNCATED_LEN: usize = OLD_LEN - !PUSH as usize;

    //   unsafe { __objrs_root::__objrs::TransmuteHack::<_, [u8; TRUNCATED_LEN + 1]> { from: __objrs_root::__objrs::Packed2(__objrs_root::__objrs::TransmuteHack::<_, [u8; TRUNCATED_LEN]> { from: WEAK_IVAR_LAYOUT }.to, NEW_LAST_BYTE) }.to }
    // }};
    // TODO: convert WEAK_IVAR_LAYOUT to nil if no ivars are weak.
    // TODO: truncate the trailing SKIPS in the array.

    field_tokens.extend(quote!{
      __objrs_root::runtime::ivar_t {
        offset: #offset,
        name: #name as *const _,
        encoded_type: #encoded_type as *const _,
        alignment_raw: #alignment_raw,
        size: __objrs_root::__objrs::core::mem::size_of::<#field_ty>() as u32,
      },
    });
  }

  if !force_extern {
    let ivar_list_ident = ivar_list_ident(class_name_str);

    has_ivars = field_count > 0;
    if has_ivars {
      let ivar_list_export_name = ["\x01l_OBJC_$_INSTANCE_VARIABLES_", class_name_str].concat();
      ivar_list = Some(quote!{
        #[doc(hidden)]
        #[repr(C)]
        struct #ivar_list_ident {
          entsize_and_flags: u32,
          count: u32,
          // TODO: use __objrs_root instead of objrs.
          ivars: [objrs::runtime::ivar_t; #field_count],
        }
        #[doc(hidden)]
        #[link_section = "__DATA,__objc_const"]
        #[export_name = #ivar_list_export_name]
        static #ivar_list_ident: #ivar_list_ident = {
          extern crate objrs as __objrs_root;
          unsafe impl __objrs_root::__objrs::core::marker::Sync for #ivar_list_ident {}
          #ivar_list_ident {
            entsize_and_flags: __objrs_root::__objrs::core::mem::size_of::<__objrs_root::runtime::ivar_t>() as u32,
            count: #field_count as u32,
            ivars: {
              #original_item
              [#field_tokens]
            },
          }
        };
      });
    } else {
      ivar_list = Some(quote!{
        #[doc(hidden)]
        static #ivar_list_ident: () = ();
      });
    }
  } else {
    has_ivars = false;
    ivar_list = None;
  }

  let mut item_fields = item.clone();
  item_fields.ident = priv_ident.clone();
  item_fields.generics = Default::default();

  match item_fields.fields {
    Fields::Named(_) => fields_init = quote!({ #fields_init }),
    Fields::Unnamed(_) => fields_init = quote!((#fields_init)),
    Fields::Unit => (),
  }

  for field in item_fields.fields.iter_mut() {
    let new_ty;
    {
      let ty = &field.ty;
      let span = field.ty.span();
      // TODO: use __objrs_root instead of objrs.
      let head = quote_spanned!(span => objrs::__objrs::Field<);
      let tail = quote_spanned!(span => >);
      new_ty = parse_quote!(#head #ty #tail);
    }
    field.ty = new_ty;
  }

  let vis = &item.vis;

  let mut unnamed = 0u32;
  let mut make_field = |ty: Type| -> Field {
    use util::priv_ident;
    let ident = ["__objrs_field_", &unnamed.to_string()].concat();
    unnamed += 1;
    let ident = priv_ident(&ident);
    return Field {
      attrs: vec![],
      vis: Visibility::Inherited,
      ident: Some(ident),
      colon_token: None,
      ty: ty,
    };
  };
  let this_field = {
    use util::priv_ident;
    priv_ident("__objrs_field_this")
  };

  // TODO: use __objrs_root instead of objrs.
  let mut fields: Punctuated<Field, Comma> = Punctuated::new();
  for generic_type in generic_types {
    fields.push(make_field(parse_quote!(objrs::__objrs::core::marker::PhantomData<#generic_type>)));
  }
  for generic_lifetime in generic_lifetimes {
    fields.push(make_field(parse_quote!{
      objrs::__objrs::core::marker::PhantomData<& #generic_lifetime u8>
    }));
  }
  fields.push(Field {
    attrs: vec![],
    vis: Visibility::Inherited,
    ident: Some(this_field.clone()),
    colon_token: None,
    ty: super_class.clone().map_or_else(|| parse_quote!(objrs::__objrs::Opaque), Type::Path),
  });

  let generics = &item.generics;
  let where_clause = &generics.where_clause;

  let impls;
  let super_instance_end;
  // let superclass_test;
  if super_class.is_none() {
    // TODO: use __objrs_root instead of objrs.
    impls = quote!{
      unsafe impl #generics objrs::runtime::__objrs::RootClass for #pub_ident <#generic_idents> #where_clause {}
      unsafe impl #generics objrs::marker::RootClass for #pub_ident <#generic_idents> #where_clause {}
    };
    super_instance_end = quote!(0);
  // superclass_test = quote!{
  //   assert_eq!(superclass, objrs::__objrs::core::ptr::null_mut(), "Objective-C class `{}` is not a root class", #self_as_class::CLASS_NAME);
  // };
  } else {
    // TODO: use __objrs_root instead of objrs.
    impls = quote!{
      impl #generics objrs::__objrs::core::ops::Deref for #pub_ident <#generic_idents> #where_clause {
        type Target = #super_class;

        #[inline(always)]
        fn deref(&self) -> &Self::Target {
          return &self.#this_field;
        }
      }

      impl #generics objrs::__objrs::core::ops::DerefMut for #pub_ident <#generic_idents> #where_clause {
        #[inline(always)]
        fn deref_mut(&mut self) -> &mut Self::Target {
          return &mut self.#this_field;
        }
      }

      unsafe impl #generics objrs::runtime::__objrs::NonRootClass for #pub_ident <#generic_idents> #where_clause {
        type Super = #super_class;
      }

      unsafe impl #generics objrs::marker::NonRootClass for #pub_ident <#generic_idents> #where_clause {
        type Super = #super_class;
      }
    };
    super_instance_end = quote!{
      <#super_class as __objrs_root::runtime::__objrs::Class>::INSTANCE_SIZE
    };

    // superclass_test = quote!{
    //     assert_ne!(superclass, objrs::__objrs::core::ptr::null_mut(), "Objective-C class `{}`'s superclass `{}` not found in the runtime", #self_as_class::CLASS_NAME, <#self_as_nonroot_class::Super as objrs::runtime::__objrs::Class>::CLASS_NAME);
    //     let expected_superclass = unsafe { objrs::runtime::objc_getClass(<#self_as_nonroot_class::Super as objrs::runtime::__objrs::Class>::CLASS_NAME_CSTR as *const _ as *const _) };
    //     assert_eq!(superclass, expected_superclass, "Objective-C class `{}` doesn't have the expected superclass `{}`", #self_as_class::CLASS_NAME, <#self_as_nonroot_class::Super as objrs::runtime::__objrs::Class>::CLASS_NAME);
    // };
  }

  // let test = quote!{
  //   #[cfg(test)]
  //   #[test]
  //   fn #ident() {
  //     extern crate objrs as __objrs_root;

  //     // TODO: improve error messages. They can be improved.
  //     let class = unsafe { __objrs_root::runtime::objc_getClass(#self_as_class::CLASS_NAME_CSTR as *const _ as *const _) };
  //     assert_ne!(class, __objrs_root::__objrs::core::ptr::null_mut(), "Objective-C class `{}` not found in the runtime", #self_as_class::CLASS_NAME);

  //     let superclass = unsafe { __objrs_root::runtime::class_getSuperclass(class) };
  //     #superclass_test
  //   }
  // };

  let cxx_construct_export_name = ["\x01-[", class_name_str, " .cxx_construct]"].concat();
  let cxx_destruct_export_name = ["\x01-[", class_name_str, " .cxx_destruct]"].concat();

  let tokens = quote!{
    // TODO: apply the item's user-provided attributes?
    // TODO: The fields here are accessible to the user. Given that these fields are internal-only
    // (and are only present to satisfy requirements to use the generic parameters), these fields
    // should not be available to the user.
    #[repr(transparent)]
    #vis struct #pub_ident #generics #where_clause {
      #fields
    }

    // TODO: The ivars feature is incomplete. We need an internal-only type that has all the
    // requested fields. And
    #ivar_list

    #item_fields

    // TODO: use __objrs_root instead of objrs.
    impl #generics objrs::__objrs::Fields for #pub_ident <#generic_idents> #where_clause {
      type Type = #priv_ident;

      // TODO: use __objrs_root instead of objrs.
      #[inline(always)]
      fn from_ref(&self) -> objrs::__objrs::ThisAndFields<Self, Self::Type> {
        extern crate objrs as __objrs_root;
        return <Self as __objrs_root::__objrs::Fields>::from_ptr(self as *const _ as *mut _);
      }

      #[inline(always)]
      fn from_ptr(this: *mut Self) -> objrs::__objrs::ThisAndFields<Self, Self::Type> {
        extern crate objrs as __objrs_root;

        #[allow(deprecated)]
        return __objrs_root::__objrs::ThisAndFields {
          this: this,
          fields: #priv_ident #fields_init,
        };
      }
    }

    // TODO: use __objrs_root instead of objrs.
    unsafe impl #generics objrs::runtime::__objrs::Class for #pub_ident <#generic_idents> #where_clause {
      const CLASS_NAME: &'static str = #class_name;
      const CLASS_NAME_CSTR: &'static str = #class_name_cstr;
      const HAS_IVARS: bool = #has_ivars;
      const IS_ROOT_CLASS: bool = #is_root_class;
      const REQUIRES_CXX_CONSTRUCT: bool = {
        extern crate objrs as __objrs_root;
        #requires_cxx_construct false
      };
      const REQUIRES_CXX_DESTRUCT: bool = {
        extern crate objrs as __objrs_root;
        #requires_cxx_destruct false
      };
      const INSTANCE_START: usize = {
        extern crate objrs as __objrs_root;
        const ALIGN: usize = {
          #original_item
          __objrs_root::__objrs::core::mem::align_of::<#original_item_ident>()
        };
        ((#super_instance_end + (ALIGN - 1)) / ALIGN) * ALIGN
      };
      #[allow(deprecated)]
      const INSTANCE_SIZE: usize = {
        extern crate objrs as __objrs_root;
        #original_item
        // core::mem::size_of includes trailing padding. Objective-C omits the trailing padding
        // here. We emulate this behavior by finding the field with the maximum offset and adding
        // the field size to the field offset, thus omitting any trailing padding.
        #unpadded_size_of
        <Self as __objrs_root::runtime::__objrs::Class>::INSTANCE_START + #prev_unpadded_size_of_ident
      };
      type FIELDS = #priv_ident;

      #[export_name = #cxx_construct_export_name]
      extern "C" fn cxx_construct(this: *mut Self, _: usize) -> *mut Self {
        extern crate objrs as __objrs_root;
        let this_as_u8 = this as *mut u8;
        let mut this_and_fields = <Self as __objrs_root::__objrs::Fields>::from_ptr(this);
        unsafe {
          #cxx_construct
        }
        return this;
      }

      #[export_name = #cxx_destruct_export_name]
      extern "C" fn cxx_destruct(this: *mut Self, _: usize) {
        extern crate objrs as __objrs_root;
        let this_as_u8 = this as *mut u8;
        let mut this_and_fields = <Self as __objrs_root::__objrs::Fields>::from_ptr(this);
        unsafe {
          #cxx_destruct
        }
      }
    }

    // TODO: use __objrs_root instead of objrs.
    unsafe impl #generics objrs::marker::Class for #pub_ident <#generic_idents> #where_clause {}

    #impls

    #statics
  };

  return Ok(tokens.into());
}

// TODO: require ivars to implement the Default trait.
