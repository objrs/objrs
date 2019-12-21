#[allow(unused_imports)]
use crate::objc::*;
use crate::CoreFoundation::FSRef;
use crate::CoreFoundation::__CFAllocator;
use crate::CoreFoundation::__CFArray;
use crate::CoreFoundation::__CFDictionary;
use crate::CoreFoundation::__CFError;
use crate::CoreFoundation::__CFRunLoop;
use crate::CoreFoundation::__CFString;
use crate::CoreFoundation::__CFURL;
use crate::CoreServices::LaunchServices::OpaqueIconRef;
use crate::Security::AuthorizationOpaqueRef;
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
pub struct OpaqueLSSharedFileListRef {
  opaque: u32,
}
#[repr(C)]
pub struct OpaqueLSSharedFileListItemRef {
  opaque: u32,
}
pub type LSSharedFileListResolutionFlags = u32;
extern "C" {
  pub fn LSSharedFileListGetTypeID() -> usize;
  pub fn LSSharedFileListItemGetTypeID() -> usize;
  pub fn LSSharedFileListCreate(
    inAllocator: *mut __CFAllocator,
    inListType: *mut __CFString,
    listOptions: *mut c_void,
  ) -> *mut OpaqueLSSharedFileListRef;
  pub fn LSSharedFileListSetAuthorization(
    inList: *mut OpaqueLSSharedFileListRef,
    inAuthorization: *mut AuthorizationOpaqueRef,
  ) -> i32;
  pub fn LSSharedFileListAddObserver(
    inList: *mut OpaqueLSSharedFileListRef,
    inRunloop: *mut __CFRunLoop,
    inRunloopMode: *mut __CFString,
    callback: Option<extern "C" fn(*mut OpaqueLSSharedFileListRef, *mut c_void) -> ()>,
    context: *mut c_void,
  ) -> ();
  pub fn LSSharedFileListRemoveObserver(
    inList: *mut OpaqueLSSharedFileListRef,
    inRunloop: *mut __CFRunLoop,
    inRunloopMode: *mut __CFString,
    callback: Option<extern "C" fn(*mut OpaqueLSSharedFileListRef, *mut c_void) -> ()>,
    context: *mut c_void,
  ) -> ();
  pub fn LSSharedFileListGetSeedValue(inList: *mut OpaqueLSSharedFileListRef) -> u32;
  pub fn LSSharedFileListCopyProperty(
    inList: *mut OpaqueLSSharedFileListRef,
    inPropertyName: *mut __CFString,
  ) -> *mut c_void;
  pub fn LSSharedFileListSetProperty(
    inList: *mut OpaqueLSSharedFileListRef,
    inPropertyName: *mut __CFString,
    inPropertyData: *mut c_void,
  ) -> i32;
  pub fn LSSharedFileListCopySnapshot(
    inList: *mut OpaqueLSSharedFileListRef,
    outSnapshotSeed: *mut u32,
  ) -> *mut __CFArray;
  pub fn LSSharedFileListInsertItemURL(
    inList: *mut OpaqueLSSharedFileListRef,
    insertAfterThisItem: *mut OpaqueLSSharedFileListItemRef,
    inDisplayName: *mut __CFString,
    inIconRef: *mut OpaqueIconRef,
    inURL: *mut __CFURL,
    inPropertiesToSet: *mut __CFDictionary,
    inPropertiesToClear: *mut __CFArray,
  ) -> *mut OpaqueLSSharedFileListItemRef;
  pub fn LSSharedFileListInsertItemFSRef(
    inList: *mut OpaqueLSSharedFileListRef,
    insertAfterThisItem: *mut OpaqueLSSharedFileListItemRef,
    inDisplayName: *mut __CFString,
    inIconRef: *mut OpaqueIconRef,
    inFSRef: *mut FSRef,
    inPropertiesToSet: *mut __CFDictionary,
    inPropertiesToClear: *mut __CFArray,
  ) -> *mut OpaqueLSSharedFileListItemRef;
  pub fn LSSharedFileListItemMove(
    inList: *mut OpaqueLSSharedFileListRef,
    inItem: *mut OpaqueLSSharedFileListItemRef,
    inMoveAfterItem: *mut OpaqueLSSharedFileListItemRef,
  ) -> i32;
  pub fn LSSharedFileListItemRemove(
    inList: *mut OpaqueLSSharedFileListRef,
    inItem: *mut OpaqueLSSharedFileListItemRef,
  ) -> i32;
  pub fn LSSharedFileListRemoveAllItems(inList: *mut OpaqueLSSharedFileListRef) -> i32;
  pub fn LSSharedFileListItemGetID(inItem: *mut OpaqueLSSharedFileListItemRef) -> u32;
  pub fn LSSharedFileListItemCopyIconRef(
    inItem: *mut OpaqueLSSharedFileListItemRef,
  ) -> *mut OpaqueIconRef;
  pub fn LSSharedFileListItemCopyDisplayName(
    inItem: *mut OpaqueLSSharedFileListItemRef,
  ) -> *mut __CFString;
  pub fn LSSharedFileListItemResolve(
    inItem: *mut OpaqueLSSharedFileListItemRef,
    inFlags: u32,
    outURL: *mut *mut __CFURL,
    outRef: *mut FSRef,
  ) -> i32;
  pub fn LSSharedFileListItemCopyResolvedURL(
    inItem: *mut OpaqueLSSharedFileListItemRef,
    inFlags: u32,
    outError: *mut *mut __CFError,
  ) -> *mut __CFURL;
  pub fn LSSharedFileListItemCopyProperty(
    inItem: *mut OpaqueLSSharedFileListItemRef,
    inPropertyName: *mut __CFString,
  ) -> *mut c_void;
  pub fn LSSharedFileListItemSetProperty(
    inItem: *mut OpaqueLSSharedFileListItemRef,
    inPropertyName: *mut __CFString,
    inPropertyData: *mut c_void,
  ) -> i32;
}
