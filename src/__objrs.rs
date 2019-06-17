// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

pub extern crate core;
pub extern crate libc;
pub extern crate objrs_macros;

use crate::{marker, TypeEncoding};

pub mod runtime {
  pub use crate::runtime::*;
}

mod primitive_types {
  pub type U8 = u8;
  pub type U16 = u16;
  pub type U32 = u32;
  pub type U64 = u64;
  pub type U128 = u128;

  pub type I8 = i8;
  pub type I16 = i16;
  pub type I32 = i32;
  pub type I64 = i64;
  pub type I128 = i128;

  pub type Usize = usize;
  pub type Isize = isize;

  pub type Bool = bool;
  pub type Char = char;

  pub type F32 = f32;
  pub type F64 = f64;

  pub type Str = str;
}

#[allow(non_camel_case_types)]
pub type u8 = primitive_types::U8;
#[allow(non_camel_case_types)]
pub type u16 = primitive_types::U16;
#[allow(non_camel_case_types)]
pub type u32 = primitive_types::U32;
#[allow(non_camel_case_types)]
pub type u64 = primitive_types::U64;
#[allow(non_camel_case_types)]
pub type u128 = primitive_types::U128;

#[allow(non_camel_case_types)]
pub type i8 = primitive_types::I8;
#[allow(non_camel_case_types)]
pub type i16 = primitive_types::I16;
#[allow(non_camel_case_types)]
pub type i32 = primitive_types::I32;
#[allow(non_camel_case_types)]
pub type i64 = primitive_types::I64;
#[allow(non_camel_case_types)]
pub type i128 = primitive_types::I128;

#[allow(non_camel_case_types)]
pub type usize = primitive_types::Usize;
#[allow(non_camel_case_types)]
pub type isize = primitive_types::Isize;

#[allow(non_camel_case_types)]
pub type bool = primitive_types::Bool;
#[allow(non_camel_case_types)]
pub type char = primitive_types::Char;

#[allow(non_camel_case_types)]
pub type f32 = primitive_types::F32;
#[allow(non_camel_case_types)]
pub type f64 = primitive_types::F64;

#[allow(non_camel_case_types)]
pub type str = primitive_types::Str;

#[derive(Copy, Clone)]
#[repr(C)]
struct Zst;

#[derive(Copy, Clone)]
#[repr(C)]
pub union UninitPtr {
  zst: Zst,
  never_use_this_field: *const u8,
}

pub const UNINIT_PTR: UninitPtr = UninitPtr {
  zst: Zst,
};

#[repr(transparent)]
pub struct SyncHack<T>(pub T);
unsafe impl<T> Sync for SyncHack<T> {}

// If core::mem::transmute ever becomes a const fn, use it instead of this hack.
// https://github.com/rust-lang/rust/issues/49450
#[repr(C)]
pub union TransmuteHack<T: Copy, U: Copy> {
  pub from: T,
  pub to: U,
}

#[repr(C, packed)]
pub struct Packed2<T: Copy, U: Copy>(pub T, pub U);
impl<T: Copy, U: Copy> Clone for Packed2<T, U> {
  fn clone(&self) -> Self {
    return *self;
  }
}
impl<T: Copy, U: Copy> Copy for Packed2<T, U> {}
unsafe impl<T: Send + Copy, U: Send + Copy> Send for Packed2<T, U> {}
unsafe impl<T: Sync + Copy, U: Sync + Copy> Sync for Packed2<T, U> {}

#[repr(C, packed)]
pub struct Packed3<T: Copy, U: Copy, V: Copy>(pub T, pub U, pub V);
impl<T: Copy, U: Copy, V: Copy> Clone for Packed3<T, U, V> {
  fn clone(&self) -> Self {
    return *self;
  }
}
impl<T: Copy, U: Copy, V: Copy> Copy for Packed3<T, U, V> {}
unsafe impl<T: Send + Copy, U: Send + Copy, V: Send + Copy> Send for Packed3<T, U, V> {}
unsafe impl<T: Sync + Copy, U: Sync + Copy, V: Sync + Copy> Sync for Packed3<T, U, V> {}

const fn log_2(value: usize) -> u32 {
  return 0usize.count_zeros() - (value != 0) as u32 - value.leading_zeros();
}

const fn log_10(value: usize) -> u32 {
  // This algorithm is from https://graphics.stanford.edu/~seander/bithacks.html#IntegerLog10
  let approx_log_10 = (log_2(value) + 1) * 1233 / 4096;
  const POWERS: [u64; 20] = [
    0,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
    100000000000,
    1000000000000,
    10000000000000,
    100000000000000,
    1000000000000000,
    10000000000000000,
    100000000000000000,
    1000000000000000000,
    10000000000000000000,
  ];

  return approx_log_10 - (POWERS[approx_log_10 as usize] > value as u64) as u32;
}

pub struct ToAsciiHack2 {
  pub len: usize,
  pub ascii: [u8; 21],
}

// TODO: use this!
pub const fn to_ascii_hack(value: usize) -> ToAsciiHack2 {
  let len = 1 + log_10(value) as usize;

  // TODO: once pointer juggling works in a const fn, we can just transmute the lut (at the right
  // offset).
  let lut: [u8; 39] = [
    (value / 10000000000000000000 % 10) as u8 + b'0',
    (value / 1000000000000000000 % 10) as u8 + b'0',
    (value / 100000000000000000 % 10) as u8 + b'0',
    (value / 10000000000000000 % 10) as u8 + b'0',
    (value / 1000000000000000 % 10) as u8 + b'0',
    (value / 100000000000000 % 10) as u8 + b'0',
    (value / 10000000000000 % 10) as u8 + b'0',
    (value / 1000000000000 % 10) as u8 + b'0',
    (value / 100000000000 % 10) as u8 + b'0',
    (value / 10000000000 % 10) as u8 + b'0',
    (value / 1000000000 % 10) as u8 + b'0',
    (value / 100000000 % 10) as u8 + b'0',
    (value / 10000000 % 10) as u8 + b'0',
    (value / 1000000 % 10) as u8 + b'0',
    (value / 100000 % 10) as u8 + b'0',
    (value / 10000 % 10) as u8 + b'0',
    (value / 1000 % 10) as u8 + b'0',
    (value / 100 % 10) as u8 + b'0',
    (value / 10 % 10) as u8 + b'0',
    (value % 10) as u8 + b'0',
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
  ];

  let ascii: [u8; 21] = [
    lut[20 - len],
    lut[21 - len],
    lut[22 - len],
    lut[23 - len],
    lut[24 - len],
    lut[25 - len],
    lut[26 - len],
    lut[27 - len],
    lut[28 - len],
    lut[29 - len],
    lut[30 - len],
    lut[31 - len],
    lut[32 - len],
    lut[33 - len],
    lut[34 - len],
    lut[35 - len],
    lut[36 - len],
    lut[37 - len],
    lut[38 - len],
    lut[39 - len],
    0,
  ];

  return ToAsciiHack2 {
    len: len,
    ascii: ascii,
  };
}

pub trait ToAsciiHack {
  const VALUE: usize;

  const LEN: usize = 1
    + (Self::VALUE >= 10) as usize
    + (Self::VALUE >= 100) as usize
    + (Self::VALUE >= 1000) as usize
    + (Self::VALUE >= 10000) as usize
    + (Self::VALUE >= 100000) as usize
    + (Self::VALUE >= 1000000) as usize
    + (Self::VALUE >= 10000000) as usize
    + (Self::VALUE >= 100000000) as usize
    + (Self::VALUE >= 1000000000) as usize;

  const LUT: [u8; 19] = [
    (Self::VALUE / 1000000000 % 10) as u8 + b'0',
    (Self::VALUE / 100000000 % 10) as u8 + b'0',
    (Self::VALUE / 10000000 % 10) as u8 + b'0',
    (Self::VALUE / 1000000 % 10) as u8 + b'0',
    (Self::VALUE / 100000 % 10) as u8 + b'0',
    (Self::VALUE / 10000 % 10) as u8 + b'0',
    (Self::VALUE / 1000 % 10) as u8 + b'0',
    (Self::VALUE / 100 % 10) as u8 + b'0',
    (Self::VALUE / 10 % 10) as u8 + b'0',
    (Self::VALUE % 10) as u8 + b'0',
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
  ];

  const STR: [u8; 11] = [
    Self::LUT[10 - Self::LEN],
    Self::LUT[11 - Self::LEN],
    Self::LUT[12 - Self::LEN],
    Self::LUT[13 - Self::LEN],
    Self::LUT[14 - Self::LEN],
    Self::LUT[15 - Self::LEN],
    Self::LUT[16 - Self::LEN],
    Self::LUT[17 - Self::LEN],
    Self::LUT[18 - Self::LEN],
    Self::LUT[19 - Self::LEN],
    0,
  ];
}

#[repr(transparent)]
pub struct Field<T>(*mut T);

impl<T> Field<T> {
  pub const unsafe fn with_offset(offset: &'static usize) -> Field<T> {
    return Field(offset as *const _ as *const () as *mut T);
  }

  #[inline(always)]
  pub unsafe fn load(&mut self, this_as_u8: *mut u8) {
    let offset = core::ptr::read_volatile(self.0 as *const _ as *const usize);
    self.0 = core::mem::transmute(this_as_u8.add(offset));
  }

  #[inline(always)]
  pub unsafe fn construct_with_value(&mut self, this_as_u8: *mut u8, value: T) {
    let offset = core::ptr::read_volatile(self.0 as *const _ as *const usize);
    self.0 = core::mem::transmute(this_as_u8.add(offset));
    core::ptr::write(self.0, value);
  }

  #[inline(always)]
  pub unsafe fn destruct(&mut self, this_as_u8: *mut u8) {
    let offset = core::ptr::read_volatile(self.0 as *const _ as *const usize);
    self.0 = core::mem::transmute(this_as_u8.add(offset));
    core::ptr::drop_in_place(self.0);
  }

  #[inline(always)]
  pub fn as_ref(&self) -> &T {
    return unsafe { &*self.0 };
  }

  #[inline(always)]
  pub fn as_mut(&mut self) -> &mut T {
    return unsafe { &mut *self.0 };
  }
}

impl<T: core::default::Default> Field<T> {
  #[inline(always)]
  pub unsafe fn construct(&mut self, this_as_u8: *mut u8) {
    Self::construct_with_value(self, this_as_u8, core::default::Default::default());
  }
}

pub struct ThisAndFields<T: ?Sized, F> {
  pub this: *mut T,
  pub fields: F,
}

impl<T: ?Sized, F> ThisAndFields<T, F> {
  #[inline(always)]
  pub unsafe fn extend_ref<'a, 'b: 'a>(&'a self, _: &'b T) -> &'b Self {
    return core::mem::transmute(self);
  }

  #[inline(always)]
  pub unsafe fn extend_mut<'a, 'b: 'a>(&'a mut self, _: &'b mut T) -> &'b mut Self {
    return core::mem::transmute(self);
  }

  #[inline(always)]
  pub fn as_ref(&self) -> &T {
    return unsafe { &*self.this };
  }

  #[inline(always)]
  pub fn as_mut(&mut self) -> &mut T {
    return unsafe { &mut *self.this };
  }
}

pub const fn align_log_2<T>() -> u32 {
  return log_2(core::mem::align_of::<T>());
}

// impl<T, F> core::ops::Deref for ThisAndFields<T, F>
// where
//   T: ?Sized + runtime::__objrs::NonRootClass + core::ops::Deref,
//   <T as core::ops::Deref>::Target: Fields + runtime::__objrs::Class,
//   F: ?Sized,
// {
//   type Target = ThisAndFields<
//     <T as core::ops::Deref>::Target,
//     <<T as core::ops::Deref>::Target as Fields>::Type,
//   >;

//   #[inline(always)]
//   fn deref(&self) -> &Self::Target {
//     return Fields::new(core::ops::Deref::deref(core::convert::AsRef::as_ref(self)));
//   }
// }

// impl<T, F> core::ops::DerefMut for ThisAndFields<T, F>
// where
//   T: ?Sized + runtime::__objrs::NonRootClass + core::ops::Deref + core::ops::DerefMut,
//   <T as core::ops::Deref>::Target: Fields + runtime::__objrs::Class,
//   F: ?Sized,
// {
//   #[inline(always)]
//   fn deref_mut(&mut self) -> &mut Self::Target {
//     return Fields::new(core::ops::DerefMut::deref_mut(core::convert::AsMut::as_mut(self)));
//   }
// }

pub trait Fields {
  type Type;

  fn from_ref(&self) -> ThisAndFields<Self, Self::Type>;
  fn from_ptr(this: *mut Self) -> ThisAndFields<Self, Self::Type>;
}

pub trait RequiresCxxConstruct {
  const VALUE: bool;
}

impl<T: ?Sized> RequiresCxxConstruct for T {
  default const VALUE: bool = true;
}

impl<T: ?Sized + marker::Zeroed> RequiresCxxConstruct for T {
  const VALUE: bool = false;
}

pub trait IsCopy {
  const VALUE: bool;
}

impl<T: ?Sized> IsCopy for T {
  default const VALUE: bool = false;
}

impl<T: ?Sized + Copy> IsCopy for T {
  const VALUE: bool = true;
}

pub trait NeedsDrop {
  const VALUE: bool;
}

impl<T: ?Sized> NeedsDrop for T {
  default const VALUE: bool = true;
}

impl<T> NeedsDrop for T {
  const VALUE: bool = core::mem::needs_drop::<T>();
}

pub trait RequiresCxxDestruct {
  const VALUE: bool;
}

impl<T: ?Sized + NeedsDrop + IsCopy> RequiresCxxDestruct for T {
  default const VALUE: bool = <T as NeedsDrop>::VALUE && !<T as IsCopy>::VALUE;
}

impl<T: ?Sized + NeedsDrop + IsCopy + marker::Forgettable> RequiresCxxDestruct for T {
  const VALUE: bool = false;
}

pub trait IsWeak {
  const VALUE: bool;
}

impl<T: ?Sized> IsWeak for T {
  default const VALUE: bool = false;
}

impl<T: ?Sized + marker::Weak> IsWeak for T {
  const VALUE: bool = true;
}

pub trait TypeEncodingHack {
  type Type: Copy + Send + Sync;
  const BYTES: Self::Type;
}

impl<T: ?Sized> TypeEncodingHack for T {
  default type Type = [u8; 1];
  default const BYTES: Self::Type = unsafe {
    TransmuteHack {
      from: [b'?'],
    }
    .to
  };
}

impl<T: ?Sized + TypeEncoding> TypeEncodingHack for T {
  type Type = <T as TypeEncoding>::Type;
  const BYTES: Self::Type = <T as TypeEncoding>::BYTES;
}

#[cfg(test)]
pub mod test {
  pub use super::super::test::*;
}
