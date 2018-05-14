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
use parser::AttributeParser;
use proc_macro::Diagnostic;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, Tokens};
use syn::{
  buffer::TokenBuffer, parse_quote, punctuated::Punctuated, spanned::Spanned, synom::Synom,
  token::Comma, FnArg, GenericParam, Ident, ImplItemMethod, LitByteStr, LitStr, Meta, NestedMeta,
  Pat, PatIdent, ReturnType, Type,
};

fn is_instance_method(args: &Punctuated<FnArg, Comma>) -> bool {
  if args.is_empty() {
    return false;
  }

  // TODO: handle arbitrary_self_types feature (see https://github.com/rust-lang/rust/issues/46906 for a small example).
  match args[0] {
    FnArg::SelfRef(_) => return true,
    FnArg::SelfValue(_) => return true,
    _ => return false,
  }
}

fn validate_selector(sel: &LitStr) -> Result<String, Diagnostic> {
  let string = sel.value();
  let mut allow_digit = false;
  for byte in string.bytes() {
    if !byte.is_ascii() {
      return Err(
        sel
          .span()
          .unstable()
          .error(format!(
            "selector contains non-ASCII byte `0x{:02x?}`",
            byte
          ))
          .note("Objective-C only permits ASCII characters in selector names"),
      );
    }
    if byte.is_ascii_control() {
      return Err(
        sel
          .span()
          .unstable()
          .error(format!(
            "selector contains ASCII control character `0x{:02x?}`",
            byte
          ))
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
          ))
          .note("Objective-C does not permit whitespace in selector names"),
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
          ))
          .note("Objective-C does not permit punctuation (except for `$`) in selector names"),
      );
    }
    allow_digit = byte != b':'
  }
  return Ok(string);
}

fn msg_send_fn(
  selector: &LitStr,
  method: &ImplItemMethod,
  name: &Ident,
  inline: &ToTokens,
) -> Result<Tokens, Diagnostic> {
  let mut selector_string = validate_selector(selector)?;

  let random_id = &random_identifier();
  let random_id = unsafe { core::str::from_utf8_unchecked(random_id) };

  let meth_name_export_name = [
    "\x01L_OBJC_METH_VAR_NAME_.__objrs.",
    random_id,
    ".",
    &selector_string,
  ].concat();

  let sel_ref_export_name = [
    "\x01L_OBJC_SELECTOR_REFERENCES_.__objrs.",
    random_id,
    ".",
    &selector_string,
  ].concat();

  selector_string.push('\0');
  let selector = LitByteStr::new(selector_string.as_bytes(), selector.span());
  let selector_len = selector_string.len();

  let empty_tuple = parse_quote!(());
  let return_type = match method.sig.decl.output {
    ReturnType::Default => &empty_tuple,
    ReturnType::Type(_, ref ty) => ty.as_ref(),
  };

  let generics = &method.sig.decl.generics;
  let where_clause = &generics.where_clause;
  let mut inputs = method.sig.decl.inputs.clone();
  let self_arg_type;
  let self_arg_value;
  if !is_instance_method(&inputs) {
    inputs.insert(0, parse_quote!(_: usize));
    self_arg_type = quote!(objrs::runtime::Class);
    self_arg_value = quote!(Self::__objrs_class_ref());
  } else {
    self_arg_type = quote!(&Self);
    self_arg_value = quote!(self);
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

  let msg_send = quote!{
    // TODO: can this be inline(always) for LTO builds?
    #[allow(dead_code)]
    #[inline #inline]
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    extern "C" fn #name #generics(#inputs) #output #where_clause {
      extern crate objrs;

      #[link_section = "__TEXT,__objc_methname,cstring_literals"]
      #[export_name = #meth_name_export_name]
      static METH_NAME: [u8; #selector_len] = * #selector;

      #[link_section = "__DATA,__objc_selrefs,literal_pointers,no_dead_strip"]
      #[export_name = #sel_ref_export_name]
      static SEL_REF: &'static [u8; #selector_len] = &METH_NAME;

      #[cfg(target_arch = "x86_64")]
      let msg_send = if objrs::__objrs::core::mem::size_of::<#return_type>() <= 16 {
        objrs::runtime::objc_msgSend
      } else {
        objrs::runtime::objc_msgSend_stret
      };
      #[cfg(target_arch = "aarch64")]
      let msg_send = objrs::runtime::objc_msgSend;

      // Use a pointer (*const [u8; N]) for the selector type rather than a reference (&[u8; N]). If we used a reference, users would have to explicitly annotate most functions with lifetime parameters.
      let msg_send: unsafe extern fn(#self_arg_type, *const [u8; #selector_len], #tail_arg_types) #output = unsafe { objrs::__objrs::core::mem::transmute(msg_send as *const ()) };

      // TODO: Revert this hack! It's needed because #[inline(never)] doesn't really do anything for
      // generic functions (inluding non-generic methods for generic types). Also, incremental
      // compilation can wreck havoc with this (it seems to compile things into lots of separate
      // object files, which breaks references to L_* locals).
      #[inline(never)]
      fn sel_ref_hack() -> *const [u8; #selector_len] {
        return unsafe { objrs::__objrs::core::ptr::read_volatile(&SEL_REF as *const _) } as *const _;
      }
      let sel = sel_ref_hack();
      // let sel = unsafe { objrs::__objrs::core::ptr::read_volatile(&SEL_REF as *const _) } as *const _;

      return unsafe { msg_send(#self_arg_value, sel, #tail_arg_values) };

      // compile_error!("impl blocks must use the #[objrs(impl)] attribute");
    }
  };

  // let test = quote!{
  //   #[cfg(test)]
  //   #[test]
  //   fn #name() {
  //     extern crate objrs;

  //     let class = objrs::runtime::objc_getClass(#class_name);
  //     assert_ne!(class, objrs::__objrs::core::ptr::null());

  //     let sel = objrs::runtime::sel_registerName(#sel);

  //     assert!(objrs::runtime::class_respondsToSelector(class, sel));

  //     let method;
  //     if is_instance_method {
  //       method = objrs::runtime::class_getInstanceMethod(class, sel);
  //     } else {
  //       method = objrs::runtime::class_getClassMethod(class, sel);
  //     }
  //     assert_ne!(method, objrs::__objrs::core::ptr::null_mut());

  //     let argument_count = objrs::runtime::method_getNumberOfArguments(method);
  //     assert_eq!(argument_count, #inputs.len());

  //     let type_encoding = objrs::runtime::method_getTypeEncoding(method);
  //     assert_ne!(type_encoding, objrs::__objrs::core::ptr::null());

  //     // TODO: iterate over the type encoding, making sure the parameters all match up.
  //   }
  // };

  return Ok(msg_send);
}

pub fn parse_selector(
  mut args_parser: AttributeParser,
  input: TokenStream,
) -> Result<TokenStream, Diagnostic> {
  args_parser.op('=')?;
  let selector = args_parser.string()?;
  args_parser.eof()?;

  let input = TokenBuffer::new2(input);
  let mut method = match <ImplItemMethod as Synom>::parse(input.begin()) {
    Ok((impl_item_method, _)) => impl_item_method,
    Err(error) => {
      return Err(
        input
          .begin()
          .token_stream()
          .span()
          .unstable()
          .error(format!("failed to parse method: {}", error.to_string()))
          .note("#[objrs(selector ...)] must only be applied to methods within an impl block"),
      );
    }
  };

  let mut inline = quote!((never));
  for attr in method.attrs.iter_mut() {
    if attr.path.leading_colon.is_none() && attr.path.segments.len() == 1 {
      let segment = attr
        .path
        .segments
        .iter()
        .next()
        .expect("BUG: expected exactly 1 segment");
      if segment.ident.as_ref() == "inline" && segment.arguments.is_empty() {
        let meta = attr
          .interpret_meta()
          .expect("BUG: failed to parse meta for `#[inline ...]`");
        attr.tts = quote!((always)).into();
        match meta {
          Meta::Word(_) => {
            inline = quote!();
          }
          Meta::List(list) => {
            assert_eq!(
              list.nested.len(),
              1,
              "BUG: expected `...` in `#[inline(...)]` to have 1 item"
            );
            match list.nested.iter().next() {
              Some(NestedMeta::Meta(Meta::Word(ident))) => match ident.as_ref() {
                "always" => inline = quote!((always)),
                "never" => (),
                _ => panic!("BUG: expected `always` or `never`"),
              },
              _ => panic!("BUG: expected `(always)` or `(never)`"),
            }
          }
          Meta::NameValue(_) => panic!("BUG: unexpected name-value meta pair"),
        }
        break;
      }
    }
  }

  let self_arg_value;
  if !is_instance_method(&method.sig.decl.inputs) {
    self_arg_value = quote!(unsafe { objrs::__objrs::core::mem::uninitialized() });
  } else {
    self_arg_value = quote!(self);
  }

  for (n, arg) in method.sig.decl.inputs.iter_mut().enumerate() {
    match arg {
      FnArg::SelfRef(_) => continue,
      FnArg::SelfValue(_) => continue,
      FnArg::Captured(ref mut captured) => {
        let span;
        match captured.pat {
          Pat::Ident(ref mut ident) => {
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
              let pat_span = at.span()
                .unstable()
                .join(pat.span().unstable())
                .expect("BUG: unexpected spans from different files");
              pat_span
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

        captured.pat = Pat::Ident(PatIdent {
          by_ref: None,
          mutability: None,
          ident: Ident::new(&format!("arg{}", n), span),
          subpat: None,
        });
      }
      FnArg::Inferred(_) => panic!("BUG: unexpected inferred function argument"),
      FnArg::Ignored(_) => panic!("BUG: unexpected ignored function argument"),
    }
  }

  let mut tail_arg_values: Punctuated<&Ident, Comma> = Punctuated::new();
  for arg in method.sig.decl.inputs.iter() {
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

  if !method.block.stmts.is_empty() {
    return Err(
      method
        .block
        .span()
        .unstable()
        .error("expected empty block, found non-empty block")
        .note("objrs will replace the method's block with its own implementation"),
    );
  }

  let msg_send_name = Ident::from(["__objrs_msg_send_", method.sig.ident.as_ref()].concat());
  method.block = parse_quote!{{
    extern crate objrs;
    #[allow(unused_unsafe)]
    return Self::#msg_send_name::<#generics>(#self_arg_value, unsafe{ objrs::__objrs::core::mem::uninitialized() }, #tail_arg_values);
  }};

  let msg_send = msg_send_fn(&selector, &method, &msg_send_name, &inline)?;

  let tokens = quote!{
    #method
    #msg_send
  };

  return Ok(tokens.into());
}
