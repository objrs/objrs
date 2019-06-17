// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

#[cfg(feature = "alloc")]
extern crate alloc;
extern crate objrs_macros;

use crate::__objrs::TransmuteHack;
use crate::marker;
use crate::runtime;

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

impl<T: marker::Class + ?Sized> core::fmt::Pointer for Alloc<T> {
  #[inline(always)]
  fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
    return core::fmt::Pointer::fmt(&self.0, formatter);
  }
}

impl<T: marker::Class + ?Sized> core::fmt::Debug for Alloc<T> {
  #[inline(always)]
  fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
    return core::fmt::Pointer::fmt(&self, formatter);
  }
}

#[repr(transparent)]
pub struct Strong<T: marker::Class + ?Sized>(core::ptr::NonNull<T>);

impl<T: marker::Class + ?Sized> Strong<T> {
  #[inline(always)]
  pub fn release(self) {
    unsafe {
      let objc_release: unsafe extern "C" fn(Strong<T>) =
        core::mem::transmute(runtime::objc_release as *const ());
      objc_release(self);
    }
  }

  #[inline(always)]
  pub fn autorelease(self) -> Auto<T> {
    unsafe {
      let objc_autorelease: unsafe extern "C" fn(Strong<T>) -> Auto<T> =
        core::mem::transmute(runtime::objc_autorelease as *const ());
      return objc_autorelease(self);
    }
  }
}

impl<T: marker::NonRootClass + marker::Class + ?Sized> Strong<T> {
  #[inline(always)]
  pub fn into_super(self) -> Strong<<T as marker::NonRootClass>::Super> {
    let ptr = self.0.as_ptr();
    core::mem::forget(self);
    return Strong(unsafe {
      TransmuteHack {
        from: ptr,
      }
      .to
    });
  }
}

// unsafe impl<T: marker::Class + ?Sized> marker::Class for Strong<T> {}
// unsafe impl<T: marker::RootClass + ?Sized> marker::RootClass for Strong<T> {}
// unsafe impl<T: marker::NonRootClass + ?Sized> marker::NonRootClass for Strong<T> {
//   type Super = <T as marker::NonRootClass>::Super;
// }

impl<T: marker::Class + ?Sized> core::fmt::Pointer for Strong<T> {
  #[inline(always)]
  fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
    return core::fmt::Pointer::fmt(&self.0, formatter);
  }
}

impl<T: marker::Class + ?Sized> core::fmt::Debug for Strong<T> {
  #[inline(always)]
  fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
    return core::fmt::Pointer::fmt(&self, formatter);
  }
}

// See https://github.com/rust-lang/rust/issues/47442
impl<T: marker::Class + ?Sized> core::ops::Drop for Strong<T> {
  #[inline(always)]
  fn drop(&mut self) {
    unsafe {
      let objc_release: unsafe extern "C" fn(*mut T) =
        core::mem::transmute(runtime::objc_release as *const ());
      objc_release(self.0.as_ptr());
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

impl<T: marker::Class + ?Sized> core::convert::AsRef<runtime::Id> for Strong<T> {
  #[inline(always)]
  fn as_ref(&self) -> &runtime::Id {
    return unsafe { core::mem::transmute(self) };
  }
}

impl<T: marker::Class + ?Sized> core::convert::AsMut<runtime::Id> for Strong<T> {
  #[inline(always)]
  fn as_mut(&mut self) -> &mut runtime::Id {
    return unsafe { core::mem::transmute(self) };
  }
}

impl<T: marker::Class + ?Sized> core::convert::From<Auto<T>> for Strong<T> {
  #[inline(always)]
  fn from(this: Auto<T>) -> Strong<T> {
    return this.retain();
  }
}

#[cfg(feature = "alloc")]
impl<T: marker::Class + ?Sized> core::convert::TryFrom<Weak<T>> for Strong<T> {
  type Error = ();
  fn try_from(weak: Weak<T>) -> Result<Strong<T>, ()> {
    return weak.retain().ok_or(());
  }
}

#[repr(transparent)]
pub struct Auto<T: marker::Class + ?Sized>(core::ptr::NonNull<T>);

impl<T: marker::Class + ?Sized> Auto<T> {
  #[inline(always)]
  pub fn retain(self) -> Strong<T> {
    unsafe {
      let objc_retain: unsafe extern "C" fn(Auto<T>) -> Strong<T> =
        core::mem::transmute(runtime::objc_retain as *const ());
      return objc_retain(self);
    }
  }
}

// unsafe impl<T: marker::Class + ?Sized> marker::Class for Auto<T> {}
// unsafe impl<T: marker::RootClass + ?Sized> marker::RootClass for Auto<T> {}
// unsafe impl<T: marker::NonRootClass + ?Sized> marker::NonRootClass for Auto<T> {
//   type Super = <T as marker::NonRootClass>::Super;
// }

impl<T: marker::Class + ?Sized> core::fmt::Pointer for Auto<T> {
  #[inline(always)]
  fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
    return core::fmt::Pointer::fmt(&self.0, formatter);
  }
}

impl<T: marker::Class + ?Sized> core::fmt::Debug for Auto<T> {
  #[inline(always)]
  fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
    return core::fmt::Pointer::fmt(&self, formatter);
  }
}

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

impl<T: marker::Class + ?Sized> core::convert::AsRef<runtime::Id> for Auto<T> {
  #[inline(always)]
  fn as_ref(&self) -> &runtime::Id {
    return unsafe { core::mem::transmute(self) };
  }
}

impl<T: marker::Class + ?Sized> core::convert::AsMut<runtime::Id> for Auto<T> {
  #[inline(always)]
  fn as_mut(&mut self) -> &mut runtime::Id {
    return unsafe { core::mem::transmute(self) };
  }
}

impl<T: marker::Class + ?Sized> core::convert::From<Strong<T>> for Auto<T> {
  #[inline(always)]
  fn from(this: Strong<T>) -> Auto<T> {
    return this.autorelease();
  }
}

#[cfg(feature = "alloc")]
impl<T: marker::Class + ?Sized> core::convert::TryFrom<Weak<T>> for Auto<T> {
  type Error = ();
  fn try_from(weak: Weak<T>) -> Result<Auto<T>, ()> {
    return weak.autorelease().ok_or(());
  }
}

impl<T: marker::NonRootClass + marker::Class + ?Sized> Auto<T> {
  #[inline(always)]
  pub fn into_super(self) -> Auto<<T as marker::NonRootClass>::Super> {
    let ptr = self.0.as_ptr();
    core::mem::forget(self);
    return Auto(unsafe {
      TransmuteHack {
        from: ptr,
      }
      .to
    });
  }
}

/// `Weak` is not FFI-safe and should not be used as a parameter type for Objective-C methods. The
/// Objective-C runtime requires weak pointers to not be moved or copied (naively), which Rust does.
/// Thus, weak pointers in Rust require extra levels of indirection and do not have the same ABI as
/// Objective-C weak pointers.
#[cfg(feature = "alloc")]
pub struct Weak<T: marker::Class + ?Sized>(alloc::boxed::Box<*mut T>);

#[cfg(feature = "alloc")]
impl<T: marker::Class + ?Sized> Weak<T> {
  pub fn retain(mut self) -> Option<Strong<T>> {
    let ptr_to_weak = core::ops::DerefMut::deref_mut(&mut self.0);
    unsafe {
      let load_weak_retained: unsafe extern "C" fn(&mut *mut T) -> Option<Strong<T>> =
        core::mem::transmute(runtime::objc_loadWeakRetained as *const ());
      return load_weak_retained(ptr_to_weak);
    }
  }

  pub fn autorelease(mut self) -> Option<Auto<T>> {
    let ptr_to_weak = core::ops::DerefMut::deref_mut(&mut self.0);
    unsafe {
      let load_weak: unsafe extern "C" fn(&mut *mut T) -> Option<Auto<T>> =
        core::mem::transmute(runtime::objc_loadWeak as *const ());
      return load_weak(ptr_to_weak);
    }
  }
}

// unsafe impl<T: marker::Class + ?Sized> marker::Class for Weak<T> {}
// unsafe impl<T: marker::RootClass + ?Sized> marker::RootClass for Weak<T> {}
// unsafe impl<T: marker::NonRootClass + ?Sized> marker::NonRootClass for Weak<T> {
//   type Super = <T as marker::NonRootClass>::Super;
// }
#[cfg(feature = "alloc")]
unsafe impl<T: marker::Class + ?Sized> marker::Weak for Weak<T> {}

#[cfg(feature = "alloc")]
impl<T: marker::Class + ?Sized> core::ops::Drop for Weak<T> {
  #[inline(always)]
  fn drop(&mut self) {
    let ptr_to_weak = core::ops::DerefMut::deref_mut(&mut self.0);
    unsafe {
      let destroy_weak: unsafe extern "C" fn(&mut *mut T) =
        core::mem::transmute(runtime::objc_destroyWeak as *const ());
      destroy_weak(ptr_to_weak);
    }
  }
}

#[cfg(feature = "alloc")]
impl<T: marker::Class + ?Sized> core::convert::From<Strong<T>> for Weak<T> {
  #[inline(always)]
  fn from(this: Strong<T>) -> Weak<T> {
    // TODO: replace this transmute with a nil value.
    let mut weak = Weak(alloc::boxed::Box::new(unsafe {
      TransmuteHack {
        from: 0usize,
      }
      .to
    }));
    let ptr_to_weak = core::ops::DerefMut::deref_mut(&mut weak.0);
    unsafe {
      let init_weak: unsafe extern "C" fn(&mut *mut T, *mut T) -> *mut T =
        core::mem::transmute(runtime::objc_initWeak as *const ());
      init_weak(ptr_to_weak, this.0.as_ptr());
    }
    return weak;
  }
}

#[cfg(feature = "alloc")]
impl<T: marker::Class + ?Sized> core::fmt::Pointer for Weak<T> {
  fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
    let ptr_to_weak = core::ops::Deref::deref(&self.0);
    let ptr;
    unsafe {
      let load_weak: unsafe extern "C" fn(&*mut T) -> *const T =
        core::mem::transmute(runtime::objc_loadWeak as *const ());
      ptr = load_weak(ptr_to_weak);
    }
    return core::fmt::Pointer::fmt(&ptr, formatter);
  }
}

#[cfg(feature = "alloc")]
impl<T: marker::Class + ?Sized> core::fmt::Debug for Weak<T> {
  #[inline(always)]
  fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
    return core::fmt::Pointer::fmt(&self, formatter);
  }
}

// TODO: What happens to autoreleasepools when an exception is thrown (before the pool is popped)? How does Objective-C handle it, and does objrs match it?
// We only take Fn instead of FnOnce or FnMut in order to avoid allowing the user to accidentally create dangling references to autoreleased values.
#[inline(always)]
pub fn autoreleasepool<F: Fn()>(f: F) {
  let pool = unsafe { runtime::objc_autoreleasePoolPush() };
  f();
  unsafe { runtime::objc_autoreleasePoolPop(pool) };
}
