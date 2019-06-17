// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

extern crate core;

// For Objective-C property attributes, see https://github.com/llvm-mirror/clang/blob/master/lib/Parse/ParseObjc.cpp

use crate::gen::gen_selector::{
  gen_instance_msg_send, gen_msg_recv_sig, gen_msg_send_sig, gen_trampoline,
};
use crate::parse::protocol_attr::Protocol;
use crate::parse::selector_attr::Method;
use crate::util::priv_ident;
use proc_macro::Diagnostic;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, GenericParam, Ident, TraitItem, TraitItemMethod};

fn gen_send_recv(method: &Method, objrs_root: &Ident) -> (TraitItemMethod, TraitItemMethod) {
  let sig = gen_msg_send_sig(method.sig(), method.is_instance_method, objrs_root);
  let panic_message = format!("private objrs method `{}` called", sig.ident);
  let msg_send = TraitItemMethod {
    attrs: vec![parse_quote!(#[doc(hidden)]), parse_quote!(#[allow(unused_variables)])],
    sig: sig,
    default: Some(parse_quote! {{
      #objrs_root::__objrs::core::panic!(#panic_message);
    }}),
    semi_token: None,
  };

  let sig = gen_msg_recv_sig(method.sig(), method.is_instance_method, objrs_root);
  let panic_message = format!("private objrs method `{}` called", sig.ident);
  let msg_recv = TraitItemMethod {
    attrs: msg_send.attrs.clone(),
    sig: sig,
    default: Some(parse_quote! {{
      #objrs_root::__objrs::core::panic!(#panic_message);
    }}),
    semi_token: None,
  };

  return (msg_send, msg_recv);
}

fn transform_method(method: Method, objrs_root: &Ident) -> TraitItemMethod {
  if method.attr.optional.is_none() {
    return method.into_trait_method();
  }

  let sel = method.attr.sel.value();
  let mut method = method.into_trait_method();
  method.attrs.push(parse_quote!(#[allow(unused_variables)]));
  method.semi_token = None;

  let panic_message =
    format!("unimplemented optional method `{}` (selector: \"{}\") called", method.sig.ident, sel,);
  method.default = Some(parse_quote! {{
    #objrs_root::__objrs::core::panic!(#panic_message);
  }});

  return method;
}

fn transgen_and_extend(method: Method, objrs_root: &Ident, items: &mut Vec<TraitItem>) {
  let (send, recv) = gen_send_recv(&method, objrs_root);
  let trampoline = transform_method(method, objrs_root);
  items.push(TraitItem::Method(trampoline));
  items.push(TraitItem::Method(send));
  items.push(TraitItem::Method(recv));
}

pub fn transform_protocol(protocol: Protocol) -> Result<TokenStream, Diagnostic> {
  let objrs_root = &protocol.objrs;
  let mut id_items = TokenStream::new();
  let mut item = protocol.item;

  for method in protocol.class_methods {
    let sig = method.sig();
    let panic_message = format!(
      "class method `{}` (selector: \"{}\") called on `{}::Id`, which is not a real class type \
       and does not implement any class methods",
      sig.ident,
      method.attr.sel.value(),
      objrs_root
    );
    id_items.extend(quote! {
      #[doc(hidden)]
      #[allow(unused_variables)]
      #sig {
        #objrs_root::__objrs::core::panic!(#panic_message);
      }
    });

    transgen_and_extend(method, objrs_root, &mut item.items);
  }

  for method in protocol.instance_methods {
    gen_trampoline(&method, objrs_root)?.to_tokens(&mut id_items);
    gen_instance_msg_send(&method, true, objrs_root)?.to_tokens(&mut id_items);

    transgen_and_extend(method, objrs_root, &mut item.items);
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

  let ident = &item.ident;
  let (_, ty_generics, where_clause) = item.generics.split_for_impl();

  let any = priv_ident("T");
  let mut generics = item.generics.clone();
  generics.params.push(GenericParam::Type(
    parse_quote!(#any: #ident #ty_generics + ?#objrs_root::__objrs::core::marker::Sized),
  ));
  let (impl_generics, _, _) = generics.split_for_impl();

  let tokens = quote! {
    extern crate #objrs_root;

    #item

    // TODO: this is broken by https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md
    // unsafe impl #impl_generics #objrs_root::marker::Protocol for #ident #ty_generics #where_clause {}

    // #[repr(transparent)]
    // #vis struct #id_ident #impl_generics(#objrs_root::__objrs::Opaque) #where_clause;
    // unsafe impl #impl_generics #objrs_root::marker::Class for #id_ident #ty_generics #where_clause {}

    // This impl is generic. If it is ever made non-generic, update the call to
    // gen_instance_msg_send.
    impl #impl_generics #ident #ty_generics for #objrs_root::Id<#any> #where_clause {
      #id_items
    }
  };
  return Ok(tokens);
}
