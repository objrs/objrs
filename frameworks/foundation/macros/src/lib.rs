// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

#![feature(proc_macro_diagnostic)]
#![recursion_limit = "128"]

extern crate core;
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro2::Span;
use quote::quote;
use syn::{parse2, punctuated::Punctuated, spanned::Spanned, token::Comma, LitByteStr, LitStr};

// TODO: pull this out into a separate library that can be shared between this crate and /macros.
#[link(name = "c")]
extern "C" {
  fn arc4random_buf(buf: *mut u8, nbytes: usize);
}

#[inline(always)]
fn random<T>() -> T {
  let mut buf = unsafe { core::mem::uninitialized() };
  unsafe {
    arc4random_buf(&mut buf as *mut _ as *mut u8, core::mem::size_of::<T>());
  }
  return buf;
}

#[inline]
fn random_identifier() -> [u8; 32] {
  let uuid = random::<[u64; 2]>();
  let mask = 0x0f0f0f0f0f0f0f0f;
  let aaaaaaaa = 0x6161616161616161;
  let symbol: [u64; 4] = [
    aaaaaaaa + (uuid[0] & mask),
    aaaaaaaa + ((uuid[0] >> 4) & mask),
    aaaaaaaa + (uuid[1] & mask),
    aaaaaaaa + ((uuid[1] >> 4) & mask),
  ];
  return unsafe { core::mem::transmute(symbol) };
}

fn make_literal(mut value: String) -> proc_macro2::TokenStream {
  let use_utf16 = value.bytes().any(|b| b == 0 || !b.is_ascii());
  value.push('\x00');

  let random_id = &random_identifier();
  let random_id = unsafe { core::str::from_utf8_unchecked(random_id) };

  let string_export_name = ["\x01L__unnamed_cfstring_.__objrs_str.", random_id].concat();

  let bytes_link_section;
  let info_flags;
  let char_type;
  let array_length;
  let chars;
  let bytes_export_name;
  if use_utf16 {
    bytes_link_section = "__TEXT,__ustring";
    info_flags = 2000u32; // This value assume little endian.
    char_type = quote!(u16);
    let utf16: Punctuated<u16, Comma> = value.encode_utf16().collect();
    array_length = utf16.len();
    chars = quote!([#utf16]);
    bytes_export_name = ["\x01l_.str.__objrs_str.", random_id].concat();
  } else {
    bytes_link_section = "__TEXT,__cstring,cstring_literals";
    info_flags = 1992u32; // This value assume little endian.
    char_type = quote!(u8);
    let bytes = value.as_bytes();
    array_length = bytes.len();
    let bytes = LitByteStr::new(bytes, Span::call_site()); // TODO: use def_site().
    chars = quote!(*#bytes);
    bytes_export_name = ["\x01L_.str.__objrs_str.", random_id].concat();
  }

  return quote! {{
      extern crate objrs_frameworks_foundation as __objrs_root;

      #[link_section = #bytes_link_section]
      #[export_name = #bytes_export_name]
      #[doc(hidden)]
      static BYTES: [__objrs_root::__objrs::#char_type; #array_length] = #chars;

      #[link_section = "__DATA,__cfstring"]
      #[export_name = #string_export_name]
      #[doc(hidden)]
      static STRING: __objrs_root::__objrs::CFConstantString = __objrs_root::__objrs::CFConstantString{
          isa:    unsafe { &__objrs_root::__objrs::CFConstantStringClassReference },
          info:   #info_flags,
          ptr:    unsafe { __objrs_root::__objrs::TransmuteHack { from: &BYTES }.to },
          length: #array_length - 1,
      };

      unsafe { __objrs_root::__objrs::TransmuteHack::<_, &'static __objrs_root::NSString> { from: &STRING }.to }
  }};
}

#[proc_macro]
pub fn nsstring(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input: proc_macro2::TokenStream = input.into();
  let span = input.span();
  match parse2::<LitStr>(input.into()) {
    Ok(value) => return make_literal(value.value()).into(),
    Err(err) => {
      span
        .unstable()
        .error(err.to_string())
        .note(
          "nsstring! requires a single string literal. Examples: nsstring!(\"Hello, world!\"); \
           nsstring!(\"こんにちは世界！\"); nsstring!(r#\"नमस्ते दुनिया!\"#);",
        )
        .emit();
      // Return an empty NSString. This should help other diagnostic messages.
      return make_literal(String::new()).into();
    }
  }
}
