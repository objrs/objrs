#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use bitflags::bitflags;
#[allow(unused_imports)]
use objrs::objrs;
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum acl_perm_t {
  ACL_READ_DATA = 2,
  ACL_WRITE_DATA = 4,
  ACL_EXECUTE = 8,
  ACL_DELETE = 16,
  ACL_APPEND_DATA = 32,
  ACL_DELETE_CHILD = 64,
  ACL_READ_ATTRIBUTES = 128,
  ACL_WRITE_ATTRIBUTES = 256,
  ACL_READ_EXTATTRIBUTES = 512,
  ACL_WRITE_EXTATTRIBUTES = 1024,
  ACL_READ_SECURITY = 2048,
  ACL_WRITE_SECURITY = 4096,
  ACL_CHANGE_OWNER = 8192,
  ACL_SYNCHRONIZE = 1048576,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum acl_tag_t {
  ACL_UNDEFINED_TAG = 0,
  ACL_EXTENDED_ALLOW = 1,
  ACL_EXTENDED_DENY = 2,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum acl_type_t {
  ACL_TYPE_EXTENDED = 256,
  ACL_TYPE_ACCESS = 0,
  ACL_TYPE_DEFAULT = 1,
  ACL_TYPE_AFS = 2,
  ACL_TYPE_CODA = 3,
  ACL_TYPE_NTFS = 4,
  ACL_TYPE_NWFS = 5,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum acl_entry_id_t {
  ACL_FIRST_ENTRY = 0,
  ACL_NEXT_ENTRY = -1,
  ACL_LAST_ENTRY = -2,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum acl_flag_t {
  ACL_FLAG_DEFER_INHERIT = 1,
  ACL_FLAG_NO_INHERIT = 131072,
  ACL_ENTRY_INHERITED = 16,
  ACL_ENTRY_FILE_INHERIT = 32,
  ACL_ENTRY_DIRECTORY_INHERIT = 64,
  ACL_ENTRY_LIMIT_INHERIT = 128,
  ACL_ENTRY_ONLY_INHERIT = 256,
}
#[repr(C)]
pub struct _acl {
  opaque: u32,
}
#[repr(C)]
pub struct _acl_entry {
  opaque: u32,
}
#[repr(C)]
pub struct _acl_permset {
  opaque: u32,
}
#[repr(C)]
pub struct _acl_flagset {
  opaque: u32,
}
extern "C" {
  pub fn acl_dup(acl: *mut _acl) -> *mut _acl;
  pub fn acl_free(obj_p: *mut c_void) -> i32;
  pub fn acl_init(count: i32) -> *mut _acl;
  pub fn acl_copy_entry(dest_d: *mut _acl_entry, src_d: *mut _acl_entry) -> i32;
  pub fn acl_create_entry(acl_p: *mut *mut _acl, entry_p: *mut *mut _acl_entry) -> i32;
  pub fn acl_create_entry_np(
    acl_p: *mut *mut _acl,
    entry_p: *mut *mut _acl_entry,
    entry_index: i32,
  ) -> i32;
  pub fn acl_delete_entry(acl: *mut _acl, entry_d: *mut _acl_entry) -> i32;
  pub fn acl_get_entry(acl: *mut _acl, entry_id: i32, entry_p: *mut *mut _acl_entry) -> i32;
  pub fn acl_valid(acl: *mut _acl) -> i32;
  pub fn acl_valid_fd_np(fd: i32, type_: acl_type_t, acl: *mut _acl) -> i32;
  pub fn acl_valid_file_np(path: *mut i8, type_: acl_type_t, acl: *mut _acl) -> i32;
  pub fn acl_valid_link_np(path: *mut i8, type_: acl_type_t, acl: *mut _acl) -> i32;
  pub fn acl_add_perm(permset_d: *mut _acl_permset, perm: acl_perm_t) -> i32;
  pub fn acl_calc_mask(acl_p: *mut *mut _acl) -> i32;
  pub fn acl_clear_perms(permset_d: *mut _acl_permset) -> i32;
  pub fn acl_delete_perm(permset_d: *mut _acl_permset, perm: acl_perm_t) -> i32;
  pub fn acl_get_perm_np(permset_d: *mut _acl_permset, perm: acl_perm_t) -> i32;
  pub fn acl_get_permset(entry_d: *mut _acl_entry, permset_p: *mut *mut _acl_permset) -> i32;
  pub fn acl_set_permset(entry_d: *mut _acl_entry, permset_d: *mut _acl_permset) -> i32;
  pub fn acl_maximal_permset_mask_np(mask_p: *mut i32) -> i32;
  pub fn acl_get_permset_mask_np(entry_d: *mut _acl_entry, mask_p: *mut i32) -> i32;
  pub fn acl_set_permset_mask_np(entry_d: *mut _acl_entry, mask: i32) -> i32;
  pub fn acl_add_flag_np(flagset_d: *mut _acl_flagset, flag: acl_flag_t) -> i32;
  pub fn acl_clear_flags_np(flagset_d: *mut _acl_flagset) -> i32;
  pub fn acl_delete_flag_np(flagset_d: *mut _acl_flagset, flag: acl_flag_t) -> i32;
  pub fn acl_get_flag_np(flagset_d: *mut _acl_flagset, flag: acl_flag_t) -> i32;
  pub fn acl_get_flagset_np(obj_p: *mut c_void, flagset_p: *mut *mut _acl_flagset) -> i32;
  pub fn acl_set_flagset_np(obj_p: *mut c_void, flagset_d: *mut _acl_flagset) -> i32;
  pub fn acl_get_qualifier(entry_d: *mut _acl_entry) -> *mut c_void;
  pub fn acl_get_tag_type(entry_d: *mut _acl_entry, tag_type_p: *mut acl_tag_t) -> i32;
  pub fn acl_set_qualifier(entry_d: *mut _acl_entry, tag_qualifier_p: *mut c_void) -> i32;
  pub fn acl_set_tag_type(entry_d: *mut _acl_entry, tag_type: acl_tag_t) -> i32;
  pub fn acl_delete_def_file(path_p: *mut i8) -> i32;
  pub fn acl_get_fd(fd: i32) -> *mut _acl;
  pub fn acl_get_fd_np(fd: i32, type_: acl_type_t) -> *mut _acl;
  pub fn acl_get_file(path_p: *mut i8, type_: acl_type_t) -> *mut _acl;
  pub fn acl_get_link_np(path_p: *mut i8, type_: acl_type_t) -> *mut _acl;
  pub fn acl_set_fd(fd: i32, acl: *mut _acl) -> i32;
  pub fn acl_set_fd_np(fd: i32, acl: *mut _acl, acl_type: acl_type_t) -> i32;
  pub fn acl_set_file(path_p: *mut i8, type_: acl_type_t, acl: *mut _acl) -> i32;
  pub fn acl_set_link_np(path_p: *mut i8, type_: acl_type_t, acl: *mut _acl) -> i32;
  pub fn acl_copy_ext(buf_p: *mut c_void, acl: *mut _acl, size: i32) -> i32;
  pub fn acl_copy_ext_native(buf_p: *mut c_void, acl: *mut _acl, size: i32) -> i32;
  pub fn acl_copy_int(buf_p: *mut c_void) -> *mut _acl;
  pub fn acl_copy_int_native(buf_p: *mut c_void) -> *mut _acl;
  pub fn acl_from_text(buf_p: *mut i8) -> *mut _acl;
  pub fn acl_size(acl: *mut _acl) -> i32;
  pub fn acl_to_text(acl: *mut _acl, len_p: *mut i32) -> *mut i8;
}
