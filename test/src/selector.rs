// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

//! Tests the `selector!` macro.

extern crate objrs;

use objrs::__objrs::runtime;

#[cfg_attr(test, test)]
fn test_selector() {
  validate_selector(objrs::selector!(r#"HelloWorldThisIsATestForObjectiveRust"#));
  validate_selector(objrs::selector!(r#"HelloWorldThisIsATestForObjectiveRust:::::::"#));
  validate_selector(objrs::selector!(r#"HelloWorld:::::ThisIsATest:::ForObjectiveRust:"#));
  validate_selector(objrs::selector!(r#"HelloWorld:This:Is:A:Test:For:Objective:Rust:"#));
}

pub fn validate_selector(sel: &objrs::Sel) {
  assert!(unsafe { runtime::sel_isMapped(sel) });
}

#[cfg(not(test))]
pub fn run_tests() {
  test_selector();
}
