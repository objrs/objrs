// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

extern crate core;

use crate::util::{priv_ident, priv_ident_at};
use proc_macro::Diagnostic;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::collections::HashSet;
use syn::{
  spanned::Spanned, visit_mut::visit_block_mut, visit_mut::visit_expr_mut, visit_mut::VisitMut,
  Abi, AngleBracketedGenericArguments, ArgCaptured, ArgSelf, ArgSelfRef, AttrStyle, Attribute,
  BareFnArg, BareFnArgName, BinOp, Binding, Block, BoundLifetimes, ConstParam, Data, DataEnum,
  DataStruct, DataUnion, DeriveInput, Expr, ExprBlock, ExprVerbatim, Field, Fields, FieldsNamed,
  FieldsUnnamed, File, FnArg, FnDecl, ForeignItem, ForeignItemFn, ForeignItemStatic,
  ForeignItemType, ForeignItemVerbatim, GenericArgument, GenericMethodArgument, GenericParam,
  Generics, Ident, ImplItem, ImplItemConst, ImplItemMacro, ImplItemMethod, ImplItemType,
  ImplItemVerbatim, Index, Item, ItemConst, ItemEnum, ItemExternCrate, ItemFn, ItemForeignMod,
  ItemImpl, ItemMacro, ItemMacro2, ItemMod, ItemStatic, ItemStruct, ItemTrait, ItemType, ItemUnion,
  ItemUse, ItemVerbatim, Label, Lit, LitBool, LitByte, LitByteStr, LitChar, LitFloat, LitInt,
  LitStr, LitVerbatim, MacroDelimiter, Member, Meta, MetaList, MetaNameValue, MethodSig,
  MethodTurbofish, NestedMeta, ParenthesizedGenericArguments, Pat, PatBox, PatIdent, PatLit,
  PatMacro, PatPath, PatRange, PatRef, PatSlice, PatStruct, PatTuple, PatTupleStruct, PatVerbatim,
  PatWild, PredicateEq, PredicateLifetime, PredicateType, QSelf, RangeLimits, ReturnType, Stmt,
  TraitBound, TraitBoundModifier, TraitItem, TraitItemConst, TraitItemMacro, TraitItemMethod,
  TraitItemType, TraitItemVerbatim, Type, TypeArray, TypeBareFn, TypeGroup, TypeImplTrait,
  TypeInfer, TypeMacro, TypeNever, TypeParam, TypeParamBound, TypeParen, TypePath, TypePtr,
  TypeReference, TypeSlice, TypeTraitObject, TypeTuple, TypeVerbatim, UnOp, UseGlob, UseGroup,
  UseName, UsePath, UseRename, UseTree, Variant, VisCrate, VisPublic, VisRestricted, Visibility,
  WhereClause, WherePredicate,
};

#[derive(Hash, Eq, PartialEq)]
enum LiteMember {
  Named(String),
  Unnamed(u32),
}

impl LiteMember {
  fn new(member: &Member) -> LiteMember {
    match member {
      Member::Named(ident) => return LiteMember::Named(ident.to_string()),
      Member::Unnamed(index) => return LiteMember::Unnamed(index.index),
    }
  }
}

struct MethodVisitor<'a> {
  self_name: String,
  self_arg: Ident,
  priv_name: String,
  objrs_root: &'a Ident,
  as_fn: TokenStream,
  take_ref: TokenStream,
  // Rust's HashMap is very innefficient for String keys (because all the methods take an owned key
  // rather than a borrowed key, which means you have to copy the string all the time). But the syn
  // and proc_macro types call to_string() all the time internally, so we can't really avoid the
  // extra string allocations. Might as well put them to use.
  ivar_refs: HashSet<LiteMember>,
  load_ivars: TokenStream,
  modified: bool,
}

impl<'a> MethodVisitor<'a> {
  fn new(as_ref: bool, self_arg: Ident, objrs_root: &'a Ident, priv_name: String) -> MethodVisitor {
    return MethodVisitor {
      self_name: self_arg.to_string(),
      self_arg: self_arg,
      priv_name: priv_name,
      objrs_root: objrs_root,
      as_fn: if as_ref { quote!(as_ref) } else { quote!(as_mut) },
      take_ref: if as_ref { quote!(&) } else { quote!(&mut) },
      ivar_refs: HashSet::new(),
      load_ivars: TokenStream::new(),
      modified: false,
    };
  }

  fn expr_is_self(&self, expr: &Expr) -> bool {
    if let Expr::Path(expr_path) = expr {
      let path = &expr_path.path;
      return path.segments.len() == 1 && path.segments[0].ident == self.self_name;
    }
    return false;
  }
}

impl<'a> VisitMut for MethodVisitor<'a> {
  fn visit_expr_mut(&mut self, expr: &mut Expr) {
    let new_expr;
    if self.expr_is_self(expr) {
      let objrs_root = self.objrs_root;
      let span = expr.span();
      let objrs_self = priv_ident_at(&self.priv_name, span);
      let as_fn = &self.as_fn;
      new_expr = Some(Expr::Verbatim(ExprVerbatim {
        tts: quote!(#objrs_root::__objrs::ThisAndFields::#as_fn(#objrs_self)),
      }));
    } else if let Expr::Field(ref mut expr_field) = expr {
      if self.expr_is_self(&expr_field.base) {
        let objrs_root = self.objrs_root;
        let objrs_self = priv_ident_at(&self.priv_name, expr_field.base.span());
        let fields = priv_ident_at("fields", expr_field.base.span());
        let dot = &expr_field.dot_token;
        let member = &expr_field.member;
        let as_fn = &self.as_fn;
        let take_ref = &self.take_ref;
        new_expr = Some(Expr::Verbatim(ExprVerbatim {
          tts: quote!((*#objrs_root::__objrs::Field::#as_fn(#take_ref #objrs_self #dot #fields #dot #member))),
        }));
        let self_arg = &self.self_arg;
        if self.ivar_refs.insert(LiteMember::new(member)) {
          self.load_ivars.extend(quote!{
            unsafe {
              #objrs_root::__objrs::Field::load(&mut #objrs_self #dot #fields #dot #member, #objrs_root::__objrs::core::mem::transmute(&*#self_arg));
            }
          });
        }
      } else {
        new_expr = None;
      }
    } else {
      new_expr = None;
    }

    if let Some(new_expr) = new_expr {
      *expr = new_expr;
      self.modified = true;
      return;
    }

    visit_expr_mut(self, expr);
  }

  fn visit_abi_mut(&mut self, _: &mut Abi) {}
  fn visit_angle_bracketed_generic_arguments_mut(
    &mut self,
    _: &mut AngleBracketedGenericArguments,
  ) {
  }
  fn visit_arg_captured_mut(&mut self, _: &mut ArgCaptured) {}
  fn visit_arg_self_mut(&mut self, _: &mut ArgSelf) {}
  fn visit_arg_self_ref_mut(&mut self, _: &mut ArgSelfRef) {}
  fn visit_attr_style_mut(&mut self, _: &mut AttrStyle) {}
  fn visit_attribute_mut(&mut self, _: &mut Attribute) {}
  fn visit_bare_fn_arg_mut(&mut self, _: &mut BareFnArg) {}
  fn visit_bare_fn_arg_name_mut(&mut self, _: &mut BareFnArgName) {}
  fn visit_bin_op_mut(&mut self, _: &mut BinOp) {}
  fn visit_binding_mut(&mut self, _: &mut Binding) {}
  fn visit_bound_lifetimes_mut(&mut self, _: &mut BoundLifetimes) {}
  fn visit_const_param_mut(&mut self, _: &mut ConstParam) {}
  fn visit_data_mut(&mut self, _: &mut Data) {}
  fn visit_data_enum_mut(&mut self, _: &mut DataEnum) {}
  fn visit_data_struct_mut(&mut self, _: &mut DataStruct) {}
  fn visit_data_union_mut(&mut self, _: &mut DataUnion) {}
  fn visit_derive_input_mut(&mut self, _: &mut DeriveInput) {}
  fn visit_field_mut(&mut self, _: &mut Field) {}
  fn visit_fields_mut(&mut self, _: &mut Fields) {}
  fn visit_fields_named_mut(&mut self, _: &mut FieldsNamed) {}
  fn visit_fields_unnamed_mut(&mut self, _: &mut FieldsUnnamed) {}
  fn visit_file_mut(&mut self, _: &mut File) {}
  fn visit_fn_arg_mut(&mut self, _: &mut FnArg) {}
  fn visit_fn_decl_mut(&mut self, _: &mut FnDecl) {}
  fn visit_foreign_item_mut(&mut self, _: &mut ForeignItem) {}
  fn visit_foreign_item_fn_mut(&mut self, _: &mut ForeignItemFn) {}
  fn visit_foreign_item_static_mut(&mut self, _: &mut ForeignItemStatic) {}
  fn visit_foreign_item_type_mut(&mut self, _: &mut ForeignItemType) {}
  fn visit_foreign_item_verbatim_mut(&mut self, _: &mut ForeignItemVerbatim) {}
  fn visit_generic_argument_mut(&mut self, _: &mut GenericArgument) {}
  fn visit_generic_method_argument_mut(&mut self, _: &mut GenericMethodArgument) {}
  fn visit_generic_param_mut(&mut self, _: &mut GenericParam) {}
  fn visit_generics_mut(&mut self, _: &mut Generics) {}
  fn visit_impl_item_mut(&mut self, _: &mut ImplItem) {}
  fn visit_impl_item_const_mut(&mut self, _: &mut ImplItemConst) {}
  fn visit_impl_item_macro_mut(&mut self, _: &mut ImplItemMacro) {}
  fn visit_impl_item_method_mut(&mut self, _: &mut ImplItemMethod) {}
  fn visit_impl_item_type_mut(&mut self, _: &mut ImplItemType) {}
  fn visit_impl_item_verbatim_mut(&mut self, _: &mut ImplItemVerbatim) {}
  fn visit_index_mut(&mut self, _: &mut Index) {}
  fn visit_item_mut(&mut self, _: &mut Item) {}
  fn visit_item_const_mut(&mut self, _: &mut ItemConst) {}
  fn visit_item_enum_mut(&mut self, _: &mut ItemEnum) {}
  fn visit_item_extern_crate_mut(&mut self, _: &mut ItemExternCrate) {}
  fn visit_item_fn_mut(&mut self, _: &mut ItemFn) {}
  fn visit_item_foreign_mod_mut(&mut self, _: &mut ItemForeignMod) {}
  fn visit_item_impl_mut(&mut self, _: &mut ItemImpl) {}
  fn visit_item_macro_mut(&mut self, _: &mut ItemMacro) {}
  fn visit_item_macro2_mut(&mut self, _: &mut ItemMacro2) {}
  fn visit_item_mod_mut(&mut self, _: &mut ItemMod) {}
  fn visit_item_static_mut(&mut self, _: &mut ItemStatic) {}
  fn visit_item_struct_mut(&mut self, _: &mut ItemStruct) {}
  fn visit_item_trait_mut(&mut self, _: &mut ItemTrait) {}
  fn visit_item_type_mut(&mut self, _: &mut ItemType) {}
  fn visit_item_union_mut(&mut self, _: &mut ItemUnion) {}
  fn visit_item_use_mut(&mut self, _: &mut ItemUse) {}
  fn visit_item_verbatim_mut(&mut self, _: &mut ItemVerbatim) {}
  fn visit_label_mut(&mut self, _: &mut Label) {}
  fn visit_lit_mut(&mut self, _: &mut Lit) {}
  fn visit_lit_bool_mut(&mut self, _: &mut LitBool) {}
  fn visit_lit_byte_mut(&mut self, _: &mut LitByte) {}
  fn visit_lit_byte_str_mut(&mut self, _: &mut LitByteStr) {}
  fn visit_lit_char_mut(&mut self, _: &mut LitChar) {}
  fn visit_lit_float_mut(&mut self, _: &mut LitFloat) {}
  fn visit_lit_int_mut(&mut self, _: &mut LitInt) {}
  fn visit_lit_str_mut(&mut self, _: &mut LitStr) {}
  fn visit_lit_verbatim_mut(&mut self, _: &mut LitVerbatim) {}
  fn visit_macro_delimiter_mut(&mut self, _: &mut MacroDelimiter) {}
  fn visit_member_mut(&mut self, _: &mut Member) {}
  fn visit_meta_mut(&mut self, _: &mut Meta) {}
  fn visit_meta_list_mut(&mut self, _: &mut MetaList) {}
  fn visit_meta_name_value_mut(&mut self, _: &mut MetaNameValue) {}
  fn visit_method_sig_mut(&mut self, _: &mut MethodSig) {}
  fn visit_method_turbofish_mut(&mut self, _: &mut MethodTurbofish) {}
  fn visit_nested_meta_mut(&mut self, _: &mut NestedMeta) {}
  fn visit_parenthesized_generic_arguments_mut(&mut self, _: &mut ParenthesizedGenericArguments) {}
  fn visit_pat_mut(&mut self, _: &mut Pat) {}
  fn visit_pat_box_mut(&mut self, _: &mut PatBox) {}
  fn visit_pat_ident_mut(&mut self, _: &mut PatIdent) {}
  fn visit_pat_lit_mut(&mut self, _: &mut PatLit) {}
  fn visit_pat_macro_mut(&mut self, _: &mut PatMacro) {}
  fn visit_pat_path_mut(&mut self, _: &mut PatPath) {}
  fn visit_pat_range_mut(&mut self, _: &mut PatRange) {}
  fn visit_pat_ref_mut(&mut self, _: &mut PatRef) {}
  fn visit_pat_slice_mut(&mut self, _: &mut PatSlice) {}
  fn visit_pat_struct_mut(&mut self, _: &mut PatStruct) {}
  fn visit_pat_tuple_mut(&mut self, _: &mut PatTuple) {}
  fn visit_pat_tuple_struct_mut(&mut self, _: &mut PatTupleStruct) {}
  fn visit_pat_verbatim_mut(&mut self, _: &mut PatVerbatim) {}
  fn visit_pat_wild_mut(&mut self, _: &mut PatWild) {}
  fn visit_predicate_eq_mut(&mut self, _: &mut PredicateEq) {}
  fn visit_predicate_lifetime_mut(&mut self, _: &mut PredicateLifetime) {}
  fn visit_predicate_type_mut(&mut self, _: &mut PredicateType) {}
  fn visit_qself_mut(&mut self, _: &mut QSelf) {}
  fn visit_range_limits_mut(&mut self, _: &mut RangeLimits) {}
  fn visit_return_type_mut(&mut self, _: &mut ReturnType) {}
  fn visit_span_mut(&mut self, _: &mut Span) {}
  fn visit_trait_bound_mut(&mut self, _: &mut TraitBound) {}
  fn visit_trait_bound_modifier_mut(&mut self, _: &mut TraitBoundModifier) {}
  fn visit_trait_item_mut(&mut self, _: &mut TraitItem) {}
  fn visit_trait_item_const_mut(&mut self, _: &mut TraitItemConst) {}
  fn visit_trait_item_macro_mut(&mut self, _: &mut TraitItemMacro) {}
  fn visit_trait_item_method_mut(&mut self, _: &mut TraitItemMethod) {}
  fn visit_trait_item_type_mut(&mut self, _: &mut TraitItemType) {}
  fn visit_trait_item_verbatim_mut(&mut self, _: &mut TraitItemVerbatim) {}
  fn visit_type_mut(&mut self, _: &mut Type) {}
  fn visit_type_array_mut(&mut self, _: &mut TypeArray) {}
  fn visit_type_bare_fn_mut(&mut self, _: &mut TypeBareFn) {}
  fn visit_type_group_mut(&mut self, _: &mut TypeGroup) {}
  fn visit_type_impl_trait_mut(&mut self, _: &mut TypeImplTrait) {}
  fn visit_type_infer_mut(&mut self, _: &mut TypeInfer) {}
  fn visit_type_macro_mut(&mut self, _: &mut TypeMacro) {}
  fn visit_type_never_mut(&mut self, _: &mut TypeNever) {}
  fn visit_type_param_mut(&mut self, _: &mut TypeParam) {}
  fn visit_type_param_bound_mut(&mut self, _: &mut TypeParamBound) {}
  fn visit_type_paren_mut(&mut self, _: &mut TypeParen) {}
  fn visit_type_path_mut(&mut self, _: &mut TypePath) {}
  fn visit_type_ptr_mut(&mut self, _: &mut TypePtr) {}
  fn visit_type_reference_mut(&mut self, _: &mut TypeReference) {}
  fn visit_type_slice_mut(&mut self, _: &mut TypeSlice) {}
  fn visit_type_trait_object_mut(&mut self, _: &mut TypeTraitObject) {}
  fn visit_type_tuple_mut(&mut self, _: &mut TypeTuple) {}
  fn visit_type_verbatim_mut(&mut self, _: &mut TypeVerbatim) {}
  fn visit_un_op_mut(&mut self, _: &mut UnOp) {}
  fn visit_use_glob_mut(&mut self, _: &mut UseGlob) {}
  fn visit_use_group_mut(&mut self, _: &mut UseGroup) {}
  fn visit_use_name_mut(&mut self, _: &mut UseName) {}
  fn visit_use_path_mut(&mut self, _: &mut UsePath) {}
  fn visit_use_rename_mut(&mut self, _: &mut UseRename) {}
  fn visit_use_tree_mut(&mut self, _: &mut UseTree) {}
  fn visit_variant_mut(&mut self, _: &mut Variant) {}
  fn visit_vis_crate_mut(&mut self, _: &mut VisCrate) {}
  fn visit_vis_public_mut(&mut self, _: &mut VisPublic) {}
  fn visit_vis_restricted_mut(&mut self, _: &mut VisRestricted) {}
  fn visit_visibility_mut(&mut self, _: &mut Visibility) {}
  fn visit_where_clause_mut(&mut self, _: &mut WhereClause) {}
  fn visit_where_predicate_mut(&mut self, _: &mut WherePredicate) {}
}

pub fn transform_ivars(method: &mut ImplItemMethod, objrs_root: &Ident) -> Result<(), Diagnostic> {
  if method.block.stmts.is_empty() {
    return Ok(());
  }

  let as_ref;
  let priv_name;
  let self_arg;
  match method.sig.decl.inputs.first().expect("BUG: missing self arg in the ivar transform").value()
  {
    FnArg::SelfRef(ref self_ref) => {
      as_ref = self_ref.mutability.is_none();
      self_arg = Ident::new("self", self_ref.self_token.span());
      priv_name = "__objrs_self".to_string();
    }
    FnArg::SelfValue(ref self_value) => {
      as_ref = self_value.mutability.is_none();
      self_arg = Ident::new("self", self_value.self_token.span());
      priv_name = "__objrs_self".to_string();
    }
    FnArg::Captured(ref arg) => match arg.pat {
      Pat::Ident(ref pat_ident) => {
        if let Some(ref by_ref) = pat_ident.by_ref {
          return Err(
            by_ref
              .span()
              .unstable()
              .error("method parameters cannot be a `ref`")
              .note("the current version of objrs disallows them"),
          );
        }
        as_ref = false;
        self_arg = pat_ident.ident.clone();
        let self_arg_name = self_arg.to_string();
        if self_arg_name == "self" {
          priv_name = "__objrs_self".to_string();
        } else {
          priv_name = self_arg_name;
        }
      }
      _ => {
        return Err(
          arg
            .pat
            .span()
            .unstable()
            .error("expected argument, found pattern")
            .note("objrs does not support this pattern as a function argument"),
        );
      }
    },
    arg => {
      return Err(
        arg
          .span()
          .unstable()
          .error("expected argument, found inferred or ignored argument")
          .note("objrs does not support inferred or ignored self arguments"),
      );
    }
  }

  let objrs_self = priv_ident(&priv_name);
  let mut visitor = MethodVisitor::new(as_ref, self_arg, objrs_root, priv_name);
  visit_block_mut(&mut visitor, &mut method.block);

  if !visitor.modified {
    return Ok(());
  }

  let maybe_mut = if as_ref { None } else { Some(quote!(mut)) };
  let extend = if as_ref { quote!(extend_ref) } else { quote!(extend_mut) };

  let self_arg = visitor.self_arg;
  let take_ref = visitor.take_ref;
  let load_ivars = visitor.load_ivars;
  let objrs_root = visitor.objrs_root;
  let brace_token = method.block.brace_token.clone();
  let original_block = core::mem::replace(
    &mut method.block,
    Block {
      brace_token: brace_token,
      stmts: Vec::new(),
    },
  );
  let stmts = vec![
    Stmt::Item(Item::Verbatim(ItemVerbatim {
      tts: quote! {
        let mut #objrs_self = <Self as #objrs_root::__objrs::Fields>::from_ref(#self_arg);
        #load_ivars
        let #maybe_mut #objrs_self = unsafe { #objrs_root::__objrs::ThisAndFields::#extend(#take_ref #objrs_self, #self_arg) };
      },
    })),
    Stmt::Expr(Expr::Block(ExprBlock {
      attrs: Vec::new(),
      label: None,
      block: original_block,
    })),
  ];
  method.block.stmts = stmts;
  return Ok(());
}

#[cfg(test)]
mod tests {
  use super::*;
  use quote::ToTokens;
  use syn::parse_quote;

  #[test]
  fn nop() {
    // TODO: should transform_ivars() support class methods? It currently doesn't.
    let objrs_root: Ident = parse_quote!(__objrs_root);
    let original: ImplItemMethod = parse_quote!(fn foo(self) {});
    let mut method = original.clone();
    assert!(transform_ivars(&mut method, &objrs_root).is_ok());
    assert_eq!(method, original);

    let original: ImplItemMethod = parse_quote!(fn foo(self) { do_something(); });
    let mut method = original.clone();
    assert!(transform_ivars(&mut method, &objrs_root).is_ok());
    assert_eq!(method, original);
  }

  #[test]
  fn pass_self() {
    let objrs_root: Ident = parse_quote!(__objrs_root);
    let mut method: ImplItemMethod = parse_quote!(fn foo(self) { do_something(self); });
    assert!(transform_ivars(&mut method, &objrs_root).is_ok());

    let expected: ImplItemMethod = parse_quote!(fn foo(self) {
      let mut __objrs_self = <Self as __objrs_root::__objrs::Fields>::from_ref(self);
      let __objrs_self = unsafe { __objrs_root::__objrs::ThisAndFields::extend_ref(&__objrs_self, self) };
      {
        do_something(__objrs_root::__objrs::ThisAndFields::as_ref(__objrs_self));
      }
    });
    assert_eq!(method.into_token_stream().to_string(), expected.into_token_stream().to_string());
  }

  #[test]
  fn call_method() {
    let objrs_root: Ident = parse_quote!(__objrs_root);
    let mut method: ImplItemMethod = parse_quote!(fn foo(self) { self.do_something(); });
    assert!(transform_ivars(&mut method, &objrs_root).is_ok());

    let expected: ImplItemMethod = parse_quote!(fn foo(self) {
      let mut __objrs_self = <Self as __objrs_root::__objrs::Fields>::from_ref(self);
      let __objrs_self = unsafe { __objrs_root::__objrs::ThisAndFields::extend_ref(&__objrs_self, self) };
      {
        __objrs_root::__objrs::ThisAndFields::as_ref(__objrs_self).do_something();
      }
    });
    assert_eq!(method.into_token_stream().to_string(), expected.into_token_stream().to_string());
  }

  #[test]
  fn read_ivars() {
    let objrs_root: Ident = parse_quote!(__objrs_root);
    let mut method: ImplItemMethod = parse_quote!(fn foo(self) { do_something(self.foo); });
    assert!(transform_ivars(&mut method, &objrs_root).is_ok());

    let expected: ImplItemMethod = parse_quote!(fn foo(self) {
      let mut __objrs_self = <Self as __objrs_root::__objrs::Fields>::from_ref(self);
      unsafe { __objrs_root::__objrs::Field::load(&mut __objrs_self.fields.foo, __objrs_root::__objrs::core::mem::transmute(&*self)); }
      let __objrs_self = unsafe { __objrs_root::__objrs::ThisAndFields::extend_ref(&__objrs_self, self) };
      {
        do_something((*__objrs_root::__objrs::Field::as_ref(&__objrs_self.fields.foo)));
      }
    });
    assert_eq!(method.into_token_stream().to_string(), expected.into_token_stream().to_string());
  }

  #[test]
  fn read_write_ivars() {
    let objrs_root: Ident = parse_quote!(__objrs_root);
    let mut method: ImplItemMethod = parse_quote!(fn foo(mut self) { self.foo = self.bar; });
    assert!(transform_ivars(&mut method, &objrs_root).is_ok());

    let expected: ImplItemMethod = parse_quote!(fn foo(mut self) {
      let mut __objrs_self = <Self as __objrs_root::__objrs::Fields>::from_ref(self);
      unsafe { __objrs_root::__objrs::Field::load(&mut __objrs_self.fields.foo, __objrs_root::__objrs::core::mem::transmute(&*self)); }
      unsafe { __objrs_root::__objrs::Field::load(&mut __objrs_self.fields.bar, __objrs_root::__objrs::core::mem::transmute(&*self)); }
      let mut __objrs_self = unsafe { __objrs_root::__objrs::ThisAndFields::extend_mut(&mut __objrs_self, self) };
      {
        (*__objrs_root::__objrs::Field::as_mut(&mut __objrs_self.fields.foo)) = (*__objrs_root::__objrs::Field::as_mut(&mut __objrs_self.fields.bar));
      }
    });
    assert_eq!(method.into_token_stream().to_string(), expected.into_token_stream().to_string());
  }

  #[test]
  fn tuples() {
    let objrs_root: Ident = parse_quote!(__objrs_root);
    let mut method: ImplItemMethod = parse_quote!(fn foo(self) { do_something(self.0); });
    assert!(transform_ivars(&mut method, &objrs_root).is_ok());

    let expected: ImplItemMethod = parse_quote!(fn foo(self) {
      let mut __objrs_self = <Self as __objrs_root::__objrs::Fields>::from_ref(self);
      unsafe { __objrs_root::__objrs::Field::load(&mut __objrs_self.fields.0, __objrs_root::__objrs::core::mem::transmute(&*self)); }
      let __objrs_self = unsafe { __objrs_root::__objrs::ThisAndFields::extend_ref(&__objrs_self, self) };
      {
        do_something((*__objrs_root::__objrs::Field::as_ref(&__objrs_self.fields.0)));
      }
    });
    assert_eq!(method.into_token_stream().to_string(), expected.into_token_stream().to_string());
  }

  #[test]
  fn ignore_items() {
    let objrs_root: Ident = parse_quote!(__objrs_root);
    let original: ImplItemMethod = parse_quote!(fn foo(self) {
      impl Ignored {
        fn bar(self) {
          self.this_should_not_be_modified();
        }
      }
    });
    let mut method = original.clone();
    assert!(transform_ivars(&mut method, &objrs_root).is_ok());
    assert_eq!(method, original);
  }
}
