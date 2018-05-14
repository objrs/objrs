// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

#![feature(extern_types, proc_macro, repr_transparent, used)]
#![no_std]

extern crate objrs;
extern crate objrs_frameworks_foundation_macros;

mod nsarray;
mod nsmutable_string;
mod nsobject;
mod nsstring;

pub use nsarray::*;
pub use nsmutable_string::*;
pub use nsobject::*;
pub use nsstring::*;

#[doc(hidden)]
pub mod __objrs {
  pub extern crate objrs;
  pub use objrs::__objrs::*;

  pub use nsstring::__objrs::*;
}
