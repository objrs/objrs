#[allow(unused_imports)]
use crate::objc::*;
use crate::CoreFoundation::CFRange;
use crate::CoreFoundation::__CFString;
#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use bitflags::bitflags;
#[allow(unused_imports)]
use libc::c_void;
#[allow(unused_imports)]
use objrs::objrs;
#[allow(unused_imports)]
use std::mem;
#[allow(unused_imports)]
use std::ptr;
#[repr(C)]
pub struct __DCSDictionary {
  opaque: u32,
}
extern "C" {
  pub fn DCSGetTermRangeInString(
    dictionary: *mut __DCSDictionary,
    textString: *mut __CFString,
    offset: isize,
  ) -> CFRange;
  pub fn DCSCopyTextDefinition(
    dictionary: *mut __DCSDictionary,
    textString: *mut __CFString,
    range: CFRange,
  ) -> *mut __CFString;
}
