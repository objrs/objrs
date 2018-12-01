# How objrs works

Objective-C compiles down to assembly. Rust compiles down do assembly. Is it possible to get Rust code to compile down to the same assembly as Objective-C code? Yes! And that's what objrs does, by using a collection of (hacky) macros.

Consider the following simple Objective-C program:
```objective_c
#import <Foundation/Foundation.h>
#include <stdio.h>

int main() {
  puts(@"Hello, world!".UTF8String);
  return 0;
}
```

This compiles down to the following x86_64 assembly:

```
  .section  __TEXT,__text,regular,pure_instructions
  .macosx_version_min 10, 13
  .globl    _main
  .p2align  4, 0x90
_main:                                  ## @main
  .cfi_startproc
## BB#0:
  pushq  %rax
Lcfi0:
  .cfi_def_cfa_offset 16
  leaq   L__unnamed_cfstring_(%rip), %rdi
  callq  _objc_retainAutorelease
  movq   L_OBJC_SELECTOR_REFERENCES_(%rip), %rsi
  movq   %rax, %rdi
  callq  *_objc_msgSend@GOTPCREL(%rip)
  movq   %rax, %rdi
  callq  _puts
  xorl   %eax, %eax
  popq   %rcx
  retq
  .cfi_endproc

  .section  __TEXT,__cstring,cstring_literals
L_.str:                                 ## @.str
  .asciz  "Hello, world!"

  .section  __DATA,__cfstring
  .p2align  3               ## @_unnamed_cfstring_
L__unnamed_cfstring_:
  .quad   ___CFConstantStringClassReference
  .long   1992                    ## 0x7c8
  .space  4
  .quad   L_.str
  .quad   13                      ## 0xd

  .section  __TEXT,__objc_methname,cstring_literals
L_OBJC_METH_VAR_NAME_:                  ## @OBJC_METH_VAR_NAME_
  .asciz  "UTF8String"

  .section  __DATA,__objc_selrefs,literal_pointers,no_dead_strip
  .p2align  3               ## @OBJC_SELECTOR_REFERENCES_
L_OBJC_SELECTOR_REFERENCES_:
  .quad  L_OBJC_METH_VAR_NAME_

  .section  __DATA,__objc_imageinfo,regular,no_dead_strip
L_OBJC_IMAGE_INFO:
  .long  0
  .long  64
```

We can generate (approximately) the same assembly using the following Rust code:

```rust
extern crate core;

#[link(name = "c")]
extern {
  fn puts(msg: *const i8) -> i32;
}

#[link(name = "objc")]
extern {
  fn objc_retainAutorelease(obj: id) -> id;
  fn objc_msgSend();
}

#[link(name = "CoreFoundation", kind = "framework")]
extern {
  static __CFConstantStringClassReference: *mut i32;
}

#[repr(C)]
struct SyncWrapper<T>(T);
unsafe impl<T> core::marker::Sync for SyncWrapper<T> {}

enum objc_object {}
type id = *mut objc_object;
type SEL = *const i8;

#[repr(C)]
struct CFString {
  isa: *const *mut i32,
  info: u32,
  ptr: *const i8,
  length: u64,
}

#[link_section = "__TEXT,__cstring,cstring_literals"]
#[export_name = "\x01L_.str"]
static L_str: [u8; 14] = *b"Hello, world!\0";

#[link_section = "__DATA,__cfstring"]
#[export_name = "\x01L__unnamed_cfstring_"]
static L__unnamed_cfstring_: SyncWrapper<CFString> = SyncWrapper(CFString {
  isa: unsafe { &__CFConstantStringClassReference },
  info: 1992,
  ptr: &L_str as *const _ as *const i8,
  length: 13,
});

#[link_section = "__TEXT,__objc_methname,cstring_literals"]
#[export_name = "\x01L_OBJC_METH_VAR_NAME_"]
static L_OBJC_METH_VAR_NAME_: [u8; 11] = *b"UTF8String\0";

#[link_section = "__DATA,__objc_selrefs,literal_pointers,no_dead_strip"]
#[export_name = "\x01L_OBJC_SELECTOR_REFERENCES_"]
static L_OBJC_SELECTOR_REFERENCES_: SyncWrapper<*const i8> =
    SyncWrapper(&L_OBJC_METH_VAR_NAME_ as *const _ as SEL);

#[link_section = "__DATA,__objc_imageinfo,regular,no_dead_strip"]
#[export_name = "\x01L_OBJC_IMAGE_INFO"]
#[used]
static IMAGE_INFO: [u32; 2] = [0, 64];

fn main() {
  unsafe {
    let msg_send: unsafe extern fn(id, SEL) -> *const i8 =
        core::mem::transmute(objc_msgSend as *const ());
    let obj = core::mem::transmute(&L__unnamed_cfstring_);
    let obj = objc_retainAutorelease(obj);
    let sel = core::ptr::read_volatile(&L_OBJC_SELECTOR_REFERENCES_.0 as *const _);
    puts(msg_send(obj, sel));
  }
}
```

With a little bit of `no_std` boilerplate (merely to reduce the amount of assembly code generated), we can get this to compile to almost identical assembly as the original Objective-C code. The diff (with some cosmetic changes applied like deleting comments and reordering some `.setction`s so they match up) between the Objective-C and Rust assembly is:

```
diff --git a/objc.s b/rust.s
index 4a8f091..0212fe3 100644
--- a/objc.s
+++ b/rust.s
@@ -1,23 +1,19 @@
   .section  __TEXT,__text,regular,pure_instructions
-  .macosx_version_min 10, 13
   .globl    _main
   .p2align  4, 0x90
 _main:
-  .cfi_startproc
   pushq  %rax
-Lcfi0:
-  .cfi_def_cfa_offset 16
   leaq   L__unnamed_cfstring_(%rip), %rdi
   callq  _objc_retainAutorelease
   movq   L_OBJC_SELECTOR_REFERENCES_(%rip), %rsi
   movq   %rax, %rdi
-  callq  *_objc_msgSend@GOTPCREL(%rip)
+  callq  _objc_msgSend
   movq   %rax, %rdi
   callq  _puts
   xorl   %eax, %eax
   popq   %rcx
   retq
-  .cfi_endproc

   .section  __TEXT,__cstring,cstring_literals
 L_.str:
@@ -42,9 +38,11 @@ L_OBJC_SELECTOR_REFERENCES_:
   .quad  L_OBJC_METH_VAR_NAME_

   .section  __DATA,__objc_imageinfo,regular,no_dead_strip
+  .p2align  2
 L_OBJC_IMAGE_INFO:
   .long  0
   .long  64

+  .no_dead_strip  L_OBJC_IMAGE_INFO
```

As you can see, the final difference between the Objective-C code and the Rust code (that masquerades as Objective-C code) is trivial.

But writing that raw Rust code is painful. It's tedious, verbose, and very error prone. This is where objrs comes into the picture. Its macros perform the necessary transformations (and mutilations) to your code so that writing Objective-Rust code is a breeze.

# License

See the [COPYRIGHT](COPYRIGHT) file. objrs is triple-licensed under the Apache License 2.0, MIT License, and Mozilla Public License 2.0 terms.
