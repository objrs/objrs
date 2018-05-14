// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

// See https://github.com/opensource-apple/objc4/blob/master/runtime/objc-abi.h

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct objc_image_info {
  version: u32,
  flags: u32,
}

const IS_IOS_SIMULATOR: bool = cfg!(all(
  target_os = "ios",
  any(target_arch = "x86", target_arch = "x86_64")
));

// For flag values, see:
// https://github.com/apple-opensource/ld64/blob/master/src/other/objcimageinfo.cpp
// https://github.com/opensource-apple/objc4/blob/master/runtime/objc-abi.h
// https://github.com/llvm-mirror/clang/blob/master/lib/CodeGen/CGObjCMac.cpp
// https://github.com/opensource-apple/objc4/blob/master/test/unload3.c
const FLAGS_CLASS_PROPERTIES: u32 = 64;
const FLAGS_IS_SIMULATOR: u32 = 32;

impl objc_image_info {
  pub const DEFAULT: objc_image_info = objc_image_info {
    version: 0,
    flags: FLAGS_CLASS_PROPERTIES | (FLAGS_IS_SIMULATOR * IS_IOS_SIMULATOR as u32),
  };
}
