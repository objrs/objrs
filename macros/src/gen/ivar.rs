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
  spanned::Spanned, visit_mut, visit_mut::VisitMut, Abi, AngleBracketedGenericArguments, Arm,
  AttrStyle, Attribute, BareFnArg, BinOp, Binding, Block, BoundLifetimes, ConstParam, Constraint,
  Data, DataEnum, DataStruct, DataUnion, DeriveInput, Expr, ExprArray, ExprAssign, ExprAssignOp,
  ExprAsync, ExprAwait, ExprBinary, ExprBlock, ExprBox, ExprBreak, ExprCall, ExprCast, ExprClosure,
  ExprContinue, ExprField, ExprForLoop, ExprGroup, ExprIf, ExprIndex, ExprLet, ExprLit, ExprLoop,
  ExprMacro, ExprMatch, ExprMethodCall, ExprParen, ExprPath, ExprRange, ExprReference, ExprRepeat,
  ExprReturn, ExprStruct, ExprTry, ExprTryBlock, ExprTuple, ExprType, ExprUnary, ExprUnsafe,
  ExprWhile, ExprYield, Field, FieldPat, FieldValue, Fields, FieldsNamed, FieldsUnnamed, File,
  FnArg, ForeignItem, ForeignItemFn, ForeignItemMacro, ForeignItemStatic, ForeignItemType,
  GenericArgument, GenericMethodArgument, GenericParam, Generics, Ident, ImplItem, ImplItemConst,
  ImplItemMacro, ImplItemMethod, ImplItemType, Index, Item, ItemConst, ItemEnum, ItemExternCrate,
  ItemFn, ItemForeignMod, ItemImpl, ItemMacro, ItemMacro2, ItemMod, ItemStatic, ItemStruct,
  ItemTrait, ItemTraitAlias, ItemType, ItemUnion, ItemUse, Label, Lifetime, LifetimeDef, Lit,
  LitBool, LitByte, LitByteStr, LitChar, LitFloat, LitInt, LitStr, Local, Macro, MacroDelimiter,
  Member, Meta, MetaList, MetaNameValue, MethodTurbofish, NestedMeta,
  ParenthesizedGenericArguments, Pat, PatBox, PatIdent, PatLit, PatMacro, PatOr, PatPath, PatRange,
  PatReference, PatRest, PatSlice, PatStruct, PatTuple, PatTupleStruct, PatType, PatWild, Path,
  PathArguments, PathSegment, PredicateEq, PredicateLifetime, PredicateType, QSelf, RangeLimits,
  Receiver, ReturnType, Signature, Stmt, TraitBound, TraitBoundModifier, TraitItem, TraitItemConst,
  TraitItemMacro, TraitItemMethod, TraitItemType, Type, TypeArray, TypeBareFn, TypeGroup,
  TypeImplTrait, TypeInfer, TypeMacro, TypeNever, TypeParam, TypeParamBound, TypeParen, TypePath,
  TypePtr, TypeReference, TypeSlice, TypeTraitObject, TypeTuple, UnOp, UseGlob, UseGroup, UseName,
  UsePath, UseRename, UseTree, Variadic, Variant, VisCrate, VisPublic, VisRestricted, Visibility,
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
      new_expr = Some(Expr::Verbatim(quote! {
        #objrs_root::__objrs::ThisAndFields::#as_fn(#objrs_self)
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
        new_expr = Some(Expr::Verbatim(quote! {
          (*#objrs_root::__objrs::Field::#as_fn(#take_ref #objrs_self #dot #fields #dot #member))
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

    visit_mut::visit_expr_mut(self, expr);
  }

  fn visit_block_mut(&mut self, block: &mut Block) {
    visit_mut::visit_block_mut(self, block);
  }
  fn visit_expr_array_mut(&mut self, expr: &mut ExprArray) {
    visit_mut::visit_expr_array_mut(self, expr);
  }
  fn visit_expr_assign_mut(&mut self, expr: &mut ExprAssign) {
    visit_mut::visit_expr_assign_mut(self, expr);
  }
  fn visit_expr_assign_op_mut(&mut self, expr: &mut ExprAssignOp) {
    visit_mut::visit_expr_assign_op_mut(self, expr);
  }
  fn visit_expr_async_mut(&mut self, expr: &mut ExprAsync) {
    visit_mut::visit_expr_async_mut(self, expr);
  }
  fn visit_expr_await_mut(&mut self, expr: &mut ExprAwait) {
    visit_mut::visit_expr_await_mut(self, expr);
  }
  fn visit_expr_binary_mut(&mut self, expr: &mut ExprBinary) {
    visit_mut::visit_expr_binary_mut(self, expr);
  }
  fn visit_expr_block_mut(&mut self, expr: &mut ExprBlock) {
    visit_mut::visit_expr_block_mut(self, expr);
  }
  fn visit_expr_box_mut(&mut self, expr: &mut ExprBox) {
    visit_mut::visit_expr_box_mut(self, expr);
  }
  fn visit_expr_break_mut(&mut self, expr: &mut ExprBreak) {
    visit_mut::visit_expr_break_mut(self, expr);
  }
  fn visit_expr_call_mut(&mut self, expr: &mut ExprCall) {
    visit_mut::visit_expr_call_mut(self, expr);
  }
  fn visit_expr_cast_mut(&mut self, expr: &mut ExprCast) {
    visit_mut::visit_expr_cast_mut(self, expr);
  }
  fn visit_expr_closure_mut(&mut self, expr: &mut ExprClosure) {
    visit_mut::visit_expr_closure_mut(self, expr);
  }
  fn visit_expr_continue_mut(&mut self, expr: &mut ExprContinue) {
    visit_mut::visit_expr_continue_mut(self, expr);
  }
  fn visit_expr_field_mut(&mut self, expr: &mut ExprField) {
    visit_mut::visit_expr_field_mut(self, expr);
  }
  fn visit_expr_for_loop_mut(&mut self, expr: &mut ExprForLoop) {
    visit_mut::visit_expr_for_loop_mut(self, expr);
  }
  fn visit_expr_group_mut(&mut self, expr: &mut ExprGroup) {
    visit_mut::visit_expr_group_mut(self, expr);
  }
  fn visit_expr_if_mut(&mut self, expr: &mut ExprIf) {
    visit_mut::visit_expr_if_mut(self, expr);
  }
  fn visit_expr_index_mut(&mut self, expr: &mut ExprIndex) {
    visit_mut::visit_expr_index_mut(self, expr);
  }
  fn visit_expr_let_mut(&mut self, expr: &mut ExprLet) {
    visit_mut::visit_expr_let_mut(self, expr);
  }
  fn visit_expr_lit_mut(&mut self, expr: &mut ExprLit) {
    visit_mut::visit_expr_lit_mut(self, expr);
  }
  fn visit_expr_loop_mut(&mut self, expr: &mut ExprLoop) {
    visit_mut::visit_expr_loop_mut(self, expr);
  }
  fn visit_expr_macro_mut(&mut self, expr: &mut ExprMacro) {
    visit_mut::visit_expr_macro_mut(self, expr);
  }
  fn visit_expr_match_mut(&mut self, expr: &mut ExprMatch) {
    visit_mut::visit_expr_match_mut(self, expr);
  }
  fn visit_expr_method_call_mut(&mut self, expr: &mut ExprMethodCall) {
    visit_mut::visit_expr_method_call_mut(self, expr);
  }
  fn visit_expr_paren_mut(&mut self, expr: &mut ExprParen) {
    visit_mut::visit_expr_paren_mut(self, expr);
  }
  fn visit_expr_path_mut(&mut self, expr: &mut ExprPath) {
    visit_mut::visit_expr_path_mut(self, expr);
  }
  fn visit_expr_range_mut(&mut self, expr: &mut ExprRange) {
    visit_mut::visit_expr_range_mut(self, expr);
  }
  fn visit_expr_reference_mut(&mut self, expr: &mut ExprReference) {
    visit_mut::visit_expr_reference_mut(self, expr);
  }
  fn visit_expr_repeat_mut(&mut self, expr: &mut ExprRepeat) {
    visit_mut::visit_expr_repeat_mut(self, expr);
  }
  fn visit_expr_return_mut(&mut self, expr: &mut ExprReturn) {
    visit_mut::visit_expr_return_mut(self, expr);
  }
  fn visit_expr_struct_mut(&mut self, expr: &mut ExprStruct) {
    visit_mut::visit_expr_struct_mut(self, expr);
  }
  fn visit_expr_try_mut(&mut self, expr: &mut ExprTry) {
    visit_mut::visit_expr_try_mut(self, expr);
  }
  fn visit_expr_try_block_mut(&mut self, expr: &mut ExprTryBlock) {
    visit_mut::visit_expr_try_block_mut(self, expr);
  }
  fn visit_expr_tuple_mut(&mut self, expr: &mut ExprTuple) {
    visit_mut::visit_expr_tuple_mut(self, expr);
  }
  fn visit_expr_type_mut(&mut self, expr: &mut ExprType) {
    visit_mut::visit_expr_type_mut(self, expr);
  }
  fn visit_expr_unary_mut(&mut self, expr: &mut ExprUnary) {
    visit_mut::visit_expr_unary_mut(self, expr);
  }
  fn visit_expr_unsafe_mut(&mut self, expr: &mut ExprUnsafe) {
    visit_mut::visit_expr_unsafe_mut(self, expr);
  }
  fn visit_expr_while_mut(&mut self, expr: &mut ExprWhile) {
    visit_mut::visit_expr_while_mut(self, expr);
  }
  fn visit_expr_yield_mut(&mut self, expr: &mut ExprYield) {
    visit_mut::visit_expr_yield_mut(self, expr);
  }
  fn visit_field_pat_mut(&mut self, field: &mut FieldPat) {
    visit_mut::visit_field_pat_mut(self, field);
  }
  fn visit_field_value_mut(&mut self, field: &mut FieldValue) {
    visit_mut::visit_field_value_mut(self, field);
  }
  fn visit_ident_mut(&mut self, ident: &mut Ident) {
    visit_mut::visit_ident_mut(self, ident);
  }
  fn visit_local_mut(&mut self, local: &mut Local) {
    visit_mut::visit_local_mut(self, local);
  }
  fn visit_macro_mut(&mut self, m: &mut Macro) {
    visit_mut::visit_macro_mut(self, m);
  }
  fn visit_path_mut(&mut self, path: &mut Path) {
    visit_mut::visit_path_mut(self, path);
  }
  fn visit_path_arguments_mut(&mut self, args: &mut PathArguments) {
    visit_mut::visit_path_arguments_mut(self, args);
  }
  fn visit_path_segment_mut(&mut self, segment: &mut PathSegment) {
    visit_mut::visit_path_segment_mut(self, segment);
  }
  fn visit_stmt_mut(&mut self, stmt: &mut Stmt) {
    visit_mut::visit_stmt_mut(self, stmt);
  }

  fn visit_abi_mut(&mut self, _: &mut Abi) {}
  fn visit_angle_bracketed_generic_arguments_mut(
    &mut self,
    _: &mut AngleBracketedGenericArguments,
  ) {
  }
  fn visit_arm_mut(&mut self, _: &mut Arm) {}
  fn visit_attr_style_mut(&mut self, _: &mut AttrStyle) {}
  fn visit_attribute_mut(&mut self, _: &mut Attribute) {}
  fn visit_bare_fn_arg_mut(&mut self, _: &mut BareFnArg) {}
  fn visit_bin_op_mut(&mut self, _: &mut BinOp) {}
  fn visit_binding_mut(&mut self, _: &mut Binding) {}
  fn visit_bound_lifetimes_mut(&mut self, _: &mut BoundLifetimes) {}
  fn visit_const_param_mut(&mut self, _: &mut ConstParam) {}
  fn visit_constraint_mut(&mut self, _: &mut Constraint) {}
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
  fn visit_foreign_item_mut(&mut self, _: &mut ForeignItem) {}
  fn visit_foreign_item_fn_mut(&mut self, _: &mut ForeignItemFn) {}
  fn visit_foreign_item_macro_mut(&mut self, _: &mut ForeignItemMacro) {}
  fn visit_foreign_item_static_mut(&mut self, _: &mut ForeignItemStatic) {}
  fn visit_foreign_item_type_mut(&mut self, _: &mut ForeignItemType) {}
  fn visit_generic_argument_mut(&mut self, _: &mut GenericArgument) {}
  fn visit_generic_method_argument_mut(&mut self, _: &mut GenericMethodArgument) {}
  fn visit_generic_param_mut(&mut self, _: &mut GenericParam) {}
  fn visit_generics_mut(&mut self, _: &mut Generics) {}
  fn visit_impl_item_mut(&mut self, _: &mut ImplItem) {}
  fn visit_impl_item_const_mut(&mut self, _: &mut ImplItemConst) {}
  fn visit_impl_item_macro_mut(&mut self, _: &mut ImplItemMacro) {}
  fn visit_impl_item_method_mut(&mut self, _: &mut ImplItemMethod) {}
  fn visit_impl_item_type_mut(&mut self, _: &mut ImplItemType) {}
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
  fn visit_item_trait_alias_mut(&mut self, _: &mut ItemTraitAlias) {}
  fn visit_item_type_mut(&mut self, _: &mut ItemType) {}
  fn visit_item_union_mut(&mut self, _: &mut ItemUnion) {}
  fn visit_item_use_mut(&mut self, _: &mut ItemUse) {}
  fn visit_label_mut(&mut self, _: &mut Label) {}
  fn visit_lifetime_mut(&mut self, _: &mut Lifetime) {}
  fn visit_lifetime_def_mut(&mut self, _: &mut LifetimeDef) {}
  fn visit_lit_mut(&mut self, _: &mut Lit) {}
  fn visit_lit_bool_mut(&mut self, _: &mut LitBool) {}
  fn visit_lit_byte_mut(&mut self, _: &mut LitByte) {}
  fn visit_lit_byte_str_mut(&mut self, _: &mut LitByteStr) {}
  fn visit_lit_char_mut(&mut self, _: &mut LitChar) {}
  fn visit_lit_float_mut(&mut self, _: &mut LitFloat) {}
  fn visit_lit_int_mut(&mut self, _: &mut LitInt) {}
  fn visit_lit_str_mut(&mut self, _: &mut LitStr) {}
  fn visit_macro_delimiter_mut(&mut self, _: &mut MacroDelimiter) {}
  fn visit_member_mut(&mut self, _: &mut Member) {}
  fn visit_meta_mut(&mut self, _: &mut Meta) {}
  fn visit_meta_list_mut(&mut self, _: &mut MetaList) {}
  fn visit_meta_name_value_mut(&mut self, _: &mut MetaNameValue) {}
  fn visit_method_turbofish_mut(&mut self, _: &mut MethodTurbofish) {}
  fn visit_nested_meta_mut(&mut self, _: &mut NestedMeta) {}
  fn visit_parenthesized_generic_arguments_mut(&mut self, _: &mut ParenthesizedGenericArguments) {}
  fn visit_pat_mut(&mut self, _: &mut Pat) {}
  fn visit_pat_box_mut(&mut self, _: &mut PatBox) {}
  fn visit_pat_ident_mut(&mut self, _: &mut PatIdent) {}
  fn visit_pat_lit_mut(&mut self, _: &mut PatLit) {}
  fn visit_pat_macro_mut(&mut self, _: &mut PatMacro) {}
  fn visit_pat_or_mut(&mut self, _: &mut PatOr) {}
  fn visit_pat_path_mut(&mut self, _: &mut PatPath) {}
  fn visit_pat_range_mut(&mut self, _: &mut PatRange) {}
  fn visit_pat_reference_mut(&mut self, _: &mut PatReference) {}
  fn visit_pat_rest_mut(&mut self, _: &mut PatRest) {}
  fn visit_pat_slice_mut(&mut self, _: &mut PatSlice) {}
  fn visit_pat_struct_mut(&mut self, _: &mut PatStruct) {}
  fn visit_pat_tuple_mut(&mut self, _: &mut PatTuple) {}
  fn visit_pat_tuple_struct_mut(&mut self, _: &mut PatTupleStruct) {}
  fn visit_pat_type_mut(&mut self, _: &mut PatType) {}
  fn visit_pat_wild_mut(&mut self, _: &mut PatWild) {}
  fn visit_predicate_eq_mut(&mut self, _: &mut PredicateEq) {}
  fn visit_predicate_lifetime_mut(&mut self, _: &mut PredicateLifetime) {}
  fn visit_predicate_type_mut(&mut self, _: &mut PredicateType) {}
  fn visit_qself_mut(&mut self, _: &mut QSelf) {}
  fn visit_range_limits_mut(&mut self, _: &mut RangeLimits) {}
  fn visit_receiver_mut(&mut self, _: &mut Receiver) {}
  fn visit_return_type_mut(&mut self, _: &mut ReturnType) {}
  fn visit_signature_mut(&mut self, _: &mut Signature) {}
  fn visit_span_mut(&mut self, _: &mut Span) {}
  fn visit_trait_bound_mut(&mut self, _: &mut TraitBound) {}
  fn visit_trait_bound_modifier_mut(&mut self, _: &mut TraitBoundModifier) {}
  fn visit_trait_item_mut(&mut self, _: &mut TraitItem) {}
  fn visit_trait_item_const_mut(&mut self, _: &mut TraitItemConst) {}
  fn visit_trait_item_macro_mut(&mut self, _: &mut TraitItemMacro) {}
  fn visit_trait_item_method_mut(&mut self, _: &mut TraitItemMethod) {}
  fn visit_trait_item_type_mut(&mut self, _: &mut TraitItemType) {}
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
  fn visit_un_op_mut(&mut self, _: &mut UnOp) {}
  fn visit_use_glob_mut(&mut self, _: &mut UseGlob) {}
  fn visit_use_group_mut(&mut self, _: &mut UseGroup) {}
  fn visit_use_name_mut(&mut self, _: &mut UseName) {}
  fn visit_use_path_mut(&mut self, _: &mut UsePath) {}
  fn visit_use_rename_mut(&mut self, _: &mut UseRename) {}
  fn visit_use_tree_mut(&mut self, _: &mut UseTree) {}
  fn visit_variadic_mut(&mut self, _: &mut Variadic) {}
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

  let as_mut;
  let priv_name;
  let self_arg;
  match method.sig.inputs.first().expect("BUG: missing self arg in the ivar transform") {
    FnArg::Receiver(ref receiver) => {
      as_mut = receiver.mutability.is_some();
      self_arg = Ident::new("self", receiver.self_token.span());
      priv_name = "__objrs_self".to_string();
    }
    FnArg::Typed(ref pat_ty) => match *pat_ty.pat {
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
        as_mut = true;
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
          pat_ty
            .pat
            .span()
            .unstable()
            .error("expected argument, found pattern")
            .note("objrs does not support this pattern as a function argument"),
        );
      }
    },
  }

  let objrs_self = priv_ident(&priv_name);
  let mut visitor = MethodVisitor::new(!as_mut, self_arg, objrs_root, priv_name);
  visit_mut::visit_block_mut(&mut visitor, &mut method.block);

  if !visitor.modified {
    return Ok(());
  }

  let maybe_mut = if as_mut { Some(quote!(mut)) } else { None };
  let extend = if as_mut { quote!(extend_mut) } else { quote!(extend_ref) };

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
    Stmt::Item(Item::Verbatim(quote! {
      let mut #objrs_self = <Self as #objrs_root::__objrs::Fields>::from_ref(#self_arg);
      #load_ivars
      let #maybe_mut #objrs_self = unsafe { #objrs_root::__objrs::ThisAndFields::#extend(#take_ref #objrs_self, #self_arg) };
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
    assert_eq!(method.into_token_stream().to_string(), original.into_token_stream().to_string());

    let original: ImplItemMethod = parse_quote!(fn foo(self) { do_something(); });
    let mut method = original.clone();
    assert!(transform_ivars(&mut method, &objrs_root).is_ok());
    assert_eq!(method.into_token_stream().to_string(), original.into_token_stream().to_string());
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
    assert_eq!(method.into_token_stream().to_string(), original.into_token_stream().to_string());
  }
}
