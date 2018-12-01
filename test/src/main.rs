// // The contents of this file is licensed by its authors and copyright holders under the Apache
// // License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// // contents of this file may not be copied, modified, or distributed except according to those
// // terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// // licenses and more information.

// #![feature(
//   lang_items,
//   start,
//   libc,
//   rust_2018_preview,
//   // proc_macro_non_items,  // Required for nsstring!.
// )]
// #![no_std]

// // fn decompress_lzma<'a>(src: &[u8], dst: &'a mut [u8]) -> &'a mut [u8] {
// //   const COMPRESSION_LZMA: i32 = 0x306;
// //   #[link(name = "compression")]
// //   extern "C" {
// //     fn compression_decode_buffer(
// //       dst_buffer: *mut u8,
// //       dst_size: usize,
// //       src_buffer: *const u8,
// //       src_size: usize,
// //       scratch_buffer: *mut u8,
// //       algorithm: i32,
// //     ) -> usize;
// //   }
// //   let len;
// //   unsafe {
// //     len = compression_decode_buffer(
// //       dst.as_mut_ptr(),
// //       dst.len(),
// //       src.as_ptr(),
// //       src.len(),
// //       core::ptr::null_mut(),
// //       COMPRESSION_LZMA,
// //     );
// //   }
// //   return dst.split_at_mut(len).0;
// // }

// // Logo taken from: https://www.rust-lang.org/logos/rust-logo-blk.svg
// // Cleaned up in Inkscape to make it a single path object. Triangulated with:
// // https://github.com/mattdesl/svg-mesh-3d
// // static LOGO: [[[i16; 2]; 3]; 16141] = {
// //   union RawBytes {
// //     u8: [u8; 193692],
// //     i16: [[[i16; 2]; 3]; 16141],
// //   }

// //   const RAW_BYTES: RawBytes = RawBytes {
// //       u8: *include_bytes!("logo_triangles.i16"),
// //   };

// //   unsafe { RAW_BYTES.i16 }
// // };
// // static LOGO: ([[u16; 3]; 16141], [[i16; 2]; 15613]) = {
// //   #[repr(C)]
// //   struct Logo {
// //     // Each [u16; 3] element is a triangle, each u16 is an index into point_list.
// //     index_list: [[u16; 3]; 16141],
// //     // Each [i16; 2] is a 2D (x, y) point.
// //     point_list: [[i16; 2]; 15613],
// //   }
// //   union RawBytes {
// //     u8: [u8; 159298],
// //     logo: Logo,
// //   }

// //   const RAW_BYTES: RawBytes = RawBytes {
// //       u8: *include_bytes!("logo_index_points.i16"),
// //   };

// //   unsafe { (RAW_BYTES.logo.index_list, RAW_BYTES.logo.point_list) }
// // };

// // TODO: Test with the following:
// // #![no_implicit_prelude]
// // struct BogusU8;
// // struct BogusU16;
// // struct BogusU32;
// // struct BogusU64;
// // struct BogusU128;

// // struct BogusI8;
// // struct BogusI16;
// // struct BogusI32;
// // struct BogusI64;
// // struct BogusI128;

// // struct BogusBool;

// // struct BogusChar;

// // struct BogusUsize;
// // struct BogusIsize;

// // struct BogusF32;
// // struct BogusF64;

// // struct BogusStr;

// // type u8 = BogusU8;
// // type u16 = BogusU16;
// // type u32 = BogusU32;
// // type u64 = BogusU64;
// // type u128 = BogusU128;

// // type i8 = BogusI8;
// // type i16 = BogusI16;
// // type i32 = BogusI32;
// // type i64 = BogusI64;
// // type i128 = BogusI128;

// // type bool = BogusBool;

// // type char = BogusChar;

// // type usize = BogusUsize;
// // type isize = BogusIsize;

// // type f32 = BogusF32;
// // type f64 = BogusF64;

// // type str = BogusStr;

// // struct BogusOption<T>(core::marker::PhantomData<T>);
// // type Option<T> = BogusOption<T>;
// // fn Some() {}
// // fn None() {}

// // struct BogusResult<T, E>(core::marker::PhantomData<T>, core::marker::PhantomData<E>);
// // type Result<T, E> = BogusResult<T, E>;
// // fn Ok() {}
// // fn Err() {}

// // Copy, Send, Sized, Sync
// // Drop, Fn, FnMut, FnOnce
// // Box
// // ToOwned
// // Clone
// // PartialEq, PartialOrd, Eq, Ord
// // AsRef, AsMut, Into, From
// // Default
// // Iterator, Extend, IntoIterator, DoubleEndedIterator, ExactSizeIterator
// // SliceConcatExt
// // String, ToString
// // Vec

// // fn drop() {}

// // extern crate core;
// extern crate libc;
// extern crate objrs;

// use objrs::objrs;

// #[lang = "eh_personality"]
// extern "C" fn eh_personality() {}
// #[lang = "panic_impl"]
// fn panic_impl(_: &core::panic::PanicInfo) -> ! {
//   loop {}
// }

// #[objrs(class, root_class)]
// #[link(name = "Foundation", kind = "framework")]
// #[repr(C)]
// pub struct NSObject {
//   #[deprecated]
//   isa: objrs::runtime::Class,
// }

// // #[objrs(impl)]
// // #[link(name = "Foundation", kind = "framework")]
// // impl NSObject {
// //   // #[objrs(selector = "alloc")]
// //   // pub fn alloc() -> objrs::Alloc<NSObject> {}

// //   // #[objrs(selector = "init")]
// //   // pub fn init(self: objrs::Alloc<NSObject>) -> objrs::Strong<NSObject> {}

// //   // #[objrs(selector = "init", instance)]
// //   // pub fn init(this: objrs::Alloc<NSObject>) -> objrs::Strong<NSObject> {}

// //   // #[objrs(selector = "new")]
// //   // pub fn new() -> objrs::Strong<NSObject> {}

// //   // #[objrs(selector = "hash")]
// //   // #[inline(always)]
// //   // pub fn hash(&self) -> usize {}
// // }

// #[objrs(class, super = NSObject)]
// pub struct CustomClass;

// #[objrs(impl)]
// impl CustomClass {
//   #[objrs(selector = "bogus:bar:")]
//   pub fn bogus(_: usize, _: usize) {}

//   // #[objrs(selector = "alloc")]
//   // pub fn alloc() -> objrs::Alloc<CustomClass> {
//   //   return unsafe { core::mem::transmute(Self::super_alloc()) };
//   // }

//   // #[objrs(selector = "new", no_impl)]
//   // #[inline(always)]
//   // pub fn new() -> *mut CustomClass {}

//   // #[objrs(selector = "dealloc")]
//   // #[inline(always)]
//   // pub fn dealloc(&self) {
//   //   unsafe { libc::puts(b"Goodbye, world!\0" as *const _ as *const _) };
//   // }

//   // #[objrs(selector = "hash")]
//   // #[inline(always)]
//   // pub fn hash(&self) -> usize {
//   //   unsafe { libc::puts(b"Hello, world!\0" as *const _ as *const _) };
//   //   // return super(self);
//   //   return 0;
//   // }

//   // #[objrs(selector = "doStuff")]
//   // #[inline(always)]
//   // pub fn do_stuff(&mut self) {
//   //   self.print_it();
//   //   self.0 = 42;
//   //   self.print_it();
//   // }

//   // #[objrs(selector = "print_it")]
//   // #[inline(always)]
//   // pub fn print_it(&self) {
//   //   unsafe { libc::printf(b"%u\n\x00" as *const _ as *const _, self.0) };
//   // }
// }

// // #[objrs(class, super = NSObject)]
// // #[link(name = "Foundation", kind = "framework")]
// // pub struct NSArray<T: ?Sized + objrs::marker::Class>;

// // #[objrs(impl)]
// // #[link(name = "Foundation", kind = "framework")]
// // impl<T: ?Sized + objrs::marker::Class> NSArray<T> {
// //   #[objrs(selector = "array")]
// //   pub fn array() -> objrs::Strong<NSArray<T>> {}
// // }

// // #[objrs(class, super = NSArray<T>)]
// // #[link(name = "Foundation", kind = "framework")]
// // pub struct NSMutableArray<T: ?Sized + objrs::marker::Class>;

// // #[objrs(impl)]
// // #[link(name = "Foundation", kind = "framework")]
// // impl<T: ?Sized + objrs::marker::Class> NSMutableArray<T> {
// //   #[objrs(selector = "array")]
// //   pub fn array() -> objrs::Strong<NSMutableArray<T>> {}

// //   #[objrs(selector = "addObject:")]
// //   pub fn add_object(&mut self, object: objrs::Strong<T>) {}
// // }

// #[repr(C)]
// pub struct NSZone;

// // #[objrs(protocol)]
// // #[link(name = "Foundation", kind = "framework")]
// // pub trait NSCopying : objrs::marker::Class {
// //   #[objrs(selector = "copyWithZone:")]
// //   fn copy_with_zone(&mut self, zone: *mut NSZone) -> objrs::Strong<Self>;
// // }

// // #[objrs(class, super = NSObject)]
// // #[link(name = "Foundation", kind = "framework")]
// // #[repr(C)]
// // pub struct NSNotification {}

// // #[objrs(impl)]
// // #[link(name = "Foundation", kind = "framework")]
// // impl NSCopying for NSNotification {
// //   #[objrs(selector = "copyWithZone:")]
// //   fn copy_with_zone(&mut self, zone: *mut NSZone) -> objrs::Strong<Self> {}
// // }

// // #[objrs(protocol)]
// // #[link(name = "AppKit", kind = "framework")]
// // pub trait NSApplicationDelegate {
// //   #[objrs(selector = "applicationDidFinishLaunching:")]
// //   fn application_did_finish_launching(&mut self, notification: &NSNotification);
// // }

// // #[objrs(impl)]
// // impl NSApplicationDelegate for CustomClass {
// //   #[objrs(selector = "applicationDidFinishLaunching:")]
// //   fn application_did_finish_launching(&mut self, _notification: &NSNotification) {
// //   }
// // }

// // #[objrs(protocol)]
// // #[link(name = "Foundation", kind = "framework")]
// // pub trait NSCopying : objrs::marker::Class {
// //   #[objrs(selector = "copyWithZone:")]
// //   fn copy_with_zone(&mut self, zone: *mut NSZone) -> *mut Self;
// // }

// // #[objrs(impl)]
// // impl NSCopying for CustomClass {
// //   #[objrs(selector = "copyWithZone:")]
// //   fn copy_with_zone(&mut self, _zone: *mut NSZone) -> *mut CustomClass {
// //     return CustomClass::new();
// //   }
// // }

// #[start]
// fn main(_argc: isize, _argv: *const *const u8) -> isize {
//   // fn main() {
//   // let object = CustomClass::new();
//   // unsafe { &*object }.hash();
//   // unsafe { &mut *object }.do_stuff();
//   // unsafe { objrs::runtime::objc_release(objrs::runtime::id(object as *mut _)) };

//   // unsafe { &mut *object }.copy_with_zone(core::ptr::null_mut());

//   return 0;
// }

#![feature(rust_2018_preview, proc_macro_hygiene)]

// extern crate objrs;
// use objrs::objrs;

extern crate objrs;
extern crate objrs_frameworks_foundation;

use objrs::objrs;
use objrs_frameworks_foundation::NSObject;

// objrs supports instance variables, even if they have non-trivial constructors and destructors.
#[objrs(class, super = NSObject)]
pub struct StringCollection {
  strings: Vec<String>,
}

#[objrs(impl)]
impl StringCollection {
  // Using `no_impl` will tell objrs to just let the super class handle the message. Don't populate
  // the method body; objrs will do that automatically.
  #[objrs(selector = "new", no_impl)]
  pub fn new() -> objrs::Strong<StringCollection> {}

  // You can still use normal Rust methods. These methods are callable from Rust but not
  // Objective-C.
  pub fn add_string(&mut self, string: String) {
    self.strings.push(string);
  }

  // This method is callable from Objective-C because it has been given a selector.
  #[objrs(selector = "printStrings")]
  pub fn print_strings(&self) {
    for string in self.strings.iter() {
      println!("String: {}", string);
    }
  }
}

fn main() {
  let mut string_collection = StringCollection::new();
  string_collection.add_string(String::from("Hello, world!"));
  string_collection.print_strings();
}
