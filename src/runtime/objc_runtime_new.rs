// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

// See https://github.com/opensource-apple/objc4/blob/master/runtime/objc-runtime-new.h

extern crate libc;

use crate::runtime::objc;

#[cfg(target_pointer_width = "64")]
#[allow(non_camel_case_types)]
pub type mask_t = u32;

#[cfg(target_pointer_width = "32")]
#[allow(non_camel_case_types)]
pub type mask_t = u16;

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct bucket_t {
  pub key: usize,
  pub imp: *mut objc::Imp,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct cache_t {
  pub buckets: *mut bucket_t,
  pub mask: mask_t,
  pub occupied: mask_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct method_t {
  pub name: *mut objc::Sel,
  pub types: *const libc::c_char,
  pub imp: *mut objc::Imp,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct ivar_t {
  pub offset: *mut i32,
  pub name: *const libc::c_char,
  pub encoded_type: *const libc::c_char,
  pub alignment_raw: u32,
  pub size: u32,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct property_t {
  pub name: *const libc::c_char,
  pub attributes: *const libc::c_char,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct method_list_t {
  pub entsize_and_flags: u32,
  pub count: u32,
  // pub first: [method_t],
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct ivar_list_t {
  pub entsize_and_flags: u32,
  pub count: u32,
  // pub first: [ivar_t],
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct property_list_t {
  pub entsize_and_flags: u32,
  pub count: u32,
  // pub first: [property_t],
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct protocol_t {
  pub mangled_name: *const libc::c_char,
  pub protocols: *mut usize,
  pub instance_methods: *mut method_list_t,
  pub class_methods: *mut method_list_t,
  pub optional_instance_methods: *mut method_list_t,
  pub optional_class_methods: *mut method_list_t,
  pub instance_properties: *mut property_list_t,
  pub size: u32,
  pub flags: u32,
  pub extended_method_types: *mut *const libc::c_char,
  pub demangled_name: *const libc::c_char,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct protocol_list_t {
  pub count: usize,
  // pub list: [usize],
}

// #[repr(C)]
// #[allow(non_camel_case_types)]
// pub struct locstamped_category_t {
//   pub cat: *mut category_t,
//   pub hi: *mut header_info,
// }

// #[repr(C)]
// #[allow(non_camel_case_types)]
// pub struct locstamped_category_list_t {
//   pub count: u32,
//   #[cfg(target_pointer_width = "64")]
//   pub reserved: u32,
//   pub list: [locstamped_category_t],
// }

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct class_ro_t {
  pub flags: u32,
  pub instance_start: u32,
  pub instance_size: u32,
  #[cfg(target_pointer_width = "64")]
  pub reserved: u32,

  pub ivar_layout: *const u8,

  pub name: *const libc::c_char,
  pub base_method_list: *mut method_list_t,
  pub base_protocols: *mut protocol_list_t,
  pub ivars: *const ivar_list_t,

  pub weak_ivar_layout: *const u8,
  pub base_properties: *mut property_list_t,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub union method_array_t {
  pub list: *mut method_list_t,
  pub array_and_flag: usize,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub union property_array_t {
  pub list: *mut property_list_t,
  pub array_and_flag: usize,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub union protocol_array_t {
  pub list: *mut protocol_list_t,
  pub array_and_flag: usize,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct class_rw_t {
  pub flags: u32,
  pub version: u32,
  pub ro: *const class_ro_t,
  pub methods: method_array_t,
  pub properties: property_array_t,
  pub protocols: protocol_array_t,
  pub first_subclass: *mut objc::Class,
  pub next_sibling_class: *mut objc::Class,
  pub demangled_name: *mut libc::c_char,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct objc_class {
  pub isa: *mut objc::Class,
  pub superclass: *mut objc::Class,
  pub cache: cache_t,
  // This should be a usize (to match the C code), but due to Rust issue #54709, we have to use a
  // pointer type.
  pub bits: *const class_ro_t,
}

#[allow(non_camel_case_types)]
pub enum classref {} // TODO: just use an extern type here?
#[allow(non_camel_case_types)]
pub type classref_t = *mut classref;

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct category_t {
  pub name: *const libc::c_char,
  pub cls: classref_t,
  pub instance_methods: *mut method_list_t,
  pub class_methods: *mut method_list_t,
  pub protocols: *mut protocol_list_t,
  pub instance_properties: *mut property_list_t,
  // objc-runtime-new.h is missing the following properties. See `_category_t` and `CategorynfABITy` in https://github.com/llvm-mirror/clang/blob/master/lib/CodeGen/CGObjCMac.cpp for how they get generated (and observe the generated assembly).
  pub class_properties: *mut property_list_t,
  pub size: u32,
}

// #[repr(C)]
// #[allow(non_camel_case_types)]
// pub struct objc_super2 {
//   pub receiver: *mut objc::Id,
//   pub current_class: *mut objc::Class,
// }

// #[repr(C)]
// #[allow(non_camel_case_types)]
// pub struct message_ref_t {
//   pub imp: *mut objc::Imp,
//   pub sel: *mut objc::Sel,
// }
