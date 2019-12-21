// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::env;

fn main (){
	if cfg!(feature = "objrs") {
		return;
	}

	let target_os = env::var("TARGET").unwrap();

	let mut out_dir = env::var("OUT_DIR").unwrap();

	if cfg!(feature = "objrs") {
		out_dir.push_str("/../../../../../../objrs/macos/src/");
	}

	rustkit_bindgen::bind_all(&target_os, &out_dir);
}
