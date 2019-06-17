// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

extern crate libc;

use crate::arc;
use crate::cstr;
use crate::marker;
use crate::runtime;

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

primitive_type_encoding! {
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
}

macro_rules! transparent_generic_type_encoding {
  ([where $($bounds:tt)*]) => {};
  ([where $($bounds:tt)*] $ty:ty => $primitive_ty:ty, [where $($new_bounds:tt)*] $($tt:tt)*) => {
    transparent_generic_type_encoding!{[where $($bounds)*] $ty => $primitive_ty,}
    transparent_generic_type_encoding!{[where $($new_bounds)*] $($tt)*}
  };

  ([where $($bounds:tt)*] $ty:ty => $primitive_ty:ty, $($tt:tt)*) => {
    unsafe impl<$($bounds)*> TypeEncoding for $ty where $primitive_ty: TypeEncoding {
      type Type = <$primitive_ty as TypeEncoding>::Type;
      const BYTES: Self::Type = <$primitive_ty as TypeEncoding>::BYTES;
    }
    transparent_generic_type_encoding!{[where $($bounds)*] $($tt)*}
  };
}

macro_rules! transparent_type_encoding {
  () => {};
  ($ty:ty => $primitive_ty:ty, $($tt:tt)*) => {
    transparent_generic_type_encoding!{
      [where]
      $ty => $primitive_ty,
      $($tt)*
    }
  };
}

transparent_generic_type_encoding! {
  [where T: ?Sized]
  core::ptr::NonNull<T> => *mut T,
  Option<&T> => *const T,
  Option<&mut T> => *mut T,
  Option<core::ptr::NonNull<T>> => *mut T,
  core::cell::Cell<T> => T,
  core::cell::UnsafeCell<T> => T,
  core::mem::ManuallyDrop<T> => T,

  [where T: Sized]
  core::mem::MaybeUninit<T> => T,
  core::num::Wrapping<T> => T,

  [where T: marker::Class + ?Sized]
  arc::Strong<T> => *mut T,
  arc::Auto<T> => *mut T,
}

#[cfg(feature = "alloc")]
transparent_generic_type_encoding! {
  [where T: marker::Class + ?Sized]
  arc::Weak<T> => *mut T,
}

transparent_type_encoding! {
  core::num::NonZeroU8 => u8,
  core::num::NonZeroU16 => u16,
  core::num::NonZeroU32 => u32,
  core::num::NonZeroU64 => u64,
  core::num::NonZeroU128 => u128,
  core::num::NonZeroUsize => usize,

  core::num::NonZeroI8 => i8,
  core::num::NonZeroI16 => i16,
  core::num::NonZeroI32 => i32,
  core::num::NonZeroI64 => i64,
  core::num::NonZeroI128 => i128,
  core::num::NonZeroIsize => isize,

  Option<core::num::NonZeroU8> => u8,
  Option<core::num::NonZeroU16> => u16,
  Option<core::num::NonZeroU32> => u32,
  Option<core::num::NonZeroU64> => u64,
  Option<core::num::NonZeroU128> => u128,
  Option<core::num::NonZeroUsize> => usize,

  Option<core::num::NonZeroI8> => i8,
  Option<core::num::NonZeroI16> => i16,
  Option<core::num::NonZeroI32> => i32,
  Option<core::num::NonZeroI64> => i64,
  Option<core::num::NonZeroI128> => i128,
  Option<core::num::NonZeroIsize> => isize,
}

macro_rules! atomic_type_encoding {
  () => {};
  ($ty:ty => $primitive_ty:ty, $($tt:tt)*) => {
    unsafe impl TypeEncoding for $ty {
      type Type = [u8; 2];
      const BYTES: Self::Type = unsafe { core::mem::transmute(crate::__objrs::Packed2(b'A', <$primitive_ty as TypeEncoding>::BYTES)) };
    }

    atomic_type_encoding!{$($tt)*}
  };
}

atomic_type_encoding! {
  core::sync::atomic::AtomicU8 => u8,
  core::sync::atomic::AtomicU16 => u16,
  core::sync::atomic::AtomicU32 => u32,
  core::sync::atomic::AtomicU64 => u64,
  core::sync::atomic::AtomicI8 => i8,
  core::sync::atomic::AtomicI16 => i16,
  core::sync::atomic::AtomicI32 => i32,
  core::sync::atomic::AtomicI64 => i64,
  core::sync::atomic::AtomicUsize => usize,
  core::sync::atomic::AtomicIsize => isize,
  core::sync::atomic::AtomicBool => bool,

  // TODO: Fix AtomicPtr so it can point to unsized types (in particular, extern types).
  // TODO: if T is a class/object, this should be a different type.
  // unsafe impl<T> TypeEncoding for core::sync::atomic::AtomicPtr<T> {}
}

macro_rules! special_pointer_type_encoding {
  () => {};
  ($ty:ty => $encoding:expr, $($tt:tt)*) => {
    primitive_type_encoding!{
      *const $ty => $encoding,
      *mut $ty => $encoding,
      &$ty => $encoding,
      &mut $ty => $encoding,
    }

    special_pointer_type_encoding!{$($tt)*}
  };
}

special_pointer_type_encoding! {
  runtime::Sel => b':',
  runtime::Id => b'@',
  runtime::Class => b'#',
  cstr::CStr => b'*',
}

// transparent_generic_type_encoding! {
//   [where T: marker::Class + ?Sized]
//   *const T => *const runtime::Id,
//   *mut T => *mut runtime::Id,
//   &T => &'static runtime::Id,
//   &mut T => &'static mut runtime::Id,
// }

// TODO: The const should only apply to the outer-most pointer. e.g. *const *const _ should be "r^^", not "r^r^".
// unsafe impl<T: TypeEncoding + ?Sized> TypeEncoding for *const T {
//   type Type = [u8; 2 + core::mem::size_of::<<T as TypeEncoding>::Type>()];
//   const BYTES: Self::Type = unsafe { core::mem::transmute(__objrs::Packed2([b'r', b'^'], <T as TypeEncoding>::BYTES)) };
// }

// unsafe impl<T: TypeEncoding + ?Sized> TypeEncoding for *mut T {
//   type Type = [u8; 1 + core::mem::size_of::<<T as TypeEncoding>::Type>()];
//   const BYTES: Self::Type = unsafe { core::mem::transmute(__objrs::Packed2(b'^', <T as TypeEncoding>::BYTES)) };
// }

// TODO: The const should only apply to the outer-most pointer. e.g. && _ should be "r^^", not "r^r^".
// unsafe impl<'a, T: TypeEncoding + ?Sized> TypeEncoding for &'a T {
//   type Type = [u8; 2 + core::mem::size_of::<<T as TypeEncoding>::Type>()];
//   const BYTES: Self::Type = unsafe { core::mem::transmute(__objrs::Packed2([b'r', b'^'], <T as TypeEncoding>::BYTES)) };
// }

// unsafe impl<'a, T: TypeEncoding + ?Sized> TypeEncoding for &'a mut T {
//   type Type = [u8; 1 + core::mem::size_of::<<T as TypeEncoding>::Type>()];
//   const BYTES: Self::Type = unsafe { core::mem::transmute(__objrs::Packed2(b'^', <T as TypeEncoding>::BYTES)) };
// }

// unsafe impl<T: ?Sized> TypeEncoding for [T; 0] {}
// unsafe impl<T: TypeEncoding + ?Sized> TypeEncoding for [T; 1] {}
// ...

// Types that need to be made repr(transparent) first:
// core::cmp::Reverse
// core::cell::Ref?
// core::cell::RefCell?
// core::cell::RefMut?

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
