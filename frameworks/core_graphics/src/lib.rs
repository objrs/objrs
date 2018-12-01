// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

#![feature(extern_types, rust_2018_preview)]
#![no_std]

// TODO: does CoreGraphics use Objective-C at all? If not, drop this.
extern crate objrs;

mod cgbase;
mod cggeometry;

pub use cgbase::*;
pub use cggeometry::*;
