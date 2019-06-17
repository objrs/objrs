// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

use crate::cgbase::CGFloat;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct CGPoint {
  pub x: CGFloat,
  pub y: CGFloat,
}

unsafe impl objrs::marker::Zeroed for CGPoint {}
impl objrs::marker::Forgettable for CGPoint {}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct CGSize {
  pub width: CGFloat,
  pub height: CGFloat,
}

unsafe impl objrs::marker::Zeroed for CGSize {}
impl objrs::marker::Forgettable for CGSize {}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct CGVector {
  pub dx: CGFloat,
  pub dy: CGFloat,
}

unsafe impl objrs::marker::Zeroed for CGVector {}
impl objrs::marker::Forgettable for CGVector {}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct CGRect {
  pub origin: CGPoint,
  pub size: CGSize,
}

unsafe impl objrs::marker::Zeroed for CGRect {}
impl objrs::marker::Forgettable for CGRect {}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct CGRectEdge(u32);

unsafe impl objrs::marker::Zeroed for CGRectEdge {}
impl objrs::marker::Forgettable for CGRectEdge {}

#[allow(non_upper_case_globals)]
pub const CGRectMinXEdge: CGRectEdge = CGRectEdge(0);
#[allow(non_upper_case_globals)]
pub const CGRectMinYEdge: CGRectEdge = CGRectEdge(1);
#[allow(non_upper_case_globals)]
pub const CGRectMaxXEdge: CGRectEdge = CGRectEdge(2);
#[allow(non_upper_case_globals)]
pub const CGRectMaxYEdge: CGRectEdge = CGRectEdge(3);

impl CGRectEdge {
  pub const MIN_X: CGRectEdge = CGRectMinXEdge;
  pub const MIN_Y: CGRectEdge = CGRectMinYEdge;
  pub const MAX_X: CGRectEdge = CGRectMaxXEdge;
  pub const MAX_Y: CGRectEdge = CGRectMaxYEdge;
}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
  pub static CGPointZero: CGPoint;
  pub static CGSizeZero: CGSize;
  pub static CGRectZero: CGRect;
  pub static CGRectNull: CGRect;
  pub static CGRectInfinite: CGRect;
}

#[allow(non_snake_case)]
#[inline(always)]
pub fn CGRectMake(x: CGFloat, y: CGFloat, width: CGFloat, height: CGFloat) -> CGRect {
  return CGRect {
    origin: CGPoint {
      x: x,
      y: y,
    },
    size: CGSize {
      width: width,
      height: height,
    },
  };
}
