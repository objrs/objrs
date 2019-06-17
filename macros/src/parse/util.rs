// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

use crate::span_ext::SpanExt;
use crate::util::priv_ident;
use syn::Ident;

pub fn objrs_root(objrs: Option<Ident>) -> Ident {
  return objrs.map(SpanExt::resolved_at_def_site).unwrap_or_else(|| priv_ident("objrs"));
}

// macro_rules! if_let {
//   ($enum:ident :: $variant:ident (_) = $value:expr) => {{
//     use core::option::Option;
//     match $value {
//         $enum::$variant(value) => Option::Some(value),
//         _ => Option::None,
//     }
//   }};
// }
