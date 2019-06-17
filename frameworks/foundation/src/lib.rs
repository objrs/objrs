// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

#![feature(extern_types)]
#![no_std]

extern crate objrs;
extern crate objrs_frameworks_foundation_macros;

mod nsarray;
mod nserror;
mod nsmutable_string;
mod nsnotification;
mod nsobject;
mod nsprocess_info;
mod nsstring;

pub use crate::nsarray::*;
pub use crate::nserror::*;
pub use crate::nsmutable_string::*;
pub use crate::nsnotification::*;
pub use crate::nsobject::*;
pub use crate::nsprocess_info::*;
pub use crate::nsstring::*;

#[doc(hidden)]
pub mod __objrs {
  pub extern crate objrs;
  pub use objrs::__objrs::*;

  pub use crate::nsstring::__objrs::*;
}
