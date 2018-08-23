// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

#![feature(arbitrary_self_types, extern_types, rust_2018_preview, used)]
#![no_std]

extern crate objrs;
extern crate objrs_frameworks_core_graphics;
extern crate objrs_frameworks_foundation;

mod nsapplication;
mod nsapplication_delegate;
mod nsmenu;
mod nsmenu_item;
mod nsresponder;
mod nsview;
mod nsview_controller;
mod nswindow;

pub use nsapplication::*;
pub use nsapplication_delegate::*;
pub use nsmenu::*;
pub use nsmenu_item::*;
pub use nsresponder::*;
pub use nsview::*;
pub use nsview_controller::*;
pub use nswindow::*;
