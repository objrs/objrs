#[allow(unused_imports)]
use crate::objc::*;
use crate::CoreFoundation::__CFDictionary;
use crate::CoreFoundation::__CFString;
use crate::Foundation::NSDictionary;
use crate::Foundation::NSSecureCodingProto;
use crate::Foundation::NSString;
use crate::NSObject::NSObject;
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
pub type IOSurfaceID = u32;
bitflags! { # [ repr ( C ) ] pub struct IOSurfaceLockOptions : u32 { const kIOSurfaceLockReadOnly = 1 ; const kIOSurfaceLockAvoidSync = 2 ; } }
bitflags! { # [ repr ( C ) ] pub struct IOSurfacePurgeabilityState : u32 { const kIOSurfacePurgeableNonVolatile = 0 ; const kIOSurfacePurgeableVolatile = 1 ; const kIOSurfacePurgeableEmpty = 2 ; const kIOSurfacePurgeableKeepCurrent = 3 ; } }
#[repr(C)]
pub struct __IOSurface {
  opaque: u32,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "IOSurface", kind = "framework")]
pub struct IOSurface;
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum IOSurfaceComponentName {
  kIOSurfaceComponentNameUnknown = 0,
  kIOSurfaceComponentNameAlpha = 1,
  kIOSurfaceComponentNameRed = 2,
  kIOSurfaceComponentNameGreen = 3,
  kIOSurfaceComponentNameBlue = 4,
  kIOSurfaceComponentNameLuma = 5,
  kIOSurfaceComponentNameChromaRed = 6,
  kIOSurfaceComponentNameChromaBlue = 7,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum IOSurfaceComponentType {
  kIOSurfaceComponentTypeUnknown = 0,
  kIOSurfaceComponentTypeUnsignedInteger = 1,
  kIOSurfaceComponentTypeSignedInteger = 2,
  kIOSurfaceComponentTypeFloat = 3,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum IOSurfaceComponentRange {
  kIOSurfaceComponentRangeUnknown = 0,
  kIOSurfaceComponentRangeFullRange = 1,
  kIOSurfaceComponentRangeVideoRange = 2,
  kIOSurfaceComponentRangeWideRange = 3,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum IOSurfaceSubsampling {
  kIOSurfaceSubsamplingUnknown = 0,
  kIOSurfaceSubsamplingNone = 1,
  kIOSurfaceSubsampling422 = 2,
  kIOSurfaceSubsampling420 = 3,
  kIOSurfaceSubsampling411 = 4,
}
#[cfg(feature = "RK_IOSurface")]
#[link(name = "IOSurface", kind = "framework")]
extern "C" {
  pub fn IOSurfaceGetTypeID() -> usize;
  pub fn IOSurfaceCreate(properties: *mut __CFDictionary) -> *mut __IOSurface;
  pub fn IOSurfaceLookup(csid: u32) -> *mut __IOSurface;
  pub fn IOSurfaceGetID(buffer: *mut __IOSurface) -> u32;
  pub fn IOSurfaceLock(
    buffer: *mut __IOSurface,
    options: IOSurfaceLockOptions,
    seed: *mut u32,
  ) -> i32;
  pub fn IOSurfaceUnlock(
    buffer: *mut __IOSurface,
    options: IOSurfaceLockOptions,
    seed: *mut u32,
  ) -> i32;
  pub fn IOSurfaceGetAllocSize(buffer: *mut __IOSurface) -> usize;
  pub fn IOSurfaceGetWidth(buffer: *mut __IOSurface) -> usize;
  pub fn IOSurfaceGetHeight(buffer: *mut __IOSurface) -> usize;
  pub fn IOSurfaceGetBytesPerElement(buffer: *mut __IOSurface) -> usize;
  pub fn IOSurfaceGetBytesPerRow(buffer: *mut __IOSurface) -> usize;
  pub fn IOSurfaceGetBaseAddress(buffer: *mut __IOSurface) -> *mut c_void;
  pub fn IOSurfaceGetElementWidth(buffer: *mut __IOSurface) -> usize;
  pub fn IOSurfaceGetElementHeight(buffer: *mut __IOSurface) -> usize;
  pub fn IOSurfaceGetPixelFormat(buffer: *mut __IOSurface) -> u32;
  pub fn IOSurfaceGetSeed(buffer: *mut __IOSurface) -> u32;
  pub fn IOSurfaceGetPlaneCount(buffer: *mut __IOSurface) -> usize;
  pub fn IOSurfaceGetWidthOfPlane(buffer: *mut __IOSurface, planeIndex: usize) -> usize;
  pub fn IOSurfaceGetHeightOfPlane(buffer: *mut __IOSurface, planeIndex: usize) -> usize;
  pub fn IOSurfaceGetBytesPerElementOfPlane(buffer: *mut __IOSurface, planeIndex: usize) -> usize;
  pub fn IOSurfaceGetBytesPerRowOfPlane(buffer: *mut __IOSurface, planeIndex: usize) -> usize;
  pub fn IOSurfaceGetBaseAddressOfPlane(buffer: *mut __IOSurface, planeIndex: usize)
    -> *mut c_void;
  pub fn IOSurfaceGetElementWidthOfPlane(buffer: *mut __IOSurface, planeIndex: usize) -> usize;
  pub fn IOSurfaceGetElementHeightOfPlane(buffer: *mut __IOSurface, planeIndex: usize) -> usize;
  pub fn IOSurfaceGetNumberOfComponentsOfPlane(
    buffer: *mut __IOSurface,
    planeIndex: usize,
  ) -> usize;
  pub fn IOSurfaceGetNameOfComponentOfPlane(
    buffer: *mut __IOSurface,
    planeIndex: usize,
    componentIndex: usize,
  ) -> IOSurfaceComponentName;
  pub fn IOSurfaceGetTypeOfComponentOfPlane(
    buffer: *mut __IOSurface,
    planeIndex: usize,
    componentIndex: usize,
  ) -> IOSurfaceComponentType;
  pub fn IOSurfaceGetRangeOfComponentOfPlane(
    buffer: *mut __IOSurface,
    planeIndex: usize,
    componentIndex: usize,
  ) -> IOSurfaceComponentRange;
  pub fn IOSurfaceGetBitDepthOfComponentOfPlane(
    buffer: *mut __IOSurface,
    planeIndex: usize,
    componentIndex: usize,
  ) -> usize;
  pub fn IOSurfaceGetBitOffsetOfComponentOfPlane(
    buffer: *mut __IOSurface,
    planeIndex: usize,
    componentIndex: usize,
  ) -> usize;
  pub fn IOSurfaceGetSubsampling(buffer: *mut __IOSurface) -> IOSurfaceSubsampling;
  pub fn IOSurfaceSetValue(
    buffer: *mut __IOSurface,
    key: *mut __CFString,
    value: *mut c_void,
  ) -> ();
  pub fn IOSurfaceCopyValue(buffer: *mut __IOSurface, key: *mut __CFString) -> *mut c_void;
  pub fn IOSurfaceRemoveValue(buffer: *mut __IOSurface, key: *mut __CFString) -> ();
  pub fn IOSurfaceSetValues(buffer: *mut __IOSurface, keysAndValues: *mut __CFDictionary) -> ();
  pub fn IOSurfaceCopyAllValues(buffer: *mut __IOSurface) -> *mut __CFDictionary;
  pub fn IOSurfaceRemoveAllValues(buffer: *mut __IOSurface) -> ();
  pub fn IOSurfaceCreateMachPort(buffer: *mut __IOSurface) -> u32;
  pub fn IOSurfaceLookupFromMachPort(port: u32) -> *mut __IOSurface;
  pub fn IOSurfaceGetPropertyMaximum(property: *mut __CFString) -> usize;
  pub fn IOSurfaceGetPropertyAlignment(property: *mut __CFString) -> usize;
  pub fn IOSurfaceAlignProperty(property: *mut __CFString, value: usize) -> usize;
  pub fn IOSurfaceIncrementUseCount(buffer: *mut __IOSurface) -> ();
  pub fn IOSurfaceDecrementUseCount(buffer: *mut __IOSurface) -> ();
  pub fn IOSurfaceGetUseCount(buffer: *mut __IOSurface) -> i32;
  pub fn IOSurfaceIsInUse(buffer: *mut __IOSurface) -> u8;
  pub fn IOSurfaceAllowsPixelSizeCasting(buffer: *mut __IOSurface) -> u8;
  pub fn IOSurfaceSetPurgeable(buffer: *mut __IOSurface, newState: u32, oldState: *mut u32) -> i32;
  pub fn IOSurfaceCreateXPCObject(aSurface: *mut __IOSurface) -> *mut NSObject;
  pub fn IOSurfaceLookupFromXPCObject(xobj: *mut NSObject) -> *mut __IOSurface;
}
