// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

//! The `#[objrs(protocol)]` macro parser module.
//!
//! Objective-C protocols are implemented in objrs as traits. Use `#[objrs(protocol)]` on a Rust
//! trait to declare it as an Objective-C protocol.
//!
//! **⚠ It is undefined behavior for a Rust protocol trait to deviate from its Objective-C
//! implementation.** Objective-C implements protocols using weak symbol names. They are
//! instantiated in every translation unit in which they are used, and the linker deduplicates the
//! symbols during the linking phase (much like how templates work in C++). If your Rust protocol
//! trait does not *exactly* match the Objective-C implementation for that protocol, it is an
//! [ODR](https://en.wikipedia.org/wiki/One_Definition_Rule) violation. This caveat does not apply
//! to custom protocols (that aren't defined in an external framework or library), as the only
//! definition that exists for these custom protocols is in the Rust code. If you are creating a
//! Rust protocol trait for an external protocol, it is highly recommended that you use an automated
//! tool (like [bindgen](https://github.com/rust-lang/rust-bindgen)) to generate the Rust code
//! directly from the Objective-C header file. This way you can be confident the cross-language
//! definitions are consistent.
//!
//! ```ignore
//! #[objrs(protocol
//!         [, name = "name"]
//!         [, property(...)]
//!         [, objrs = IDENT]
//!         [,])]
//! trait ThisTraitIsAProtocol {
//! }
//! ```
//!
//! Parameters:
//!
//! - `name = "LITERAL_STR"`. Optional (default value: the trait's name). The literal string is the
//!   protocol's name (i.e. what
//!   [`NSStringFromProtocol`](https://developer.apple.com/documentation/foundation/1395298-nsstringfromprotocol?language=objc)
//!   would return). If this is omitted, the protocol name will be the same as the trait's
//!   identifier (e.g. `trait NSCopying` → `"NSCopying"`). It is suggested you omit this and just
//!   give the trait the same name as the protocol it represents. If you use this, additional
//!   parameters must be passed to `#[objrs(...)]` attributes for implementations.
//! - `property(...)`. Optional (repeated).
//! - `objrs = IDENT`. Optional (default value: `objrs`). The name of the objrs crate. The macro
//!   needs access to the objrs crate, and if you have renamed the crate, you must use this
//!   parameter to inform the macro of the crate's name.
//!
//! # Syntax
//!
//! Building on [Rust's syntax and EBNF dialect](https://doc.rust-lang.org/grammar.html):
//!
//! ```text
//! objrs_protocol: '#' '[' "objrs" '(' "protocol" name? property* objrs? ','? ')' ']'
//! name: ',' "name" '=' string_lit
//! property: ',' "property" '(' ident ':' type attributes ','? ')'
//! objrs: ',' "objrs" '=' ident
//!
//! attributes: class? read_write? xor_attributes? atomicity? null_resettable? getter? setter?
//! xor_attributes: [copy_strong nullability?] | nullability | [weak restricted_nullability?]
//! class: ',' "class"
//! read_write: ',' ["readonly" | "readwrite"]
//! assign: ',' "assign"
//! unsafe_unretained: ',' "unsafe_unretained"
//! copy: ',' "copy"
//! weak: ',' "weak"
//! strong: ',' "strong"
//! retain: ',' "retain"
//! nullability: ',' ["nullable" | "nonnull" | "null_unspecified"]
//! restricted_nullability: ',' ["nullable" | "null_unspecified"]
//! copy_strong: assign unsafe_unretained? | unsafe_unretained | copy | strong retain? | retain
//! atomicity: ',' ["atomic" | "nonatomic"]
//! null_resettable: ',' "null_resettable"
//! getter: ',' "getter" '=' string_lit
//! setter: ',' "setter" '=' string_lit
//! ```

// For Objective-C property attributes, see https://github.com/llvm-mirror/clang/blob/master/lib/Parse/ParseObjc.cpp

use crate::parse::attr::take_objrs_attr;
use crate::parse::property_attr::PropertyAttr;
use crate::parse::selector_attr::{ItemMethod, Method};
use crate::parse::util::objrs_root;
use proc_macro::Diagnostic;
use proc_macro2::TokenStream;
use std::collections::HashSet;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{parse2, Ident, ItemTrait, LitStr, TraitItem, TraitItemMethod};

pub struct ProtocolAttr {
  pub name: Option<LitStr>,
  pub properties: Vec<PropertyAttr>,
  pub objrs: Option<Ident>,
}

impl Parse for ProtocolAttr {
  fn parse(input: ParseStream) -> syn::parse::Result<Self> {
    use crate::parse::attr::{name, objrs, property, protocol, KV};

    let mut kv = KV::new(input);
    kv.parse::<protocol, _>()?;
    let name: Option<LitStr> = kv.parse::<name, _>()?;
    let mut properties = vec![];
    while let Some(property) = kv.parse::<property, _>()? {
      properties.push(property);
    }
    let objrs: Option<Ident> = kv.parse::<objrs, _>()?;
    kv.eof()?;
    return Ok(ProtocolAttr {
      name: name,
      properties: properties,
      objrs: objrs,
    });
  }
}

fn parse_method(mut method: TraitItemMethod) -> Result<Method, Diagnostic> {
  if let Some(ref default) = method.default {
    return Err(
      default
        .span()
        .unstable()
        .error("expected `;`, found `{`")
        .note("objrs does not allow default trait implementations for protocols"),
    );
  }

  match take_objrs_attr(&mut method.attrs)? {
    Some(objrs_attr) => {
      let selector_attr =
        parse2(objrs_attr.tts).map_err(|e| e.span().unstable().error(e.to_string()))?;
      let objrs_method = Method::new(ItemMethod::Trait(method), selector_attr)?;

      if objrs_method.is_instance_method && objrs_method.attr.sel.value() == "dealloc" {
        return Err(
          objrs_method
            .attr
            .sel
            .span()
            .unstable()
            .error("method must not use the selector \"dealloc\"")
            .note("objrs does not allow a protocol that defines a \"dealloc\" method"),
        );
      }

      return Ok(objrs_method);
    }
    None => {
      return Err(
        method
          .sig
          .span()
          .unstable()
          .error("function is missing #[objrs(selector ...)] attribute")
          .note("objrs requires all trait methods in a protocol to specify a selector"),
      );
    }
  }
}

pub struct Protocol {
  pub objrs: Ident,
  pub trait_name: Option<LitStr>,
  pub item: ItemTrait,
  pub class_methods: Vec<Method>,
  pub instance_methods: Vec<Method>,
}

impl Protocol {
  pub fn new(input: TokenStream, attr: ProtocolAttr) -> Result<Protocol, Diagnostic> {
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

    // TODO: support custom protocols. Currently only extern protocols are supported.

    let mut expected_class_properties = HashSet::new();
    let mut expected_instance_properties = HashSet::new();
    for property in attr.properties.iter() {
      let expected_properties;
      if property.class.is_some() {
        expected_properties = &mut expected_class_properties;
      } else {
        expected_properties = &mut expected_instance_properties;
      }
      expected_properties.insert(property.getter());
      if let Some(setter) = property.setter() {
        expected_properties.insert(setter);
      }
    }

    let mut class_methods = Vec::new();
    let mut instance_methods = Vec::new();
    let mut non_methods = Vec::new();
    for sub_item in item.items {
      match sub_item {
        TraitItem::Method(method) => {
          let method = parse_method(method)?;
          if method.is_instance_method {
            expected_instance_properties.remove(&method.attr.sel);
            instance_methods.push(method);
          } else {
            expected_class_properties.remove(&method.attr.sel);
            class_methods.push(method);
          }
        }
        non_method => non_methods.push(non_method),
      }
    }
    item.items = non_methods;

    for selector in expected_class_properties.iter().chain(expected_instance_properties.iter()) {
      return Err(
        selector
          .span()
          .unstable()
          .error(format!(
            "selector \"{}\" is missing from the trait's method list",
            selector.value()
          ))
          .note(format!(
            "objrs requires a method in the trait to have an #[objrs(selector = \"{}\")] attribute",
            selector.value()
          )),
      );
    }

    return Ok(Protocol {
      objrs: objrs_root(attr.objrs),
      trait_name: attr.name,
      item: item,
      class_methods: class_methods,
      instance_methods: instance_methods,
    });
  }
}
