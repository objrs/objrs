// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

// See https://github.com/opensource-apple/objc4/blob/master/runtime/objc.h

extern crate core;
extern crate libc;

use objc_runtime_new;

pub type Class = *mut objc_runtime_new::objc_class;

#[repr(C)] // TODO: should this be repr(transparent)?
pub struct objc_object(Class);

#[allow(non_camel_case_types)]
pub type id = *mut objc_object;

// #[repr(C)]
// pub struct objc_selector;
// pub type SEL = *mut objc_selector;
pub type SEL = *const libc::c_char;

pub type IMP = unsafe extern "C" fn();

pub type Method = *mut objc_runtime_new::method_t;
