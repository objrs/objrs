// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

//! The `#[objrs(selector)]` macro parser module.
//!
//! #[objrs(impl)]
//! impl MyCustomObject {
//!   #[objrs(selector = "LITERAL_STR"
//!           [, super|no_impl]
//!           [, class|instance]
//!           [, optional][,])]
//!   pub fn foo() -> usize {}
//! }
//! Use `#[objrs(selector)]` on a method to declare it as an Objective-C method.
//!
//! Parameters:
//!
//! - `selector = "LITERAL_STR"`. Required. This is the actual name of the selector
//!   (e.g. `"doFoo:withBar:"`).
//! - `super` or `no_impl`. Optional. Use `super` to declare the method as a super method
//!   invocation. Use `no_impl` to simply declare that the class responds to that selector but does
//!   so by using the super's implementation. The difference between `super` and `no_impl` is
//!   subtle, so see the examples below.
//! - `class` or `instance`. Optional. objrs will inspect the method's signature and if it takes
//!   `self`, the method will be an instance method. If it lacks `self`, it will be a class method.
//!   Sometimes you can't take a `self` parameter when you need to, though, which means objrs's
//!   auto-deductions aren't always useful. In these situations, you can explicitly declare the
//!   method as being a `class` or `instance` method.
//! - `optional`. Optional. Equivalent to Objective-C's `@optional`. Only use this in protocol
//!   traits. Attempting to call an `optional` method that the class doesn't implement will result
//!   in a panic (or exception, if calling from Objective-C).
//!
//! Calling a super's method:
//!
//! ```ignore
//! #[objrs(impl)]
//! impl MyCustomObject {
//!   #[objrs(selector = "hash")]
//!   pub fn hash(&self) -> usize {
//!     println!("-[MyCustomObject hash] is being called. Forwarding to [super hash]");
//!     return self.super_hash();
//!   }
//!
//!   #[objrs(selector = "hash", super)]
//!   fn super_hash(&self) -> usize {}
//! }
//! ```
//!
//! Here, the super's `hash` method will be invoked. Using `no_impl` can be useful for class
//! methods, where our `Deref` hack can't emulate inheritance.
//!
//! ```ignore
//! #[objrs(impl)]
//! impl MyCustomObject {
//!   #[objrs(selector = "hash", no_impl)]
//!   pub fn hash(&self) -> usize {}
//! }
//! ```
//!
//! Both `init1` and `init2` do the same thing. objrs will inspect the method, and if it takes a
//! `self` parameter, it will make the method an instance method. If the method lacks a `self`
//! parameter, it will be a class method. You can use [arbitrary self
//! types](https://github.com/rust-lang/rust/issues/44874). If you don't want to use arbitrary self
//! types, but find yourself in a position where you can't use `self`, you can use `class` or
//! `instance` to explicitly tell objrs whether the method is a class or instance method.
//!
//! ```ignore
//! #[objrs(impl)]
//! impl MyCustomObject {
//!   // Without arbitrary_self_types, we need to tell objrs the init method is an instance method.
//!   #[objrs(selector = "init", no_impl, instance)]
//!   pub fn init1(this: objrs::Alloc<MyCustomObject>) -> usize {}
//!
//!   // With arbitrary_self_types, objrs will detect this as an instance method since it takes self.
//!   #[objrs(selector = "init", no_impl)]
//!   pub fn init2(self: objrs::Alloc<MyCustomObject>) -> usize {}
//! }
//! ```
//!
//! # Syntax
//!
//! Building on [Rust's syntax and EBNF dialect](https://doc.rust-lang.org/grammar.html):
//!
//! ```text
//! objrs_protocol: '#' '[' "objrs" '(' "selector" '=' string_lit impl? type? optional? ','? ')' ']'
//! impl: ',' ["super" | "no_impl"]
//! type: ',' ["class" | "instance"]
//! optional: ',' "optional"
//! ```

extern crate proc_macro2;
extern crate syn;

use crate::parse::sel_ref_attr::validate_selector;
use proc_macro::Diagnostic;
use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};
use syn::{
  punctuated::Punctuated, spanned::Spanned, token::Comma, token::Default, ArgCaptured, Attribute,
  Block, FnArg, ImplItemMethod, LitStr, MethodSig, Pat, TraitItemMethod, Type, Visibility,
};

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

impl Parse for SelectorAttr {
  fn parse(input: ParseStream) -> syn::parse::Result<Self> {
    use crate::parse::attr::{class, instance, no_impl, optional, selector, KV};
    use syn::parenthesized;

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
    return Ok(SelectorAttr {
      sel: sel,
      call_super: call_super.is_some(),
      no_impl: no_impl.is_some(),
      optional: optional,
      method_type: method_type,
    });
  }
}

fn is_instance_method(args: &Punctuated<FnArg, Comma>) -> bool {
  if args.is_empty() {
    return false;
  }

  let is_self_ident = |arg: &ArgCaptured| {
    if let Pat::Ident(ref pat_ident) = arg.pat {
      return pat_ident.ident == "self";
    }
    return false;
  };

  match args[0] {
    FnArg::SelfRef(_) => return true,
    FnArg::SelfValue(_) => return true,
    FnArg::Captured(ref arg) => return is_self_ident(arg),
    _ => return false,
  }
}

fn is_impl_trait(arg: &FnArg) -> bool {
  match arg {
    FnArg::Captured(ArgCaptured {
      ty: Type::ImplTrait(_), ..
    }) => return true,
    _ => return false,
  }
}

pub enum ItemMethod {
  Impl(ImplItemMethod),
  Trait(TraitItemMethod),
}

impl ItemMethod {
  fn sig(&self) -> &MethodSig {
    match self {
      ItemMethod::Impl(method) => return &method.sig,
      ItemMethod::Trait(method) => return &method.sig,
    }
  }

  fn block(&self) -> Option<&Block> {
    match self {
      ItemMethod::Impl(ref method) => return Some(&method.block),
      ItemMethod::Trait(ref method) => return method.default.as_ref(),
    }
  }
}

pub struct Method {
  pub attr: SelectorAttr,
  pub method: ItemMethod,
  pub is_instance_method: bool,
  pub is_generic: bool,
}

impl Method {
  pub fn new(method: ItemMethod, attr: SelectorAttr) -> Result<Method, Diagnostic> {
    let is_protocol = if let ItemMethod::Trait(_) = method { true } else { false };

    let sig = method.sig();
    if let Some(constness) = sig.constness {
      return Err(constness.span.unstable().error("selector methods may not be `const`"));
    }
    if let Some(asyncness) = sig.asyncness {
      return Err(asyncness.span.unstable().error("selector methods may not be `async`"));
    }

    // TODO: do some basic validation (e.g., # of arguments is correct, etc.).
    let is_instance_method = attr.method_type == MethodType::Instance
      || (attr.method_type == MethodType::Auto && is_instance_method(&sig.decl.inputs));

    let (sel_string, expected_arg_count) = validate_selector(&attr.sel)?;
    let arg_count = sig.decl.inputs.len() - is_instance_method as usize;
    if arg_count != expected_arg_count {
      let error_msg = format!(
        "method `{}` has {} parameter{}{} but the selector has {}",
        sig.ident,
        arg_count,
        plural_s(arg_count),
        if is_instance_method { " (not counting self)" } else { "" },
        expected_arg_count
      );
      let note_msg = format!(
        "selector requires {} parameter{}",
        expected_arg_count,
        plural_s(expected_arg_count)
      );
      return Err(
        sig.span().unstable().error(error_msg).span_note(attr.sel.span().unstable(), note_msg),
      );
    }

    if is_protocol && method.block().is_some() {
      return Err(
        method
          .block()
          .span()
          .unstable()
          .error("expected `;`, found `{`")
          .note("objrs does not allow default trait implementations for protocols"),
      );
    }

    if is_protocol && is_instance_method && sel_string == "dealloc" {
      return Err(
        attr
          .sel
          .span()
          .unstable()
          .error("method must not use the selector \"dealloc\"")
          .note("objrs does not allow a protocol that defines a \"dealloc\" method"),
      );
    }

    let is_generic =
      !sig.decl.generics.params.is_empty() || sig.decl.inputs.iter().any(is_impl_trait);

    return Ok(Method {
      attr: attr,
      method: method,
      is_instance_method: is_instance_method,
      is_generic: is_generic,
    });
  }

  pub fn sig(&self) -> &MethodSig {
    return self.method.sig();
  }

  pub fn attrs(&self) -> &Vec<Attribute> {
    match self.method {
      ItemMethod::Impl(ref method) => return &method.attrs,
      ItemMethod::Trait(ref method) => return &method.attrs,
    }
  }

  pub fn vis(&self) -> Option<&Visibility> {
    match self.method {
      ItemMethod::Impl(ref method) => return Some(&method.vis),
      ItemMethod::Trait(_) => return None,
    }
  }

  pub fn defaultness(&self) -> Option<&Default> {
    match self.method {
      ItemMethod::Impl(ref method) => return method.defaultness.as_ref(),
      ItemMethod::Trait(_) => return None,
    }
  }

  pub fn block(&self) -> Option<&Block> {
    return self.method.block();
  }

  pub fn impl_method(&self) -> Option<&ImplItemMethod> {
    match self.method {
      ItemMethod::Impl(ref method) => return Some(method),
      _ => return None,
    }
  }

  pub fn into_trait_method(self) -> TraitItemMethod {
    match self.method {
      ItemMethod::Impl(method) => {
        return TraitItemMethod {
          attrs: method.attrs,
          sig: method.sig,
          default: Some(method.block),
          semi_token: None,
        };
      }
      ItemMethod::Trait(method) => return method,
    }
  }
}

// TODO: move this into somewhere else.
pub fn plural_s<T: From<u8> + core::cmp::PartialEq + Copy>(value: T) -> &'static str {
  if value == T::from(1u8) {
    return "";
  } else {
    return "s";
  }
}
