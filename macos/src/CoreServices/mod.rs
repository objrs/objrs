#[allow(unused_imports)]
use crate::objc::*;
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
pub mod AE;
pub mod CarbonCore;
pub mod DictionaryServices;
pub mod FSEvents;
pub mod LaunchServices;
pub mod Metadata;
pub mod OSServices;
pub mod SearchKit;
pub mod SharedFileList;
#[cfg(feature = "RK_CoreServices")]
#[link(name = "CoreServices", kind = "framework")]
extern "C" {}
