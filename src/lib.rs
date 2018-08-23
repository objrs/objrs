// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

#![feature(extern_types, specialization)]
#![no_std]

pub mod marker;
#[cfg(test)]
mod test;

extern crate libc;
#[allow(unused_imports)]
#[macro_use]
extern crate objrs_macros;

pub extern crate objrs_runtime as runtime;

#[doc(hidden)]
pub use objrs_macros::*;

pub unsafe trait TypeEncoding {
  type Type: Copy + core::cmp::PartialEq + core::fmt::Debug + Send + Sync;
  const BYTES: Self::Type;
}

macro_rules! primitive_type_encoding {
  () => {};
  ($ty:ty => $encoding:expr, $($tt:tt)*) => {
    unsafe impl TypeEncoding for $ty {
      type Type = [u8; 1];
      const BYTES: Self::Type = [$encoding];
    }

    primitive_type_encoding!{$($tt)*}
  };
}

primitive_type_encoding!{
  u8 => b'C',
  u16 => b'S',
  u32 => b'I',
  u64 => b'Q',
  u128 => b'T',

  i8 => b'c',
  i16 => b's',
  i32 => b'i',
  i64 => b'q',
  i128 => b't',

  bool => b'B',

  char => b'I',  // Same as u32.

  usize => b'A' + 2 * core::mem::size_of::<usize>() as u8,  // 'I' or 'Q'.
  isize => b'a' + 2 * core::mem::size_of::<usize>() as u8,  // 'i' or 'q'.

  f32 => b'f',
  f64 => b'd',

  () => b'v',
  libc::c_void => b'v',

  core::num::NonZeroU8 => b'C',
  core::num::NonZeroU16 => b'S',
  core::num::NonZeroU32 => b'I',
  core::num::NonZeroU64 => b'Q',
  core::num::NonZeroU128 => b'T',
  core::num::NonZeroUsize => b'A' + 2 * core::mem::size_of::<usize>() as u8,  // 'I' or 'Q'.

  Option<core::num::NonZeroU8> => b'C',
  Option<core::num::NonZeroU16> => b'S',
  Option<core::num::NonZeroU32> => b'I',
  Option<core::num::NonZeroU64> => b'Q',
  Option<core::num::NonZeroU128> => b'T',
  Option<core::num::NonZeroUsize> => b'A' + 2 * core::mem::size_of::<usize>() as u8,  // 'I' or 'Q'.
}

macro_rules! transparent_type_encoding {
  () => {};
  ($ty:ty => $primitive_ty:ty, $($tt:tt)*) => {
    unsafe impl<'a, T: TypeEncoding + ?Sized> TypeEncoding for $ty {
      type Type = <$primitive_ty as TypeEncoding>::Type;
      const BYTES: Self::Type = <$primitive_ty as TypeEncoding>::BYTES;
    }

    transparent_type_encoding!{$($tt)*}
  };
}

transparent_type_encoding!{
  // core::ptr::NonNull<T> => *mut T,

  // Option<&'a T> => *const T,
  // Option<&'a mut T> => *mut T,
  // Option<core::ptr::NonNull<T>> => *mut T,

  // core::cell::Cell<T> => T,
  // core::cell::UnsafeCell<T> => T,
}

// unsafe impl<T: TypeEncoding + ?Sized> TypeEncoding for core::num::Wrapping<T> {
//   type Type = <T as TypeEncoding>::Type;
//   const BYTES: Self::Type = <T as TypeEncoding>::BYTES;
// }

macro_rules! atomic_type_encoding {
  () => {};
  ($ty:ty => $primitive_ty:ty, $($tt:tt)*) => {
    unsafe impl TypeEncoding for $ty {
      type Type = [u8; 2];
      const BYTES: Self::Type = unsafe { __objrs::TransmuteHack { from: __objrs::Packed2(b'A', <$primitive_ty as TypeEncoding>::BYTES) }.to };
    }

    atomic_type_encoding!{$($tt)*}
  };
}

atomic_type_encoding! {
  // Requires integer_atomics.
  // core::sync::atomic::AtomicU8
  // core::sync::atomic::AtomicU16
  // core::sync::atomic::AtomicU32
  // core::sync::atomic::AtomicU64
  // core::sync::atomic::AtomicI8
  // core::sync::atomic::AtomicI16
  // core::sync::atomic::AtomicI32
  // core::sync::atomic::AtomicI64

  // TODO: this depends on making Atomic* repr(transparent).
  core::sync::atomic::AtomicUsize => usize,
  core::sync::atomic::AtomicIsize => isize,
  core::sync::atomic::AtomicBool => bool,
  // unsafe impl<T> TypeEncoding for core::sync::atomic::AtomicPtr<T> {}
}

// TODO: char*, id, Class, SEL

// TODO: The const should only apply to the outer-most pointer. e.g. *const *const _ should be "r^^", not "r^r^".
// unsafe impl<T: TypeEncoding> TypeEncoding for *const T {
//   type Type = [u8; 2 + core::mem::size_of::<<T as TypeEncoding>::Type>()];
//   const BYTES: Self::Type = unsafe { __objrs::TransmuteHack { from: __objrs::Packed2([b'r', b'^'], <T as TypeEncoding>::BYTES) }.to };
// }

// unsafe impl<T: TypeEncoding> TypeEncoding for *mut T {
//   type Type = [u8; 1 + core::mem::size_of::<<T as TypeEncoding>::Type>()];
//   const BYTES: Self::Type = unsafe { __objrs::TransmuteHack { from: __objrs::Packed2(b'^', <T as TypeEncoding>::BYTES) }.to };
// }

// TODO: The const should only apply to the outer-most pointer. e.g. && _ should be "r^^", not "r^r^".
// unsafe impl<'a, T: TypeEncoding> TypeEncoding for &'a T {
//   type Type = [u8; 2 + core::mem::size_of::<<T as TypeEncoding>::Type>()];
//   const BYTES: Self::Type = unsafe { __objrs::TransmuteHack { from: __objrs::Packed2([b'r', b'^'], <T as TypeEncoding>::BYTES) }.to };
// }

// unsafe impl<'a, T: TypeEncoding> TypeEncoding for &'a mut T {
//   type Type = [u8; 1 + core::mem::size_of::<<T as TypeEncoding>::Type>()];
//   const BYTES: Self::Type = unsafe { __objrs::TransmuteHack { from: __objrs::Packed2(b'^', <T as TypeEncoding>::BYTES) }.to };
// }

// // unsafe impl<T> TypeEncoding for [T; 0] {}
// // unsafe impl<T: TypeEncoding> TypeEncoding for [T; 1] {}
// // ...

// // Types that need to be made repr(transparent) first:
// // core::mem::ManuallyDrop
// // core::cmp::Reverse
// // core::cell::Ref?
// // core::cell::RefCell?
// // core::cell::RefMut?

#[repr(transparent)]
pub struct Alloc<T: marker::Class + ?Sized>(Strong<T>);

// unsafe impl<T: marker::Class + ?Sized> marker::Class for Alloc<T> {}
// unsafe impl<T: marker::RootClass + ?Sized> marker::RootClass for Alloc<T> {}
// unsafe impl<T: marker::NonRootClass + ?Sized> marker::NonRootClass for Alloc<T> {
//   type Super = <T as marker::NonRootClass>::Super;
// }

// Deref is implemented for Alloc to allow `fn init(self: Alloc<T>) -> Strong<T>`.
impl<T: marker::Class + ?Sized> core::ops::Deref for Alloc<T> {
  type Target = *mut T;

  #[inline(always)]
  fn deref(&self) -> &*mut T {
    return unsafe { core::mem::transmute(self) };
  }
}

#[repr(transparent)]
pub struct Strong<T: marker::Class + ?Sized>(core::ptr::NonNull<T>);

impl<T: marker::Class + ?Sized> Strong<T> {
  pub fn autorelease(self) -> Auto<T> {
    let ptr = self.0.as_ptr();
    core::mem::forget(self);
    return Auto(unsafe {
      __objrs::TransmuteHack {
        from: runtime::objc_autorelease(runtime::id(ptr as *mut _)),
      }.to
    });
  }
}

// unsafe impl<T: marker::Class + ?Sized> marker::Class for Strong<T> {}
// unsafe impl<T: marker::RootClass + ?Sized> marker::RootClass for Strong<T> {}
// unsafe impl<T: marker::NonRootClass + ?Sized> marker::NonRootClass for Strong<T> {
//   type Super = <T as marker::NonRootClass>::Super;
// }

// See https://github.com/rust-lang/rust/issues/47442
impl<T: marker::Class + ?Sized> core::ops::Drop for Strong<T> {
  #[inline(always)]
  fn drop(&mut self) {
    unsafe {
      runtime::objc_release(runtime::id(self.0.as_ptr() as *mut _));
    }
  }
}

impl<T: marker::Class + ?Sized> core::ops::Deref for Strong<T> {
  type Target = T;

  #[inline(always)]
  fn deref(&self) -> &T {
    return unsafe { self.0.as_ref() };
  }
}

impl<T: marker::Class + ?Sized> core::ops::DerefMut for Strong<T> {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut T {
    return unsafe { self.0.as_mut() };
  }
}

impl<T: marker::Class + ?Sized> core::convert::AsRef<runtime::id> for Strong<T> {
  #[inline(always)]
  fn as_ref(&self) -> &runtime::id {
    return unsafe { core::mem::transmute(self) };
  }
}

impl<T: marker::Class + ?Sized> core::convert::AsMut<runtime::id> for Strong<T> {
  #[inline(always)]
  fn as_mut(&mut self) -> &mut runtime::id {
    return unsafe { core::mem::transmute(self) };
  }
}

impl<T: marker::NonRootClass + marker::Class + ?Sized> Strong<T> {
  #[inline(always)]
  pub fn into_super(self) -> Strong<<T as marker::NonRootClass>::Super> {
    let ptr = self.0.as_ptr();
    core::mem::forget(self);
    return Strong(unsafe {
      __objrs::TransmuteHack {
        from: ptr,
      }.to
    });
  }
}

#[repr(transparent)]
pub struct Auto<T: marker::Class + ?Sized>(core::ptr::NonNull<T>);

impl<T: marker::Class + ?Sized> Auto<T> {
  pub fn retain(self) -> Strong<T> {
    let ptr = self.0.as_ptr();
    core::mem::forget(self);
    return Strong(unsafe {
      __objrs::TransmuteHack {
        from: runtime::objc_retain(runtime::id(ptr as *mut _)),
      }.to
    });
  }
}

// unsafe impl<T: marker::Class + ?Sized> marker::Class for Auto<T> {}
// unsafe impl<T: marker::RootClass + ?Sized> marker::RootClass for Auto<T> {}
// unsafe impl<T: marker::NonRootClass + ?Sized> marker::NonRootClass for Auto<T> {
//   type Super = <T as marker::NonRootClass>::Super;
// }

impl<T: marker::Class + ?Sized> core::ops::Deref for Auto<T> {
  type Target = T;

  #[inline(always)]
  fn deref(&self) -> &T {
    return unsafe { self.0.as_ref() };
  }
}

impl<T: marker::Class + ?Sized> core::ops::DerefMut for Auto<T> {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut T {
    return unsafe { self.0.as_mut() };
  }
}

impl<T: marker::Class + ?Sized> core::convert::AsRef<runtime::id> for Auto<T> {
  #[inline(always)]
  fn as_ref(&self) -> &runtime::id {
    return unsafe { core::mem::transmute(self) };
  }
}

impl<T: marker::Class + ?Sized> core::convert::AsMut<runtime::id> for Auto<T> {
  #[inline(always)]
  fn as_mut(&mut self) -> &mut runtime::id {
    return unsafe { core::mem::transmute(self) };
  }
}

impl<T: marker::NonRootClass + marker::Class + ?Sized> Auto<T> {
  #[inline(always)]
  pub fn into_super(self) -> Auto<<T as marker::NonRootClass>::Super> {
    let ptr = self.0.as_ptr();
    core::mem::forget(self);
    return Auto(unsafe {
      __objrs::TransmuteHack {
        from: ptr,
      }.to
    });
  }
}

#[repr(transparent)]
pub struct Weak<T: marker::Class + ?Sized>(*mut T);

// unsafe impl<T: marker::Class + ?Sized> marker::Class for Weak<T> {}
// unsafe impl<T: marker::RootClass + ?Sized> marker::RootClass for Weak<T> {}
// unsafe impl<T: marker::NonRootClass + ?Sized> marker::NonRootClass for Weak<T> {
//   type Super = <T as marker::NonRootClass>::Super;
// }
unsafe impl<T: marker::Class + ?Sized> marker::Weak for Weak<T> {}

// This is broken by https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md
// #[repr(transparent)]
// pub struct Id<T>(core::marker::PhantomData<T>, Opaque) where T: marker::Protocol + ?Sized;
// unsafe impl<T> marker::Class for Id<T> where  T: marker::Protocol + ?Sized {}

#[repr(transparent)]
pub struct CStr(__objrs::Opaque);

impl CStr {
  pub unsafe fn new(string: &str) -> &CStr {
    return Self::with_bytes(string.as_bytes());
  }

  pub unsafe fn with_bytes(bytes: &[u8]) -> &CStr {
    assert_eq!(bytes.last(), Some(&b'\0'));
    return core::mem::transmute(bytes.as_ptr());
  }

  #[inline(always)]
  pub fn as_ptr(&self) -> *const libc::c_char {
    return unsafe { core::mem::transmute(self) };
  }
}

#[doc(hidden)]
pub mod __objrs {
  use super::*;

  extern "C" {
    pub type Opaque;
  }
  unsafe impl Sync for Opaque {}

  pub extern crate core;
  pub extern crate libc;

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
  pub struct Field<T>(pub *mut T);

  impl<T> Field<T> {
    #[inline(always)]
    pub fn load(&mut self, this_as_u8: *mut u8) {
      let offset = unsafe { core::ptr::read_volatile(self.0 as *const _ as *const usize) };
      self.0 = unsafe { core::mem::transmute(this_as_u8.add(offset)) };
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

  pub trait RequiresCxxDestruct {
    const VALUE: bool;
  }

  impl<T: ?Sized + IsCopy> RequiresCxxDestruct for T {
    default const VALUE: bool = !<T as IsCopy>::VALUE;
  }

  impl<T: ?Sized + IsCopy + marker::Forgettable> RequiresCxxDestruct for T {
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
      __objrs::TransmuteHack {
        from: [b'?'],
      }.to
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
}

#[cfg(test)]
#[test]
fn type_encoding_test() {
  assert_eq!(<u8 as TypeEncoding>::BYTES, *b"C");
  assert_eq!(<u16 as TypeEncoding>::BYTES, *b"S");
  assert_eq!(<u32 as TypeEncoding>::BYTES, *b"I");
  assert_eq!(<u64 as TypeEncoding>::BYTES, *b"Q");
  assert_eq!(<u128 as TypeEncoding>::BYTES, *b"T");

  assert_eq!(<i8 as TypeEncoding>::BYTES, *b"c");
  assert_eq!(<i16 as TypeEncoding>::BYTES, *b"s");
  assert_eq!(<i32 as TypeEncoding>::BYTES, *b"i");
  assert_eq!(<i64 as TypeEncoding>::BYTES, *b"q");
  assert_eq!(<i128 as TypeEncoding>::BYTES, *b"t");

  assert_eq!(<bool as TypeEncoding>::BYTES, *b"B");

  assert_eq!(<char as TypeEncoding>::BYTES, *b"I");

  // assert_eq!(<usize as TypeEncoding>::BYTES, *b"A");
  // assert_eq!(<isize as TypeEncoding>::BYTES, *b"a");

  assert_eq!(<f32 as TypeEncoding>::BYTES, *b"f");
  assert_eq!(<f64 as TypeEncoding>::BYTES, *b"d");

  assert_eq!(<() as TypeEncoding>::BYTES, *b"v");
  assert_eq!(<libc::c_void as TypeEncoding>::BYTES, *b"v");

  assert_eq!(<core::num::NonZeroU8 as TypeEncoding>::BYTES, *b"C");
  assert_eq!(<core::num::NonZeroU16 as TypeEncoding>::BYTES, *b"S");
  assert_eq!(<core::num::NonZeroU32 as TypeEncoding>::BYTES, *b"I");
  assert_eq!(<core::num::NonZeroU64 as TypeEncoding>::BYTES, *b"Q");
  assert_eq!(<core::num::NonZeroU128 as TypeEncoding>::BYTES, *b"T");
  // assert_eq!(<core::num::NonZeroUsize as TypeEncoding>::BYTES, *b"A");

  assert_eq!(<Option<core::num::NonZeroU8> as TypeEncoding>::BYTES, *b"C");
  assert_eq!(<Option<core::num::NonZeroU16> as TypeEncoding>::BYTES, *b"S");
  assert_eq!(<Option<core::num::NonZeroU32> as TypeEncoding>::BYTES, *b"I");
  assert_eq!(<Option<core::num::NonZeroU64> as TypeEncoding>::BYTES, *b"Q");
  assert_eq!(<Option<core::num::NonZeroU128> as TypeEncoding>::BYTES, *b"T");
  // assert_eq!(<Option<core::num::NonZeroUsize> as TypeEncoding>::BYTES, *b"A");

  // assert_eq!(<*mut u8 as TypeEncoding>::BYTES, *b"^C");
  // assert_eq!(<*mut *mut u8 as TypeEncoding>::BYTES, *b"^^C");
  // assert_eq!(<*const u8 as TypeEncoding>::BYTES, *b"r^C");
  // assert_eq!(<*const *const u8 as TypeEncoding>::BYTES, *b"r^^C");

  // assert_eq!(<&'static mut u8 as TypeEncoding>::BYTES, *b"^C");
  // assert_eq!(<&'static mut &'static mut u8 as TypeEncoding>::BYTES, *b"^^C");
  // assert_eq!(<&'static u8 as TypeEncoding>::BYTES, *b"r^C");
  // assert_eq!(<&'static &'static u8 as TypeEncoding>::BYTES, *b"r^^C");

  // core::ptr::NonNull<T>

  // Option<&'a T>
  // Option<&'a mut T>
  // Option<core::ptr::NonNull<T>>

  // assert_eq!(<core::cell::Cell<u8> as TypeEncoding>::BYTES, <u8 as TypeEncoding>::BYTES);
  // assert_eq!(<core::cell::Cell<u16> as TypeEncoding>::BYTES, <u16 as TypeEncoding>::BYTES);
  // assert_eq!(<core::cell::Cell<u32> as TypeEncoding>::BYTES, <u32 as TypeEncoding>::BYTES);
  // assert_eq!(<core::cell::Cell<u64> as TypeEncoding>::BYTES, <u64 as TypeEncoding>::BYTES);
  // assert_eq!(<core::cell::Cell<u128> as TypeEncoding>::BYTES, <u128 as TypeEncoding>::BYTES);

  // assert_eq!(<core::cell::UnsafeCell<u8> as TypeEncoding>::BYTES, <u8 as TypeEncoding>::BYTES);
  // assert_eq!(<core::cell::UnsafeCell<u16> as TypeEncoding>::BYTES, <u16 as TypeEncoding>::BYTES);
  // assert_eq!(<core::cell::UnsafeCell<u32> as TypeEncoding>::BYTES, <u32 as TypeEncoding>::BYTES);
  // assert_eq!(<core::cell::UnsafeCell<u64> as TypeEncoding>::BYTES, <u64 as TypeEncoding>::BYTES);
  // assert_eq!(<core::cell::UnsafeCell<u128> as TypeEncoding>::BYTES, <u128 as TypeEncoding>::BYTES);

  // core::sync::atomic::AtomicUsize => usize,
  // core::sync::atomic::AtomicIsize => isize,
  // core::sync::atomic::AtomicBool => bool,
  // core::sync::atomic::AtomicPtr<T>

  // assert_eq!(<core::num::Wrapping<u8> as TypeEncoding>::BYTES, <u8 as TypeEncoding>::BYTES);
  // assert_eq!(<core::num::Wrapping<u16> as TypeEncoding>::BYTES, <u16 as TypeEncoding>::BYTES);
  // assert_eq!(<core::num::Wrapping<u32> as TypeEncoding>::BYTES, <u32 as TypeEncoding>::BYTES);
  // assert_eq!(<core::num::Wrapping<u64> as TypeEncoding>::BYTES, <u64 as TypeEncoding>::BYTES);
  // assert_eq!(<core::num::Wrapping<u128> as TypeEncoding>::BYTES, <u128 as TypeEncoding>::BYTES);

  // assert_eq!(<core::num::Wrapping<i8> as TypeEncoding>::BYTES, <i8 as TypeEncoding>::BYTES);
  // assert_eq!(<core::num::Wrapping<i16> as TypeEncoding>::BYTES, <i16 as TypeEncoding>::BYTES);
  // assert_eq!(<core::num::Wrapping<i32> as TypeEncoding>::BYTES, <i32 as TypeEncoding>::BYTES);
  // assert_eq!(<core::num::Wrapping<i64> as TypeEncoding>::BYTES, <i64 as TypeEncoding>::BYTES);
  // assert_eq!(<core::num::Wrapping<i128> as TypeEncoding>::BYTES, <i128 as TypeEncoding>::BYTES);

  // assert_eq!(<core::num::Wrapping<usize> as TypeEncoding>::BYTES, <usize as TypeEncoding>::BYTES);
  // assert_eq!(<core::num::Wrapping<isize> as TypeEncoding>::BYTES, <isize as TypeEncoding>::BYTES);
}
