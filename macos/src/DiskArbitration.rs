#[allow(unused_imports)]
use crate::objc::*;
use crate::CoreFoundation::__CFAllocator;
use crate::CoreFoundation::__CFArray;
use crate::CoreFoundation::__CFDictionary;
use crate::CoreFoundation::__CFRunLoop;
use crate::CoreFoundation::__CFString;
use crate::CoreFoundation::__CFURL;
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
#[repr(C)]
pub struct __DASession {
  opaque: u32,
}
#[repr(C)]
pub struct __DADisk {
  opaque: u32,
}
pub type DAReturn = i32;
#[repr(C)]
pub struct __DADissenter {
  opaque: u32,
}
pub type DADiskMountOptions = u32;
pub type DADiskRenameOptions = u32;
pub type DADiskUnmountOptions = u32;
pub type DADiskEjectOptions = u32;
pub type DADiskClaimOptions = u32;
pub type DADiskOptions = u32;
#[cfg(feature = "RK_DiskArbitration")]
#[link(name = "DiskArbitration", kind = "framework")]
extern "C" {
  pub fn DASessionGetTypeID() -> usize;
  pub fn DASessionCreate(allocator: *mut __CFAllocator) -> *mut __DASession;
  pub fn DASessionScheduleWithRunLoop(
    session: *mut __DASession,
    runLoop: *mut __CFRunLoop,
    runLoopMode: *mut __CFString,
  ) -> ();
  pub fn DASessionUnscheduleFromRunLoop(
    session: *mut __DASession,
    runLoop: *mut __CFRunLoop,
    runLoopMode: *mut __CFString,
  ) -> ();
  pub fn DASessionSetDispatchQueue(session: *mut __DASession, queue: *mut NSObject) -> ();
  pub fn DADiskGetTypeID() -> usize;
  pub fn DADiskCreateFromBSDName(
    allocator: *mut __CFAllocator,
    session: *mut __DASession,
    name: *mut i8,
  ) -> *mut __DADisk;
  pub fn DADiskCreateFromIOMedia(
    allocator: *mut __CFAllocator,
    session: *mut __DASession,
    media: u32,
  ) -> *mut __DADisk;
  pub fn DADiskCreateFromVolumePath(
    allocator: *mut __CFAllocator,
    session: *mut __DASession,
    path: *mut __CFURL,
  ) -> *mut __DADisk;
  pub fn DADiskGetBSDName(disk: *mut __DADisk) -> *mut i8;
  pub fn DADiskCopyIOMedia(disk: *mut __DADisk) -> u32;
  pub fn DADiskCopyDescription(disk: *mut __DADisk) -> *mut __CFDictionary;
  pub fn DADiskCopyWholeDisk(disk: *mut __DADisk) -> *mut __DADisk;
  pub fn DADissenterCreate(
    allocator: *mut __CFAllocator,
    status: i32,
    string: *mut __CFString,
  ) -> *mut __DADissenter;
  pub fn DADissenterGetStatus(dissenter: *mut __DADissenter) -> i32;
  pub fn DADissenterGetStatusString(dissenter: *mut __DADissenter) -> *mut __CFString;
  pub fn DARegisterDiskAppearedCallback(
    session: *mut __DASession,
    match_: *mut __CFDictionary,
    callback: extern "C" fn(*mut __DADisk, *mut c_void) -> (),
    context: *mut c_void,
  ) -> ();
  pub fn DARegisterDiskDescriptionChangedCallback(
    session: *mut __DASession,
    match_: *mut __CFDictionary,
    watch: *mut __CFArray,
    callback: extern "C" fn(*mut __DADisk, *mut __CFArray, *mut c_void) -> (),
    context: *mut c_void,
  ) -> ();
  pub fn DARegisterDiskDisappearedCallback(
    session: *mut __DASession,
    match_: *mut __CFDictionary,
    callback: extern "C" fn(*mut __DADisk, *mut c_void) -> (),
    context: *mut c_void,
  ) -> ();
  pub fn DADiskMount(
    disk: *mut __DADisk,
    path: *mut __CFURL,
    options: u32,
    callback: Option<extern "C" fn(*mut __DADisk, *mut __DADissenter, *mut c_void) -> ()>,
    context: *mut c_void,
  ) -> ();
  pub fn DADiskMountWithArguments(
    disk: *mut __DADisk,
    path: *mut __CFURL,
    options: u32,
    callback: Option<extern "C" fn(*mut __DADisk, *mut __DADissenter, *mut c_void) -> ()>,
    context: *mut c_void,
    arguments: *mut *mut __CFString,
  ) -> ();
  pub fn DARegisterDiskMountApprovalCallback(
    session: *mut __DASession,
    match_: *mut __CFDictionary,
    callback: extern "C" fn(*mut __DADisk, *mut c_void) -> *mut __DADissenter,
    context: *mut c_void,
  ) -> ();
  pub fn DADiskRename(
    disk: *mut __DADisk,
    name: *mut __CFString,
    options: u32,
    callback: Option<extern "C" fn(*mut __DADisk, *mut __DADissenter, *mut c_void) -> ()>,
    context: *mut c_void,
  ) -> ();
  pub fn DADiskUnmount(
    disk: *mut __DADisk,
    options: u32,
    callback: Option<extern "C" fn(*mut __DADisk, *mut __DADissenter, *mut c_void) -> ()>,
    context: *mut c_void,
  ) -> ();
  pub fn DARegisterDiskUnmountApprovalCallback(
    session: *mut __DASession,
    match_: *mut __CFDictionary,
    callback: extern "C" fn(*mut __DADisk, *mut c_void) -> *mut __DADissenter,
    context: *mut c_void,
  ) -> ();
  pub fn DADiskEject(
    disk: *mut __DADisk,
    options: u32,
    callback: Option<extern "C" fn(*mut __DADisk, *mut __DADissenter, *mut c_void) -> ()>,
    context: *mut c_void,
  ) -> ();
  pub fn DARegisterDiskEjectApprovalCallback(
    session: *mut __DASession,
    match_: *mut __CFDictionary,
    callback: extern "C" fn(*mut __DADisk, *mut c_void) -> *mut __DADissenter,
    context: *mut c_void,
  ) -> ();
  pub fn DADiskClaim(
    disk: *mut __DADisk,
    options: u32,
    release: Option<extern "C" fn(*mut __DADisk, *mut c_void) -> *mut __DADissenter>,
    releaseContext: *mut c_void,
    callback: Option<extern "C" fn(*mut __DADisk, *mut __DADissenter, *mut c_void) -> ()>,
    callbackContext: *mut c_void,
  ) -> ();
  pub fn DADiskIsClaimed(disk: *mut __DADisk) -> u8;
  pub fn DADiskUnclaim(disk: *mut __DADisk) -> ();
  pub fn DARegisterDiskPeekCallback(
    session: *mut __DASession,
    match_: *mut __CFDictionary,
    order: isize,
    callback: extern "C" fn(*mut __DADisk, *mut c_void) -> (),
    context: *mut c_void,
  ) -> ();
  pub fn DADiskGetOptions(disk: *mut __DADisk) -> u32;
  pub fn DADiskSetOptions(disk: *mut __DADisk, options: u32, value: u8) -> i32;
  pub fn DAUnregisterCallback(
    session: *mut __DASession,
    callback: *mut c_void,
    context: *mut c_void,
  ) -> ();
}
