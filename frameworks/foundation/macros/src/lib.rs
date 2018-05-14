// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

#![feature(proc_macro, proc_macro_non_items)]

extern crate core;
extern crate literalext;
extern crate proc_macro;

// fn stream_with_span(token_stream: proc_macro::TokenStream, span: proc_macro::Span) -> proc_macro::TokenStream {
//     let iter = token_stream.into_iter().map(|mut token_tree| {
//         match token_tree {
//             proc_macro::TokenTree::Group(g) => token_tree = proc_macro::TokenTree::Group(proc_macro::Group::new(g.delimiter(), stream_with_span(g.stream(), span))),
//             _ => token_tree.set_span(span),
//         }

//         return token_tree;
//     });
//     return iter.collect();
// }

// macro_rules! unhygienic_quote {
//     ($($t: tt)*) => {
//         {
//             stream_with_span(proc_macro::quote!($($t)*), proc_macro::Span::call_site())
//         }
//     }
// }

// macro_rules! quote {
//     ($($t: tt)*) => {
//         {
//             stream_with_span(proc_macro::quote!($($t)*), proc_macro::Span::def_site())
//         }
//     }
// }

const HELP_MESSAGE: &'static str = "nsstring! requires a single string literal. Examples: nsstring!(\"Hello, world!\"); nsstring!(\"こんにちは世界！\"); nsstring!(r#\"नमस्ते दुनिया!\"#);";

fn quote_term(term: &str) -> proc_macro::TokenTree {
  return proc_macro::TokenTree::Term(proc_macro::Term::new(term, proc_macro::Span::def_site()));
}

trait FromLiteral<T> {
  fn from_literal(literal: T) -> Self;
}

impl FromLiteral<u8> for proc_macro::TokenTree {
  fn from_literal(literal: u8) -> proc_macro::TokenTree {
    return proc_macro::TokenTree::Literal(proc_macro::Literal::u8_suffixed(literal));
  }
}

impl FromLiteral<u16> for proc_macro::TokenTree {
  fn from_literal(literal: u16) -> proc_macro::TokenTree {
    return proc_macro::TokenTree::Literal(proc_macro::Literal::u16_suffixed(literal));
  }
}

impl FromLiteral<u32> for proc_macro::TokenTree {
  fn from_literal(literal: u32) -> proc_macro::TokenTree {
    return proc_macro::TokenTree::Literal(proc_macro::Literal::u32_suffixed(literal));
  }
}

impl FromLiteral<usize> for proc_macro::TokenTree {
  fn from_literal(literal: usize) -> proc_macro::TokenTree {
    return proc_macro::TokenTree::Literal(proc_macro::Literal::usize_suffixed(literal));
  }
}

impl<'a> FromLiteral<&'a str> for proc_macro::TokenTree {
  fn from_literal(literal: &'a str) -> proc_macro::TokenTree {
    return proc_macro::TokenTree::Literal(proc_macro::Literal::string(literal));
  }
}

trait ElementCounter {
  fn initial_value(&self) -> usize;
  fn update(&self, value: &mut usize);
}

impl<'a> ElementCounter for core::str::Bytes<'a> {
  fn initial_value(&self) -> usize {
    return self.len() + 1;
  }

  fn update(&self, _value: &mut usize) {}
}

impl<'a> ElementCounter for std::str::EncodeUtf16<'a> {
  fn initial_value(&self) -> usize {
    return 0;
  }

  fn update(&self, value: &mut usize) {
    *value += 1;
  }
}

struct StringToTokenStream<'a, T> {
  iter: T,
  emit_comma: bool,
  finished: bool,
  counter: &'a mut usize,
}

impl<'a, T: ElementCounter> StringToTokenStream<'a, T> {
  fn new(iter: T, counter: &mut usize) -> StringToTokenStream<T> {
    *counter = ElementCounter::initial_value(&iter);
    return StringToTokenStream {
      iter: iter,
      emit_comma: false,
      finished: false,
      counter: counter,
    };
  }
}

impl<'a, T> core::iter::Iterator for StringToTokenStream<'a, T>
where
  T: core::iter::Iterator + ElementCounter,
  T::Item: core::default::Default,
  proc_macro::TokenTree: FromLiteral<T::Item>,
{
  type Item = proc_macro::TokenTree;

  fn next(&mut self) -> Option<proc_macro::TokenTree> {
    if self.finished {
      return None;
    }

    let emit_comma = self.emit_comma;
    self.emit_comma = !emit_comma;

    if emit_comma {
      return Some(proc_macro::TokenTree::Op(proc_macro::Op::new(
        ',',
        proc_macro::Spacing::Alone,
      )));
    }

    ElementCounter::update(&self.iter, self.counter);
    match self.iter.next() {
      Some(value) => {
        return Some(proc_macro::TokenTree::from_literal(value));
      }
      None => {
        self.finished = true;
        return Some(proc_macro::TokenTree::from_literal(
          core::default::Default::default(),
        ));
      }
    }
  }
}

fn string_iter_to_token_stream<T>(string_iter: T) -> (proc_macro::TokenStream, usize)
where
  T: core::iter::Iterator + ElementCounter,
  T::Item: core::default::Default,
  proc_macro::TokenTree: FromLiteral<T::Item>,
{
  let mut counter = 0;
  let token_stream =
    core::iter::FromIterator::from_iter(StringToTokenStream::new(string_iter, &mut counter));
  return (token_stream, counter);
}

#[proc_macro]
pub fn nsstring(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let mut iter = input.into_iter();
  let token_tree = iter.next().expect(HELP_MESSAGE);
  if iter.next().is_some() {
    panic!(HELP_MESSAGE);
  }

  let literal = match token_tree {
    proc_macro::TokenTree::Literal(l) => l,
    _ => panic!(HELP_MESSAGE),
  };

  let value = literalext::LiteralExt::parse_string(&literal).expect(HELP_MESSAGE);
  let use_utf16 = value.bytes().any(|b| b == 0 || !b.is_ascii());

  let bytes_link_section;
  let info_flags;
  let char_type;
  let element_count;
  let chars;
  if use_utf16 {
    bytes_link_section = proc_macro::TokenTree::from_literal("__TEXT,__ustring");
    info_flags = proc_macro::TokenTree::from_literal(2000u32); // This value assume little endian.
    char_type = quote_term("u16");
    let (a, b) = string_iter_to_token_stream(value.encode_utf16());
    chars = a;
    element_count = b;
  } else {
    bytes_link_section = proc_macro::TokenTree::from_literal("__TEXT,__cstring,cstring_literals");
    info_flags = proc_macro::TokenTree::from_literal(1992u32); // This value assume little endian.
    char_type = quote_term("u8");
    let (a, b) = string_iter_to_token_stream(value.bytes());
    chars = a;
    element_count = b;
  }

  let chars = proc_macro::TokenTree::Group(proc_macro::Group::new(
    proc_macro::Delimiter::Bracket,
    chars,
  ));
  let array_length = proc_macro::TokenTree::from_literal(element_count);

  return proc_macro::quote!{{
      extern crate objrs_frameworks_foundation;

      #[link_section = $bytes_link_section]
      #[doc(hidden)]
      static BYTES: [$char_type; $array_length] = $chars;

      #[link_section = "__DATA,__cfstring"]
      #[doc(hidden)]
      static STRING: objrs_frameworks_foundation::__objrs::CFConstantString = objrs_frameworks_foundation::__objrs::CFConstantString{
          isa:    unsafe { &objrs_frameworks_foundation::__objrs::CFConstantStringClassReference },
          info:   $info_flags,
          ptr:    unsafe { objrs_frameworks_foundation::__objrs::TransmuteHack { from: &BYTES }.to },
          length: $array_length - 1,
      };

      unsafe { objrs_frameworks_foundation::__objrs::TransmuteHack::<_, &'static objrs_frameworks_foundation::NSString> { from: &STRING }.to }
  }};
}
