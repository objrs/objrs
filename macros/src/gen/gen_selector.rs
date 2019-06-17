// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

extern crate core;
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use crate::gen::class_ref::{
  gen_class_ref_value, gen_super_class_ref_value, gen_super_meta_ref_value,
};
use crate::gen::ivar::transform_ivars;
use crate::gen::sel_ref::gen_sel_ref;
use crate::parse::sel_ref_attr::SelRef;
use crate::parse::selector_attr::Method;
use crate::util::{priv_ident, priv_ident_at, DrainExt};
use proc_macro::Diagnostic;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
  parse_quote, punctuated::Punctuated, spanned::Spanned, token::Comma, token::Extern, Abi,
  AttrStyle, Attribute, Block, Expr, ExprVerbatim, FnArg, FnDecl, GenericParam, Ident,
  ImplItemMethod, Item, ItemVerbatim, LitStr, MethodSig, Pat, PatIdent, ReturnType, Stmt, Type,
  Visibility,
};

/// Filters out any unwanted attributes from the vector.
fn filter_out_attrs(attrs: &mut Vec<Attribute>, unwanted: &[&str]) {
  attrs.retain(|attr| {
    if attr.style != AttrStyle::Outer {
      return true;
    }

    return !unwanted.iter().any(|unwanted_ident| attr.path.is_ident(unwanted_ident));
  });
}

/// Takes an ABI and transforms it to be `Some(extern "C")`. Preserves span info if possible.
fn make_extern_c_abi(abi: &mut Option<Abi>) {
  if let Some(ref mut abi) = abi {
    if let Some(ref mut name) = abi.name {
      if name.value() == "C" {
        return;
      }
    }
    abi.name = Some(LitStr::new("C", Span::call_site()));
    return;
  }

  *abi = Some(Abi {
    extern_token: Extern(Span::call_site()),
    name: Some(LitStr::new("C", Span::call_site())),
  });
}

pub fn gen_msg_recv_sig(
  sig: &MethodSig,
  is_instance_method: bool,
  objrs_root: &Ident,
) -> MethodSig {
  // TODO: let objc_method_family to be explicitly specified?
  // TODO: take the method family into account.

  let mut sig = sig.clone();
  sig.ident = priv_ident_at(
    &["__objrs_msg_recv_", sig.ident.to_string().as_ref()].concat(),
    sig.ident.span(),
  );
  make_extern_c_abi(&mut sig.abi);

  if !is_instance_method {
    sig
      .decl
      .inputs
      .insert(0, parse_quote!(_: #objrs_root::__objrs::core::ptr::NonNull<#objrs_root::Class>));
  }
  // TODO: consider allowing the selector to be accessed like in Objective-C (via _cmd).
  sig.decl.inputs.insert(1, parse_quote!(_: &'static #objrs_root::Sel));

  return sig;
}

pub fn gen_msg_recv(
  method: &Method,
  class_name: &str,
  category: Option<&str>,
  objrs_root: &Ident,
) -> Result<ImplItemMethod, Diagnostic> {
  assert!(!method.attr.call_super);
  assert!(!method.attr.no_impl);

  assert!(method.defaultness().is_none());
  assert!(method.sig().constness.is_none());
  assert!(method.sig().asyncness.is_none());

  let is_instance_method = method.is_instance_method;
  let selector = &method.attr.sel;

  let mut method = ImplItemMethod {
    attrs: method.attrs().clone(),
    vis: Visibility::Inherited,
    defaultness: None,
    sig: gen_msg_recv_sig(method.sig(), is_instance_method, objrs_root),
    block: method.block().cloned().unwrap_or_else(|| parse_quote!({})),
  };

  filter_out_attrs(&mut method.attrs, &["doc", "export_name"]);
  method.attrs.push(parse_quote!(#[doc(hidden)]));

  // Set the span of the export name to the selector string so it's clear where the problem is if
  // there are duplicate symbol conflicts.
  let export_name = &[
    if is_instance_method { "\x01-[" } else { "\x01+[" },
    class_name,
    if category.is_some() { "(" } else { "" },
    category.unwrap_or(""),
    if category.is_some() { ") " } else { " " },
    selector.value().as_ref(),
    "]",
  ]
  .concat();
  let export_name = LitStr::new(export_name, selector.span());
  method.attrs.push(parse_quote!(#[export_name = #export_name]));

  if is_instance_method {
    transform_ivars(&mut method, objrs_root)?;
  }

  return Ok(method);
}

fn sel_ref_value(sel: LitStr, objrs_root: &Ident) -> Result<TokenStream, Diagnostic> {
  let sel_ref_ident = priv_ident("SEL_REF");
  let sel_string = sel.value();
  let sel_ref = gen_sel_ref(SelRef {
    sel: sel,
    sel_string: sel_string,
    ident: sel_ref_ident.clone(),
    u8_ty: parse_quote!(#objrs_root::__objrs::u8),
  })?;

  return Ok(quote! {{
    #sel_ref
    let sel_ref_ptr = & #sel_ref_ident as *const _;
    let sel_ref = unsafe { #objrs_root::__objrs::core::ptr::read_volatile(sel_ref_ptr) };
    unsafe { #objrs_root::__objrs::core::mem::transmute::<_, &'static #objrs_root::Sel>(sel_ref) }
  }});
}

fn self_ty_and_value(
  decl: &FnDecl,
  class_name: &str,
  call_super: bool,
  is_instance_method: bool,
  objrs_root: &Ident,
) -> Result<(TokenStream, TokenStream), Diagnostic> {
  if call_super {
    let self_arg_type = quote!(*mut #objrs_root::__objrs::runtime::objc_super);
    let super_class;
    let receiver;
    if is_instance_method {
      receiver = quote!(self);
      super_class = gen_super_class_ref_value(class_name, objrs_root)?;
    } else {
      receiver = gen_class_ref_value(class_name, objrs_root)?;
      super_class = gen_super_meta_ref_value(class_name, objrs_root)?;
    }
    let self_arg_value = quote! {{
      &mut #objrs_root::__objrs::runtime::objc_super {
        receiver: #objrs_root::__objrs::core::mem::transmute(#receiver),
        super_class: #super_class,
      } as *mut _
    }};
    return Ok((self_arg_type, self_arg_value));
  }

  if !is_instance_method {
    let self_arg_value = gen_class_ref_value(class_name, objrs_root)?;
    return Ok((quote!(&'static #objrs_root::Class), self_arg_value));
  }

  match decl.inputs[0] {
    FnArg::SelfRef(ref self_ref) => {
      let mutability = &self_ref.mutability;
      return Ok((quote!(&#mutability Self), quote!(self)));
    }
    FnArg::SelfValue(_) => {
      return Ok((quote!(Self), quote!(self)));
    }
    FnArg::Captured(ref arg) => {
      let ty = &arg.ty;
      let pat = &arg.pat;
      return Ok((quote!(#ty), quote!(#pat)));
    }
    _ => panic!("BUG: unhandled self type"),
  }
}

pub fn gen_msg_send_sig(
  sig: &MethodSig,
  is_instance_method: bool,
  objrs_root: &Ident,
) -> MethodSig {
  // TODO: let objc_method_family to be explicitly specified?
  // TODO: take the method family into account.

  let mut sig = sig.clone();
  sig.ident = priv_ident_at(
    &["__objrs_msg_send_", sig.ident.to_string().as_ref()].concat(),
    sig.ident.span(),
  );
  make_extern_c_abi(&mut sig.abi);

  if !is_instance_method {
    // Inject a parameter that is compatible with *mut objrs::Class.
    sig.decl.inputs.insert(0, parse_quote!(_: #objrs_root::__objrs::UninitPtr));
  }
  // Inject a parameter that is compatible with *mut objrs::Sel.
  sig.decl.inputs.insert(1, parse_quote!(_: #objrs_root::__objrs::UninitPtr));

  return sig;
}

pub fn gen_msg_send(
  method: &Method,
  class_name: &str,
  is_generic_class: bool,
  objrs_root: &Ident,
) -> Result<ImplItemMethod, Diagnostic> {
  let sel_ref = sel_ref_value(method.attr.sel.clone(), objrs_root)?;
  let call_super = method.attr.call_super;
  let is_instance_method = method.is_instance_method;
  let is_generic = is_generic_class || method.is_generic;
  let mut method = ImplItemMethod {
    attrs: method.attrs().clone(),
    vis: method.vis().cloned().unwrap_or(Visibility::Inherited),
    defaultness: method.defaultness().cloned(),
    sig: gen_msg_send_sig(method.sig(), is_instance_method, objrs_root),
    block: Block {
      brace_token: method.block().map(|block| block.brace_token.clone()).unwrap_or_default(),
      stmts: Vec::new(),
    },
  };

  fn is_inline_attr(attr: &mut Attribute) -> bool {
    return attr.style == AttrStyle::Outer && attr.path.is_ident("inline");
  }
  let inline = DrainExt::drain(&mut method.attrs, is_inline_attr).last().map(|attr| attr.tts);
  let mut inline =
    inline.unwrap_or_else(|| if call_super { quote!((always)) } else { quote!((never)) });
  let ref_hack_inline;
  if is_generic {
    // This is needed because #[inline(never)] doesn't really do anything for generic functions
    // (inluding non-generic methods for generic types). Also, incremental compilation can wreck
    // havoc with this (it seems to compile things into lots of separate object files, which breaks
    // references to L_* locals). See:
    // https://github.com/rust-lang/rust/issues/53929#ref-pullrequest-362799104
    ref_hack_inline = inline;
    inline = quote!((always));
  } else {
    ref_hack_inline = quote!((always));
  }
  method.attrs.push(parse_quote!(#[inline #inline]));

  let objc_send;
  let objc_send_stret;
  if call_super {
    objc_send = priv_ident("objc_msgSendSuper2");
    objc_send_stret = priv_ident("objc_msgSendSuper2_stret");
  } else {
    objc_send = priv_ident("objc_msgSend");
    objc_send_stret = priv_ident("objc_msgSend_stret");
  }

  let output = &method.sig.decl.output;
  let msg_send;
  match output {
    ReturnType::Default => {
      msg_send = quote!(#objrs_root::__objrs::runtime::#objc_send);
    }
    ReturnType::Type(_, return_type) => {
      msg_send = quote! {{
        #[cfg(target_arch = "x86_64")]
        let msg_send = if #objrs_root::__objrs::core::mem::size_of::<#return_type>() <= 16 {
          #objrs_root::__objrs::runtime::#objc_send
        } else {
          #objrs_root::__objrs::runtime::#objc_send_stret
        };
        #[cfg(target_arch = "aarch64")]
        let msg_send = #objrs_root::__objrs::runtime::#objc_send;
        msg_send
      }};
    }
  }

  let (self_arg_type, self_arg_value) =
    self_ty_and_value(&method.sig.decl, class_name, call_super, is_instance_method, objrs_root)?;

  let mut tail_arg_types: Punctuated<&Type, Comma> = Punctuated::new();
  let mut tail_arg_values: Punctuated<&Ident, Comma> = Punctuated::new();
  for arg in method.sig.decl.inputs.iter().skip(2) {
    let captured = if let FnArg::Captured(ref captured) = arg { Some(captured) } else { None };
    let captured = captured.unwrap();
    tail_arg_types.push(&captured.ty);

    let ident = if let Pat::Ident(ref ident) = captured.pat { Some(ident) } else { None };
    let ident = ident.unwrap();
    tail_arg_values.push(&ident.ident);
  }

  let ref_hack;
  if is_instance_method {
    ref_hack = quote! {(#self_arg_value, {
      #[inline #ref_hack_inline]
      fn ref_hack() -> &'static #objrs_root::Sel {
        return #sel_ref;
      }
      ref_hack()
    })};
  } else {
    ref_hack = quote! {{
      #[inline #ref_hack_inline]
      fn ref_hack() -> (&'static #objrs_root::Class, &'static #objrs_root::Sel) {
        return (#self_arg_value, #sel_ref);
      }
      ref_hack()
    }};
  }

  let fn_body = quote! {
    let (this, sel) = #ref_hack;

    let msg_send: unsafe extern "C" fn(#self_arg_type,
                                       *const #objrs_root::Sel,
                                       #tail_arg_types) #output;
    msg_send = unsafe { #objrs_root::__objrs::core::mem::transmute(#msg_send as *const ()) };

    return unsafe { msg_send(this, sel as *const _, #tail_arg_values) };
  };
  method.block.stmts.push(Stmt::Item(Item::Verbatim(ItemVerbatim {
    tts: fn_body,
  })));

  return Ok(method);
}

pub fn gen_instance_msg_send(
  method: &Method,
  is_generic_class: bool,
  objrs_root: &Ident,
) -> Result<ImplItemMethod, Diagnostic> {
  assert!(method.is_instance_method);
  return gen_msg_send(method, "", is_generic_class, objrs_root);
}

pub fn gen_trampoline(method: &Method, objrs_root: &Ident) -> Result<ImplItemMethod, Diagnostic> {
  let is_instance_method = method.is_instance_method;
  let mut method = ImplItemMethod {
    attrs: method.attrs().clone(),
    vis: method.vis().cloned().unwrap_or(Visibility::Inherited),
    defaultness: method.defaultness().cloned(),
    sig: method.sig().clone(),
    block: Block {
      brace_token: method.block().map(|block| block.brace_token.clone()).unwrap_or_default(),
      stmts: Vec::new(),
    },
  };

  let mut self_arg_value;
  if is_instance_method {
    self_arg_value = quote!(self);
  } else {
    self_arg_value = quote!(#objrs_root::__objrs::UNINIT_PTR);
  }

  for (n, arg) in method.sig.decl.inputs.iter_mut().enumerate() {
    match arg {
      FnArg::SelfRef(_) => continue,
      FnArg::SelfValue(_) => continue,
      FnArg::Captured(ref mut captured) => {
        match captured.pat {
          Pat::Ident(ref mut ident) => {
            if is_instance_method && n == 0 && ident.ident == "self" {
              continue;
            }

            if let Some(ref by_ref) = ident.by_ref {
              return Err(
                by_ref
                  .span()
                  .unstable()
                  .error("method parameter cannot be a `ref`")
                  .note("the current version of objrs disallows `ref` parameters"),
              );
            }
            if let Some((ref at, ref pat)) = ident.subpat {
              return Err(
                at.span()
                  .unstable()
                  .join(pat.span().unstable())
                  .expect("BUG: unexpected spans from different files")
                  .error("method parameter cannot use pattern matching")
                  .note("the current version of objrs disallows pattern matching in parameters"),
              );
            }
            ident.ident = Ident::new(&format!("_arg{}", n), ident.ident.span());
          }
          Pat::Wild(ref pat_wild) => {
            let ident = Ident::new(&format!("_arg{}", n), pat_wild.span());
            captured.pat = Pat::Ident(PatIdent {
              by_ref: None,
              mutability: None,
              ident: ident,
              subpat: None,
            });
          }
          _ => {
            return Err(
              captured
                .pat
                .span()
                .unstable()
                .error("expected argument, found pattern")
                .note("objrs does not support this pattern as a function argument"),
            );
          }
        }

        if is_instance_method && n == 0 {
          self_arg_value = quote!(_arg0);
        }
      }
      FnArg::Inferred(_) => panic!("BUG: unexpected inferred function argument"),
      FnArg::Ignored(_) => panic!("BUG: unexpected ignored function argument"),
    }
  }

  let mut tail_arg_values: Punctuated<&Ident, Comma> = Punctuated::new();
  for arg in method.sig.decl.inputs.iter().skip(is_instance_method as usize) {
    if let FnArg::Captured(ref captured) = arg {
      if let Pat::Ident(ref ident) = captured.pat {
        tail_arg_values.push(&ident.ident);
      }
    }
  }

  let mut generics: Punctuated<&dyn ToTokens, Comma> = Punctuated::new();
  for generic in method.sig.decl.generics.params.iter() {
    match generic {
      GenericParam::Type(ref param) => generics.push(&param.ident),
      GenericParam::Const(ref param) => generics.push(&param.ident),
      GenericParam::Lifetime(_) => continue,
    }
  }

  let msg_send_name = priv_ident_at(
    &["__objrs_msg_send_", method.sig.ident.to_string().as_ref()].concat(),
    method.sig.ident.span(),
  );
  method.block.stmts.push(Stmt::Expr(Expr::Verbatim(ExprVerbatim {
    tts: quote!{
      return Self::#msg_send_name::<#generics>(#self_arg_value, #objrs_root::__objrs::UNINIT_PTR, #tail_arg_values);
    },
  })));

  return Ok(method);
}

// fn msg_send_fn(
//   selector: &LitStr,
//   call_super: bool,
//   method: &ImplItemMethod,
//   name: &Ident,
//   inline: &ToTokens,
//   is_instance_method: bool,
//   is_generic_class: bool,
//   objrs_root: &Ident,
// ) -> Result<TokenStream, Diagnostic> {
//   let mut selector_string =
//     validate_selector_for_method(selector, &method.sig, is_instance_method)?;

//   let objc_send;
//   let objc_send_stret;
//   if call_super {
//     objc_send = quote!(objc_msgSendSuper2);
//     objc_send_stret = quote!(objc_msgSendSuper2_stret);
//   } else {
//     objc_send = quote!(objc_msgSend);
//     objc_send_stret = quote!(objc_msgSend_stret);
//   }

//   let random_id = &RandomIdentifier::new();

//   let meth_name_export_name =
//     ["\x01L_OBJC_METH_VAR_NAME_.__objrs_meth.", random_id, ".", &selector_string].concat();

//   let sel_ref_export_name =
//     ["\x01L_OBJC_SELECTOR_REFERENCES_.__objrs_sel.", random_id, ".", &selector_string].concat();

//   selector_string.push('\x00');
//   let selector = LitByteStr::new(selector_string.as_bytes(), selector.span());
//   let selector_len = selector_string.len();

//   let empty_tuple = parse_quote!(());
//   let return_type = match method.sig.decl.output {
//     ReturnType::Default => &empty_tuple,
//     ReturnType::Type(_, ref ty) => ty.as_ref(),
//   };

//   let generics = &method.sig.decl.generics;

//   let native_ty = quote!(#objrs_root::__objrs);
//   let ref_hack;
//   if !is_generic_class && generics.params.is_empty() {
//     ref_hack = quote! {
//       let sel = unsafe { #objrs_root::__objrs::core::ptr::read_volatile(&SEL_REF as *const _) } as *const _;
//     };
//   } else {
//     // TODO: Make this optional! It's needed because #[inline(never)] doesn't really do anything
//     // for generic functions (inluding non-generic methods for generic types). Also, incremental
//     // compilation can wreck havoc with this (it seems to compile things into lots of separate
//     // object files, which breaks references to L_* locals). See:
//     // https://github.com/rust-lang/rust/issues/53929#ref-pullrequest-362799104
//     ref_hack = quote! {
//       #[inline(never)]
//       fn sel_ref_hack() -> *const [#native_ty::u8; #selector_len] {
//         return unsafe { #objrs_root::__objrs::core::ptr::read_volatile(&SEL_REF as *const _) } as *const _;
//       }
//       let sel = sel_ref_hack();
//     };
//   }

//   let where_clause = &generics.where_clause;
//   let mut inputs = method.sig.decl.inputs.clone();
//   let self_arg_type;
//   let self_arg_value;
//   if !is_instance_method {
//     inputs.insert(0, parse_quote!(_: #native_ty::usize));
//     if call_super {
//       self_arg_type = quote!(*mut #objrs_root::__objrs::runtime::objc_super);
//       self_arg_value = quote! {&mut #objrs_root::__objrs::runtime::objc_super {
//         receiver: #objrs_root::__objrs::core::mem::transmute(Self::__objrs_class_ref()),
//         super_class: Self::__objrs_super_meta_ref(),
//       } as *mut _};
//     } else {
//       self_arg_type = quote!(#objrs_root::__objrs::runtime::Class);
//       self_arg_value = quote!(Self::__objrs_class_ref());
//     }
//   } else {
//     match method.sig.decl.inputs[0] {
//       FnArg::SelfRef(ref self_ref) => {
//         if call_super {
//           self_arg_type = quote!(*mut #objrs_root::__objrs::runtime::objc_super);
//           self_arg_value = quote! {&mut #objrs_root::__objrs::runtime::objc_super {
//             receiver: #objrs_root::__objrs::core::mem::transmute(self),
//             super_class: Self::__objrs_super_class_ref(),
//           } as *mut _};
//         } else if self_ref.mutability.is_some() {
//           self_arg_type = quote!(&mut Self);
//           self_arg_value = quote!(self);
//         } else {
//           self_arg_type = quote!(&Self);
//           self_arg_value = quote!(self);
//         }
//       }
//       FnArg::SelfValue(_) => {
//         if call_super {
//           self_arg_type = quote!(*mut #objrs_root::__objrs::runtime::objc_super);
//           self_arg_value = quote! {&mut #objrs_root::__objrs::runtime::objc_super {
//             receiver: #objrs_root::__objrs::core::mem::transmute(self),
//             super_class: Self::__objrs_super_class_ref(),
//           } as *mut _};
//         } else {
//           self_arg_type = quote!(Self);
//           self_arg_value = quote!(self);
//         }
//       }
//       FnArg::Captured(ref arg) => {
//         let pat = &arg.pat;
//         let ty = &arg.ty;
//         if call_super {
//           self_arg_type = quote!(*mut #objrs_root::__objrs::runtime::objc_super);
//           self_arg_value = quote! {&mut #objrs_root::__objrs::runtime::objc_super {
//             receiver: #objrs_root::__objrs::core::mem::transmute(#pat),
//             super_class: Self::__objrs_super_class_ref(),
//           } as *mut _};
//         } else {
//           self_arg_type = quote!(#ty);
//           self_arg_value = quote!(#pat);
//         }
//       }
//       _ => panic!("BUG: unhandled self type"),
//     }
//   }
//   inputs.insert(1, parse_quote!(_: #native_ty::usize));
//   // TODO: handle variadic.
//   let output = &method.sig.decl.output;

//   let mut tail_arg_types: Punctuated<&Type, Comma> = Punctuated::new();
//   let mut tail_arg_values: Punctuated<&Ident, Comma> = Punctuated::new();
//   for arg in inputs.iter().skip(2) {
//     if let FnArg::Captured(ref captured) = arg {
//       tail_arg_types.push(&captured.ty);
//       if let Pat::Ident(ref ident) = captured.pat {
//         tail_arg_values.push(&ident.ident);
//       } else {
//         panic!("BUG: unexpected uncaptured function argument");
//       }
//     } else {
//       panic!("BUG: unexpected uncaptured function argument");
//     }
//   }

//   let unsafety = &method.sig.unsafety;

//   let msg_send = quote! {
//     // TODO: can this be inline(always) for LTO builds?
//     #[allow(dead_code)]
//     #[inline #inline]
//     #[doc(hidden)]
//     #[allow(non_upper_case_globals)]
//     #unsafety extern "C" fn #name #generics(#inputs) #output #where_clause {
//       #[link_section = "__TEXT,__objc_methname,cstring_literals"]
//       #[export_name = #meth_name_export_name]
//       static METH_NAME: [#native_ty::u8; #selector_len] = * #selector;

//       #[link_section = "__DATA,__objc_selrefs,literal_pointers,no_dead_strip"]
//       #[export_name = #sel_ref_export_name]
//       static SEL_REF: &'static [#native_ty::u8; #selector_len] = &METH_NAME;

//       #[cfg(target_arch = "x86_64")]
//       let msg_send = if #objrs_root::__objrs::core::mem::size_of::<#return_type>() <= 16 {
//         #objrs_root::__objrs::runtime::#objc_send
//       } else {
//         #objrs_root::__objrs::runtime::#objc_send_stret
//       };
//       #[cfg(target_arch = "aarch64")]
//       let msg_send = #objrs_root::__objrs::runtime::objc_msgSend;

//       // Use a pointer (*const [#native_ty::u8; N]) for the selector type rather than a reference (&[#native_ty::u8; N]). If we used a reference, users would have to explicitly annotate most functions with lifetime parameters.
//       let msg_send: unsafe extern fn(#self_arg_type, *const [#native_ty::u8; #selector_len], #tail_arg_types) #output = unsafe { #objrs_root::__objrs::core::mem::transmute(msg_send as *const ()) };

//       #ref_hack

//       return unsafe { msg_send(#self_arg_value, sel, #tail_arg_values) };

//       // compile_error!("impl blocks must use the #[objrs(impl)] attribute");
//     }
//   };

//   // let test = quote!{
//   //   #[cfg(test)]
//   //   #[test]
//   //   fn #name() {
//   //     let class = #objrs_root::__objrs::runtime::objc_getClass(#class_name);
//   //     assert_ne!(class, #objrs_root::__objrs::core::ptr::null());

//   //     let sel = #objrs_root::__objrs::runtime::sel_registerName(#sel);

//   //     assert!(#objrs_root::__objrs::runtime::class_respondsToSelector(class, sel));

//   //     let method;
//   //     if is_instance_method {
//   //       method = #objrs_root::__objrs::runtime::class_getInstanceMethod(class, sel);
//   //     } else {
//   //       method = #objrs_root::__objrs::runtime::class_getClassMethod(class, sel);
//   //     }
//   //     assert_ne!(method, #objrs_root::__objrs::core::ptr::null_mut());

//   //     let argument_count = #objrs_root::__objrs::runtime::method_getNumberOfArguments(method);
//   //     assert_eq!(argument_count, #inputs.len());

//   //     let type_encoding = #objrs_root::__objrs::runtime::method_getTypeEncoding(method);
//   //     assert_ne!(type_encoding, #objrs_root::__objrs::core::ptr::null());

//   //     // TODO: iterate over the type encoding, making sure the parameters all match up.
//   //   }
//   // };

//   return Ok(msg_send);
// }

// fn gen_msg_send() -> Result<ImplItem, Diagnostic> {
//   let msg_send = quote! {
//     // TODO: can this be inline(always) for LTO builds?
//     #[allow(dead_code)]
//     #[inline #inline]
//     #[doc(hidden)]
//     #[allow(non_upper_case_globals)]
//     #unsafety extern "C" fn #name #generics(#inputs) #output #where_clause {
//       #[link_section = "__TEXT,__objc_methname,cstring_literals"]
//       #[export_name = #meth_name_export_name]
//       static METH_NAME: [#native_ty::u8; #selector_len] = * #selector;

//       #[link_section = "__DATA,__objc_selrefs,literal_pointers,no_dead_strip"]
//       #[export_name = #sel_ref_export_name]
//       static SEL_REF: &'static [#native_ty::u8; #selector_len] = &METH_NAME;

//       #[cfg(target_arch = "x86_64")]
//       let msg_send = if #objrs_root::__objrs::core::mem::size_of::<#return_type>() <= 16 {
//         #objrs_root::__objrs::runtime::#objc_send
//       } else {
//         #objrs_root::__objrs::runtime::#objc_send_stret
//       };
//       #[cfg(target_arch = "aarch64")]
//       let msg_send = #objrs_root::__objrs::runtime::objc_msgSend;

//       // Use a pointer (*const [#native_ty::u8; N]) for the selector type rather than a reference (&[#native_ty::u8; N]). If we used a reference, users would have to explicitly annotate most functions with lifetime parameters.
//       let msg_send: unsafe extern fn(#self_arg_type, *const [#native_ty::u8; #selector_len], #tail_arg_types) #output = unsafe { #objrs_root::__objrs::core::mem::transmute(msg_send as *const ()) };

//       #ref_hack

//       return unsafe { msg_send(#self_arg_value, sel, #tail_arg_values) };

//       // compile_error!("impl blocks must use the #[objrs(impl)] attribute");
//     }
//   };

//   return Ok(ImplItem::Verbatim(ImplItemVerbatim { tts: msg_send }));
// }

// pub fn transform_selector(
//   attr: &SelectorAttr,
//   mut method: ImplItemMethod,
//   is_generic_class: bool,
//   empty_msg_recv: bool,
//   fn_span: Option<Span>,
//   objrs_root: &Ident,
// ) -> Result<TokenStream, Diagnostic> {
//   let mut inline;
//   if attr.call_super {
//     inline = quote!((always));
//   } else {
//     inline = quote!((never));
//   }
//   for attr in method.attrs.iter_mut() {
//     if attr.path.leading_colon.is_none() && attr.path.segments.len() == 1 {
//       let segment = attr.path.segments.iter().next().expect("BUG: expected exactly 1 segment");
//       if segment.ident == "inline" && segment.arguments.is_empty() {
//         inline = core::mem::replace(&mut attr.tts, quote!((always)));
//         break;
//       }
//     }
//   }

//   let is_instance_method = attr.method_type == MethodType::Instance
//     || (attr.method_type == MethodType::Auto && is_instance_method(&method.sig.decl.inputs));

//   let mut self_arg_value;
//   if is_instance_method {
//     self_arg_value = quote!(self);
//   } else {
//     self_arg_value = quote!(unsafe { #objrs_root::__objrs::core::mem::uninitialized() });
//   }

//   for (n, arg) in method.sig.decl.inputs.iter_mut().enumerate() {
//     match arg {
//       FnArg::SelfRef(_) => continue,
//       FnArg::SelfValue(_) => continue,
//       FnArg::Captured(ref mut captured) => {
//         let span;
//         match captured.pat {
//           Pat::Ident(ref mut ident) => {
//             if n == 0 && ident.ident == "self" {
//               continue;
//             }

//             if let Some(ref by_ref) = ident.by_ref {
//               // TODO: should we let parameters be a `ref`? It could, in theory, be nice to take a parameter by value at the Rust level (moving it), and pass it to the Objective-C level as a pointer. For example:
//               //
//               // struct MyStruct {...}  // Non-copyable struct.
//               // #[objrs(selector = "foo:")]
//               // fn foo(ref s: MyStruct) {}
//               //
//               // This would expand into (simplified):
//               //
//               // #[inline(always)]
//               // fn foo(ref s: MyStruct) {
//               //   #[inline(never)]
//               //   fn msg_send(_: usize, _: usize, s: &MyStruct) {
//               //     // ...
//               //   }
//               //   msg_send(_, _, s);
//               // }
//               //
//               // This is nice if you want to move a object that Objective-C is trying to take by address. If supporting this is desirable, then `ref mut` should also be allowed (so Objective-C gets a `&mut` parameter).
//               //
//               // It's a bit of a foot-gun though. It introduces a subtle difference between Rust and Objective-C, and it's not immediately obvious how it works.
//               return Err(
//                 by_ref
//                   .span()
//                   .unstable()
//                   .error("method parameters cannot be a `ref`")
//                   .note("the current version of objrs disallows them"),
//               );
//             }
//             if let Some(ref mutability) = ident.mutability {
//               mutability
//                 .span()
//                 .unstable()
//                 .warning("variable does not need to be mutable")
//                 .note("objrs will remove this `mut` keyword")
//                 .emit();
//             }
//             if let Some((ref at, ref pat)) = ident.subpat {
//               at.span()
//                 .unstable()
//                 .join(pat.span().unstable())
//                 .expect("BUG: unexpected spans from different files")
//                 .warning("pattern will not be used")
//                 .note("objrs will remove this pattern")
//                 .emit();
//             }
//             span = ident.ident.span();
//           }
//           Pat::Wild(ref pat_wild) => {
//             span = pat_wild.span();
//           }
//           _ => {
//             return Err(
//               captured
//                 .pat
//                 .span()
//                 .unstable()
//                 .error("expected argument, found pattern")
//                 .note("objrs does not support this pattern as a function argument"),
//             );
//           }
//         }

//         if let Type::ImplTrait(_) = captured.ty {
//           return Err(
//             captured
//               .ty
//               .span()
//               .unstable()
//               .error("expected type, found `impl Trait`")
//               .note("objrs does not support `impl Trait` in the argument position"),
//           );
//         }

//         let ident = Ident::new(&format!("_arg{}", n), span);
//         if is_instance_method && n == 0 {
//           self_arg_value = quote!(#ident);
//         }

//         captured.pat = Pat::Ident(PatIdent {
//           by_ref: None,
//           mutability: None,
//           ident: ident,
//           subpat: None,
//         });
//       }
//       FnArg::Inferred(_) => panic!("BUG: unexpected inferred function argument"),
//       FnArg::Ignored(_) => panic!("BUG: unexpected ignored function argument"),
//     }
//   }

//   let mut tail_arg_values: Punctuated<&Ident, Comma> = Punctuated::new();
//   for arg in method.sig.decl.inputs.iter().skip(is_instance_method as usize) {
//     if let FnArg::Captured(ref captured) = arg {
//       if let Pat::Ident(ref ident) = captured.pat {
//         tail_arg_values.push(&ident.ident);
//       }
//     }
//   }

//   let mut generics: Punctuated<&ToTokens, Comma> = Punctuated::new();
//   for generic in method.sig.decl.generics.params.iter() {
//     match generic {
//       GenericParam::Type(ref param) => generics.push(&param.ident),
//       GenericParam::Const(ref param) => generics.push(&param.ident),
//       GenericParam::Lifetime(_) => continue,
//     }
//   }

//   let msg_send_name_str = &["__objrs_msg_send_", method.sig.ident.to_string().as_ref()].concat();
//   let msg_send_name;
//   if let Some(span) = fn_span {
//     msg_send_name = Ident::new(msg_send_name_str, span);
//   } else {
//     msg_send_name = priv_ident_at(msg_send_name_str, method.sig.ident.span());
//   }
//   method.block.stmts.clear();
//   method.block.stmts.push(Stmt::Expr(Expr::Verbatim(ExprVerbatim {
//     tts: quote!{
//       #[allow(unused_unsafe)]
//       return Self::#msg_send_name::<#generics>(#self_arg_value, unsafe{ #objrs_root::__objrs::core::mem::uninitialized() }, #tail_arg_values);
//     },
//   })));

//   let msg_send = msg_send_fn(
//     &attr.sel,
//     attr.call_super,
//     &method,
//     &msg_send_name,
//     &inline,
//     is_instance_method,
//     is_generic_class,
//     objrs_root,
//   )?;

//   let native_ty = quote!(#objrs_root::__objrs);
//   let msg_recv;
//   if empty_msg_recv {
//     let mut clone = method.clone();
//     let msg_recv_name_str = &["__objrs_msg_recv_", method.sig.ident.to_string().as_ref()].concat();
//     if let Some(span) = fn_span {
//       clone.sig.ident = Ident::new(msg_recv_name_str, span);
//     } else {
//       clone.sig.ident = priv_ident_at(msg_recv_name_str, method.sig.ident.span());
//     }
//     clone.sig.abi = Some(Abi {
//       extern_token: Extern(Span::call_site()),
//       name: Some(LitStr::new("C", Span::call_site())),
//     });
//     // TODO: set the span of this to either the method name or the selector string so it's clear where the problem if there are duplicate symbol conflicts.
//     clone.sig.decl.inputs.insert(1, parse_quote!(_: #native_ty::usize)); // objrs::runtime::SEL
//     let brace = clone.block.brace_token.clone();
//     clone.block = parse_quote! {{
//       return unsafe { #objrs_root::__objrs::core::mem::uninitialized() };
//     }};
//     clone.block.brace_token = brace;
//     msg_recv = Some(clone);
//   } else {
//     msg_recv = None;
//   }

//   let tokens = quote! {
//     #method
//     #msg_send
//     #msg_recv
//   };

//   return Ok(tokens.into());
// }

#[cfg(test)]
mod tests {
  use super::*;
  use crate::parse::selector_attr::{ItemMethod, MethodType, SelectorAttr};
  use quote::ToTokens;
  use syn::parse_quote;

  #[test]
  fn gen_msg_recv_instance() {
    let objrs_root: Ident = parse_quote!(__objrs_root);
    let method = Method {
      attr: SelectorAttr {
        sel: LitStr::new("foo:bar:", Span::call_site()),
        call_super: false,
        no_impl: false,
        optional: None,
        method_type: MethodType::Auto,
      },
      method: ItemMethod::Impl(parse_quote! {
        #[doc = "This should be removed."]
        pub extern "Rust" fn foo_bar(&self, arg1: u32, arg2: bool) -> f32 {
          foo(arg1);
          bar(arg2);
          return baz();
        }
      }),
      is_instance_method: true,
      is_generic: false,
    };
    let method = gen_msg_recv(&method, "ClassName", None, &objrs_root).unwrap();

    let expected: ImplItemMethod = parse_quote! {
      #[doc(hidden)]
      #[export_name = "\u{1}-[ClassName foo:bar:]"]
      extern "C" fn __objrs_msg_recv_foo_bar(&self, _: &'static __objrs_root::Sel, arg1: u32, arg2: bool) -> f32 {
        foo(arg1);
        bar(arg2);
        return baz();
      }
    };
    assert_eq!(method.into_token_stream().to_string(), expected.into_token_stream().to_string());
  }

  #[test]
  fn gen_msg_recv_class_category() {
    let objrs_root: Ident = parse_quote!(__objrs_root);
    let method = Method {
      attr: SelectorAttr {
        sel: LitStr::new("baz", Span::call_site()),
        call_super: false,
        no_impl: false,
        optional: None,
        method_type: MethodType::Auto,
      },
      method: ItemMethod::Impl(parse_quote! {
        unsafe fn baz() {
        }
      }),
      is_instance_method: false,
      is_generic: false,
    };
    let method = gen_msg_recv(&method, "ClassName", Some("Category"), &objrs_root).unwrap();

    let expected: ImplItemMethod = parse_quote! {
      #[doc(hidden)]
      #[export_name = "\u{1}+[ClassName(Category) baz]"]
      unsafe extern "C" fn __objrs_msg_recv_baz(_: __objrs_root::__objrs::core::ptr::NonNull<__objrs_root::Class>, _: &'static __objrs_root::Sel) {
      }
    };
    assert_eq!(method.into_token_stream().to_string(), expected.into_token_stream().to_string());
  }
}
