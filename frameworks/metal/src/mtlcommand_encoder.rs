// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

use objrs::objrs;

#[objrs(protocol, id_ident = MTLCommandEncoderId)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLCommandEncoder: objrs::marker::Class {
  #[objrs(selector = "endEncoding")]
  fn end_encoding(self: objrs::Strong<Self>);
}
