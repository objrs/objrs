// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

#![feature(arbitrary_self_types, extern_types)]
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

pub use crate::nsapplication::*;
pub use crate::nsapplication_delegate::*;
pub use crate::nsmenu::*;
pub use crate::nsmenu_item::*;
pub use crate::nsresponder::*;
pub use crate::nsview::*;
pub use crate::nsview_controller::*;
pub use crate::nswindow::*;
