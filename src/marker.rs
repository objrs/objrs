// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

extern crate core;

pub unsafe trait Class {}

pub unsafe trait RootClass: Class {}

pub unsafe trait NonRootClass: Class {
  type Super: Class + ?Sized;
}

// The Protocol trait is unusable due to https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md
// pub unsafe trait Protocol {}

// Types that implement Weak are considered Weak types (needed for class RO weakIvarLayout).
pub unsafe trait Weak {}

// Safe to initialize via core::mem::zeroed.
// TODO: maybe rename to Zeroable?
pub unsafe trait Zeroed {}

unsafe impl Zeroed for u8 {}
unsafe impl Zeroed for u16 {}
unsafe impl Zeroed for u32 {}
unsafe impl Zeroed for u64 {}
unsafe impl Zeroed for u128 {}

unsafe impl Zeroed for i8 {}
unsafe impl Zeroed for i16 {}
unsafe impl Zeroed for i32 {}
unsafe impl Zeroed for i64 {}
unsafe impl Zeroed for i128 {}

unsafe impl Zeroed for usize {}
unsafe impl Zeroed for isize {}

unsafe impl Zeroed for bool {}

unsafe impl Zeroed for char {}

unsafe impl Zeroed for f32 {}
unsafe impl Zeroed for f64 {}

unsafe impl<T> Zeroed for *const T {}
unsafe impl<T> Zeroed for *mut T {}

unsafe impl<T> Zeroed for [T; 0] {}
unsafe impl<T: Zeroed> Zeroed for [T; 1] {}
unsafe impl<T: Zeroed> Zeroed for [T; 2] {}
unsafe impl<T: Zeroed> Zeroed for [T; 3] {}
unsafe impl<T: Zeroed> Zeroed for [T; 4] {}
unsafe impl<T: Zeroed> Zeroed for [T; 5] {}
unsafe impl<T: Zeroed> Zeroed for [T; 6] {}
unsafe impl<T: Zeroed> Zeroed for [T; 7] {}
unsafe impl<T: Zeroed> Zeroed for [T; 8] {}
unsafe impl<T: Zeroed> Zeroed for [T; 9] {}
unsafe impl<T: Zeroed> Zeroed for [T; 10] {}
unsafe impl<T: Zeroed> Zeroed for [T; 11] {}
unsafe impl<T: Zeroed> Zeroed for [T; 12] {}
unsafe impl<T: Zeroed> Zeroed for [T; 13] {}
unsafe impl<T: Zeroed> Zeroed for [T; 14] {}
unsafe impl<T: Zeroed> Zeroed for [T; 15] {}
unsafe impl<T: Zeroed> Zeroed for [T; 16] {}
unsafe impl<T: Zeroed> Zeroed for [T; 17] {}
unsafe impl<T: Zeroed> Zeroed for [T; 18] {}
unsafe impl<T: Zeroed> Zeroed for [T; 19] {}
unsafe impl<T: Zeroed> Zeroed for [T; 20] {}
unsafe impl<T: Zeroed> Zeroed for [T; 21] {}
unsafe impl<T: Zeroed> Zeroed for [T; 22] {}
unsafe impl<T: Zeroed> Zeroed for [T; 23] {}
unsafe impl<T: Zeroed> Zeroed for [T; 24] {}
unsafe impl<T: Zeroed> Zeroed for [T; 25] {}
unsafe impl<T: Zeroed> Zeroed for [T; 26] {}
unsafe impl<T: Zeroed> Zeroed for [T; 27] {}
unsafe impl<T: Zeroed> Zeroed for [T; 28] {}
unsafe impl<T: Zeroed> Zeroed for [T; 29] {}
unsafe impl<T: Zeroed> Zeroed for [T; 30] {}
unsafe impl<T: Zeroed> Zeroed for [T; 31] {}
unsafe impl<T: Zeroed> Zeroed for [T; 32] {}

unsafe impl Zeroed for () {}
unsafe impl<T0: Zeroed> Zeroed for (T0,) {}
unsafe impl<T0: Zeroed, T1: Zeroed> Zeroed for (T0, T1) {}
unsafe impl<T0: Zeroed, T1: Zeroed, T2: Zeroed> Zeroed for (T0, T1, T2) {}
unsafe impl<T0: Zeroed, T1: Zeroed, T2: Zeroed, T3: Zeroed> Zeroed for (T0, T1, T2, T3) {}
unsafe impl<T0: Zeroed, T1: Zeroed, T2: Zeroed, T3: Zeroed, T4: Zeroed> Zeroed
  for (T0, T1, T2, T3, T4)
{
}
unsafe impl<T0: Zeroed, T1: Zeroed, T2: Zeroed, T3: Zeroed, T4: Zeroed, T5: Zeroed> Zeroed
  for (T0, T1, T2, T3, T4, T5)
{
}
unsafe impl<T0: Zeroed, T1: Zeroed, T2: Zeroed, T3: Zeroed, T4: Zeroed, T5: Zeroed, T6: Zeroed>
  Zeroed for (T0, T1, T2, T3, T4, T5, T6)
{
}
unsafe impl<
    T0: Zeroed,
    T1: Zeroed,
    T2: Zeroed,
    T3: Zeroed,
    T4: Zeroed,
    T5: Zeroed,
    T6: Zeroed,
    T7: Zeroed,
  > Zeroed for (T0, T1, T2, T3, T4, T5, T6, T7)
{
}
unsafe impl<
    T0: Zeroed,
    T1: Zeroed,
    T2: Zeroed,
    T3: Zeroed,
    T4: Zeroed,
    T5: Zeroed,
    T6: Zeroed,
    T7: Zeroed,
    T8: Zeroed,
  > Zeroed for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
{
}
unsafe impl<
    T0: Zeroed,
    T1: Zeroed,
    T2: Zeroed,
    T3: Zeroed,
    T4: Zeroed,
    T5: Zeroed,
    T6: Zeroed,
    T7: Zeroed,
    T8: Zeroed,
    T9: Zeroed,
  > Zeroed for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
{
}
unsafe impl<
    T0: Zeroed,
    T1: Zeroed,
    T2: Zeroed,
    T3: Zeroed,
    T4: Zeroed,
    T5: Zeroed,
    T6: Zeroed,
    T7: Zeroed,
    T8: Zeroed,
    T9: Zeroed,
    T10: Zeroed,
  > Zeroed for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
{
}
unsafe impl<
    T0: Zeroed,
    T1: Zeroed,
    T2: Zeroed,
    T3: Zeroed,
    T4: Zeroed,
    T5: Zeroed,
    T6: Zeroed,
    T7: Zeroed,
    T8: Zeroed,
    T9: Zeroed,
    T10: Zeroed,
    T11: Zeroed,
  > Zeroed for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}

unsafe impl<T: Zeroed> Zeroed for core::num::Wrapping<T> {}

unsafe impl<T: ?Sized> Zeroed for core::marker::PhantomData<T> {}

unsafe impl<T: Zeroed> Zeroed for core::mem::ManuallyDrop<T> {}

unsafe impl<T: Zeroed> Zeroed for core::cmp::Reverse<T> {}

// core::cell::RefCell?
unsafe impl<T: Zeroed> Zeroed for core::cell::Cell<T> {}
unsafe impl<T: Zeroed> Zeroed for core::cell::UnsafeCell<T> {}

unsafe impl Zeroed for core::sync::atomic::AtomicU8 {}
unsafe impl Zeroed for core::sync::atomic::AtomicU16 {}
unsafe impl Zeroed for core::sync::atomic::AtomicU32 {}
unsafe impl Zeroed for core::sync::atomic::AtomicU64 {}
unsafe impl Zeroed for core::sync::atomic::AtomicI8 {}
unsafe impl Zeroed for core::sync::atomic::AtomicI16 {}
unsafe impl Zeroed for core::sync::atomic::AtomicI32 {}
unsafe impl Zeroed for core::sync::atomic::AtomicI64 {}
unsafe impl Zeroed for core::sync::atomic::AtomicUsize {}
unsafe impl Zeroed for core::sync::atomic::AtomicIsize {}
unsafe impl Zeroed for core::sync::atomic::AtomicBool {}
unsafe impl<T> Zeroed for core::sync::atomic::AtomicPtr<T> {}

unsafe impl<'a, T> Zeroed for Option<&'a T> {}
unsafe impl<'a, T> Zeroed for Option<&'a mut T> {}
unsafe impl<T> Zeroed for Option<core::ptr::NonNull<T>> {}
unsafe impl Zeroed for Option<core::num::NonZeroU8> {}
unsafe impl Zeroed for Option<core::num::NonZeroU16> {}
unsafe impl Zeroed for Option<core::num::NonZeroU32> {}
unsafe impl Zeroed for Option<core::num::NonZeroU64> {}
unsafe impl Zeroed for Option<core::num::NonZeroU128> {}
unsafe impl Zeroed for Option<core::num::NonZeroUsize> {}

unsafe impl<T> Zeroed for core::iter::Empty<T> {}

unsafe impl Zeroed for core::time::Duration {}

// Requires allocator_api.
// alloc::alloc::Global

// Requires stdsimd.
// core::simd::u8x2
// core::simd::u8x4
// core::simd::u8x8
// core::simd::u8x16
// core::simd::u8x32
// core::simd::u8x64
// core::simd::u16x2
// core::simd::u16x4
// core::simd::u16x8
// core::simd::u16x16
// core::simd::u16x32
// core::simd::u32x2
// core::simd::u32x4
// core::simd::u32x8
// core::simd::u32x16
// core::simd::u64x2
// core::simd::u64x4
// core::simd::u64x8
// core::simd::i8x2
// core::simd::i8x4
// core::simd::i8x8
// core::simd::i8x16
// core::simd::i8x32
// core::simd::i8x64
// core::simd::i16x2
// core::simd::i16x4
// core::simd::i16x8
// core::simd::i16x16
// core::simd::i16x32
// core::simd::i32x2
// core::simd::i32x4
// core::simd::i32x8
// core::simd::i32x16
// core::simd::i64x2
// core::simd::i64x4
// core::simd::i64x8
// core::simd::m1x8
// core::simd::m1x16
// core::simd::m1x32
// core::simd::m1x64
// core::simd::m8x2
// core::simd::m8x4
// core::simd::m8x8
// core::simd::m8x16
// core::simd::m8x32
// core::simd::m16x2
// core::simd::m16x4
// core::simd::m16x8
// core::simd::m16x16
// core::simd::m32x2
// core::simd::m32x4
// core::simd::m32x8
// core::simd::m64x2
// core::simd::m64x4
// core::simd::f32x2
// core::simd::f32x4
// core::simd::f32x8
// core::simd::f32x16
// core::simd::f64x2
// core::simd::f64x4
// core::simd::f64x8

#[cfg(target_arch = "x86")]
unsafe impl Zeroed for core::arch::x86::CpuidResult {}
// Requires stdsimd.
// unsafe impl Zeroed for core::arch::x86::__m64 {}
#[cfg(target_arch = "x86")]
unsafe impl Zeroed for core::arch::x86::__m128i {}
#[cfg(target_arch = "x86")]
unsafe impl Zeroed for core::arch::x86::__m128 {}
#[cfg(target_arch = "x86")]
unsafe impl Zeroed for core::arch::x86::__m128d {}
#[cfg(target_arch = "x86")]
unsafe impl Zeroed for core::arch::x86::__m256i {}
#[cfg(target_arch = "x86")]
unsafe impl Zeroed for core::arch::x86::__m256 {}
#[cfg(target_arch = "x86")]
unsafe impl Zeroed for core::arch::x86::__m256d {}

#[cfg(target_arch = "x86_64")]
unsafe impl Zeroed for core::arch::x86_64::CpuidResult {}
// Requires stdsimd.
// unsafe impl Zeroed for core::arch::x86_64::__m64 {}
#[cfg(target_arch = "x86_64")]
unsafe impl Zeroed for core::arch::x86_64::__m128i {}
#[cfg(target_arch = "x86_64")]
unsafe impl Zeroed for core::arch::x86_64::__m128 {}
#[cfg(target_arch = "x86_64")]
unsafe impl Zeroed for core::arch::x86_64::__m128d {}
#[cfg(target_arch = "x86_64")]
unsafe impl Zeroed for core::arch::x86_64::__m256i {}
#[cfg(target_arch = "x86_64")]
unsafe impl Zeroed for core::arch::x86_64::__m256 {}
#[cfg(target_arch = "x86_64")]
unsafe impl Zeroed for core::arch::x86_64::__m256d {}

#[cfg(test)]
#[test]
fn zeroed_test() {
  use core::mem::size_of;
  assert_eq!(size_of::<core::sync::atomic::AtomicPtr<u8>>(), size_of::<*const u8>());
  assert_eq!(size_of::<core::sync::atomic::AtomicPtr<u8>>(), size_of::<*mut u8>());

  assert_eq!(size_of::<Option<&u8>>(), size_of::<&u8>());
  assert_eq!(size_of::<Option<&mut u8>>(), size_of::<&mut u8>());
  assert_eq!(size_of::<Option<core::ptr::NonNull<u8>>>(), size_of::<core::ptr::NonNull<u8>>());
  assert_eq!(size_of::<Option<core::num::NonZeroU8>>(), size_of::<core::num::NonZeroU8>());
  assert_eq!(size_of::<Option<core::num::NonZeroU16>>(), size_of::<core::num::NonZeroU16>());
  assert_eq!(size_of::<Option<core::num::NonZeroU32>>(), size_of::<core::num::NonZeroU32>());
  assert_eq!(size_of::<Option<core::num::NonZeroU128>>(), size_of::<core::num::NonZeroU128>());
  assert_eq!(size_of::<Option<core::num::NonZeroUsize>>(), size_of::<core::num::NonZeroUsize>());
}

// Safe to core::mem::forget.
// TODO: I'm sure there are types I missed that could impl this trait.
pub trait Forgettable {}

// Copy types can't implement Drop, so they have no destructor that needs to run.
// TODO: figure out a good way to do this. It conflicts with some things (e.g. ManuallyDrop
// implements Copy if T is Copy, but we can forget ManuallyDrop even if T isn't Copy).
// impl<T: ?Sized + Copy> Forgettable for T {}

impl Forgettable for u8 {}
impl Forgettable for u16 {}
impl Forgettable for u32 {}
impl Forgettable for u64 {}
impl Forgettable for u128 {}

impl Forgettable for i8 {}
impl Forgettable for i16 {}
impl Forgettable for i32 {}
impl Forgettable for i64 {}
impl Forgettable for i128 {}

impl Forgettable for usize {}
impl Forgettable for isize {}

impl Forgettable for bool {}

impl Forgettable for char {}

impl Forgettable for f32 {}
impl Forgettable for f64 {}

impl Forgettable for str {}

impl<'a, T> Forgettable for &'a T {}
impl<'a, T> Forgettable for &'a mut T {}

impl<T> Forgettable for *const T {}
impl<T> Forgettable for *mut T {}

impl<T> Forgettable for [T; 0] {}
impl<T: Forgettable> Forgettable for [T; 1] {}
impl<T: Forgettable> Forgettable for [T; 2] {}
impl<T: Forgettable> Forgettable for [T; 3] {}
impl<T: Forgettable> Forgettable for [T; 4] {}
impl<T: Forgettable> Forgettable for [T; 5] {}
impl<T: Forgettable> Forgettable for [T; 6] {}
impl<T: Forgettable> Forgettable for [T; 7] {}
impl<T: Forgettable> Forgettable for [T; 8] {}
impl<T: Forgettable> Forgettable for [T; 9] {}
impl<T: Forgettable> Forgettable for [T; 10] {}
impl<T: Forgettable> Forgettable for [T; 11] {}
impl<T: Forgettable> Forgettable for [T; 12] {}
impl<T: Forgettable> Forgettable for [T; 13] {}
impl<T: Forgettable> Forgettable for [T; 14] {}
impl<T: Forgettable> Forgettable for [T; 15] {}
impl<T: Forgettable> Forgettable for [T; 16] {}
impl<T: Forgettable> Forgettable for [T; 17] {}
impl<T: Forgettable> Forgettable for [T; 18] {}
impl<T: Forgettable> Forgettable for [T; 19] {}
impl<T: Forgettable> Forgettable for [T; 20] {}
impl<T: Forgettable> Forgettable for [T; 21] {}
impl<T: Forgettable> Forgettable for [T; 22] {}
impl<T: Forgettable> Forgettable for [T; 23] {}
impl<T: Forgettable> Forgettable for [T; 24] {}
impl<T: Forgettable> Forgettable for [T; 25] {}
impl<T: Forgettable> Forgettable for [T; 26] {}
impl<T: Forgettable> Forgettable for [T; 27] {}
impl<T: Forgettable> Forgettable for [T; 28] {}
impl<T: Forgettable> Forgettable for [T; 29] {}
impl<T: Forgettable> Forgettable for [T; 30] {}
impl<T: Forgettable> Forgettable for [T; 31] {}
impl<T: Forgettable> Forgettable for [T; 32] {}

impl Forgettable for () {}
impl<T0: Forgettable> Forgettable for (T0,) {}
impl<T0: Forgettable, T1: Forgettable> Forgettable for (T0, T1) {}
impl<T0: Forgettable, T1: Forgettable, T2: Forgettable> Forgettable for (T0, T1, T2) {}
impl<T0: Forgettable, T1: Forgettable, T2: Forgettable, T3: Forgettable> Forgettable
  for (T0, T1, T2, T3)
{
}
impl<T0: Forgettable, T1: Forgettable, T2: Forgettable, T3: Forgettable, T4: Forgettable>
  Forgettable for (T0, T1, T2, T3, T4)
{
}
impl<
    T0: Forgettable,
    T1: Forgettable,
    T2: Forgettable,
    T3: Forgettable,
    T4: Forgettable,
    T5: Forgettable,
  > Forgettable for (T0, T1, T2, T3, T4, T5)
{
}
impl<
    T0: Forgettable,
    T1: Forgettable,
    T2: Forgettable,
    T3: Forgettable,
    T4: Forgettable,
    T5: Forgettable,
    T6: Forgettable,
  > Forgettable for (T0, T1, T2, T3, T4, T5, T6)
{
}
impl<
    T0: Forgettable,
    T1: Forgettable,
    T2: Forgettable,
    T3: Forgettable,
    T4: Forgettable,
    T5: Forgettable,
    T6: Forgettable,
    T7: Forgettable,
  > Forgettable for (T0, T1, T2, T3, T4, T5, T6, T7)
{
}
impl<
    T0: Forgettable,
    T1: Forgettable,
    T2: Forgettable,
    T3: Forgettable,
    T4: Forgettable,
    T5: Forgettable,
    T6: Forgettable,
    T7: Forgettable,
    T8: Forgettable,
  > Forgettable for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
{
}
impl<
    T0: Forgettable,
    T1: Forgettable,
    T2: Forgettable,
    T3: Forgettable,
    T4: Forgettable,
    T5: Forgettable,
    T6: Forgettable,
    T7: Forgettable,
    T8: Forgettable,
    T9: Forgettable,
  > Forgettable for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
{
}
impl<
    T0: Forgettable,
    T1: Forgettable,
    T2: Forgettable,
    T3: Forgettable,
    T4: Forgettable,
    T5: Forgettable,
    T6: Forgettable,
    T7: Forgettable,
    T8: Forgettable,
    T9: Forgettable,
    T10: Forgettable,
  > Forgettable for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
{
}
impl<
    T0: Forgettable,
    T1: Forgettable,
    T2: Forgettable,
    T3: Forgettable,
    T4: Forgettable,
    T5: Forgettable,
    T6: Forgettable,
    T7: Forgettable,
    T8: Forgettable,
    T9: Forgettable,
    T10: Forgettable,
    T11: Forgettable,
  > Forgettable for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}

impl<T: Forgettable> Forgettable for core::num::Wrapping<T> {}

impl<T: ?Sized> Forgettable for core::marker::PhantomData<T> {}

impl<T> Forgettable for core::mem::ManuallyDrop<T> {}

impl<T: Forgettable> Forgettable for core::cmp::Reverse<T> {}

// core::cell::Ref?
// core::cell::RefCell?
// core::cell::RefMut?
impl<T: Forgettable> Forgettable for core::cell::Cell<T> {}
impl<T: Forgettable> Forgettable for core::cell::UnsafeCell<T> {}

// Requires integer_atomics.
impl Forgettable for core::sync::atomic::AtomicU8 {}
impl Forgettable for core::sync::atomic::AtomicU16 {}
impl Forgettable for core::sync::atomic::AtomicU32 {}
impl Forgettable for core::sync::atomic::AtomicU64 {}
impl Forgettable for core::sync::atomic::AtomicI8 {}
impl Forgettable for core::sync::atomic::AtomicI16 {}
impl Forgettable for core::sync::atomic::AtomicI32 {}
impl Forgettable for core::sync::atomic::AtomicI64 {}
impl Forgettable for core::sync::atomic::AtomicUsize {}
impl Forgettable for core::sync::atomic::AtomicIsize {}
impl Forgettable for core::sync::atomic::AtomicBool {}
impl<T> Forgettable for core::sync::atomic::AtomicPtr<T> {}

impl<T: Forgettable> Forgettable for Option<T> {}

impl<T: Forgettable, E: Forgettable> Forgettable for Result<T, E> {}

impl<T> Forgettable for core::ptr::NonNull<T> {}
impl Forgettable for core::num::NonZeroU8 {}
impl Forgettable for core::num::NonZeroU16 {}
impl Forgettable for core::num::NonZeroU32 {}
impl Forgettable for core::num::NonZeroU64 {}
impl Forgettable for core::num::NonZeroU128 {}
impl Forgettable for core::num::NonZeroUsize {}

impl<T> Forgettable for core::iter::Empty<T> {}

impl Forgettable for core::time::Duration {}

// Requires allocator_api.
// alloc::alloc::Global

// Requires stdsimd.
// core::simd::u8x2
// core::simd::u8x4
// core::simd::u8x8
// core::simd::u8x16
// core::simd::u8x32
// core::simd::u8x64
// core::simd::u16x2
// core::simd::u16x4
// core::simd::u16x8
// core::simd::u16x16
// core::simd::u16x32
// core::simd::u32x2
// core::simd::u32x4
// core::simd::u32x8
// core::simd::u32x16
// core::simd::u64x2
// core::simd::u64x4
// core::simd::u64x8
// core::simd::i8x2
// core::simd::i8x4
// core::simd::i8x8
// core::simd::i8x16
// core::simd::i8x32
// core::simd::i8x64
// core::simd::i16x2
// core::simd::i16x4
// core::simd::i16x8
// core::simd::i16x16
// core::simd::i16x32
// core::simd::i32x2
// core::simd::i32x4
// core::simd::i32x8
// core::simd::i32x16
// core::simd::i64x2
// core::simd::i64x4
// core::simd::i64x8
// core::simd::m1x8
// core::simd::m1x16
// core::simd::m1x32
// core::simd::m1x64
// core::simd::m8x2
// core::simd::m8x4
// core::simd::m8x8
// core::simd::m8x16
// core::simd::m8x32
// core::simd::m16x2
// core::simd::m16x4
// core::simd::m16x8
// core::simd::m16x16
// core::simd::m32x2
// core::simd::m32x4
// core::simd::m32x8
// core::simd::m64x2
// core::simd::m64x4
// core::simd::f32x2
// core::simd::f32x4
// core::simd::f32x8
// core::simd::f32x16
// core::simd::f64x2
// core::simd::f64x4
// core::simd::f64x8

#[cfg(target_arch = "x86")]
impl Forgettable for core::arch::x86::CpuidResult {}
// Requires stdsimd.
// impl Forgettable for core::arch::x86::__m64 {}
#[cfg(target_arch = "x86")]
impl Forgettable for core::arch::x86::__m128i {}
#[cfg(target_arch = "x86")]
impl Forgettable for core::arch::x86::__m128 {}
#[cfg(target_arch = "x86")]
impl Forgettable for core::arch::x86::__m128d {}
#[cfg(target_arch = "x86")]
impl Forgettable for core::arch::x86::__m256i {}
#[cfg(target_arch = "x86")]
impl Forgettable for core::arch::x86::__m256 {}
#[cfg(target_arch = "x86")]
impl Forgettable for core::arch::x86::__m256d {}

#[cfg(target_arch = "x86_64")]
impl Forgettable for core::arch::x86_64::CpuidResult {}
// Requires stdsimd.
// impl Forgettable for core::arch::x86_64::__m64 {}
#[cfg(target_arch = "x86_64")]
impl Forgettable for core::arch::x86_64::__m128i {}
#[cfg(target_arch = "x86_64")]
impl Forgettable for core::arch::x86_64::__m128 {}
#[cfg(target_arch = "x86_64")]
impl Forgettable for core::arch::x86_64::__m128d {}
#[cfg(target_arch = "x86_64")]
impl Forgettable for core::arch::x86_64::__m256i {}
#[cfg(target_arch = "x86_64")]
impl Forgettable for core::arch::x86_64::__m256 {}
#[cfg(target_arch = "x86_64")]
impl Forgettable for core::arch::x86_64::__m256d {}
