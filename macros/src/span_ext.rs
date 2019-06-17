// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

extern crate proc_macro;
extern crate proc_macro2;

pub trait SpanExt {
  fn resolved_at_def_site(self: Self) -> Self;
}

impl SpanExt for proc_macro2::Span {
  #[cfg(procmacro2_semver_exempt)]
  fn resolved_at_def_site(self: Self) -> Self {
    return self.resolved_at(proc_macro2::Span::def_site());
  }

  #[cfg(not(procmacro2_semver_exempt))]
  fn resolved_at_def_site(self: Self) -> Self {
    return self.unstable().resolved_at(proc_macro::Span::def_site()).into();
  }
}

impl SpanExt for proc_macro2::Ident {
  fn resolved_at_def_site(mut self: Self) -> Self {
    self.set_span(self.span().resolved_at_def_site());
    return self;
  }
}

impl SpanExt for proc_macro2::Punct {
  fn resolved_at_def_site(mut self: Self) -> Self {
    self.set_span(self.span().resolved_at_def_site());
    return self;
  }
}

impl SpanExt for proc_macro2::Literal {
  fn resolved_at_def_site(mut self: Self) -> Self {
    self.set_span(self.span().resolved_at_def_site());
    return self;
  }
}

impl SpanExt for proc_macro2::Group {
  fn resolved_at_def_site(self: Self) -> Self {
    let span = self.span().resolved_at_def_site();
    let stream = self.stream().resolved_at_def_site();
    let mut group = proc_macro2::Group::new(self.delimiter(), stream);
    group.set_span(span);
    return group;
  }
}

impl SpanExt for proc_macro2::TokenTree {
  fn resolved_at_def_site(self: Self) -> Self {
    match self {
      proc_macro2::TokenTree::Group(group) => {
        return proc_macro2::TokenTree::Group(group.resolved_at_def_site());
      }
      proc_macro2::TokenTree::Ident(ident) => {
        return proc_macro2::TokenTree::Ident(ident.resolved_at_def_site());
      }
      proc_macro2::TokenTree::Punct(punct) => {
        return proc_macro2::TokenTree::Punct(punct.resolved_at_def_site());
      }
      proc_macro2::TokenTree::Literal(literal) => {
        return proc_macro2::TokenTree::Literal(literal.resolved_at_def_site());
      }
    }
  }
}

impl SpanExt for proc_macro2::TokenStream {
  fn resolved_at_def_site(self: Self) -> Self {
    return self.into_iter().map(|tree| tree.resolved_at_def_site()).collect();
  }
}
