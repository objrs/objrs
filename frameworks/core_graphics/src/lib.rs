// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

#![feature(extern_types)]
#![no_std]

// TODO: does CoreGraphics use Objective-C at all? If not, drop this.
extern crate objrs;

mod cgbase;
mod cggeometry;

pub use crate::cgbase::*;
pub use crate::cggeometry::*;
