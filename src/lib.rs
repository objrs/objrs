// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

#![feature(
  // Features that objrs really needs:
  extern_types,
  unwind_attributes,

  // Features that objrs could live without, but would be inconvenient or result in extra overhead:
  const_transmute,
  global_asm,
  specialization,
  untagged_unions
)]
#![no_std]

#[doc(hidden)]
pub mod __objrs;
mod arc;
mod cstr;
mod exception;
pub mod marker;
mod runtime;
#[cfg(test)]
mod test;
mod type_encoding;

extern crate libc;
extern crate objrs_macros;

pub use objrs_macros::objrs;

pub use arc::*;
pub use cstr::*;
pub use exception::*;
pub use type_encoding::*;

#[macro_export]
macro_rules! selector {
  ($sel:literal $(,)?) => {{
    use $crate::__objrs::objrs_macros::sel_ref;

    // The `#[sel_ref]` attribute will replace the value (and type) of the static variable. The
    // initial type and value here are only used as a fallback if the macro encounters an error, and
    // is only used to prevent the compiler from emitting additional errors.
    #[sel_ref($sel)]
    static SEL_REF: &'static [$crate::__objrs::u8; 0] = &[];

    let sel_ref = unsafe { $crate::__objrs::core::ptr::read_volatile(&SEL_REF as *const _) };
    unsafe { $crate::__objrs::core::mem::transmute::<_, &'static $crate::Sel>(sel_ref) }
  }};
}

// TODO: this results in a `&'static Class`. But if the Objective-C class weak-linked (e.g.,
// https://stackoverflow.com/a/16936512) it can be nil. Maybe change this to be
// `Option<&'static Class>`? Or maybe `*mut Class` (in case someone makes an `&mut` of the class).
#[macro_export]
macro_rules! class {
  ($class:literal $(,)?) => {{
    use $crate::__objrs::objrs_macros::class_ref;

    // The `#[class_ref]` attribute will replace the value of the static variable. If there is an
    // error, it will emit a compiler error and leave the value unmodified. Thus, the use of uninit
    // here is safe since it will always be replaced if compiling succeeds. It's useful to have if
    // the attribute does fail and emit an error, though, because the compiler will think the
    // variable has a value and won't emit additional (unrelated) errors.
    #[class_ref($class)]
    static CLASS_REF: $crate::__objrs::SyncHack<&'static $crate::Class> = unsafe {
      $crate::__objrs::TransmuteHack {
        from: &$crate::__objrs::UNINIT_PTR,
      }
      .to
    };

    unsafe { $crate::__objrs::core::ptr::read_volatile(&CLASS_REF.0 as *const _) }
  }};
}

pub use runtime::{Class, Id, Imp, Method, Sel};

// TODO: I think that this macro should be removed. Consider the following code:
// fn do_stuff() {
//   autoreleasepool!{
//     return;
//   }
// }
// I believe the return statement would result in the autoreleasepool not being popped.
// This could possibly be fixed by using a gaurd variable with a Drop implementation that pops the pool, but I'm not sure that's foolproof.
// #[macro_export]
// macro_rules! autoreleasepool {
//   ($($tt: tt)*) => {
//     // Compiling the Objective-C program `int main() {@autoreleasepool{} return 0;}` shows that
//     // clang doesn't check the return value of the pool push. If clang never does it, then I feel
//     // comfortable not doing it here either (plus I've looked through the Objective-C runtime code
//     // and I don't think our lack of null-checking here can be problematic).
//     // TODO: does this need to be surrounded in {} for any reason?
//     let pool = unsafe { objc_autoreleasePoolPush() };
//     {$($tt)*;}
//     unsafe { objc_autoreleasePoolPop(pool) };
//   };
// }
