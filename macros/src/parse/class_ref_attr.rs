// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

extern crate proc_macro2;
extern crate syn;

use proc_macro::Diagnostic;
use proc_macro2::TokenStream;
use syn::{parse2, GenericArgument, Ident, ItemStatic, LitStr, PathArguments, Type};

pub struct ClassRef {
  pub class: LitStr,
  pub ident: Ident,
  pub sync_hack_ty: Box<Type>,
  pub class_ty: Box<Type>,
}

impl ClassRef {
  pub fn new(args: TokenStream, input: TokenStream) -> Result<ClassRef, (Diagnostic, TokenStream)> {
    let class;
    match parse2::<LitStr>(args) {
      Ok(value) => {
        class = value;
      }
      Err(err) => {
        let diagnostic = err.span().unstable().error(err.to_string()).note(
          "class! requires a single string literal. Examples: class!(\"NSObject\"); \
           class!(\"NSArray\"); class!(r#\"NSMutableDictionary\"#);",
        );
        return Err((diagnostic, input));
      }
    }

    let mut item = parse2::<ItemStatic>(input).unwrap();

    let ty_path = if let Type::Path(ref mut ty_path) = *item.ty { Some(ty_path) } else { None };
    let ty_path = ty_path.unwrap();
    let last = if let Some(pair) = ty_path.path.segments.last_mut() { Some(pair) } else { None };
    let mut last = last.unwrap();
    let args = core::mem::replace(&mut last.value_mut().arguments, PathArguments::None);
    let args = if let PathArguments::AngleBracketed(args) = args { Some(args) } else { None };
    let args = args.unwrap();
    let first = args.args.into_iter().next().unwrap();
    let ref_ty = if let GenericArgument::Type(ref_ty) = first { Some(ref_ty) } else { None };
    let ref_ty = if let Some(Type::Reference(ref_ty)) = ref_ty { Some(ref_ty) } else { None };
    let ref_ty = ref_ty.unwrap();
    let class_ty = ref_ty.elem;

    return Ok(ClassRef {
      class: class,
      ident: item.ident,
      sync_hack_ty: item.ty,
      class_ty: class_ty,
    });
  }
}
