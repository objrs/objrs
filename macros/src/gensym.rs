// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

extern crate core;

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
pub fn random_identifier() -> [u8; 32] {
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

// #[proc_macro]
// pub fn gensym(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//   const HELP_MESSAGE: &'static str = "gensym! requires exactly zero or one identifier. Examples: gensym!(), gensym!(foo); gensym!(bar); gensym!(this_is_an_identifier);";
//   let mut iter = input.into_iter();
//   let token_tree = iter
//     .next()
//     .unwrap_or_else(|| token_tree_from_term("").into());
//   if iter.next().is_some() {
//     panic!(HELP_MESSAGE);
//   }

//   let term = match token_tree.kind {
//     proc_macro::TokenTerm::Term(ref t) => t.as_str(),
//     _ => panic!(HELP_MESSAGE),
//   };

//   let random_id = &random_identifier();
//   let random_id_str = unsafe { core::str::from_utf8_unchecked(random_id) };

//   return token_tree_from_term(&[term, random_id_str].concat()).into();
// }
