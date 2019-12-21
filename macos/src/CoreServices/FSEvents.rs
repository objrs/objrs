#[allow(unused_imports)]
use crate::objc::*;
use crate::CoreFoundation::__CFAllocator;
use crate::CoreFoundation::__CFArray;
use crate::CoreFoundation::__CFRunLoop;
use crate::CoreFoundation::__CFString;
use crate::CoreFoundation::__CFUUID;
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
pub type FSEventStreamCreateFlags = u32;
pub type FSEventStreamEventFlags = u32;
pub type FSEventStreamEventId = u64;
#[repr(C)]
pub struct __FSEventStream {
  opaque: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FSEventStreamContext {
  pub version: isize,
  pub info: *mut c_void,
  pub retain: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
  pub release: Option<extern "C" fn(*mut c_void) -> ()>,
  pub copyDescription: Option<extern "C" fn(*mut c_void) -> *mut __CFString>,
}
extern "C" {
  pub fn FSEventStreamCreate(
    allocator: *mut __CFAllocator,
    callback: extern "C" fn(
      *mut __FSEventStream,
      *mut c_void,
      usize,
      *mut c_void,
      *mut u32,
      *mut u64,
    ) -> (),
    context: *mut FSEventStreamContext,
    pathsToWatch: *mut __CFArray,
    sinceWhen: u64,
    latency: f64,
    flags: u32,
  ) -> *mut __FSEventStream;
  pub fn FSEventStreamCreateRelativeToDevice(
    allocator: *mut __CFAllocator,
    callback: extern "C" fn(
      *mut __FSEventStream,
      *mut c_void,
      usize,
      *mut c_void,
      *mut u32,
      *mut u64,
    ) -> (),
    context: *mut FSEventStreamContext,
    deviceToWatch: i32,
    pathsToWatchRelativeToDevice: *mut __CFArray,
    sinceWhen: u64,
    latency: f64,
    flags: u32,
  ) -> *mut __FSEventStream;
  pub fn FSEventStreamGetLatestEventId(streamRef: *mut __FSEventStream) -> u64;
  pub fn FSEventStreamGetDeviceBeingWatched(streamRef: *mut __FSEventStream) -> i32;
  pub fn FSEventStreamCopyPathsBeingWatched(streamRef: *mut __FSEventStream) -> *mut __CFArray;
  pub fn FSEventsGetCurrentEventId() -> u64;
  pub fn FSEventsCopyUUIDForDevice(dev: i32) -> *mut __CFUUID;
  pub fn FSEventsGetLastEventIdForDeviceBeforeTime(dev: i32, time: f64) -> u64;
  pub fn FSEventsPurgeEventsForDeviceUpToEventId(dev: i32, eventId: u64) -> u8;
  pub fn FSEventStreamRetain(streamRef: *mut __FSEventStream) -> ();
  pub fn FSEventStreamRelease(streamRef: *mut __FSEventStream) -> ();
  pub fn FSEventStreamScheduleWithRunLoop(
    streamRef: *mut __FSEventStream,
    runLoop: *mut __CFRunLoop,
    runLoopMode: *mut __CFString,
  ) -> ();
  pub fn FSEventStreamUnscheduleFromRunLoop(
    streamRef: *mut __FSEventStream,
    runLoop: *mut __CFRunLoop,
    runLoopMode: *mut __CFString,
  ) -> ();
  pub fn FSEventStreamSetDispatchQueue(streamRef: *mut __FSEventStream, q: *mut NSObject) -> ();
  pub fn FSEventStreamInvalidate(streamRef: *mut __FSEventStream) -> ();
  pub fn FSEventStreamStart(streamRef: *mut __FSEventStream) -> u8;
  pub fn FSEventStreamFlushAsync(streamRef: *mut __FSEventStream) -> u64;
  pub fn FSEventStreamFlushSync(streamRef: *mut __FSEventStream) -> ();
  pub fn FSEventStreamStop(streamRef: *mut __FSEventStream) -> ();
  pub fn FSEventStreamShow(streamRef: *mut __FSEventStream) -> ();
  pub fn FSEventStreamCopyDescription(streamRef: *mut __FSEventStream) -> *mut __CFString;
  pub fn FSEventStreamSetExclusionPaths(
    streamRef: *mut __FSEventStream,
    pathsToExclude: *mut __CFArray,
  ) -> u8;
}
