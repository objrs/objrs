// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

// TODO: remove this when the refactoring is done.
#![allow(unused)]

//! The `#[objrs(impl)]` macro parser module.
//!
//! Use `#[objrs(impl)]` on an `impl` item when implementing a class (e.g. `impl Class`) or when
//! implementing a protocol for a class (e.g. `impl Protocol for Class`).
//!
//! ```ignore
//! #[objrs(impl
//!         [, class_name = "ExportName",]
//!         [, extern]
//!         [, objrs = IDENT][,])]
//! impl Class {
//! }
//!
//! #[objrs(impl
//!         [, class_name = "ExportName",]
//!         [, protocol_name = "ExportName",]
//!         [, extern]
//!         [, objrs = IDENT][,])]
//! impl Protocol for Class {
//! }
//!
//! #[objrs(impl
//!         [, class_name = "ExportName",]
//!         [, category_name = "ExportName",]
//!         [, extern]
//!         [, objrs = IDENT][,])]
//! impl Class {
//! }
//! ```
//!
//! Parameters:
//!
//! - `class_name = "LITERAL_STR"`. Optional (default value: the class's name). The literal string
//!   is the class's name, as specified in the original #[objrs(class, ...)] attribute.
//! - `protocol_name = "LITERAL_STR"`. Optional (default value: the trait's name). The literal
//!   string is the trait's name, as specified in the original #[objrs(protocol, ...)] attribute.
//! - `category_name = "LITERAL_STR"`. Optional (default value: random UUID). The literal string
//!   is the category's name.
//! - `extern`. Optional. If this is omitted, objrs will treat the implementation as external only
//!   if a `#[link(...)]` attribute is present. An external implementation is one provided by an
//!   external framework. If you do not specify a `#[link(...)]` attribute, and if this
//!   implementation external, you must specify `extern` so objrs does not attempt to create a
//!   duplicate defnitions.
//! - `objrs = IDENT`. Optional (default value: `objrs`). The name of the objrs crate. The macro
//!   needs access to the objrs crate, and if you have renamed the crate, you must use this
//!   parameter to inform the macro of the crate's name.
//!
//! Example of implementing a method `hash` for the external `NSObject` class:
//!
//! ```ignore
//! #[objrs(impl)]
//! #[link(name = "Foundation", kind = "framework")]
//! impl NSObject {
//!   #[objrs(selector = "hash")]
//!   pub fn hash(&self) -> usize {}
//! }
//! ```
//!
//! Example of implementing a method for the custom `MyCustomObject` class:
//!
//!```ignore
//! #[objrs(impl)]
//! impl MyCustomObject {
//!   #[objrs(selector = "foo")]
//!   pub fn foo(&mut self) {
//!     self.ivar1 = 42;
//!     self.ivar2 = "Hello, world!".to_string();
//!   }
//! }
//! ```
//!
//! Example of implementing the `NSObject` protocol for the custom `MyCustomObject` class:
//!
//! ```ignore
//! #[objrs(impl)]
//! impl NSObject for MyCustomObject {
//!   #[objrs(selector = "hash")]
//!   fn hash(&self) -> usize {
//!     return self.ivar1;
//!   }
//! }
//! ```
//!
//! # Syntax
//!
//! Building on [Rust's syntax and EBNF dialect](https://doc.rust-lang.org/grammar.html):
//!
//! ```text
//! objrs_impl: '#' '[' "objrs" '(' "impl" names extern? objrs? ','? ')' ']'
//! names: class_name? [protocol_name | category_name]?
//! class_name: ',' "class_name" '=' string_lit
//! protocol_name: ',' "protocol_name" '=' string_lit
//! category_name: ',' "category_name" '=' string_lit
//! extern: ',' "extern"
//! objrs: ',' "objrs" '=' ident
//! ```

use crate::parse::attr::take_objrs_attr;
use crate::parse::selector_attr::{ItemMethod, Method, SelectorAttr};
use crate::parse::util::objrs_root;
use crate::util::link_attribute;
use proc_macro::Diagnostic;
use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{parse2, Ident, ImplItem, ItemImpl, LitStr, Type};

pub struct ImplAttr {
  pub class_name: Option<LitStr>,
  pub trait_name: Option<LitStr>,
  pub force_extern: bool,
  pub objrs: Option<Ident>,
}

// #[objrs(impl
//         [, class_name = "ExportName",]
//         [, extern]
//         [, objrs = Ident][,])]
// #[objrs(impl
//         [, class_name = "ExportName",]
//         [, protocol_name = "ExportName",]
//         [, extern]
//         [, objrs = Ident][,])]
// #[objrs(impl
//         [, class_name = "ExportName",]
//         [, category_name = "ExportName",]
//         [, extern]
//         [, objrs = Ident][,])]
impl Parse for ImplAttr {
  fn parse(input: ParseStream) -> syn::parse::Result<Self> {
    use crate::parse::attr::{category_name, class_name, objrs, protocol_name, KV};
    use syn::token::{Extern, Impl};

    let mut kv = KV::new(input);
    kv.parse::<Impl, _>()?;
    let class_name: Option<LitStr> = kv.parse::<class_name, _>()?;
    let trait_name: Option<LitStr> = kv.parse_one_of::<(protocol_name, category_name), _>()?;
    let force_extern: Option<()> = kv.parse::<Extern, _>()?;
    let objrs: Option<Ident> = kv.parse::<objrs, _>()?;
    kv.eof()?;
    return Ok(ImplAttr {
      class_name: class_name,
      trait_name: trait_name,
      force_extern: force_extern.is_some(),
      objrs: objrs,
    });
  }
}

fn parse_class_name(ty: &Type) -> Result<LitStr, Diagnostic> {
  let last_segment = match ty {
    Type::Slice(_) => Err("slice type"),
    Type::Array(_) => Err("array type"),
    Type::Ptr(_) => Err("pointer type"),
    Type::Reference(_) => Err("reference type"),
    Type::BareFn(_) => Err("fn type"),
    Type::Never(_) => Err("never type"),
    Type::Tuple(_) => Err("tuple type"),
    Type::Path(path) => Ok(path.path.segments.last()),
    Type::TraitObject(_) => Err("trait object type"),
    Type::ImplTrait(_) => Err("`impl Trait` type"),
    Type::Paren(inner) => return parse_class_name(inner.elem.as_ref()),
    Type::Group(inner) => return parse_class_name(inner.elem.as_ref()),
    Type::Infer(_) => Err("inferred type"),
    Type::Macro(_) => Err("macro"),
    Type::Verbatim(_) => Err("unknown type"),
  };
  let error_prefix = "expected path type, found ";
  let note = "the #[objrs(impl)] macro may only be applied to impl blocks for path types (e.g., \
              `foo::bar::Baz`)";
  match last_segment {
    Ok(Some(pair)) => {
      let ident = &pair.value().ident;
      return Ok(LitStr::new(&ident.to_string(), ident.span()));
    }
    Ok(None) => {
      return Err(ty.span().unstable().error("expected identifer at end of type path").note(note));
    }
    Err(msg) => return Err(ty.span().unstable().error([error_prefix, msg].concat()).note(note)),
  }
}

fn validate_selector_attr(attr: &SelectorAttr) -> Result<(), Diagnostic> {
  if let Some(optional) = attr.optional {
    return Err(
      optional
        .unstable()
        .error("`optional` mut not be used outside of a protocol trait definition")
        .note("objrs only allows `optional` on methods inside of #[objrs(protocol)] items"),
    );
  }

  return Ok(());
}

fn validate_method(method: &Method, force_extern: bool) -> Result<(), Diagnostic> {
  if !method.attr.no_impl && !force_extern {
    return Ok(());
  }

  if let Some(block) = method.block() {
    if !block.stmts.is_empty() {
      return Err(
        block
          .span()
          .unstable()
          .error("expected empty block on external Objective-C method, found non-empty block")
          .note("objrs will replace the method's block with its own implementation"),
      );
    }
  }

  return Ok(());
}

pub struct ClassImpl {
  pub force_extern: bool,
  pub objrs: Ident,
  pub class_name: LitStr,
  pub trait_name: Option<LitStr>,
  // pub link_attribute: Option<Attribute>,
  pub item: ItemImpl,
  pub class_methods: Vec<Method>,
  pub instance_methods: Vec<Method>,
}

impl ClassImpl {
  pub fn new(input: TokenStream, attr: ImplAttr) -> Result<ClassImpl, Diagnostic> {
    let mut item;
    match parse2::<ItemImpl>(input) {
      Ok(value) => item = value,
      Err(error) => {
        return Err(
          error
            .span()
            .unstable()
            .error(format!("failed to parse impl item: {}", error.to_string()))
            .note("#[objrs(impl)] may only be applied to a struct's impl block"),
        );
      }
    };

    let class_name;
    if let Some(name) = attr.class_name {
      class_name = name;
    } else {
      class_name = parse_class_name(&item.self_ty)?;
    }

    let trait_name = attr.trait_name.or_else(|| {
      item.trait_.as_ref().and_then(|(_, path, _)| {
        let ident = &path.segments.last()?.value().ident;
        return Some(LitStr::new(&ident.to_string(), ident.span()));
      })
    });

    let mut class_methods = Vec::new();
    let mut instance_methods = Vec::new();
    let mut non_methods = Vec::new();

    for sub_item in item.items {
      match sub_item {
        ImplItem::Method(mut method) => match take_objrs_attr(&mut method.attrs)? {
          Some(objrs_attr) => {
            let selector_attr =
              parse2(objrs_attr.tts).map_err(|e| e.span().unstable().error(e.to_string()))?;
            validate_selector_attr(&selector_attr)?;
            if let Some(ref default) = method.defaultness {
              return Err(default.span.unstable().error("selector methods may not be `default`"));
            }
            let objrs_method = Method::new(ItemMethod::Impl(method), selector_attr)?;
            validate_method(&objrs_method, attr.force_extern)?;
            if objrs_method.is_instance_method {
              instance_methods.push(objrs_method);
            } else {
              class_methods.push(objrs_method);
            }
          }
          None => {
            non_methods.push(ImplItem::Method(method));
          }
        },
        _ => non_methods.push(sub_item),
      }
    }

    item.items = non_methods;

    // TODO: is the link attribute really needed? It'll be present in the file. I'm not sure if it's really necessary to duplicate it in multiple places.
    // let link_attribute = link_attribute(&item.attrs).cloned();

    return Ok(ClassImpl {
      force_extern: attr.force_extern || link_attribute(&item.attrs).is_some(),
      objrs: objrs_root(attr.objrs),
      class_name: class_name,
      trait_name: trait_name,
      // link_attribute: link_attribute,
      item: item,
      class_methods: class_methods,
      instance_methods: instance_methods,
    });
  }
}
