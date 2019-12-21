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
pub struct wide {
  pub lo: u32,
  pub hi: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UnsignedWide {
  pub lo: u32,
  pub hi: u32,
}
pub type Fixed = i32;
pub type Fract = i32;
pub type UnsignedFixed = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Float80 {
  pub exp: i16,
  pub man: [u16; 4],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Float96 {
  pub exp: [i16; 2],
  pub man: [u16; 4],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Float32Point {
  pub x: f32,
  pub y: f32,
}
pub type OSErr = i16;
pub type OSStatus = i32;
pub type Duration = i32;
pub type AbsoluteTime = UnsignedWide;
pub type OptionBits = u32;
pub type PBVersion = u32;
pub type ScriptCode = i16;
pub type LangCode = i16;
pub type RegionCode = i16;
pub type FourCharCode = u32;
pub type OSType = u32;
pub type ResType = u32;
pub type UniversalProcPtr = Option<extern "C" fn() -> isize>;
pub type UnicodeScalarValue = u32;
pub type UTF32Char = u32;
pub type UniChar = u16;
pub type UTF16Char = u16;
pub type UTF8Char = u8;
pub type StrFileName = [u8; 64];
pub type ConstStrFileNameParam = *mut u8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ProcessSerialNumber {
  pub highLongOfPSN: u32,
  pub lowLongOfPSN: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Point {
  pub v: i16,
  pub h: i16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Rect {
  pub top: i16,
  pub left: i16,
  pub bottom: i16,
  pub right: i16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FixedPoint {
  pub x: i32,
  pub y: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FixedRect {
  pub left: i32,
  pub top: i32,
  pub right: i32,
  pub bottom: i32,
}
pub type StyleField = u8;
pub type TimeValue = i32;
pub type TimeScale = i32;
pub type CompTimeValue = wide;
pub type TimeValue64 = i64;
#[repr(C)]
pub struct TimeBaseRecord {
  opaque: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TimeRecord {
  pub value: wide,
  pub scale: i32,
  pub base: *mut TimeBaseRecord,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NumVersion {
  pub nonRelRev: u8,
  pub stage: u8,
  pub minorAndBugRev: u8,
  pub majorRev: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union NumVersionVariant {
  pub parts: NumVersion,
  pub whole: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VersRec {
  pub numericVersion: NumVersion,
  pub countryCode: i16,
  pub shortVersion: [u8; 256],
  pub reserved: [u8; 256],
}
pub type Byte = u8;
pub type SignedByte = i8;
pub type extended80 = Float80;
pub type extended96 = Float96;
pub type VHSelect = i8;
extern "C" {
  pub fn Debugger() -> ();
  pub fn DebugStr(debuggerMsg: *mut u8) -> ();
  pub fn SysBreak() -> ();
  pub fn SysBreakStr(debuggerMsg: *mut u8) -> ();
  pub fn SysBreakFunc(debuggerMsg: *mut u8) -> ();
}
