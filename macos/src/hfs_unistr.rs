#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use bitflags::bitflags;
#[allow(unused_imports)]
use objrs::objrs;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HFSUniStr255 {
  pub length: u16,
  pub unicode: [u16; 255],
}
