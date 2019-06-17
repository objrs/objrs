// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

// See https://github.com/opensource-apple/objc4/blob/master/runtime/objc-exception.h

// A Rust panic is not caught by Objective-C @catch (id) handlers. This is good.
// An Objective-C exception is caught by Rust unwind handlers. This sucks.

extern crate core;
extern crate libc;

use crate::arc;
use crate::runtime;

// Instead of using global_asm, we could catch the exception in pure Rust code using
// core::intrinsics::try (see the following for a simple example that works, though is incomplete).
// But it's unlikely that core::intrinsics::try will ever be stabilized. Additionally, this requires
// marking functions with #[unwind], or else the optimizer will remove the exception handler (since
// there's no visible exception being thrown in the Rust code). On top of that, if the Rust code is
// compiled with panic=abort, then we can't catch the exception at all. As much as I'd love to catch
// exceptions with pure Rust code, it's just not practical.
// pub unsafe fn try_catch(fn_to_call: fn(_: *mut u8), fn_arg: *mut u8) -> bool {
//   // See __rust_maybe_catch_panic for an example of how to use core::intrinsics::try.
//   let mut exception: *mut u8 = core::ptr::null_mut();
//   if core::intrinsics::try(fn_to_call, fn_arg, &mut exception as *mut _ as *mut _) != 0 {
//     // There was an exception. We need to call objc_begin_catch and objc_end_catch (and perhaps
//     // other things that I'm ignorant of, like _Unwind_DeleteException).
//     return false;
//   } else {
//     // There was no exception.
//     return true;
//   }
// }

pub fn throw<T>(exception: T) -> !
where
  T: Into<arc::Auto<runtime::Id>>,
{
  unsafe { runtime::objc_exception_throw(exception.into()) };
}

extern "C" {
  fn __objrs_catch_exception(
    payload: *mut libc::c_void,
    function: extern "C" fn(_: *mut libc::c_void),
    exception: *mut libc::c_void, // TODO: change to MaybeUninit once it is FFI-safe.
  ) -> bool;
}

#[inline]
pub fn catch_exception<F: FnOnce() -> R, R>(f: F) -> Result<R, arc::Auto<runtime::Id>> {
  union FunctionAndRet<F, R> {
    f: core::mem::ManuallyDrop<F>,
    ret: core::mem::ManuallyDrop<R>,
  }

  extern "C" fn execute<F: FnOnce() -> R, R>(ptr: *mut libc::c_void) {
    let f_ptr = ptr as *mut core::mem::ManuallyDrop<F>;
    let ret_ptr = ptr as *mut core::mem::ManuallyDrop<R>;

    let f = core::mem::ManuallyDrop::into_inner(unsafe { core::ptr::read(f_ptr) });
    let ret = core::mem::ManuallyDrop::new(f());
    unsafe { core::ptr::write(ret_ptr, ret) };
  }

  let mut function_and_ret = FunctionAndRet::<F, R> {
    f: core::mem::ManuallyDrop::new(f),
  };
  let mut exception = core::mem::MaybeUninit::uninit();
  if unsafe {
    __objrs_catch_exception(
      &mut function_and_ret as *mut _ as *mut libc::c_void,
      execute::<F, R>,
      &mut exception as *mut _ as *mut libc::c_void,
    )
  } {
    return Err(unsafe { exception.assume_init() });
  } else {
    return Ok(core::mem::ManuallyDrop::into_inner(unsafe { function_and_ret.ret }));
  }
}

#[macro_export]
macro_rules! catch_exception {
  ($($tt: tt)*) => {
    $crate::catch_exception(|| { $($tt)* })
  };
}

// TODO: experiment with more optimal code. One possibility would be to not take a third parameter,
// and instead return the exception from the function (or the value 1 if no exception occurred). I
// think this is feasible since objects should always be properly aligned (and 1 is not).
// int __objrs_catch_exception(void *data, void (*fn)(void *), id *exception) {
//   @try {
//     fn(data);
//     return 0;
//   } @catch (id value) {
//     *exception = value;
//     return 1;
//   }
// }

#[cfg(target_arch = "x86_64")]
global_asm! {r#"
  .section  __TEXT,__text,regular,pure_instructions
  .globl  ___objrs_catch_exception
  .p2align  4, 0x90
___objrs_catch_exception:                             ## @__objrs_catch_exception
Lfunc_begin0:
  .cfi_startproc
  .cfi_personality 155, ___objc_personality_v0
  .cfi_lsda 16, Lexception0
## BB#0:
  pushq  %rbp
Lcfi0:
  .cfi_def_cfa_offset 16
  pushq  %rbx
Lcfi1:
  .cfi_def_cfa_offset 24
  pushq  %rax
Lcfi2:
  .cfi_def_cfa_offset 32
Lcfi3:
  .cfi_offset %rbx, -24
Lcfi4:
  .cfi_offset %rbp, -16
  movq  %rdx, %rbx
  xorl  %ebp, %ebp
Ltmp0:
  callq  *%rsi
Ltmp1:
LBB0_2:
  movl  %ebp, %eax
  addq  $8, %rsp
  popq  %rbx
  popq  %rbp
  retq
LBB0_1:
Ltmp2:
  movq  %rax, %rdi
  callq  _objc_begin_catch
  movq  %rax, %rbp
  movq  %rbp, %rdi
  callq  _objc_retainAutorelease
  movq  %rbp, (%rbx)
  callq  _objc_end_catch
  movl  $1, %ebp
  jmp  LBB0_2
Lfunc_end0:
  .cfi_endproc
  .section  __TEXT,__gcc_except_tab
  .p2align  2
GCC_except_table0:
Lexception0:
  .byte  255                     ## @LPStart Encoding = omit
  .byte  155                     ## @TType Encoding = indirect pcrel sdata4
  .asciz  "\242\200\200"          ## @TType base offset
  .byte  3                       ## Call site Encoding = udata4
  .byte  26                      ## Call site table length
Lset0 = Ltmp0-Lfunc_begin0              ## >> Call Site 1 <<
  .long  Lset0
Lset1 = Ltmp1-Ltmp0                     ##   Call between Ltmp0 and Ltmp1
  .long  Lset1
Lset2 = Ltmp2-Lfunc_begin0              ##     jumps to Ltmp2
  .long  Lset2
  .byte  1                       ##   On action: 1
Lset3 = Ltmp1-Lfunc_begin0              ## >> Call Site 2 <<
  .long  Lset3
Lset4 = Lfunc_end0-Ltmp1                ##   Call between Ltmp1 and Lfunc_end0
  .long  Lset4
  .long  0                       ##     has no landing pad
  .byte  0                       ##   On action: cleanup
  .byte  1                       ## >> Action Record 1 <<
                                        ##   Catch TypeInfo 1
  .byte  0                       ##   No further actions
                                        ## >> Catch TypeInfos <<
  .long  _OBJC_EHTYPE_id@GOTPCREL+4 ## TypeInfo 1
  .p2align  2
"#}

#[cfg(target_arch = "aarch64")]
global_asm! {r#"
  .section  __TEXT,__text,regular,pure_instructions
  .ios_version_min 10, 0
  .globl  ___objrs_catch_exception ; -- Begin function __objrs_catch_exception
  .p2align  2
___objrs_catch_exception:               ; @__objrs_catch_exception
Lfunc_begin0:
  .cfi_startproc
  .cfi_personality 155, ___objc_personality_v0
  .cfi_lsda 16, Lexception0
; %bb.0:
  stp  x20, x19, [sp, #-32]!   ; 8-byte Folded Spill
  stp  x29, x30, [sp, #16]     ; 8-byte Folded Spill
  .cfi_def_cfa_offset 32
  .cfi_offset w30, -8
  .cfi_offset w29, -16
  .cfi_offset w19, -24
  .cfi_offset w20, -32
  mov  x19, x2
Ltmp0:
  blr  x1
Ltmp1:
; %bb.1:
  mov  w0, #0
  ldp  x29, x30, [sp, #16]     ; 8-byte Folded Reload
  ldp  x20, x19, [sp], #32     ; 8-byte Folded Reload
  ret
LBB0_2:
Ltmp2:
  bl  _objc_begin_catch
  mov  x20, x0
  bl  _objc_retainAutorelease
  str  x20, [x19]
  bl  _objc_end_catch
  orr  w0, wzr, #0x1
  ldp  x29, x30, [sp, #16]     ; 8-byte Folded Reload
  ldp  x20, x19, [sp], #32     ; 8-byte Folded Reload
  ret
Lfunc_end0:
  .cfi_endproc
  .section  __TEXT,__gcc_except_tab
  .p2align  2
GCC_except_table0:
Lexception0:
  .byte  255                     ; @LPStart Encoding = omit
  .byte  155                     ; @TType Encoding = indirect pcrel sdata4
  .uleb128 Lttbase0-Lttbaseref0
Lttbaseref0:
  .byte  1                       ; Call site Encoding = uleb128
  .uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
  .uleb128 Ltmp0-Lfunc_begin0     ; >> Call Site 1 <<
  .uleb128 Ltmp1-Ltmp0            ;   Call between Ltmp0 and Ltmp1
  .uleb128 Ltmp2-Lfunc_begin0     ;     jumps to Ltmp2
  .byte  1                       ;   On action: 1
  .uleb128 Ltmp1-Lfunc_begin0     ; >> Call Site 2 <<
  .uleb128 Lfunc_end0-Ltmp1       ;   Call between Ltmp1 and Lfunc_end0
  .byte  0                       ;     has no landing pad
  .byte  0                       ;   On action: cleanup
Lcst_end0:
  .byte  1                       ; >> Action Record 1 <<
                                        ;   Catch TypeInfo 1
  .byte  0                       ;   No further actions
  .p2align  2
                                        ; >> Catch TypeInfos <<
Ltmp3:                                  ; TypeInfo 1
  .long  _OBJC_EHTYPE_id@GOT-Ltmp3
Lttbase0:
  .p2align  2
"#}
