// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

use objrs::objrs;
use objrs_frameworks_foundation::NSObject;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct MTLClearColor {
  pub red: f64,
  pub green: f64,
  pub blue: f64,
  pub alpha: f64,
}

unsafe impl objrs::marker::Zeroed for MTLClearColor {}
impl objrs::marker::Forgettable for MTLClearColor {}

#[allow(non_snake_case)]
#[inline(always)]
pub fn MTLClearColorMake(red: f64, green: f64, blue: f64, alpha: f64) -> MTLClearColor {
  return MTLClearColor {
    red: red,
    green: green,
    blue: blue,
    alpha: alpha,
  };
}

#[objrs(class, super = NSObject)]
#[link(name = "Metal", kind = "framework")]
pub struct MTLRenderPassDescriptor;

#[objrs(impl)]
#[link(name = "Metal", kind = "framework")]
impl MTLRenderPassDescriptor {}
