use std::iter::{Enumerate, Peekable};
use std::str::Chars;
use crate::structs::{Token, TokenKind, Error, Span};

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    chars: Peekable<Enumerate<Chars<'a>>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input,
            chars: input.chars().enumerate().peekable(),
        }
    }

    fn get_substring<F>(&mut self, start_index: usize, f: F) -> usize
    where 
        F: Fn(char) -> bool
    {
        let mut end_index = start_index;
        while let Some(&(next_index, next_element)) = self.chars.peek() {
            if !f(next_element) {
                break;
            }
            end_index = next_index;
            self.chars.next();
        }
        end_index + 1
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let Some((index, next)) = self.chars.next() else {
                return None;
            };

            let tok = |kind, end| Some(Ok(Token {kind, span: Span {start: index, len: end - index}}));
            let un_tok = |kind| tok(kind, index + 1);

            match next {
                c if c.is_ascii_whitespace() => {},
                '+' => return un_tok(TokenKind::Plus),
                '-' => return un_tok(TokenKind::Minus),
                '*' => return un_tok(TokenKind::Mul),
                '/' => return un_tok(TokenKind::Div),
                '%' => return un_tok(TokenKind::Mod),
                '^' => return un_tok(TokenKind::Pow),
                '(' => return un_tok(TokenKind::LPar),
                ')' => return un_tok(TokenKind::RPar),
                ',' => return un_tok(TokenKind::Comma),
                c if is_numeric(c) => {
                    let end_index = self.get_substring(index, |x| is_numeric(x));
                    let num = &self.input[index..end_index];
                    let Ok(parsed) = num.parse::<f64>() else {
                        return Some(Err(Error::InvalidNum(num.to_string())));
                    };
                    return tok(TokenKind::Num(parsed), end_index);
                }
                c if c.is_ascii_alphabetic() => {
                    let end_index = self.get_substring(index, |x| x.is_ascii_alphabetic());
                    return tok(TokenKind::Id(&self.input[index..end_index]), end_index);
                }
                _ => return Some(Err(Error::UnknownChar(next)))
            }
        }
    }
}

fn is_numeric(c: char) -> bool {
    c.is_ascii_digit() || c == '.'
}
