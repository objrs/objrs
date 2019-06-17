// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

extern crate core;
extern crate libc;

use crate::runtime;

struct EncodingIter<'a>(&'a u8);

#[derive(Eq, PartialEq, Debug)]
struct Encoding {
  min: usize,
  max: usize,
  primitive_type: u8,
}

impl Encoding {
  fn full_range(primitive_type: u8) -> Encoding {
    return Encoding {
      min: 0,
      max: usize::max_value(),
      primitive_type: primitive_type,
    };
  }

  fn ranged(min: usize, max: usize, primitive_type: u8) -> Encoding {
    return Encoding {
      min: min,
      max: max,
      primitive_type: primitive_type,
    };
  }

  fn exact(value: usize, primitive_type: u8) -> Encoding {
    return Self::ranged(value, value, primitive_type);
  }
}

impl<'a> EncodingIter<'a> {
  fn new(bytes: &'a u8) -> EncodingIter<'a> {
    return EncodingIter(bytes);
  }

  fn step_byte(&mut self) {
    if self.peek_byte() == b'\0' {
      return;
    }
    self.0 = unsafe { &*(self.0 as *const u8).offset(1) };
  }

  fn take_byte(&mut self) -> u8 {
    let byte = self.peek_byte();
    self.step_byte();
    return byte;
  }

  fn skip_type_name(&mut self) {
    loop {
      match self.peek_byte() {
        b'\0' | b']' | b'}' | b')' => return,
        x => {
          self.step_byte();
          if x == b'=' {
            return;
          }
        }
      }
    }
  }

  fn peek_byte(&self) -> u8 {
    return *self.0;
  }

  fn parse_usize(&mut self) -> usize {
    let mut value = 0;
    loop {
      match self.peek_byte() {
        byte @ b'0'..=b'9' => {
          value *= 10;
          value += (byte - b'0') as usize;
          self.step_byte();
        }
        _ => return value,
      }
    }
  }
}

impl<'a> core::iter::Iterator for EncodingIter<'a> {
  type Item = Encoding;

  fn next(&mut self) -> Option<Self::Item> {
    let byte = self.take_byte();
    match byte {
      b'\0' => return None,

      // is 'R' (byref) really discardable?
      b'r' | b'n' | b'N' | b'o' | b'O' | b'R' | b'V' => {
        return self.next();
      }

      b'b' => return Some(Encoding::exact((self.parse_usize() + 7) / 8, byte)),

      b'^' => {
        self.next(); // Discard the pointee.
        return Some(Encoding::exact(core::mem::size_of::<*const u8>(), byte));
      }

      b'[' => {
        let count = self.parse_usize();
        let next = self.next();
        if next.is_some() {
          self.step_byte();
        } // Skip ']'.
        let next = next.unwrap_or(Encoding::full_range(b'?'));
        return Some(Encoding::ranged(
          next.min.saturating_mul(count),
          next.max.saturating_mul(count.max(1)),
          byte,
        ));
      }

      b'{' => {
        self.skip_type_name();
        let mut sum = Encoding::full_range(byte);
        for next in self {
          sum.min += next.min;
        }
        return Some(sum);
      }

      b'(' => {
        self.skip_type_name();
        let mut max = Encoding::full_range(byte);
        for next in self {
          max.min = core::cmp::max(max.min, next.min);
        }
        return Some(max);
      }

      // Note that, according to [1], "`l` is treated as a 32-bit quantity on 64-bit programs."
      // Using c_int instead of c_long for 'l' | 'L' is intentional.
      // [1]: https://developer.apple.com/library/content/documentation/Cocoa/Conceptual/ObjCRuntimeGuide/Articles/ocrtTypeEncodings.html
      b'v' => return Some(Encoding::exact(0, b'v')),
      b'c' | b'C' => return Some(Encoding::exact(core::mem::size_of::<libc::c_char>(), byte)),
      b'i' | b'I' => return Some(Encoding::exact(core::mem::size_of::<libc::c_int>(), byte)),
      b's' | b'S' => return Some(Encoding::exact(core::mem::size_of::<libc::c_short>(), byte)),
      b'l' | b'L' => return Some(Encoding::exact(core::mem::size_of::<libc::c_int>(), byte)),
      b'q' | b'Q' => return Some(Encoding::exact(core::mem::size_of::<libc::c_longlong>(), byte)),
      b'f' => return Some(Encoding::exact(core::mem::size_of::<libc::c_float>(), byte)),
      b'd' => return Some(Encoding::exact(core::mem::size_of::<libc::c_double>(), byte)),
      b'B' => return Some(Encoding::exact(core::mem::size_of::<bool>(), byte)),
      b'*' => return Some(Encoding::exact(core::mem::size_of::<*const libc::c_char>(), byte)),
      b'@' => return Some(Encoding::exact(core::mem::size_of::<*const runtime::Id>(), byte)),
      b'#' => return Some(Encoding::exact(core::mem::size_of::<*const runtime::Class>(), byte)),
      b':' => return Some(Encoding::exact(core::mem::size_of::<*const runtime::Sel>(), byte)),
      b'?' => return Some(Encoding::full_range(byte)),

      b']' | b'}' | b')' => return None,

      _ => return None,
    }
  }
}

pub unsafe fn sane_return_type<T: core::any::Any>(
  method: core::ptr::NonNull<runtime::Method>,
) -> bool {
  let encoding = runtime::method_copyReturnType(method).as_mut().as_ptr();
  let sane = type_approximately_matches_encoding::<T>(&*(encoding as *const _ as *const u8));
  libc::free(encoding as *mut libc::c_void);
  return sane;
}

pub unsafe fn sane_argument_type<T: core::any::Any>(
  method: core::ptr::NonNull<runtime::Method>,
  index: libc::c_uint,
) -> bool {
  let encoding = runtime::method_copyArgumentType(method, index).as_mut().as_ptr();
  let sane = type_approximately_matches_encoding::<T>(&*(encoding as *const _ as *const u8));
  libc::free(encoding as *mut libc::c_void);
  return sane;
}

fn type_approximately_matches_encoding<T: core::any::Any>(encoding: &u8) -> bool {
  let size_of_t = core::mem::size_of::<T>();
  let decoded = EncodingIter::new(encoding).next().unwrap_or(Encoding::full_range(b'?'));
  if size_of_t < decoded.min || size_of_t > decoded.max {
    return false;
  }

  macro_rules! t_is_one_of {
    ($($types:ty,)*) => {
      ($(core::any::TypeId::of::<T>() == core::any::TypeId::of::<$types>() ||)* false)
    };
  }

  macro_rules! allow {
    ($($types:ty,)*) => {
      {!t_is_one_of!((), bool, char, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64,) || t_is_one_of!($($types,)*)}
    };
    ($head:ty $(, $tail:ty)*) => {
      allow!($head $(, $tail)*,)
    };
  }

  // TODO: once trait specialization becomes a thing, this can probably be expanded to do some better type matching. It won't ever be perfect, but it could improve.
  // TODO: this currently allows some mild type punning (e.g. using u8 instead of i8; using usize instead of a pointer; etc.). Is this really desirable?
  match decoded.primitive_type {
    b'^' => return allow!(usize, isize),
    b'v' => return allow!(()),
    b'c' | b'C' => return allow!(bool, u8, i8),
    b'i' | b'I' => return allow!(char, u32, i32),
    b's' | b'S' => return allow!(u16, i16),
    b'l' | b'L' => return allow!(char, u32, i32),
    b'q' | b'Q' => return allow!(u64, i64),
    b'f' => return allow!(f32),
    b'd' => return allow!(f64),
    b'B' => return allow!(bool, u8, i8),
    b'*' => return allow!(usize, isize),
    b'@' => return allow!(usize, isize),
    b'#' => return allow!(usize, isize),
    b':' => return allow!(usize, isize),
    _ => return true,
  }
}

#[cfg(test)]
#[test]
fn test_type_approximately_matches_encoding() {
  struct CStr<'a>(&'a [u8]);
  impl<'a> core::fmt::Display for CStr<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
      use core::fmt::Write;
      for byte in &self.0[..self.0.len() - 1] {
        f.write_char(*byte as char)?;
      }
      return Ok(());
    }
  }

  macro_rules! expect_match {
    ($encoding:expr $(, $types:ty)*) => {
      $(
        assert!(type_approximately_matches_encoding::<$types>(&$encoding[0]),
                "type `{}` should match encoding `{}`, but does not", stringify!($types), CStr($encoding));
      )*
    };
  }

  macro_rules! expect_no_match {
    ($encoding:expr $(, $types:ty)*) => {
      $(
        assert!(!type_approximately_matches_encoding::<$types>(&$encoding[0]),
                "type `{}` should not match encoding `{}`, but does", stringify!($types), CStr($encoding));
      )*
    };
  }

  expect_match!(b"^v\0", *const u8, usize, isize);
  expect_match!(b"v\0", ());
  expect_match!(b"c\0", bool, u8, i8);
  expect_match!(b"C\0", bool, u8, i8);
  expect_match!(b"i\0", char, u32, i32);
  expect_match!(b"I\0", char, u32, i32);
  expect_match!(b"s\0", u16, i16);
  expect_match!(b"S\0", u16, i16);
  expect_match!(b"l\0", char, u32, i32);
  expect_match!(b"L\0", char, u32, i32);
  expect_match!(b"q\0", u64, i64);
  expect_match!(b"Q\0", u64, i64);
  expect_match!(b"f\0", f32);
  expect_match!(b"d\0", f64);
  expect_match!(b"B\0", bool, u8, i8);
  expect_match!(b"*\0", *const libc::c_char, usize, isize);
  expect_match!(b"@\0", *const runtime::Id, usize, isize);
  expect_match!(b"#\0", *const runtime::Class, usize, isize);
  expect_match!(b":\0", *const runtime::Sel, usize, isize);

  expect_no_match!(
    b"^v\0",
    (),
    bool,
    char,
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    u64,
    i64,
    u128,
    i128,
    f32,
    f64
  );
  expect_no_match!(
    b"v\0", bool, char, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64
  );
  expect_no_match!(
    b"c\0",
    (),
    char,
    u16,
    i16,
    u32,
    i32,
    u64,
    i64,
    u128,
    i128,
    usize,
    isize,
    f32,
    f64
  );
  expect_no_match!(
    b"C\0",
    (),
    char,
    u16,
    i16,
    u32,
    i32,
    u64,
    i64,
    u128,
    i128,
    usize,
    isize,
    f32,
    f64
  );
  expect_no_match!(
    b"i\0",
    (),
    bool,
    u8,
    i8,
    u16,
    i16,
    u64,
    i64,
    u128,
    i128,
    usize,
    isize,
    f32,
    f64
  );
  expect_no_match!(
    b"I\0",
    (),
    bool,
    u8,
    i8,
    u16,
    i16,
    u64,
    i64,
    u128,
    i128,
    usize,
    isize,
    f32,
    f64
  );
  expect_no_match!(
    b"s\0",
    (),
    bool,
    char,
    u8,
    i8,
    u32,
    i32,
    u64,
    i64,
    u128,
    i128,
    usize,
    isize,
    f32,
    f64
  );
  expect_no_match!(
    b"S\0",
    (),
    bool,
    char,
    u8,
    i8,
    u32,
    i32,
    u64,
    i64,
    u128,
    i128,
    usize,
    isize,
    f32,
    f64
  );
  expect_no_match!(
    b"l\0",
    (),
    bool,
    u8,
    i8,
    u16,
    i16,
    u64,
    i64,
    u128,
    i128,
    usize,
    isize,
    f32,
    f64
  );
  expect_no_match!(
    b"L\0",
    (),
    bool,
    u8,
    i8,
    u16,
    i16,
    u64,
    i64,
    u128,
    i128,
    usize,
    isize,
    f32,
    f64
  );
  expect_no_match!(
    b"q\0",
    (),
    bool,
    char,
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    u128,
    i128,
    usize,
    isize,
    f32,
    f64
  );
  expect_no_match!(
    b"Q\0",
    (),
    bool,
    char,
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    u128,
    i128,
    usize,
    isize,
    f32,
    f64
  );
  expect_no_match!(
    b"f\0",
    (),
    bool,
    char,
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    u64,
    i64,
    u128,
    i128,
    usize,
    isize,
    f64
  );
  expect_no_match!(
    b"d\0",
    (),
    bool,
    char,
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    u64,
    i64,
    u128,
    i128,
    usize,
    isize,
    f32
  );
  expect_no_match!(
    b"B\0",
    (),
    char,
    u16,
    i16,
    u32,
    i32,
    u64,
    i64,
    u128,
    i128,
    usize,
    isize,
    f32,
    f64
  );
  expect_no_match!(
    b"*\0",
    (),
    bool,
    char,
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    u64,
    i64,
    u128,
    i128,
    f32,
    f64
  );
  expect_no_match!(
    b"@\0",
    (),
    bool,
    char,
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    u64,
    i64,
    u128,
    i128,
    f32,
    f64
  );
  expect_no_match!(
    b"#\0",
    (),
    bool,
    char,
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    u64,
    i64,
    u128,
    i128,
    f32,
    f64
  );
  expect_no_match!(
    b":\0",
    (),
    bool,
    char,
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    u64,
    i64,
    u128,
    i128,
    f32,
    f64
  );
}

#[cfg(test)]
#[test]
fn test_minimum_size_iter() {
  macro_rules! parse {
    ($encoding:expr) => {
      EncodingIter::new(&$encoding[0]).next()
    };
  }

  macro_rules! assert_exact {
    ($encoding:expr $(, $types:ty)*) => {
      assert_exact!($encoding, $encoding[0] $(, $types)*)
    };
    ($encoding:expr, $expected_size:expr) => {
      assert_exact!($encoding, $encoding[0], $expected_size)
    };
    ($encoding:expr, $expected_primitive_type:expr $(, $types:ty)*) => {
      assert_eq!(parse!($encoding), Some(Encoding::exact(0 $(+ core::mem::size_of::<$types>())*, $expected_primitive_type)))
    };
    ($encoding:expr, $expected_primitive_type:expr, $expected_size:expr) => {
      assert_eq!(parse!($encoding), Some(Encoding::exact($expected_size, $expected_primitive_type)))
    };
  }

  macro_rules! assert_min {
    ($encoding:expr $(, $types:ty)*) => {
      assert_min!($encoding, $encoding[0] $(, $types)*)
    };
    ($encoding:expr, $expected_size:expr) => {
      assert_min!($encoding, $encoding[0], $expected_size)
    };
    ($encoding:expr, $expected_primitive_type:expr $(, $types:ty)*) => {
      assert_eq!(parse!($encoding), Some(Encoding::ranged(0 $(+ core::mem::size_of::<$types>())*, usize::max_value(), $expected_primitive_type)))
    };
    ($encoding:expr, $expected_primitive_type:expr, $expected_size:expr) => {
      assert_eq!(parse!($encoding), Some(Encoding::ranged($expected_size, usize::max_value(), $expected_primitive_type)))
    };
  }

  macro_rules! assert_full_range {
    ($encoding:expr) => {
      assert_full_range!($encoding, $encoding[0])
    };
    ($encoding:expr, $expected_primitive_type:expr) => {
      assert_eq!(parse!($encoding), Some(Encoding::full_range($expected_primitive_type)))
    };
  }

  assert_exact!(b"c\0", libc::c_char);
  assert_exact!(b"i\0", libc::c_int);
  assert_exact!(b"s\0", libc::c_short);
  assert_exact!(b"l\0", libc::c_int);
  assert_exact!(b"q\0", libc::c_longlong);

  assert_exact!(b"C\0", libc::c_uchar);
  assert_exact!(b"I\0", libc::c_uint);
  assert_exact!(b"S\0", libc::c_ushort);
  assert_exact!(b"L\0", libc::c_uint);
  assert_exact!(b"Q\0", libc::c_ulonglong);

  assert_exact!(b"f\0", libc::c_float);
  assert_exact!(b"d\0", libc::c_double);

  assert_exact!(b"B\0", bool);
  assert_exact!(b"v\0", 0);
  assert_exact!(b"*\0", *const libc::c_char);
  assert_exact!(b"@\0", *const runtime::Id);
  assert_exact!(b"#\0", *const runtime::Class);
  assert_exact!(b":\0", *const runtime::Sel);

  assert_exact!(b"b0\0", 0);
  assert_exact!(b"b1\0", 1);
  assert_exact!(b"b2\0", 1);
  assert_exact!(b"b3\0", 1);
  assert_exact!(b"b4\0", 1);
  assert_exact!(b"b5\0", 1);
  assert_exact!(b"b6\0", 1);
  assert_exact!(b"b7\0", 1);
  assert_exact!(b"b8\0", 1);
  assert_exact!(b"b9\0", 2);
  assert_exact!(b"b15\0", 2);
  assert_exact!(b"b16\0", 2);
  assert_exact!(b"b17\0", 3);
  assert_exact!(b"b23\0", 3);
  assert_exact!(b"b24\0", 3);
  assert_exact!(b"b25\0", 4);
  assert_exact!(b"b31\0", 4);
  assert_exact!(b"b32\0", 4);
  assert_exact!(b"b33\0", 5);
  assert_exact!(b"b1024\0", 128);

  assert_full_range!(b"?\0");
  assert_full_range!(b"[]\0");
  assert_full_range!(b"()\0");
  assert_full_range!(b"{}\0");

  // Made up cases that should be sanely handled:
  assert_eq!(parse!(b"\0"), None);
  assert_exact!(b"[1^[]]\0", *const u8);
  assert_exact!(b"rnNoORV^v\0", b'^', *const u8);
  assert_min!(b"[8]\0", 0);

  // Real-world samples:
  assert_exact!(b"r^S\0", b'^', *const libc::c_ushort);
  assert_exact!(b"[12f]\0", [f32; 12]);
  assert_exact!(b"[12^f]\0", [*const f32; 12]);
  assert_exact!(b"^{example=@*i}\0", *const u8);
  assert_exact!(b"^^{example}\0", *const u8);
  assert_exact!(b"r^{opaqueCMFormatDescription=}\0", b'^', *const u8);
  assert_min!(b"{example=@*i}\0", *const runtime::Id, *const libc::c_char, libc::c_int);
  assert_min!(b"{CGRect={CGPoint=dd}{CGSize=dd}}\0", 32);
  assert_min!(b"{CGAffineTransform=dddddd}\0", 48);
  assert_min!(b"{AudioStreamBasicDescription=dIIIIIIII}\0", 40);
  assert_min!(b"{opaqueCMFormatDescription=}\0", 0);
  assert_min!(b"{AudioChannelLayout=III[1{AudioChannelDescription=II[3f]}]}\0", 32);
  assert_min!(b"(_GLKMatrix4={?=ffffffffffffffff}[16f])\0", 64);
  assert_min!(b"(vU1024=[8]{?=}{?=IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII})\0", 128);
}
