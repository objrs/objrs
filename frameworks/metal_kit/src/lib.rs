// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

#![feature(arbitrary_self_types, extern_types)]
#![no_std]

extern crate objrs;
#[cfg(target_os = "macos")]
extern crate objrs_frameworks_app_kit;
extern crate objrs_frameworks_core_graphics;
extern crate objrs_frameworks_metal;

mod mtkview;
mod mtkview_delegate;

pub use crate::mtkview::*;
pub use crate::mtkview_delegate::*;
