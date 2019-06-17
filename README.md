# objrs
Objective-Rust: the unholy union of Rust and Objective-C.

objrs is a library that makes it easy to write Rust code that interacts with Objective-C code. It's primary purpose is to help Rust applications use Apple's native Objective-C frameworks.

**Unless your code is a dumpster fire and you're looking to fuel the flames, do not use objrs in production!** The project is in its infancy and is in an extremely experimental phase. The API is not stable, there are many (known and unknown) bugs, undefined behavior is intentionally invoked, etc.

That said, take objrs for a test drive! Play with it. Feedback is very much appreciated, especially if you have any bright ideas on how to better interoperate with Objective-C (e.g. Objective-C protocols map very poorly to Rust and their implementation in objrs is... flawed).

# TL;DR: what is objrs and what does it do?
Writing a Rust program that needs to interact with Objective-C code is a painful experience. Either FFI bridges (written in C) need to be created (for every little detail), or you have to use something like [objc-rs](https://github.com/objc-rs/objc-rs) or [rust-objc](https://github.com/SSheldon/rust-objc) to do all your interaction with the Objective-C runtime (at runtime! which adds overhead and reduces compiler safety checks).

objrs is different. It provides a collection of macros that apply a series of (questionable) transformations to your code (at compile time!) so that it compiles down to (approximately) the same machine code as pure Objective-C. objrs transforms your pure Rust code so that it is ABI-compatible with Objective-C. Here's an example:

**Objective-C:**
```objective_c
#import <Foundation/Foundation.h>

int main() {
  NSObject *object = [NSObject new];
  [object hash];
  return 0;
}
```

This Objective-C program can be implemented in Rust using objrs as follows:

**Rust:**
```rust
extern crate objrs;
use objrs::objrs;

#[objrs(class, root_class)]
#[link(name = "Foundation", kind = "framework")]
struct NSObject;

#[objrs(impl)]
#[link(name = "Foundation", kind = "framework")]
impl NSObject {
  // In practice you probably wouldn't want to use #[inline(always)] unless you knew what you were
  // doing. Omitting it just results in an extra extern "C" wrapper fn being used, which introduces
  // an extra jmp indirection.
  #[objrs(selector = "new")]
  #[inline(always)]
  fn new() -> objrs::Strong<NSObject> {}

  #[objrs(selector = "hash")]
  #[inline(always)]
  fn hash(&self) -> usize {}
}

fn main() {
  let object = NSObject::new();
  object.hash();
}
```

Both the Objective-C and Rust code will compile down to the same assembly (modulo some minor costmetic differences, see [HOW_IT_WORKS.md](HOW_IT_WORKS.md) for a more detailed breakdown).

Check out the [`demo`](demo) directory for a full demo application (written in 100% Rust).

# How to use objrs

1. Use a recent nightly version of Rust on macOS.
2. Put `objrs = "0.0.2"` in your `Cargo.toml`'s `[dependencies]` section.
3. Put `use objrs::objrs` in your code to bring the `objrs` macro attribute into scope.
4. Hack away!

objrs also provides framework bindings for Apple's frameworks. The following Apple frameworks have (incomplete) Rust bindings:

- AppKit (crate: [`objrs_frameworks_app_kit`](frameworks/app_kit)).
- CoreGraphics (crate: [`objrs_frameworks_core_graphics`](frameworks/core_graphics)).
- Foundation (crate: [`objrs_frameworks_foundation`](frameworks/foundation)).
- Metal (crate: [`objrs_frameworks_metal`](frameworks/metal)).
- MetalKit (crate: [`objrs_frameworks_metal_kit`](frameworks/metal_kit)).

See [the full documentation](DOCUMENTATION.md) for more details, including an actual grammar for the attribute macros. Documentation here is intended to be simplistic and introductory. 

## Examples

### Using existing framework bindings

If you don't plan to implement your own Objective-C bindings or classes, there's no need to depend on the core `objrs` crate.

```rust
extern crate objrs_frameworks_foundation;
use objrs_frameworks_foundation::NSObject;

fn main() {
  // Create a new NSObject instance that is released when it goes out of scope.
  let object = NSObject::new();
}
```

### Create your own classes

You can create your own class types. Instance variables are supported, but they can only be accessed directly through `self` (e.g. `self.ivar` will work, `some_other_ident.ivar` will not work).

```rust
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
```

# To-do list

- [ ] Completely rewrite the project. This first version is an extremely hacky prototype.
- [ ] Figure out a better ownership and borrowing model. I thought Rust's references would be a nice way to clarify ownership and mutability in an Objective-C API, but it turns out that there is just way too much mutable aliasing in Objective-C.
- [ ] Figure out how to support protocols properly. The current implementation is both super sketchy and annoying to use.
- [ ] Properly support autoreleased and weak values. I've kinda just ignored them.
- [ ] Automatically inject unit tests into code that uses objrs's macros. The unit tests would compare class names, inheritance structure, selectors, method arguments and return values, protocol conformance, etc. This could all be done automatically today.
- [ ] Blocks. They're completely unsupported at this point.

# Requirements

- Nightly Rust.
- macOS. iOS and tvOS should work (fingers crossed), but they haven't been tested. watchOS support depends on armv7 support.
- A recent OS version. Only macOS 10.14.1 has been tested. Minimum requirements will be established later.
- An x86_64 or arm64 (aarch64) computer. armv7 support could be added, but I don't have an armv7 Apple device. i386 (32-bit x86) support could be added, but there are no plans to do so given its obsolescence.

# License

See the [COPYRIGHT](COPYRIGHT) file. objrs is triple-licensed under the Apache License 2.0, MIT License, and Mozilla Public License 2.0 terms.
