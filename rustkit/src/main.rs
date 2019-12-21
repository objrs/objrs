use std::env;

fn main() -> (){
	if cfg!(feature = "objrs") {

		let target_os = String::from("macos");
		//let target_os = env::var("TARGET").unwrap();

		let mut out_dir = env::current_dir().unwrap().to_str().unwrap().to_string();

		if cfg!(feature = "objrs") {
			out_dir.push_str("/../macos/src/");
		}

		rustkit_bindgen::bind_all(&target_os, &out_dir);
	}
}