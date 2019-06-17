// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

//! The `#[objrs(ivar)]` macro parser module.
//!
//! It's unlikely you'll need to use `#[objrs(ivar)]`. It can usually be omitted when declaring
//! instance variables in a class. But if you need it, it's available, and can be applied to
//! individual instance variables within a #[objrs(class)] item.
//!
//! ```ignore
//! #[objrs(class)]
//! struct SomeClass {
//!   #[objrs(ivar
//!           [,name = "ExportName"]
//!           [,default = EXPR][,])]
//!   ivar_name: type,
//! }
//! ```
//!
//! Parameters:
//!
//! - `name = "LITERAL_STR"`. Optional (defalt value: the field's name). This will be the name of
//!   the instance variable that the Objective-C runtime sees.
//! - `default = EXPR`. Optional. This is the initial value of the instance variable when the class
//!   is first allocated. If this is omitted and the type implements `objrs::marker::Zeroed`, the
//!   initial value of the instance variable will be zero. Otherwise, the initial value of the
//!   instance variable will be `Default::default()`.
//!
//! One major limitation of instance variables (regardless of whether `#[objrs(ivar)]` is used or
//! not) is that they can only be accessed via `self.`. Attempting to access an instance variable
//! from a non-self identifier will not work (e.g. `self.ivar` will work,
//! `some_other_instance_with_the_same_type_as_self.ivar` will not work). This is because objrs has
//! to rewrite your method such that instance variables are properly dereferenced. objrs knows that
//! `self` is a class (from which it can extract all the necessary information), but objrs does not
//! know if some arbitrary identifer is a class (or how to extract the necessary information).
//!
//! Another limitation is that function fields cannot be invoked directly through self without using
//! some extra syntax. For example, if a struct has a field named `do_something` of type `fn ()`,
//! you cannot access and invoke this function by doing `self.do_something()`. This is because objrs
//! must load ivars dynamically, as Objective-C does, but there is ambiguity in the syntax
//! `self.do_something()`: is it a method call (equivalent to `Self::do_something(self);`), or is it
//! a field access (equivalent to `let function = self.do_something; function();`)? Because objrs
//! cannot resolve this ambiguity, it assumes this syntax is a normal method call, not a field
//! access, and thus will not dynamically load the ivar. If you need to invoke a function ivar,
//! either wrap the field access in parentheses (e.g., `(self.do_something)()`), or assign it to a
//! local variable and use the local variable to invoke the function.
//!
//! # Syntax
//!
//! Building on [Rust's syntax and EBNF dialect](https://doc.rust-lang.org/grammar.html):
//!
//! ```text
//! objrs_impl: '#' '[' "objrs" '(' "ivar" name? default? ','? ')' ']'
//! name: ',' "name" '=' string_lit
//! default: ',' "default" '=' expr
//! ```

use syn::parse::{Parse, ParseStream};
use syn::{Expr, LitStr};

#[derive(Default)]
pub struct IvarAttr {
  pub name: Option<LitStr>,
  pub default: Option<Expr>,
}

impl Parse for IvarAttr {
  fn parse(input: ParseStream) -> syn::parse::Result<Self> {
    use crate::parse::attr::{default, ivar, name, KV};

    let mut kv = KV::new(input);
    kv.parse::<ivar, _>()?;
    let name: Option<LitStr> = kv.parse::<name, _>()?;
    let default: Option<Expr> = kv.parse::<default, _>()?;
    kv.eof()?;
    return Ok(IvarAttr {
      name: name,
      default: default,
    });
  }
}
