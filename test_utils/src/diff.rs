// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

extern crate difference;

use difference::{Changeset, Difference};
use std::fmt::Write;

pub struct Diff<'a> {
  labels: [&'a str; 2],
  changeset: Changeset,
}

impl<'a> Diff<'a> {
  pub fn new(left_label: &'a str, left: &'a str, right_label: &'a str, right: &'a str) -> Diff<'a> {
    return Diff {
      labels: [left_label, right_label],
      changeset: Changeset::new(left, right, "\n"),
    };
  }
}

fn write_diff(
  prefix: char,
  diff: &str,
  fmt: &mut core::fmt::Formatter,
) -> Result<(), core::fmt::Error> {
  let mut previous_index = 0;
  for (index, _) in diff.match_indices('\n') {
    let line = &diff[previous_index..(index + 1)];
    previous_index = index + 1;
    fmt.write_char(prefix)?;
    fmt.write_str(line)?;
  }
  if previous_index != diff.len() {
    let line = &diff[previous_index..diff.len()];
    fmt.write_char(prefix)?;
    fmt.write_str(line)?;
  }
  return Ok(());
}

impl<'a> core::fmt::Display for Diff<'a> {
  fn fmt(&self, fmt: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
    fmt.write_str("--- ")?;
    fmt.write_str(self.labels[0])?;
    fmt.write_str("\n+++ ")?;
    fmt.write_str(self.labels[1])?;

    for diff in self.changeset.diffs.iter() {
      fmt.write_char('\n')?;
      match diff {
        Difference::Same(string) => {
          write_diff(' ', &string, fmt)?;
        }
        Difference::Add(string) => {
          write_diff('+', &string, fmt)?;
        }
        Difference::Rem(string) => {
          write_diff('-', &string, fmt)?;
        }
      }
    }
    return Ok(());
  }
}
