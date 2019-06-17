// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

//! The `#[objrs(class)]` macro parser module.
//!
//! Apply the `#[objrs(class)]` attribute to a `struct` item to declare it as an Objective-C class.
//! Instance variables are supported with the limitation that they must implement Rust's `Default`
//! trait.
//!
//! ```ignore
//! #[objrs(class,
//!         [name = "ExportName",]
//!         root_class
//!         [, extern]
//!         [, objrs = IDENT][,])]
//! struct Foo;
//!
//! #[objrs(class,
//!         [name = "ExportName",]
//!         super = IDENT
//!         [, root_class = "ExportName"]
//!         [, extern]
//!         [, objrs = IDENT][,])]
//! struct Bar;
//!
//! #[objrs(class,
//!         [name = "ExportName",]
//!         super(name = "ExportName",
//!               type = IDENT[,])
//!         [, root_class = "ExportName"]
//!         [, extern]
//!         [, objrs = IDENT][,])]
//! struct Baz;
//! ```
//!
//! Parameters:
//!
//! - `name = "LITERAL_STR"`. Optional (default value: the type's name). The literal string is the
//!   class's name (i.e. what
//!   [`NSStringFromClass`](https://developer.apple.com/documentation/foundation/1395143-nsstringfromclass?language=objc)
//!   would return). If this is omitted, the class name will be the same as the struct's identifier
//!   (e.g. `struct NSObject` → `"NSObject"`; `struct NSArray<T>` → `"NSArray"`). It is suggested
//!   you omit this and just give the struct the same name as the class it represents. If you use
//!   this, additional parameters must be passed to `#[objrs(...)]` attributes for subclasses.
//! - `root_class`. Required (unless `super = ...` is used). One of `root_class` or `super = ...`
//!   must be specified. In practice this is only useful for `NSObject`.
//! - `super = TYPE`. Required (unless `root_class` is used). The class's super type.
//! - `extern`. Optional. If this is omitted, objrs will treat the class as an external class only
//!   if a `#[link(...)]` attribute is present. An external class is one provided by an external
//!   framework (e.g. `NSArray`, `UIApplication`, etc.). If you do not specify a `#[link(...)]`
//!   attribute, and if this class is an external class, you must specify `extern` so objrs does not
//!   attempt to create a duplicate class defnition.
//! - `objrs = IDENT`. Optional (default value: `objrs`). The name of the objrs crate. The macro
//!   needs access to the objrs crate, and if you have renamed the crate, you must use this
//!   parameter to inform the macro of the crate's name.
//!
//! Both of these definitions for `NSObject` are equivalent:
//!
//! ```ignore
//! #[objrs(class, root_class, extern)]
//! struct NSObject;
//!
//! #[objrs(class, root_class)]
//! #[link(name = "Foundation", kind = "framework")]
//! struct NSObject;
//! ```
//!
//! Both `NSString` and `NSArray` inherit from `NSObject` and are external classes:
//!
//! ```ignore
//! #[objrs(class, super = NSObject)]
//! #[link(name = "Foundation", kind = "framework")]
//! struct NSString;
//!
//! // Generics are supported, but they can only be Objective-C class types.
//! #[objrs(class, super = NSObject)]
//! #[link(name = "Foundation", kind = "framework")]
//! struct NSArray<T: objrs::marker::Class + ?Sized>;
//! ```
//!
//! Custom classes can be created by omitting `extern` and the `#[link(...)]` attribute. Instance
//! variables are supported, even if they have non-trivial constructors and destructors:
//!
//! ```ignore
//! #[objrs(class, super = NSObject)]
//! struct MyCustomObject {
//!   ivar1: usize,
//!   ivar2: String,
//! }
//! ```
//!
//! # Syntax
//!
//! Building on [Rust's syntax and EBNF dialect](https://doc.rust-lang.org/grammar.html):
//!
//! ```text
//! objrs_class: '#' '[' "objrs" '(' "class" name? root_or_super? extern? objrs? ','? ')' ']'
//! name: ',' "name" '=' string_lit
//! root_or_super: ',' ["root_class" | [super root_class?]]
//! super: 'super' [['=' type_path] | ['(' super_params ','? ')']]
//! super_params: "name" '=' string_lit ',' "type" '=' type_path
//! root_class: ',' "root_class" '=' string_lit
//! extern: ',' "extern"
//! objrs: ',' "objrs" '=' ident
//! ```

extern crate proc_macro2;
extern crate syn;

use crate::parse::attr::take_objrs_attr;
use crate::parse::ivar_attr::IvarAttr;
use crate::parse::util::objrs_root;
use crate::util::link_attribute;
use proc_macro::Diagnostic;
use proc_macro2::{Span, TokenStream};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{parse2, Field, Ident, ItemStruct, LitStr, TypePath};

pub struct ClassAttr {
  pub class_name: Option<LitStr>,
  pub super_class_name: Option<LitStr>, // Only needed due to the lack of associated extern statics.
  pub super_class: Option<TypePath>,
  pub force_extern: bool,
  pub root_class_name: Option<LitStr>, // Only needed due to the lack of associated extern statics.
  pub objrs: Option<Ident>,
}

struct SuperAttr {
  name: Option<LitStr>,
  class: TypePath,
}

impl crate::parse::attr::Value for SuperAttr {
  fn parse(input: ParseStream, _: Span) -> syn::parse::Result<Self>
  where
    Self: Sized,
  {
    use crate::parse::attr::{name, KV};
    use syn::parenthesized;
    use syn::token::{Eq, Paren, Type};

    let lookahead = input.lookahead1();
    if lookahead.peek(Eq) {
      let _: Eq = input.parse()?;
      let class: TypePath = input.parse()?;
      return Ok(SuperAttr {
        name: None,
        class: class,
      });
    } else if lookahead.peek(Paren) {
      let content;
      let _: Paren = parenthesized!(content in input);
      let input = &content;
      let mut kv = KV::new(input);
      let name: LitStr = kv.parse::<name, _>()?;
      let class: TypePath = kv.parse::<Type, _>()?;
      kv.eof()?;
      return Ok(SuperAttr {
        name: Some(name),
        class: class,
      });
    } else {
      return Err(lookahead.error());
    }
  }
}

impl Parse for ClassAttr {
  fn parse(input: ParseStream) -> syn::parse::Result<Self> {
    use crate::parse::attr::{class, name, objrs, root_class, KV};

    let mut kv = KV::new(input);
    kv.parse::<class, _>()?;
    let class_name: Option<LitStr> = kv.parse::<name, _>()?;
    let root_class: Option<()> = kv.parse::<root_class, _>()?;
    let super_class: Option<SuperAttr> =
      if root_class.is_some() { None } else { kv.parse::<syn::token::Super, _>()? };
    let (super_class_name, super_class) =
      super_class.map_or((None, None), |s| (s.name, Some(s.class)));
    kv.barrier()?;
    let root_class_name: Option<LitStr>;
    if root_class.is_some() {
      root_class_name = None;
    } else {
      root_class_name = kv.parse::<root_class, _>()?;
    }
    let force_extern: Option<()> = kv.parse::<syn::token::Extern, _>()?;
    let objrs: Option<Ident> = kv.parse::<objrs, _>()?;
    kv.eof()?;
    return Ok(ClassAttr {
      class_name: class_name,
      super_class_name: super_class_name,
      super_class: super_class,
      force_extern: force_extern.is_some(),
      root_class_name: root_class_name,
      objrs: objrs,
    });
  }
}

fn extract_ivar_attr(field: &mut Field, force_extern: bool) -> Result<IvarAttr, Diagnostic> {
  let ivar_attr: IvarAttr;
  match take_objrs_attr(&mut field.attrs)? {
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

pub struct Class {
  pub objrs: Ident,
  pub class_name: LitStr,
  pub super_class_name: Option<LitStr>, // TODO: maybe just make this a String...
  pub super_class: Option<TypePath>,
  pub force_extern: bool,
  pub root_class_name: LitStr, // TODO: maybe just make this a String...
  pub item: ItemStruct,
  pub ivar_attrs: Vec<IvarAttr>,
}

impl Class {
  pub fn new(input: TokenStream, attr: ClassAttr) -> Result<Class, Diagnostic> {
    let mut item;
    match parse2::<ItemStruct>(input) {
      Ok(value) => item = value,
      Err(error) => {
        return Err(
          error
            .span()
            .unstable()
            .error(format!("failed to parse struct: {}", error.to_string()))
            .note("#[objrs(class ...)] must only be applied to a struct item"),
        );
      }
    }

    let force_extern = attr.force_extern || link_attribute(&item.attrs).is_some();

    let class_name =
      attr.class_name.unwrap_or_else(|| LitStr::new(&item.ident.to_string(), item.ident.span()));

    let mut ivar_attrs = Vec::new();
    for field in item.fields.iter_mut() {
      ivar_attrs.push(extract_ivar_attr(field, force_extern)?);
    }

    let root_class_name;
    if let Some(ref name) = attr.root_class_name {
      root_class_name = name.clone();
    } else if attr.super_class.is_some() {
      root_class_name = LitStr::new("NSObject", Span::call_site());
    } else {
      root_class_name = LitStr::new(&item.ident.to_string(), item.ident.span());
    }

    let super_class_name;
    if attr.super_class.is_some() && attr.super_class_name.is_none() {
      let ident = &attr.super_class.as_ref().unwrap().path.segments.last().unwrap().value().ident;
      super_class_name = Some(LitStr::new(&ident.to_string(), ident.span()));
    } else {
      super_class_name = attr.super_class_name;
    }

    return Ok(Class {
      objrs: objrs_root(attr.objrs),
      class_name: class_name,
      super_class_name: super_class_name,
      super_class: attr.super_class,
      force_extern: force_extern,
      root_class_name: root_class_name,
      item: item,
      ivar_attrs: ivar_attrs,
    });
  }
}
