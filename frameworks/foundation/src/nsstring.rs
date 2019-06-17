// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

extern crate libc;

extern crate core;
extern crate objrs;

use crate::nsobject;
use objrs::objrs;

// TODO: LLVM's Early CSE pass ("early-cse": http://llvm.org/doxygen/EarlyCSE_8cpp_source.html) is breaking objrs. It optimizes out the static selref variable. Even marking the variable as #[used] doesn't fix it (that causes the variable to not be optimized out, but the LLVM IR still doesn't use it, as it prefers loading the address of the method name instead). Run rustc with ` -C opt-level=0 -C passes=early-cse` to see the impact (use `-C llvm-args=-print-after-all` to verify that it's indeed the Early CSE pass that's causing this).

#[repr(transparent)]
pub struct NSStringEncoding(usize);

#[allow(non_upper_case_globals)]
pub const NSASCIIStringEncoding: NSStringEncoding = NSStringEncoding(1);
#[allow(non_upper_case_globals)]
pub const NSNEXTSTEPStringEncoding: NSStringEncoding = NSStringEncoding(2);
#[allow(non_upper_case_globals)]
pub const NSJapaneseEUCStringEncoding: NSStringEncoding = NSStringEncoding(3);
#[allow(non_upper_case_globals)]
pub const NSUTF8StringEncoding: NSStringEncoding = NSStringEncoding(4);
#[allow(non_upper_case_globals)]
pub const NSISOLatin1StringEncoding: NSStringEncoding = NSStringEncoding(5);
#[allow(non_upper_case_globals)]
pub const NSSymbolStringEncoding: NSStringEncoding = NSStringEncoding(6);
#[allow(non_upper_case_globals)]
pub const NSNonLossyASCIIStringEncoding: NSStringEncoding = NSStringEncoding(7);
#[allow(non_upper_case_globals)]
pub const NSShiftJISStringEncoding: NSStringEncoding = NSStringEncoding(8);
#[allow(non_upper_case_globals)]
pub const NSISOLatin2StringEncoding: NSStringEncoding = NSStringEncoding(9);
#[allow(non_upper_case_globals)]
pub const NSUnicodeStringEncoding: NSStringEncoding = NSStringEncoding(10);
#[allow(non_upper_case_globals)]
pub const NSWindowsCP1251StringEncoding: NSStringEncoding = NSStringEncoding(11);
#[allow(non_upper_case_globals)]
pub const NSWindowsCP1252StringEncoding: NSStringEncoding = NSStringEncoding(12);
#[allow(non_upper_case_globals)]
pub const NSWindowsCP1253StringEncoding: NSStringEncoding = NSStringEncoding(13);
#[allow(non_upper_case_globals)]
pub const NSWindowsCP1254StringEncoding: NSStringEncoding = NSStringEncoding(14);
#[allow(non_upper_case_globals)]
pub const NSWindowsCP1250StringEncoding: NSStringEncoding = NSStringEncoding(15);
#[allow(non_upper_case_globals)]
pub const NSISO2022JPStringEncoding: NSStringEncoding = NSStringEncoding(21);
#[allow(non_upper_case_globals)]
pub const NSMacOSRomanStringEncoding: NSStringEncoding = NSStringEncoding(30);
#[allow(non_upper_case_globals)]
pub const NSUTF16StringEncoding: NSStringEncoding = NSUnicodeStringEncoding;
#[allow(non_upper_case_globals)]
pub const NSUTF16BigEndianStringEncoding: NSStringEncoding = NSStringEncoding(0x90000100);
#[allow(non_upper_case_globals)]
pub const NSUTF16LittleEndianStringEncoding: NSStringEncoding = NSStringEncoding(0x94000100);
#[allow(non_upper_case_globals)]
pub const NSUTF32StringEncoding: NSStringEncoding = NSStringEncoding(0x8c000100);
#[allow(non_upper_case_globals)]
pub const NSUTF32BigEndianStringEncoding: NSStringEncoding = NSStringEncoding(0x98000100);
#[allow(non_upper_case_globals)]
pub const NSUTF32LittleEndianStringEncoding: NSStringEncoding = NSStringEncoding(0x9c000100);
#[allow(non_upper_case_globals)]
#[deprecated]
pub const NSProprietaryStringEncoding: NSStringEncoding = NSStringEncoding(65536);

impl NSStringEncoding {
  pub const ASCII: NSStringEncoding = NSASCIIStringEncoding;
  pub const NEXTSTEP: NSStringEncoding = NSNEXTSTEPStringEncoding;
  pub const JAPANESE_EUC: NSStringEncoding = NSJapaneseEUCStringEncoding;
  pub const UTF8: NSStringEncoding = NSUTF8StringEncoding;
  pub const ISO_LATIN_1: NSStringEncoding = NSISOLatin1StringEncoding;
  pub const SYMBOL: NSStringEncoding = NSSymbolStringEncoding;
  pub const NON_LOSSY_ASCII: NSStringEncoding = NSNonLossyASCIIStringEncoding;
  pub const SHIFT_JIS: NSStringEncoding = NSShiftJISStringEncoding;
  pub const ISO_LATIN_2: NSStringEncoding = NSISOLatin2StringEncoding;
  pub const UNICODE: NSStringEncoding = NSUnicodeStringEncoding;
  pub const WINDOWS_CP_1251: NSStringEncoding = NSWindowsCP1251StringEncoding;
  pub const WINDOWS_CP_1252: NSStringEncoding = NSWindowsCP1252StringEncoding;
  pub const WINDOWS_CP_1253: NSStringEncoding = NSWindowsCP1253StringEncoding;
  pub const WINDOWS_CP_1254: NSStringEncoding = NSWindowsCP1254StringEncoding;
  pub const WINDOWS_CP_1250: NSStringEncoding = NSWindowsCP1250StringEncoding;
  pub const ISO_2022_JP: NSStringEncoding = NSISO2022JPStringEncoding;
  pub const MAC_OS_ROMAN: NSStringEncoding = NSMacOSRomanStringEncoding;
  pub const UTF16: NSStringEncoding = NSUTF16StringEncoding;
  pub const UTF16_BIG_ENDIAN: NSStringEncoding = NSUTF16BigEndianStringEncoding;
  pub const UTF16_LITTLE_ENDIAN: NSStringEncoding = NSUTF16LittleEndianStringEncoding;
  pub const UTF32: NSStringEncoding = NSUTF32StringEncoding;
  pub const UTF32_BIG_ENDIAN: NSStringEncoding = NSUTF32BigEndianStringEncoding;
  pub const UTF32_LITTLE_ENDIAN: NSStringEncoding = NSUTF32LittleEndianStringEncoding;
  #[allow(deprecated)]
  #[deprecated]
  pub const PROPRIETARY: NSStringEncoding = NSProprietaryStringEncoding;
}

#[objrs(class, super = nsobject::NSObject)]
#[link(name = "Foundation", kind = "framework")]
pub struct NSString;

#[objrs(impl)]
#[link(name = "Foundation", kind = "framework")]
impl NSString {
  #[objrs(selector = "alloc")]
  pub fn alloc() -> objrs::Alloc<NSString> {}

  #[objrs(selector = "alloc")]
  #[inline(always)]
  fn inline_alloc() -> objrs::Alloc<NSString> {}

  #[objrs(selector = "initWithBytes:length:encoding:", instance)]
  #[inline(always)]
  unsafe fn inline_init_with_bytes_length_encoding(
    this: objrs::Alloc<NSString>,
    bytes: *const libc::c_char,
    len: usize,
    encoding: NSStringEncoding,
  ) -> objrs::Strong<NSString> {
  }

  #[inline(never)]
  pub fn from_str(string: &str) -> objrs::Strong<NSString> {
    let this = Self::inline_alloc();
    unsafe {
      return Self::inline_init_with_bytes_length_encoding(
        this,
        string.as_ptr() as *const libc::c_char,
        string.len(),
        NSStringEncoding::UTF8,
      );
    }
  }

  #[objrs(selector = "string")]
  pub fn string() -> objrs::Strong<NSString> {}

  #[objrs(selector = "stringWithString:")]
  pub fn string_with_string(string: &NSString) -> objrs::Strong<NSString> {}

  #[objrs(selector = "stringWithUTF8String:")]
  pub fn string_with_utf8_string(cstr: &objrs::CStr) -> objrs::Strong<Self> {}

  #[objrs(selector = "length")]
  pub fn length(&self) -> usize {}

  // Raises an Objective-C exception if out of bounds.
  #[objrs(selector = "characterAtIndex:")]
  pub fn character_at_index(&self, index: usize) -> u16 {}

  #[objrs(selector = "hasPrefix:")]
  pub fn has_prefix(&self, prefix: &NSString) -> bool {}

  #[objrs(selector = "hasSuffix:")]
  pub fn has_suffix(&self, suffix: &NSString) -> bool {}

  #[objrs(selector = "isEqualToString:")]
  pub fn is_equal_to_string(&self, string: &NSString) -> bool {}

  #[objrs(selector = "stringByAppendingString:")]
  pub fn string_by_appending_string(&self, string: &NSString) -> objrs::Strong<NSString> {}

  // TODO: this is an example of taking a Rust type and mapping it to Objective-C.
  #[inline(always)]
  pub fn get_file_system_representation(&self, bytes: &mut [u8]) -> Result<(), ()> {
    let success = unsafe {
      self.get_file_system_representation_max_length(
        bytes as *mut _ as *mut libc::c_char,
        bytes.len(),
      )
    };
    if success {
      return Ok(());
    }
    return Err(());
  }

  #[objrs(selector = "getFileSystemRepresentation:maxLength:")]
  pub unsafe fn get_file_system_representation_max_length(
    &self,
    bytes: *mut libc::c_char,
    max_length: usize,
  ) -> bool {
  }

  #[objrs(selector = "UTF8String")]
  pub fn utf8_string(&self) -> Option<&objrs::CStr> {}

  #[objrs(selector = "lengthOfBytesUsingEncoding:")]
  pub fn length_of_bytes_using_encoding(&self, encoding: NSStringEncoding) -> usize {}
}

impl AsRef<str> for NSString {
  #[inline(always)]
  fn as_ref(&self) -> &str {
    let len = self.length_of_bytes_using_encoding(NSUTF8StringEncoding);
    let utf8 = self.utf8_string().expect("NSString has no UTF-8 representation");
    unsafe {
      let slice = core::slice::from_raw_parts(utf8.as_ptr() as *const u8, len);
      return core::str::from_utf8_unchecked(slice);
    };
  }
}

// TODO: provide From<String> and From<&str> (and maybe std::ffi::CString/CStr).
// impl From<String> for NSString {
//   fn from(string: String) -> NSString {
//     cfstring = CFStringCreateWithBytesNoCopy(core::ptr::null_mut(), string.as_bytes(), string.len(), kCFStringEncodingUTF8, false,
//   }
// }

// TODO: replace the nsstring! proc macro with this, which makes it agnostic to the crate name.
// #[macro_export]
// macro_rules! nsstring {
//   ($sel:literal $(,)?) => {{
//     use $crate::__objrs::nnstring_literal;

//     #[link_section = #bytes_link_section]
//     #[export_name = #bytes_export_name]
//     static BYTES: [$crate::__objrs::#char_type; #array_length] = #chars;

//     #[export_name = #string_export_name]
//     static STRING: $crate::__objrs::CFConstantString = $crate::__objrs::CFConstantString {
//         isa:    unsafe { &$crate::__objrs::CFConstantStringClassReference },
//         info:   0u32,
//         ptr:    &0u8,
//         length: 0usize,
//     };

//     unsafe { $crate::__objrs::TransmuteHack::<_, &'static $crate::NSString> { from: &STRING }.to }
//   }};
// }

// See https://github.com/opensource-apple/CF/blob/master/CFString.c

#[doc(hidden)]
pub mod __objrs {
  use super::*;

  #[link(name = "CoreFoundation", kind = "framework")]
  extern "C" {
    pub type CFConstantStringClassReference;

    #[link_name = "__CFConstantStringClassReference"]
    pub static CFConstantStringClassReference: CFConstantStringClassReference;
  }

  #[repr(C)]
  pub struct CFConstantString {
    pub isa: &'static CFConstantStringClassReference,
    pub info: u32,
    pub ptr: &'static u8,
    pub length: usize,
  }
  unsafe impl core::marker::Send for CFConstantString {}
  unsafe impl core::marker::Sync for CFConstantString {}
}

// TODO: add a macro for formatting a string (e.g. nsstring_format!("{}, {}! {}", nsstring!("Hello"), "world", 42))
// TODO: I don't think this merges duplicate strings (that is, nsstring!("hi"); nsstring!("hi"); will create two separate literals).
pub use objrs_frameworks_foundation_macros::nsstring;

// TODO: should NSString be sync?
unsafe impl core::marker::Sync for NSString {}
