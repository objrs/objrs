// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

// See https://github.com/opensource-apple/objc4/blob/master/runtime/objc-runtime-new.h

extern crate libc;

use objc;

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
  pub imp: objc::IMP,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct cache_t {
  pub buckets: *mut bucket_t,
  mask: mask_t,
  occupied: mask_t,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct method_t {
  name: objc::SEL,
  types: *const libc::c_char,
  imp: objc::IMP,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct ivar_t {
  offset: *mut i32,
  name: *const libc::c_char,
  encoded_type: *const libc::c_char,
  alignment: u32,
  size: u32,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct property_t {
  name: *const libc::c_char,
  attributes: *const libc::c_char,
}

// TODO: Delete this and switch to core::mem::size_of once it's const.
impl method_t {
  #[cfg(target_pointer_width = "32")]
  pub const ENTSIZE: u32 = 12;

  #[cfg(target_pointer_width = "64")]
  pub const ENTSIZE: u32 = 24;
}

impl ivar_t {
  #[cfg(target_pointer_width = "32")]
  pub const ENTSIZE: u32 = 20;

  #[cfg(target_pointer_width = "64")]
  pub const ENTSIZE: u32 = 32;
}

impl property_t {
  #[cfg(target_pointer_width = "32")]
  pub const ENTSIZE: u32 = 8;

  #[cfg(target_pointer_width = "64")]
  pub const ENTSIZE: u32 = 16;
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct method_list_t {
  entsize_and_flags: u32,
  count: u32,
  first: [method_t],
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct ivar_list_t {
  entsize_and_flags: u32,
  count: u32,
  first: [ivar_t],
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct property_list_t {
  entsize_and_flags: u32,
  count: u32,
  first: [property_t],
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct protocol_t {
  mangled_name: *const libc::c_char,
  protocols: *mut usize,
  instance_methods: *mut method_list_t,
  class_methods: *mut method_list_t,
  optional_instance_methods: *mut method_list_t,
  optional_class_methods: *mut method_list_t,
  instance_properties: *mut property_list_t,
  size: u32,
  flags: u32,
  extended_method_types: *mut *const libc::c_char,
  demangled_name: *const libc::c_char,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct protocol_list_t {
  count: usize,
  list: [usize],
}

// #[repr(C)]
// #[allow(non_camel_case_types)]
// pub struct locstamped_category_t {
//   cat: *mut category_t,
//   hi: *mut header_info,
// }

// #[repr(C)]
// #[allow(non_camel_case_types)]
// pub struct locstamped_category_list_t {
//   count: u32,
//   #[cfg(target_pointer_width = "64")]
//   reserved: u32,
//   list: [locstamped_category_t],
// }

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct class_ro_t {
  flags: u32,
  instance_start: u32,
  instance_size: u32,
  #[cfg(target_pointer_width = "64")]
  reserved: u32,

  ivar_layout: *const u8,

  name: *const libc::c_char,
  base_method_list: *mut method_list_t,
  base_protocols: *mut protocol_list_t,
  ivars: *const ivar_list_t,

  weak_ivar_layout: *const u8,
  base_properties: *mut property_list_t,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub union method_array_t {
  list: *mut method_list_t,
  array_and_flag: usize,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub union property_array_t {
  list: *mut property_list_t,
  array_and_flag: usize,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub union protocol_array_t {
  list: *mut protocol_list_t,
  array_and_flag: usize,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct class_rw_t {
  flags: u32,
  version: u32,
  ro: *const class_ro_t,
  methods: method_array_t,
  properties: property_array_t,
  protocols: protocol_array_t,
  first_subclass: objc::Class,
  next_sibling_class: objc::Class,
  demangled_name: *mut libc::c_char,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct objc_class {
  isa: objc::Class,
  superclass: objc::Class,
  cache: cache_t,
  bits: usize,
}

#[allow(non_camel_case_types)]
enum classref {}
#[allow(non_camel_case_types)]
type classref_t = *mut classref;

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct category_t {
  name: *const libc::c_char,
  cls: classref_t,
  instance_methods: *mut method_list_t,
  class_methods: *mut method_list_t,
  protocols: *mut protocol_list_t,
  instance_properties: *mut property_list_t,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct objc_super2 {
  receiver: objc::id,
  current_class: objc::Class,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct message_ref_t {
  imp: objc::IMP,
  sel: objc::SEL,
}
