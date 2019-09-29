// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

extern crate proc_macro2;
extern crate quote;

use proc_macro2::{Delimiter, Spacing, TokenStream, TokenTree};
use quote::ToTokens;
use std::fmt::Write;

#[derive(Copy, Clone)]
struct State {
  delimiter: Delimiter,
  angle_bracket_depth: i32,
  separator: char,
}

fn write_separator(
  separator: char,
  indent: i32,
  fmt: &mut std::fmt::Formatter,
) -> std::fmt::Result {
  if separator == '\x00' {
    return Ok(());
  }

  fmt.write_char(separator)?;
  if separator == '\n' {
    for _ in 0..indent {
      fmt.write_str("  ")?;
    }
  }

  return Ok(());
}

fn format_tokens(tokens: TokenStream, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
  if tokens.is_empty() {
    return Ok(());
  }

  let initial_state = State {
    delimiter: Delimiter::Brace,
    angle_bracket_depth: 0,
    separator: '\x00',
  };

  let mut first_iteration = true;
  let mut indent = 0;
  let mut next_separator = '\x00';
  let mut attr_prefix = 0u16;
  let mut tree = vec![(tokens.into_iter(), initial_state)];
  while let Some((mut iter, mut state)) = tree.pop() {
    while let Some(token_tree) = iter.next() {
      if next_separator == '\n' {
        match token_tree {
          TokenTree::Punct(ref punct) => {
            if punct.spacing() == Spacing::Alone
              && (punct.as_char() == ',' || punct.as_char() == ';')
            {
              next_separator = ' ';
            }
          }
          _ => (),
        }
      }

      write_separator(next_separator, indent, fmt)?;
      let conjoined = next_separator == '\x00' && !first_iteration;
      first_iteration = false;
      next_separator = '\x00';

      match token_tree {
        TokenTree::Group(group) => {
          tree.push((iter, state));
          iter = group.stream().into_iter();
          state.delimiter = group.delimiter();

          match state.delimiter {
            Delimiter::Parenthesis => {
              fmt.write_char('(')?;
              state.separator = ' ';
            }
            Delimiter::Brace => {
              fmt.write_char('{')?;
              state.separator = '\n';
            }
            Delimiter::Bracket => {
              fmt.write_char('[')?;
              if attr_prefix == b'#' as u16 || attr_prefix == (((b'#' as u16) << 8) | b'!' as u16) {
                state.separator = '\n';
              } else {
                state.separator = ' ';
              }
            }
            Delimiter::None => {
              state.separator = ' ';
            }
          }

          attr_prefix = 0;
          if state.delimiter == Delimiter::Brace {
            indent += 1;
            next_separator = '\n';
          } else {
            next_separator = ' ';
          }

          state.angle_bracket_depth = 0;
        }
        TokenTree::Ident(ident) => {
          fmt.write_str(&ident.to_string())?;
          attr_prefix = 0;
          next_separator = ' ';
        }
        TokenTree::Punct(punct) => {
          let character = punct.as_char();
          fmt.write_char(character)?;
          if punct.spacing() == Spacing::Alone {
            if character == '<' {
              state.angle_bracket_depth += 1;
            } else if character == '>' && state.angle_bracket_depth > 0 {
              state.angle_bracket_depth -= 1;
            }

            if !conjoined {
              // Adding a line break after ',' is meant for braced struct initialization. Don't add
              // a line break if the angle bracket depth != 1 (to avoid adding a line break in
              // generic parameter lists). This isn't a perfect check because we can't really tell
              // the difference between comparisons (`1 < 2` and `2 > 1`) and generics (without
              // running a full parser). But it's good enough.
              if (character == ';' && state.delimiter != Delimiter::Bracket)
                || (character == ','
                  && state.delimiter == Delimiter::Brace
                  && state.angle_bracket_depth == 0)
              {
                next_separator = '\n';
                attr_prefix = 0;
                continue;
              }

              if character == '#' || character == '!' {
                attr_prefix = (attr_prefix << 8) | character as u16;
              } else {
                attr_prefix = 0;
              }
            } else {
              attr_prefix = 0;
            }

            next_separator = ' ';
          } else {
            attr_prefix = 0;
          }
        }
        TokenTree::Literal(literal) => {
          fmt.write_str(&literal.to_string())?;
          attr_prefix = 0;
          next_separator = ' ';
        }
      }
    }

    if tree.len() == 0 {
      // Avoid printing a trailing extra '}'.
      break;
    }

    if state.delimiter == Delimiter::Brace {
      indent -= 1;
      if next_separator == ' ' {
        next_separator = '\n';
      }
    } else if next_separator == '\n' {
      next_separator = ' ';
    }
    write_separator(next_separator, indent, fmt)?;
    match state.delimiter {
      Delimiter::Parenthesis => fmt.write_char(')')?,
      Delimiter::Brace => fmt.write_char('}')?,
      Delimiter::Bracket => fmt.write_char(']')?,
      Delimiter::None => (),
    }
    attr_prefix = 0;
    next_separator = state.separator;
  }
  return Ok(());
}

pub struct FormatTokens<T: ToTokens>(pub T);

impl<T: ToTokens> core::fmt::Display for FormatTokens<T> {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
    return format_tokens(self.0.to_token_stream(), f);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use quote::quote;

  fn tokens_to_string(tokens: TokenStream) -> String {
    return format!("{}", FormatTokens(tokens));
  }

  #[test]
  fn attributes() {
    let tokens = quote! {
      #![foo]
      #[bar]
      pub statix X: u8 = 0;
    };
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let expected = "\
      # ! [ foo ]\n\
      # [ bar ]\n\
      pub statix X : u8 = 0 ;\
    ";
    assert_eq!(expected, tokens_to_string(tokens));
  }

  #[test]
  fn semicolons() {
    let tokens = quote! {
      let foo = "Hello, world!";
      let bar: [u8; 2] = [0u8; 2];
      {
        let baz = 123;
        let qux = 42;
      }
    };
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let expected = "\
      let foo = \"Hello, world!\" ;\n\
      let bar : [ u8 ; 2 ] = [ 0u8 ; 2 ] ;\n\
      {\n  \
        let baz = 123 ;\n  \
        let qux = 42 ;\n\
      }\
    ";
    assert_eq!(expected, tokens_to_string(tokens));
  }

  #[test]
  fn functions() {
    let tokens = quote! {
      fn foo() -> u32 {
        42
      }

      fn bar() {
        ()
      }
    };
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let expected = "\
      fn foo ( ) -> u32 {\n  \
        42\n\
      }\n\
      fn bar ( ) {\n  \
        ( )\n\
      }\
    ";
    assert_eq!(expected, tokens_to_string(tokens));
  }

  #[test]
  fn nested_groups() {
    let tokens = quote! {
      {{{{}}}};
      (((())));
      [[[[]]]];
      {({[([{()}])]})};
    };
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let expected = "\
      {\n  \
        {\n    \
          {\n      \
            {\n      \
            }\n    \
          }\n  \
        }\n\
      } ;\n\
      ( ( ( ( ) ) ) ) ;\n\
      [ [ [ [ ] ] ] ] ;\n\
      {\n  \
        ( {\n    \
          [ ( [ {\n      \
            ( )\n    \
          } ] ) ]\n  \
        } )\n\
      } ;\
    ";
    assert_eq!(expected, tokens_to_string(tokens));
  }

  #[test]
  fn brace_line_breaks() {
    let tokens = quote! {
      foo({ 0 });
      bar({ 1 }, { 2 });
      baz({
        3;
      });
      qux(
        {
          4;
        },
        {
          5;
        }
      );
      zzz(
        {
          6;
        },
        {
          7;
        },
      );
      let x = [{}];
      let y = [
        {
        },
      ];
    };
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let expected = "\
      foo ( {\n  \
        0\n\
      } ) ;\n\
      bar ( {\n  \
        1\n\
      } , {\n  \
        2\n\
      } ) ;\n\
      baz ( {\n  \
        3 ;\n\
      } ) ;\n\
      qux ( {\n  \
        4 ;\n\
      } , {\n  \
        5 ;\n\
      } ) ;\n\
      zzz ( {\n  \
        6 ;\n\
      } , {\n  \
        7 ;\n\
      } , ) ;\n\
      let x = [ {\n\
      } ] ;\n\
      let y = [ {\n\
      } , ] ;\
    ";
    assert_eq!(expected, tokens_to_string(tokens));
  }

  #[test]
  fn struct_init() {
    let tokens = quote! {
      let foo = Foo(0, 1, 2, 3);
      let bar = Bar {
        baz: 4,
        qux: 5,
      };
      let zzz = Zzz {
        aaa: 6,
        bbb: 7
      };
    };
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let expected = "\
      let foo = Foo ( 0 , 1 , 2 , 3 ) ;\n\
      let bar = Bar {\n  \
        baz : 4 ,\n  \
        qux : 5 ,\n\
      } ;\n\
      let zzz = Zzz {\n  \
        aaa : 6 ,\n  \
        bbb : 7\n\
      } ;\
    ";
    assert_eq!(expected, tokens_to_string(tokens));
  }

  #[test]
  fn punctuation() {
    let tokens = quote! {
      foo!(0 - 1 + 2);
      let mut x = 3;
      x += -4;
      x >>= 5;
      !false != !!true || !foo!(false);
      let y = 'c';
      static Z: &'static u8 = &6;
    };
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let expected = "\
      foo ! ( 0 - 1 + 2 ) ;\n\
      let mut x = 3 ;\n\
      x += - 4 ;\n\
      x >>= 5 ;\n\
      ! false != ! ! true || ! foo ! ( false ) ;\n\
      let y = 'c' ;\n\
      static Z : & 'static u8 = & 6 ;\
    ";
    assert_eq!(expected, tokens_to_string(tokens));
  }
  #[test]
  fn generics() {
    let tokens = quote! {
      let x: Result<(), ()> = Ok(());
      let foo = Foo {
        a: 0 > 1,
        b: 1 < 2,
      };
    };
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let expected = "\
      let x : Result < ( ) , ( ) > = Ok ( ( ) ) ;\n\
      let foo = Foo {\n  \
        a : 0 > 1 ,\n  \
        b : 1 < 2 ,\n\
      } ;\
    ";
    assert_eq!(expected, tokens_to_string(tokens));
  }
}
