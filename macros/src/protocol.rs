// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

extern crate core;

// For Objective-C property attributes, see https://github.com/llvm-mirror/clang/blob/master/lib/Parse/ParseObjc.cpp

use proc_macro::Diagnostic;
use proc_macro2::{Span, TokenStream};
use property::PropertyAttr;
use quote::{quote, ToTokens};
use selector::{parse_selector, MethodType, SelectorAttr};
use std::collections::HashSet;
use syn::{
parse2, parse_quote,
punctuated::Punctuated, spanned::Spanned,
  token::Extern, Abi, Attribute, Block, GenericParam, Ident, ImplItemMethod, ItemTrait, LitStr,
  TraitItem, TraitItemMethod, Visibility,
};
use syn::parse::{Parse, ParseStream};
use util::{is_instance_method, link_attribute, DrainExt};

pub struct ProtocolAttr {
  pub name: Option<LitStr>,
  pub id_ident: Ident,
  pub properties: Vec<PropertyAttr>,
  pub force_extern: bool,
}

// #[objrs(protocol
//         [,name = "name"]
//         ,id_ident = IDENT
//         [,property(...)]
//         [,extern][,])]
impl Parse for ProtocolAttr {
  fn parse(input: ParseStream) -> syn::parse::Result<Self> {
    use util::{KV, id_ident, property, name, protocol};

    let mut kv = KV::new(input);
    kv.parse::<protocol, _>()?;
    let name: Option<LitStr> = kv.parse::<name, _>()?;
    let id_ident: Ident = kv.parse::<id_ident, _>()?;
    let mut properties = vec![];
    while let Some(property) = kv.parse::<property, _>()? {
      properties.push(property);
    }
    let force_extern: Option<()> = kv.parse::<Extern, _>()?;
    kv.eof()?;
    return Ok(
      ProtocolAttr {
        name: name,
        id_ident: id_ident,
        properties: properties,
        force_extern: force_extern.is_some(),
      }
    );
  }
}

struct ProtocolMethod {
  selector: SelectorAttr,
  msg_send: TraitItemMethod,
  msg_recv: TraitItemMethod,
  is_instance_method: bool,
}

fn parse_method(method: &mut TraitItemMethod) -> Result<ProtocolMethod, Diagnostic> {
  let raw_sel_attr;
  {
    let mut iter = DrainExt::drain(&mut method.attrs, |attr: &mut Attribute| {
      let path = &attr.path;
      let segments = &path.segments;
      return path.leading_colon.is_none() && segments.len() == 1 && segments[0].ident == "objrs";
    });
    raw_sel_attr = iter.next();
    if let Some(duplicate_attr) = iter.next() {
      return Err(
        duplicate_attr.span().unstable().error("unexpected secondary objrs attribute found"),
      );
    }
  }

  let selector_attr: SelectorAttr;
  match raw_sel_attr {
    Some(attr) => {
      let span = attr.span();
      selector_attr = parse2(attr.tts).map_err(|e| span.unstable().error(e.to_string()))?;
    }
    None => {
      return Err(
        method
          .sig
          .span()
          .unstable()
          .error("function is missing #[objrs(selector ...)] attribute")
          .note("objrs requires all trait methods in a protocol to specify a selector"),
      )
    }
  }
  if let Some(ref default) = method.default {
    return Err(
      default
        .span()
        .unstable()
        .error("expected `;`, found `{`")
        .note("objrs does not allow default trait implementations for protocols"),
    );
  }

  let is_instance_method = selector_attr.method_type == MethodType::Instance
    || (selector_attr.method_type == MethodType::Auto
      && is_instance_method(&method.sig.decl.inputs));

  if is_instance_method && selector_attr.sel.value() == "dealloc" {
    return Err(
      selector_attr
        .sel
        .span()
        .unstable()
        .error("method must not use the selector \"dealloc\"")
        .note("objrs does not allow a protocol that defines a \"dealloc\" method"),
    );
  }

  let mut msg_send = method.clone();
  msg_send.sig.ident = Ident::new(
    &["__objrs_msg_send_", &method.sig.ident.to_string()].concat(),
    method.sig.ident.span(),
  );
  if !is_instance_method {
    msg_send.sig.decl.inputs.insert(0, parse_quote!(_: usize));
  }
  msg_send.sig.decl.inputs.insert(1, parse_quote!(_: usize));
  msg_send.sig.abi = Some(Abi {
    extern_token: Extern(Span::call_site()),
    name: Some(LitStr::new("C", Span::call_site())),
  });

  let mut msg_recv = msg_send.clone();
  msg_recv.sig.ident = Ident::new(
    &["__objrs_msg_recv_", &method.sig.ident.to_string()].concat(),
    method.sig.ident.span(),
  );

  if selector_attr.optional.is_some() {
    method.attrs.push(parse_quote!(#[allow(unused_variables)]));
    msg_send.attrs.push(parse_quote!(#[allow(unused_variables)]));
    msg_recv.attrs.push(parse_quote!(#[allow(unused_variables)]));
    method.semi_token = None;
    msg_send.semi_token = None;
    msg_recv.semi_token = None;

    let method_name = method.sig.ident.to_string();
    let sel = selector_attr.sel.value();

    let panic_message =
      format!("Unimplemented optional method `{}` (selector: \"{}\") called", method_name, sel,);
    method.default = Some(parse_quote!{{
      extern crate objrs as __objrs_root;
      __objrs_root::__objrs::core::panic!(#panic_message);
    }});

    let panic_message = format!(
      "Private objrs method `__objrs_msg_send_{}` (created to assist the optional method `{}` (selector: \"{}\")) called",
      method_name, method_name, sel,
    );
    msg_send.default = Some(parse_quote!{{
      extern crate objrs as __objrs_root;
      __objrs_root::__objrs::core::panic!(#panic_message);
    }});

    let panic_message = format!(
      "Private objrs method `__objrs_msg_recv_{}` (created to assist the optional method `{}` (selector: \"{}\")) called",
      method_name, method_name, sel,
    );
    msg_recv.default = Some(parse_quote!{{
      extern crate objrs as __objrs_root;
      __objrs_root::__objrs::core::panic!(#panic_message);
    }});
  }

  return Ok(ProtocolMethod {
    selector: selector_attr,
    msg_send: msg_send,
    msg_recv: msg_recv,
    is_instance_method: is_instance_method,
  });
}

pub fn parse_protocol(attr: ProtocolAttr, input: TokenStream) -> Result<TokenStream, Diagnostic> {
  let mut item;
  match parse2::<ItemTrait>(input) {
    Ok(value) => item = value,
    Err(error) => {
      return Err(
        error
          .span()
          .unstable()
          .error(format!("failed to parse trait: {}", error.to_string()))
          .note("#[objrs(protocol ...)] must only be applied to a trait item"),
      );
    }
  };

  let force_extern = attr.force_extern || link_attribute(&item.attrs).is_some();
  if !force_extern {
    return Err(
      Span::call_site()
        .unstable()
        .error("unsupported custom protocol")
        .note("objrs currently only supports external protocols"),
    );
  }

  let mut expected_instance_properties = HashSet::new();
  let mut expected_class_properties = HashSet::new();
  for property in attr.properties.iter() {
    if property.class.is_none() {
      expected_instance_properties.insert(property.getter());
      if let Some(setter) = property.setter() {
        expected_instance_properties.insert(setter);
      }
    } else {
      expected_class_properties.insert(property.getter());
      if let Some(setter) = property.setter() {
        expected_class_properties.insert(setter);
      }
    }
  }

  let mut id_items = TokenStream::new();
  let mut sub_items = Vec::new();
  for sub_item in item.items {
    match sub_item {
      TraitItem::Method(mut method) => {
        let protocol_method = parse_method(&mut method)?;

        if protocol_method.is_instance_method {
          expected_instance_properties.remove(&protocol_method.selector.sel);
        } else {
          expected_class_properties.remove(&protocol_method.selector.sel);
        }

        let id_method = ImplItemMethod {
          attrs: Vec::new(),
          vis: Visibility::Inherited,
          defaultness: None,
          sig: method.sig.clone(),
          block: Block {
            brace_token: syn::token::Brace(Span::call_site()),
            stmts: Vec::new(),
          },
        };
        id_items.extend(parse_selector(
          &protocol_method.selector,
          id_method,
          !item.generics.params.is_empty(),
          true,
          Some(method.sig.ident.span()),
        )?);

        sub_items.push(TraitItem::Method(method));
        sub_items.push(TraitItem::Method(protocol_method.msg_send));
        sub_items.push(TraitItem::Method(protocol_method.msg_recv));
      }
      _ => sub_items.push(sub_item),
    }
  }
  item.items = sub_items;

  for selector in expected_instance_properties.iter().chain(expected_class_properties.iter()) {
    return Err(
      selector
        .span()
        .unstable()
        .error(format!("selector \"{}\" is missing from the trait's method list", selector.value()))
        .note(format!(
          "objrs requires a method in the trait to have an #[objrs(selector = \"{}\")] attribute",
          selector.value()
        )),
    );
  }

  // TODO: are these needed?
  // 	.private_extern	l_OBJC_LABEL_PROTOCOL_$_Foo ## @"\01l_OBJC_LABEL_PROTOCOL_$_Foo"
  // 	.section	__DATA,__objc_protolist,coalesced,no_dead_strip
  // 	.globl	l_OBJC_LABEL_PROTOCOL_$_Foo
  // 	.weak_definition	l_OBJC_LABEL_PROTOCOL_$_Foo
  // 	.p2align	3
  // l_OBJC_LABEL_PROTOCOL_$_Foo:
  // 	.quad	l_OBJC_PROTOCOL_$_Foo

  // 	.private_extern	l_OBJC_PROTOCOL_REFERENCE_$_Foo ## @"\01l_OBJC_PROTOCOL_REFERENCE_$_Foo"
  // 	.section	__DATA,__objc_protorefs,coalesced,no_dead_strip
  // 	.globl	l_OBJC_PROTOCOL_REFERENCE_$_Foo
  // 	.weak_definition	l_OBJC_PROTOCOL_REFERENCE_$_Foo
  // 	.p2align	3
  // l_OBJC_PROTOCOL_REFERENCE_$_Foo:
  // 	.quad	l_OBJC_PROTOCOL_$_Foo

  let vis = &item.vis;
  let id_ident = &attr.id_ident;
  let ident = &item.ident;
  let generics = &item.generics;
  let where_clause = &generics.where_clause;
  let mut generic_idents: Punctuated<&ToTokens, syn::token::Comma> = Punctuated::new();
  for param in item.generics.params.iter() {
    match param {
      GenericParam::Type(ref generic_type) => {
        generic_idents.push(&generic_type.ident);
      }
      GenericParam::Lifetime(ref generic_lifetime) => {
        generic_idents.push(&generic_lifetime.lifetime);
      }
      GenericParam::Const(ref generic_const) => {
        generic_idents.push(&generic_const.ident);
      }
    }
  }

  let tokens = quote!{
    #item

    // TODO: this is broken by https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md
    // TODO: use __objrs_root instead of objrs.
    // unsafe impl #generics objrs::marker::Protocol for #ident <#generic_idents> #where_clause {}

    // TODO: use __objrs_root instead of objrs.
    #[repr(transparent)]
    #vis struct #id_ident<#generics>(objrs::__objrs::Opaque) #where_clause;
    unsafe impl #generics objrs::marker::Class for #id_ident<#generic_idents> #where_clause {}

    // TODO: use __objrs_root instead of objrs.
    impl #generics #ident<#generic_idents> for #id_ident<#generic_idents> #where_clause {
      #id_items
    }
  };
  return Ok(tokens);
}
