// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

//! Tests all objrs macros in a hostile environment (`#![no_implicit_prelude]` and all functions,
//! modules, traits, and types replaced with custom bogus user replacements).

#![no_implicit_prelude]
#![allow(dead_code)]

mod core {}
mod std {}
mod libc {}
mod objrs {}

enum BogusU8 {}
#[allow(non_camel_case_types)]
type u8 = BogusU8;
enum BogusU16 {}
#[allow(non_camel_case_types)]
type u16 = BogusU16;
enum BogusU32 {}
#[allow(non_camel_case_types)]
type u32 = BogusU32;
enum BogusU64 {}
#[allow(non_camel_case_types)]
type u64 = BogusU64;
enum BogusU128 {}
#[allow(non_camel_case_types)]
type u128 = BogusU128;

enum BogusI8 {}
#[allow(non_camel_case_types)]
type i8 = BogusI8;
enum BogusI16 {}
#[allow(non_camel_case_types)]
type i16 = BogusI16;
enum BogusI32 {}
#[allow(non_camel_case_types)]
type i32 = BogusI32;
enum BogusI64 {}
#[allow(non_camel_case_types)]
type i64 = BogusI64;
enum BogusI128 {}
#[allow(non_camel_case_types)]
type i128 = BogusI128;

enum BogusBool {}
#[allow(non_camel_case_types)]
type bool = BogusBool;

enum BogusChar {}
#[allow(non_camel_case_types)]
type char = BogusChar;

enum BogusUsize {}
#[allow(non_camel_case_types)]
type usize = BogusUsize;
enum BogusIsize {}
#[allow(non_camel_case_types)]
type isize = BogusIsize;

enum BogusF32 {}
#[allow(non_camel_case_types)]
type f32 = BogusF32;
enum BogusF64 {}
#[allow(non_camel_case_types)]
type f64 = BogusF64;

enum BogusStr {}
#[allow(non_camel_case_types)]
type str = BogusStr;

enum BogusOption {}
type Option = BogusOption;
enum BogusSome {}
type Some = BogusSome;
enum BogusNone {}
type None = BogusNone;

enum BogusResult {}
type Result = BogusResult;
enum BogusOk {}
type Ok = BogusOk;
enum BogusErr {}
type Err = BogusErr;

enum BogusDropFn {}
#[allow(non_camel_case_types)]
type drop = BogusDropFn;

enum BogusCopy {}
type Copy = BogusCopy;
enum BogusSend {}
type Send = BogusSend;
enum BogusSized {}
type Sized = BogusSized;
enum BogusSync {}
type Sync = BogusSync;

enum BogusDrop {}
type Drop = BogusDrop;
enum BogusFn {}
type Fn = BogusFn;
enum BogusFnMut {}
type FnMut = BogusFnMut;
enum BogusFnOnce {}
type FnOnce = BogusFnOnce;

enum BogusBox {}
type Box = BogusBox;

enum BogusToOwned {}
type ToOwned = BogusToOwned;

enum BogusClone {}
type Clone = BogusClone;

enum BogusPartialEq {}
type PartialEq = BogusPartialEq;
enum BogusPartialOrd {}
type PartialOrd = BogusPartialOrd;
enum BogusEq {}
type Eq = BogusEq;
enum BogusOrd {}
type Ord = BogusOrd;

enum BogusAsRef {}
type AsRef = BogusAsRef;
enum BogusAsMut {}
type AsMut = BogusAsMut;
enum BogusInto {}
type Into = BogusInto;
enum BogusFrom {}
type From = BogusFrom;

enum BogusDefault {}
type Default = BogusDefault;

enum BogusIterator {}
type Iterator = BogusIterator;
enum BogusExtend {}
type Extend = BogusExtend;
enum BogusIntoIterator {}
type IntoIterator = BogusIntoIterator;
enum BogusDoubleEndedIterator {}
type DoubleEndedIterator = BogusDoubleEndedIterator;
enum BogusExactSizeIterator {}
type ExactSizeIterator = BogusExactSizeIterator;

enum BogusSliceConcatExt {}
type SliceConcatExt = BogusSliceConcatExt;

enum BogusString {}
type String = BogusString;
enum BogusToString {}
type ToString = BogusToString;

enum BogusVec {}
type Vec = BogusVec;

enum BogusHash {}
type Hash = BogusHash;
enum BogusDebug {}
type Debug = BogusDebug;

extern crate objrs as new_objrs;

// TODO: create some external and custom classes and protocols.

#[cfg_attr(test, test)]
fn test_selector() {
  crate::selector::validate_selector(new_objrs::selector!(r#"THISisAtestSELECTOR::::objrs::"#));
}

#[cfg_attr(test, test)]
fn test_class() {
  crate::class::validate_class(new_objrs::class!(r#"NSObject"#));
}

#[cfg(not(test))]
pub fn run_tests() {
  test_class();
  test_selector();
}
