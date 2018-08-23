# objrs demo

This demo of objrs is written in 100% Rust. It uses objrs to compile to machine code that is ABI-compatible with Objective-C, allowing it to utilize native Objective-C frameworks (e.g. AppKit, Foundation, Metal, MetalKit, etc.) without any intermediary FFI bridges.

When run, the application will open a window that uses Metal to render the Rust logo. Moving your mouse across this logo will interactively distort it.

## Requirements

- A recent version of macOS (only macOS 10.13.6 has been tested).
- Nightly Rust compiler (tested on `rustc 1.30.0-nightly (b2028828d 2018-08-16)`).

## Instructions

Just `cargo run` to run the demo app. Running the latest version of the demo can be done as follows:

```sh
git clone https://gitlab.com/objrs/objrs.git
cd objrs/demo
cargo run
```

The demo is also [published on crates.io under the name `objrs_demo`](https://crates.io/crates/objrs_demo).

# License

See the [COPYRIGHT](COPYRIGHT) file. objrs (including this demo) is triple-licensed under the Apache License 2.0, MIT License, and Mozilla Public License 2.0 terms.

This demo renders a copy of the Rust logo with interactive distortions applied. The file [`logo_triangles.i16.xz`](logo_triangles.i16.xz) is a copy of the Rust logo (represented as a 2D triangulated mesh in binary form). The Rust logo is [licensed by Mozilla](https://www.rust-lang.org/en-US/legal.html) under the [Creative Commons Attribution license (CC-BY)](https://creativecommons.org/licenses/by/4.0/).
