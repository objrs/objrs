// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

// See https://github.com/opensource-apple/objc4/blob/master/runtime/objc-internal.h
// and https://clang.llvm.org/docs/AutomaticReferenceCounting.html

extern crate core;
extern crate libc;

use objc;

#[link(name = "objc")]
extern "C" {
  pub fn objc_autoreleasePoolPush() -> *mut libc::c_void;
  pub fn objc_autoreleasePoolPop(pool: *mut libc::c_void);

  pub fn objc_retain(obj: objc::id) -> objc::id;
  pub fn objc_release(obj: objc::id);
  pub fn objc_retainAutorelease(obj: objc::id) -> objc::id;

  pub fn objc_loadWeakRetained(location: *mut objc::id) -> objc::id;
}

// TODO: I think that this macro should be removed. Consider the following code:
// fn do_stuff() {
//   autoreleasepool!{
//     return;
//   }
// }
// I believe the return statement would result in the autoreleasepool not being popped.
// This could possibly be fixed by using a gaurd variable with a Drop implementation that pops the pool, but I'm not sure that's foolproof.
#[macro_export]
macro_rules! autoreleasepool {
  ($($tt: tt)*) => {
    // Compiling the Objective-C program `int main() {@autoreleasepool{} return 0;}` shows that
    // clang doesn't check the return value of the pool push. If clang never does it, then I feel
    // comfortable not doing it here either (plus I've looked through the Objective-C runtime code
    // and I don't think our lack of null-checking here can be problematic).
    // TODO: does this need to be surrounded in {} for any reason?
    let pool = unsafe { objc_autoreleasePoolPush() };
    {$($tt)*;}
    unsafe { objc_autoreleasePoolPop(pool) };
  };
}

// TODO: What happens to autoreleasepools when an exception is thrown (before the pool is popped)? How does Objective-C handle it, and does objrs match it?
#[inline]
pub fn autoreleasepool<F>(f: F)
where
  F: FnOnce(),
{
  autoreleasepool!{
    f();
  }
}
