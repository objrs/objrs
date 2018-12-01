// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

extern crate core;
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use gensym::random_identifier;
use ivar::transform_ivars;
use proc_macro::Diagnostic;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
 parse2, parse_quote,
  punctuated::Punctuated, spanned::Spanned, token::Comma, token::Extern, Abi,
  Attribute, Expr, ExprVerbatim, FnArg, GenericParam, Ident, ImplItemMethod, ImplItemVerbatim,
  LitByteStr, LitStr, MethodSig, Pat, PatIdent, ReturnType, Stmt, Type,
};
use syn::parse::{Parse, ParseStream};
use util::{is_instance_method, plural_s, priv_ident_at, DrainExt};

pub struct SelectorAttr {
  pub sel: LitStr,
  pub call_super: bool,
  pub no_impl: bool,
  pub optional: Option<Span>,
  pub method_type: MethodType,
}

#[derive(PartialEq)]
pub enum MethodType {
  Auto,
  Class,
  Instance,
}

// #[objrs(selector = "foo:bar:"
//         [, super|no_impl]
//         [, class|instance]
//         [, optional]
//         [,])]
impl Parse for SelectorAttr {
  fn parse(input: ParseStream) -> syn::parse::Result<Self> {
    use syn::parenthesized;
    use util::{KV, selector, no_impl, class, instance, optional};

    let content;
    let _: syn::token::Paren = parenthesized!(content in input);
    let input = &content;

    let mut kv = KV::new(input);
    let sel: LitStr = kv.parse::<selector, _>()?;
    let call_super: Option<()> = kv.parse::<syn::token::Super, _>()?;
    let no_impl: Option<()> = if call_super.is_some() { None } else { kv.parse::<no_impl, _>()? };
    let class: Option<()> = kv.parse::<class, _>()?;
    let instance: Option<()> = if class.is_some() { None } else { kv.parse::<instance, _>()? };
    let optional: Option<Span> = kv.parse::<optional, _>()?;
    let method_type;
    if class.is_some() {
      method_type = MethodType::Class;
    } else if instance.is_some() {
      method_type = MethodType::Instance;
    } else {
      method_type = MethodType::Auto;
    }
    kv.eof()?;
    return Ok(
      SelectorAttr {
        sel: sel,
        call_super: call_super.is_some(),
        no_impl: no_impl.is_some(),
        optional: optional,
        method_type: method_type,
      }
    );
  }
}

pub struct ObjrsMethod {
  pub msg_send: Option<ImplItemVerbatim>,
  pub msg_recv: Option<ImplItemMethod>,
  pub selector: SelectorAttr,
  pub is_instance_method: bool,
}

pub fn validate_selector(sel: &LitStr) -> Result<(String, usize), Diagnostic> {
  let string = sel.value();
  let mut allow_digit = false;
  let mut expected_arg_count = 0;
  let mut ends_with_colon = false;
  for byte in string.bytes() {
    if !byte.is_ascii() {
      return Err(
        sel
          .span()
          .unstable()
          .error(format!("selector contains non-ASCII byte `0x{:02x?}`", byte))
          .note("Objective-C only permits ASCII characters in selector names"),
      );
    }
    if byte.is_ascii_control() {
      return Err(
        sel
          .span()
          .unstable()
          .error(format!("selector contains ASCII control character `0x{:02x?}`", byte))
          .note("Objective-C does not permit ASCII control characters in selector names"),
      );
    }
    if byte.is_ascii_whitespace() {
      return Err(
        sel
          .span()
          .unstable()
          .error(format!(
            "selector contains whitespace character `{}` (ASCII value `0x{:02x?}`)",
            byte as char, byte
          )).note("Objective-C does not permit whitespace in selector names"),
      );
    }
    if byte != b':' && byte != b'_' && byte != b'$' && byte.is_ascii_punctuation() {
      return Err(sel.span().unstable().error(format!("selector contains punctuation character `{}` (ASCII value `0x{:02x?}`)", byte as char, byte)).note("Objective-C does not permit punctuation (except for `:`, `_`, and `$`) in selector names"));
    }
    if !allow_digit && byte.is_ascii_digit() {
      return Err(
        sel
          .span()
          .unstable()
          .error(format!(
            "selector contains digit `{}` (ASCII value `0x{:02x?}`) in an illegal location",
            byte as char, byte
          )).note("digits must be preceded by an alphabetic, `_`, or `$` character"),
      );
    }
    let is_colon = byte == b':';
    allow_digit = !is_colon;
    expected_arg_count += is_colon as usize;
    ends_with_colon = is_colon;
  }

  if expected_arg_count > 0 && !ends_with_colon {
    return Err(
      sel
        .span()
        .unstable()
        .error("selector expects arguments but doesn't end with `:`")
        .note("the last character in a selector that takes arguments must be `:`"),
    );
  }

  return Ok((string, expected_arg_count));
}

fn validate_selector_for_method(
  sel: &LitStr,
  sig: &MethodSig,
  is_instance_method: bool,
) -> Result<String, Diagnostic> {
  let (string, expected_arg_count) = validate_selector(sel)?;

  let args = &sig.decl.inputs;
  let arg_count = args.len() - is_instance_method as usize;
  if arg_count != expected_arg_count {
    let error_msg = format!(
      "method `{}` has {} parameter{}{} but the selector has {}",
      sig.ident,
      arg_count,
      plural_s(arg_count),
      if is_instance_method {
        " (not counting self)"
      } else {
        ""
      },
      expected_arg_count
    );
    let note_msg =
      format!("selector requires {} parameter{}", expected_arg_count, plural_s(expected_arg_count));
    return Err(sig.span().unstable().error(error_msg).span_note(sel.span().unstable(), note_msg));
  }
  return Ok(string);
}

pub fn parse_selector_ref(input: TokenStream) -> Result<TokenStream, Diagnostic> {
  let span = input.span();
  let mut selector_string;
  match parse2::<LitStr>(input.into()) {
    Ok(value) => selector_string = validate_selector(&value)?.0,
    Err(err) => {
      return Err(span.unstable().error(err.to_string()).note("selector! requires a single string literal. Examples: selector!(\"terminate:\"); nsstring!(\"length\"); nsstring!(r#\"doFoo:withBar:\"#);"));
    }
  }

  let random_id = &random_identifier();
  let random_id = unsafe { core::str::from_utf8_unchecked(random_id) };

  let meth_name_export_name =
    ["\x01L_OBJC_METH_VAR_NAME_.__objrs_meth.", random_id, ".", &selector_string].concat();

  let sel_ref_export_name =
    ["\x01L_OBJC_SELECTOR_REFERENCES_.__objrs_sel.", random_id, ".", &selector_string].concat();

  selector_string.push('\x00');
  let selector = LitByteStr::new(selector_string.as_bytes(), span);
  let selector_len = selector_string.len();

  let tokens = quote!{{
    extern crate objrs as __objrs_root;

    #[link_section = "__TEXT,__objc_methname,cstring_literals"]
    #[export_name = #meth_name_export_name]
    static METH_NAME: [u8; #selector_len] = * #selector;

    #[link_section = "__DATA,__objc_selrefs,literal_pointers,no_dead_strip"]
    #[export_name = #sel_ref_export_name]
    static SEL_REF: &'static [u8; #selector_len] = &METH_NAME;

    __objrs_root::runtime::SEL(unsafe { __objrs_root::__objrs::core::ptr::read_volatile(&SEL_REF as *const _) } as *const _ as *const _)
  }};

  return Ok(tokens);
}

pub fn parse_selector_method(
  mut method: ImplItemMethod,
  mut class_name: Option<&LitStr>,
  category: Option<&str>,
  is_generic_class: bool,
  empty_msg_recv: bool,
) -> Result<Result<ObjrsMethod, ImplItemMethod>, Diagnostic> {
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
    None => return Ok(Err(method)),
  }

  if let Some(optional) = selector_attr.optional {
    return Err(
      optional
        .unstable()
        .error("`optional` mut not be used outside of a protocol trait definition")
        .note("objrs only allows `optional` on methods inside of #[objrs(protocol)] items"),
    );
  }

  if selector_attr.no_impl {
    class_name = None;
  }
  let no_impl = class_name.is_none();
  if no_impl && !method.block.stmts.is_empty() {
    return Err(
      method
        .block
        .span()
        .unstable()
        .error("expected empty block on external Objective-C method, found non-empty block")
        .note("objrs will replace the method's block with its own implementation"),
    );
  }

  let is_instance_method = selector_attr.method_type == MethodType::Instance
    || (selector_attr.method_type == MethodType::Auto
      && is_instance_method(&method.sig.decl.inputs));

  let msg_recv;
  if let Some(class_name) = class_name {
    msg_recv = Some(make_msg_recv(
      &method,
      &selector_attr.sel.value(),
      class_name,
      category,
      is_instance_method,
    )?);
  } else {
    msg_recv = None;
  }

  let msg_send;
  if is_instance_method && selector_attr.sel.value() == "dealloc" {
    msg_send = None;
  } else {
    msg_send = Some(ImplItemVerbatim {
      tts: parse_selector(&selector_attr, method, is_generic_class, empty_msg_recv, None)?,
    });
  }

  return Ok(Ok(ObjrsMethod {
    msg_send: msg_send,
    msg_recv: msg_recv,
    selector: selector_attr,
    is_instance_method: is_instance_method,
  }));
}

fn make_msg_recv(
  method: &ImplItemMethod,
  selector: &str,
  class_name: &LitStr,
  category: Option<&str>,
  is_instance_method: bool,
) -> Result<ImplItemMethod, Diagnostic> {
  // TODO: let objc_method_family to be explicitly specified?
  // TODO: take the method family into account.
  let mut clone = method.clone();
  clone.sig.ident = priv_ident_at(
    &["__objrs_msg_recv_", clone.sig.ident.to_string().as_ref()].concat(),
    clone.sig.ident.span(),
  );
  clone.sig.abi = Some(Abi {
    extern_token: Extern(Span::call_site()),
    name: Some(LitStr::new("C", Span::call_site())),
  }); // TODO: use def_site().
  let instance_or_class = if is_instance_method {
    "-"
  } else {
    "+"
  };
  let category = &category.map_or_else(String::new, |name| ["(", name, ")"].concat());
  let export_name =
    ["\x01", instance_or_class, "[", &class_name.value(), category, " ", selector, "]"].concat();
  if !is_instance_method {
    clone.sig.decl.inputs.insert(0, parse_quote!(_: usize)); // objrs::runtime::Class
  }
  clone.sig.decl.inputs.insert(1, parse_quote!(_: usize)); // objrs::runtime::SEL
                                                           // TODO: set the span of this to either the method name or the selector string so it's clear where the problem if there are duplicate symbol conflicts.
  clone.attrs.push(parse_quote!(#[export_name = #export_name])); // TODO: remove any existing export_name attributes.
  if is_instance_method {
    transform_ivars(&mut clone)?;
  }

  return Ok(clone);
}

fn msg_send_fn(
  selector: &LitStr,
  call_super: bool,
  method: &ImplItemMethod,
  name: &Ident,
  inline: &ToTokens,
  is_instance_method: bool,
  is_generic_class: bool,
) -> Result<TokenStream, Diagnostic> {
  let mut selector_string =
    validate_selector_for_method(selector, &method.sig, is_instance_method)?;

  let objc_send;
  let objc_send_stret;
  if call_super {
    objc_send = quote!(objc_msgSendSuper2);
    objc_send_stret = quote!(objc_msgSendSuper2_stret);
  } else {
    objc_send = quote!(objc_msgSend);
    objc_send_stret = quote!(objc_msgSend_stret);
  }

  let random_id = &random_identifier();
  let random_id = unsafe { core::str::from_utf8_unchecked(random_id) };

  let meth_name_export_name =
    ["\x01L_OBJC_METH_VAR_NAME_.__objrs_meth.", random_id, ".", &selector_string].concat();

  let sel_ref_export_name =
    ["\x01L_OBJC_SELECTOR_REFERENCES_.__objrs_sel.", random_id, ".", &selector_string].concat();

  selector_string.push('\x00');
  let selector = LitByteStr::new(selector_string.as_bytes(), selector.span());
  let selector_len = selector_string.len();

  let empty_tuple = parse_quote!(());
  let return_type = match method.sig.decl.output {
    ReturnType::Default => &empty_tuple,
    ReturnType::Type(_, ref ty) => ty.as_ref(),
  };

  let generics = &method.sig.decl.generics;

  let ref_hack;
  if !is_generic_class && generics.params.is_empty() {
    ref_hack = quote!{
      let sel = unsafe { __objrs_root::__objrs::core::ptr::read_volatile(&SEL_REF as *const _) } as *const _;
    };
  } else {
    // TODO: Make this optional! It's needed because #[inline(never)] doesn't really do anything
    // for generic functions (inluding non-generic methods for generic types). Also, incremental
    // compilation can wreck havoc with this (it seems to compile things into lots of separate
    // object files, which breaks references to L_* locals).
    ref_hack = quote!{
      #[inline(never)]
      fn sel_ref_hack() -> *const [u8; #selector_len] {
        return unsafe { __objrs_root::__objrs::core::ptr::read_volatile(&SEL_REF as *const _) } as *const _;
      }
      let sel = sel_ref_hack();
    };
  }

  let where_clause = &generics.where_clause;
  let mut inputs = method.sig.decl.inputs.clone();
  let self_arg_type;
  let self_arg_value;
  if !is_instance_method {
    inputs.insert(0, parse_quote!(_: usize));
    if call_super {
      self_arg_type = quote!(*mut __objrs_root::runtime::objc_super);
      self_arg_value = quote!{&mut __objrs_root::runtime::objc_super {
        receiver: __objrs_root::__objrs::core::mem::transmute(Self::__objrs_class_ref()),
        super_class: Self::__objrs_super_meta_ref(),
      } as *mut _};
    } else {
      self_arg_type = quote!(__objrs_root::runtime::Class);
      self_arg_value = quote!(Self::__objrs_class_ref());
    }
  } else {
    match method.sig.decl.inputs[0] {
      FnArg::SelfRef(ref self_ref) => {
        if call_super {
          self_arg_type = quote!(*mut __objrs_root::runtime::objc_super);
          self_arg_value = quote!{&mut __objrs_root::runtime::objc_super {
            receiver: __objrs_root::__objrs::core::mem::transmute(self),
            super_class: Self::__objrs_super_class_ref(),
          } as *mut _};
        } else if self_ref.mutability.is_some() {
          self_arg_type = quote!(&mut Self);
          self_arg_value = quote!(self);
        } else {
          self_arg_type = quote!(&Self);
          self_arg_value = quote!(self);
        }
      }
      FnArg::SelfValue(_) => {
        if call_super {
          self_arg_type = quote!(*mut __objrs_root::runtime::objc_super);
          self_arg_value = quote!{&mut __objrs_root::runtime::objc_super {
            receiver: __objrs_root::__objrs::core::mem::transmute(self),
            super_class: Self::__objrs_super_class_ref(),
          } as *mut _};
        } else {
          self_arg_type = quote!(Self);
          self_arg_value = quote!(self);
        }
      }
      FnArg::Captured(ref arg) => {
        let pat = &arg.pat;
        let ty = &arg.ty;
        if call_super {
          self_arg_type = quote!(*mut __objrs_root::runtime::objc_super);
          self_arg_value = quote!{&mut __objrs_root::runtime::objc_super {
            receiver: __objrs_root::__objrs::core::mem::transmute(#pat),
            super_class: Self::__objrs_super_class_ref(),
          } as *mut _};
        } else {
          self_arg_type = quote!(#ty);
          self_arg_value = quote!(#pat);
        }
      }
      _ => panic!("BUG: unhandled self type"),
    }
  }
  inputs.insert(1, parse_quote!(_: usize));
  // TODO: handle variadic.
  let output = &method.sig.decl.output;

  let mut tail_arg_types: Punctuated<&Type, Comma> = Punctuated::new();
  let mut tail_arg_values: Punctuated<&Ident, Comma> = Punctuated::new();
  for arg in inputs.iter().skip(2) {
    if let FnArg::Captured(ref captured) = arg {
      tail_arg_types.push(&captured.ty);
      if let Pat::Ident(ref ident) = captured.pat {
        tail_arg_values.push(&ident.ident);
      } else {
        panic!("BUG: unexpected uncaptured function argument");
      }
    } else {
      panic!("BUG: unexpected uncaptured function argument");
    }
  }

  let unsafety = &method.sig.unsafety;

  let msg_send = quote!{
    // TODO: can this be inline(always) for LTO builds?
    #[allow(dead_code)]
    #[inline #inline]
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    #unsafety extern "C" fn #name #generics(#inputs) #output #where_clause {
      extern crate objrs as __objrs_root;

      #[link_section = "__TEXT,__objc_methname,cstring_literals"]
      #[export_name = #meth_name_export_name]
      static METH_NAME: [u8; #selector_len] = * #selector;

      #[link_section = "__DATA,__objc_selrefs,literal_pointers,no_dead_strip"]
      #[export_name = #sel_ref_export_name]
      static SEL_REF: &'static [u8; #selector_len] = &METH_NAME;

      #[cfg(target_arch = "x86_64")]
      let msg_send = if __objrs_root::__objrs::core::mem::size_of::<#return_type>() <= 16 {
        __objrs_root::runtime::#objc_send
      } else {
        __objrs_root::runtime::#objc_send_stret
      };
      #[cfg(target_arch = "aarch64")]
      let msg_send = __objrs_root::runtime::objc_msgSend;

      // Use a pointer (*const [u8; N]) for the selector type rather than a reference (&[u8; N]). If we used a reference, users would have to explicitly annotate most functions with lifetime parameters.
      let msg_send: unsafe extern fn(#self_arg_type, *const [u8; #selector_len], #tail_arg_types) #output = unsafe { __objrs_root::__objrs::core::mem::transmute(msg_send as *const ()) };

      #ref_hack

      return unsafe { msg_send(#self_arg_value, sel, #tail_arg_values) };

      // compile_error!("impl blocks must use the #[objrs(impl)] attribute");
    }
  };

  // let test = quote!{
  //   #[cfg(test)]
  //   #[test]
  //   fn #name() {
  //     extern crate objrs as __objrs_root;

  //     let class = __objrs_root::runtime::objc_getClass(#class_name);
  //     assert_ne!(class, __objrs_root::__objrs::core::ptr::null());

  //     let sel = __objrs_root::runtime::sel_registerName(#sel);

  //     assert!(__objrs_root::runtime::class_respondsToSelector(class, sel));

  //     let method;
  //     if is_instance_method {
  //       method = __objrs_root::runtime::class_getInstanceMethod(class, sel);
  //     } else {
  //       method = __objrs_root::runtime::class_getClassMethod(class, sel);
  //     }
  //     assert_ne!(method, __objrs_root::__objrs::core::ptr::null_mut());

  //     let argument_count = __objrs_root::runtime::method_getNumberOfArguments(method);
  //     assert_eq!(argument_count, #inputs.len());

  //     let type_encoding = __objrs_root::runtime::method_getTypeEncoding(method);
  //     assert_ne!(type_encoding, __objrs_root::__objrs::core::ptr::null());

  //     // TODO: iterate over the type encoding, making sure the parameters all match up.
  //   }
  // };

  return Ok(msg_send);
}

pub fn parse_selector(
  attr: &SelectorAttr,
  mut method: ImplItemMethod,
  is_generic_class: bool,
  empty_msg_recv: bool,
  fn_span: Option<Span>,
) -> Result<TokenStream, Diagnostic> {
  let mut inline;
  if attr.call_super {
    inline = quote!((always));
  } else {
    inline = quote!((never));
  }
  for attr in method.attrs.iter_mut() {
    if attr.path.leading_colon.is_none() && attr.path.segments.len() == 1 {
      let segment = attr.path.segments.iter().next().expect("BUG: expected exactly 1 segment");
      if segment.ident == "inline" && segment.arguments.is_empty() {
        inline = core::mem::replace(&mut attr.tts, quote!((always)));
        break;
      }
    }
  }

  let is_instance_method = attr.method_type == MethodType::Instance
    || (attr.method_type == MethodType::Auto && is_instance_method(&method.sig.decl.inputs));

  let mut self_arg_value;
  if is_instance_method {
    self_arg_value = quote!(self);
  } else {
    self_arg_value = quote!(unsafe { __objrs_root::__objrs::core::mem::uninitialized() });
  }

  for (n, arg) in method.sig.decl.inputs.iter_mut().enumerate() {
    match arg {
      FnArg::SelfRef(_) => continue,
      FnArg::SelfValue(_) => continue,
      FnArg::Captured(ref mut captured) => {
        let span;
        match captured.pat {
          Pat::Ident(ref mut ident) => {
            if n == 0 && ident.ident == "self" {
              continue;
            }

            if let Some(ref by_ref) = ident.by_ref {
              // TODO: should we let parameters be a `ref`? It could, in theory, be nice to take a parameter by value at the Rust level (moving it), and pass it to the Objective-C level as a pointer. For example:
              //
              // struct MyStruct {...}  // Non-copyable struct.
              // #[objrs(selector = "foo:")]
              // fn foo(ref s: MyStruct) {}
              //
              // This would expand into (simplified):
              //
              // #[inline(always)]
              // fn foo(ref s: MyStruct) {
              //   #[inline(never)]
              //   fn msg_send(_: usize, _: usize, s: &MyStruct) {
              //     // ...
              //   }
              //   msg_send(_, _, s);
              // }
              //
              // This is nice if you want to move a object that Objective-C is trying to take by address. If supporting this is desirable, then `ref mut` should also be allowed (so Objective-C gets a `&mut` parameter).
              //
              // It's a bit of a foot-gun though. It introduces a subtle difference between Rust and Objective-C, and it's not immediately obvious how it works.
              return Err(
                by_ref
                  .span()
                  .unstable()
                  .error("method parameters cannot be a `ref`")
                  .note("the current version of objrs disallows them"),
              );
            }
            if let Some(ref mutability) = ident.mutability {
              mutability
                .span()
                .unstable()
                .warning("variable does not need to be mutable")
                .note("objrs will remove this `mut` keyword")
                .emit();
            }
            if let Some((ref at, ref pat)) = ident.subpat {
              at.span()
                .unstable()
                .join(pat.span().unstable())
                .expect("BUG: unexpected spans from different files")
                .warning("pattern will not be used")
                .note("objrs will remove this pattern")
                .emit();
            }
            span = ident.ident.span();
          }
          Pat::Wild(ref pat_wild) => {
            span = pat_wild.span();
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

        if let Type::ImplTrait(_) = captured.ty {
          return Err(
            captured
              .ty
              .span()
              .unstable()
              .error("expected type, found `impl Trait`")
              .note("objrs does not support `impl Trait` in the argument position"),
          );
        }

        let ident = Ident::new(&format!("_arg{}", n), span);
        if is_instance_method && n == 0 {
          self_arg_value = quote!(#ident);
        }

        captured.pat = Pat::Ident(PatIdent {
          by_ref: None,
          mutability: None,
          ident: ident,
          subpat: None,
        });
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

  let mut generics: Punctuated<&ToTokens, Comma> = Punctuated::new();
  for generic in method.sig.decl.generics.params.iter() {
    match generic {
      GenericParam::Type(ref param) => generics.push(&param.ident),
      GenericParam::Const(ref param) => generics.push(&param.ident),
      GenericParam::Lifetime(_) => continue,
    }
  }

  let msg_send_name_str = &["__objrs_msg_send_", method.sig.ident.to_string().as_ref()].concat();
  let msg_send_name;
  if let Some(span) = fn_span {
    msg_send_name = Ident::new(msg_send_name_str, span);
  } else {
    msg_send_name = priv_ident_at(msg_send_name_str, method.sig.ident.span());
  }
  method.block.stmts.clear();
  method.block.stmts.push(Stmt::Expr(Expr::Verbatim(ExprVerbatim {
    tts: quote!{
      extern crate objrs as __objrs_root;
      #[allow(unused_unsafe)]
      return Self::#msg_send_name::<#generics>(#self_arg_value, unsafe{ __objrs_root::__objrs::core::mem::uninitialized() }, #tail_arg_values);
    },
  })));

  let msg_send = msg_send_fn(
    &attr.sel,
    attr.call_super,
    &method,
    &msg_send_name,
    &inline,
    is_instance_method,
    is_generic_class,
  )?;

  let msg_recv;
  if empty_msg_recv {
    let mut clone = method.clone();
    let msg_recv_name_str = &["__objrs_msg_recv_", method.sig.ident.to_string().as_ref()].concat();
    if let Some(span) = fn_span {
      clone.sig.ident = Ident::new(msg_recv_name_str, span);
    } else {
      clone.sig.ident = priv_ident_at(msg_recv_name_str, method.sig.ident.span());
    }
    clone.sig.abi = Some(Abi {
      extern_token: Extern(Span::call_site()),
      name: Some(LitStr::new("C", Span::call_site())),
    });
    // TODO: set the span of this to either the method name or the selector string so it's clear where the problem if there are duplicate symbol conflicts.
    clone.sig.decl.inputs.insert(1, parse_quote!(_: usize)); // objrs::runtime::SEL
    let brace = clone.block.brace_token.clone();
    clone.block = parse_quote!{{
      extern crate objrs as __objrs_root;
      return unsafe { __objrs_root::__objrs::core::mem::uninitialized() };
    }};
    clone.block.brace_token = brace;
    msg_recv = Some(clone);
  } else {
    msg_recv = None;
  }

  let tokens = quote!{
    #method
    #msg_send
    #msg_recv
  };

  return Ok(tokens.into());
}
