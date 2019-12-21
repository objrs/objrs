#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use bitflags::bitflags;
#[allow(unused_imports)]
use objrs::objrs;
pub type mach_msg_timeout_t = u32;
pub type mach_msg_size_t = u32;
pub type mach_msg_id_t = i32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mach_msg_type_descriptor_t {
  pub pad1: u32,
  pub pad2: u32,
  pub pad3: u32,
  pub type_: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mach_msg_port_descriptor_t {
  pub name: u32,
  pub pad1: u32,
  pub pad2: u32,
  pub disposition: u32,
  pub type_: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mach_msg_ool_descriptor32_t {
  pub address: u32,
  pub size: u32,
  pub deallocate: u32,
  pub copy: u32,
  pub pad1: u32,
  pub type_: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mach_msg_ool_descriptor64_t {
  pub address: u64,
  pub deallocate: u32,
  pub copy: u32,
  pub pad1: u32,
  pub type_: u32,
  pub size: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mach_msg_ool_descriptor_t {
  pub address: *mut c_void,
  pub deallocate: u32,
  pub copy: u32,
  pub pad1: u32,
  pub type_: u32,
  pub size: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mach_msg_ool_ports_descriptor32_t {
  pub address: u32,
  pub count: u32,
  pub deallocate: u32,
  pub copy: u32,
  pub disposition: u32,
  pub type_: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mach_msg_ool_ports_descriptor64_t {
  pub address: u64,
  pub deallocate: u32,
  pub copy: u32,
  pub disposition: u32,
  pub type_: u32,
  pub count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mach_msg_ool_ports_descriptor_t {
  pub address: *mut c_void,
  pub deallocate: u32,
  pub copy: u32,
  pub disposition: u32,
  pub type_: u32,
  pub count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union mach_msg_descriptor_t {
  pub port: mach_msg_port_descriptor_t,
  pub out_of_line: mach_msg_ool_descriptor_t,
  pub ool_ports: mach_msg_ool_ports_descriptor_t,
  pub type_: mach_msg_type_descriptor_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mach_msg_body_t {
  pub msgh_descriptor_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mach_msg_header_t {
  pub msgh_bits: u32,
  pub msgh_size: u32,
  pub msgh_remote_port: u32,
  pub msgh_local_port: u32,
  pub msgh_voucher_port: u32,
  pub msgh_id: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mach_msg_base_t {
  pub header: mach_msg_header_t,
  pub body: mach_msg_body_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mach_msg_trailer_t {
  pub msgh_trailer_type: u32,
  pub msgh_trailer_size: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mach_msg_seqno_trailer_t {
  pub msgh_trailer_type: u32,
  pub msgh_trailer_size: u32,
  pub msgh_seqno: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct security_token_t {
  pub val: [u32; 2],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mach_msg_security_trailer_t {
  pub msgh_trailer_type: u32,
  pub msgh_trailer_size: u32,
  pub msgh_seqno: u32,
  pub msgh_sender: security_token_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_token_t {
  pub val: [u32; 8],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mach_msg_audit_trailer_t {
  pub msgh_trailer_type: u32,
  pub msgh_trailer_size: u32,
  pub msgh_seqno: u32,
  pub msgh_sender: security_token_t,
  pub msgh_audit: audit_token_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mach_msg_context_trailer_t {
  pub msgh_trailer_type: u32,
  pub msgh_trailer_size: u32,
  pub msgh_seqno: u32,
  pub msgh_sender: security_token_t,
  pub msgh_audit: audit_token_t,
  pub msgh_context: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_labels_t {
  pub sender: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mach_msg_mac_trailer_t {
  pub msgh_trailer_type: u32,
  pub msgh_trailer_size: u32,
  pub msgh_seqno: u32,
  pub msgh_sender: security_token_t,
  pub msgh_audit: audit_token_t,
  pub msgh_context: u64,
  pub msgh_ad: i32,
  pub msgh_labels: msg_labels_t,
}
pub type mach_msg_max_trailer_t = mach_msg_mac_trailer_t;
pub type mach_msg_format_0_trailer_t = mach_msg_security_trailer_t;
pub type mach_msg_options_t = i32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mach_msg_empty_send_t {
  pub header: mach_msg_header_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mach_msg_empty_rcv_t {
  pub header: mach_msg_header_t,
  pub trailer: mach_msg_trailer_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union mach_msg_empty_t {
  pub send: mach_msg_empty_send_t,
  pub rcv: mach_msg_empty_rcv_t,
}
pub type mach_msg_type_size_t = u32;
pub type mach_msg_type_number_t = u32;
pub type mach_msg_option_t = i32;
pub type mach_msg_return_t = i32;
extern "C" {
  pub fn mach_msg_overwrite(
    msg: *mut mach_msg_header_t,
    option: i32,
    send_size: u32,
    rcv_size: u32,
    rcv_name: u32,
    timeout: u32,
    notify: u32,
    rcv_msg: *mut mach_msg_header_t,
    rcv_limit: u32,
  ) -> i32;
  pub fn mach_msg(
    msg: *mut mach_msg_header_t,
    option: i32,
    send_size: u32,
    rcv_size: u32,
    rcv_name: u32,
    timeout: u32,
    notify: u32,
  ) -> i32;
  pub fn mach_voucher_deallocate(voucher: u32) -> i32;
}
