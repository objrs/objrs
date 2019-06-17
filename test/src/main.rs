// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

#![cfg_attr(all(feature = "alloc", not(feature = "std")), feature(alloc_error_handler))]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(test), no_main)]

mod class;
mod no_prelude;
mod nsobject;
mod selector;
mod simple;

#[cfg(all(feature = "alloc", not(feature = "std")))]
#[derive(Clone, Copy)]
struct Allocator;

#[cfg(all(feature = "alloc", not(feature = "std")))]
unsafe impl core::alloc::GlobalAlloc for Allocator {
  unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
    extern "C" {
      fn malloc(len: usize) -> *mut u8;
    }
    return malloc(core::cmp::max(layout.size(), layout.align()));
  }

  unsafe fn dealloc(&self, ptr: *mut u8, _: core::alloc::Layout) {
    extern "C" {
      fn free(ptr: *mut u8);
    }
    return free(ptr);
  }
}

#[cfg(all(feature = "alloc", not(feature = "std")))]
#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

#[cfg(all(feature = "alloc", not(feature = "std"), not(test)))]
#[alloc_error_handler]
fn alloc_error(_: core::alloc::Layout) -> ! {
  loop {}
}

#[cfg(not(any(feature = "std", test)))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[cfg(not(test))]
#[no_mangle]
fn main(_argc: i32, _argv: *const *const u8) -> i32 {
  class::run_tests();
  no_prelude::run_tests();
  nsobject::run_tests();
  selector::run_tests();
  simple::run_tests();
  return 0;
}
