use crate::structs::{Token};
use std::iter::{Enumerate, Peekable};
use std::str::Chars;


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
        end_index
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, &'static str>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let Some(&(index, next)) = self.chars.peek() else {
                return None;
            };
            self.chars.next();
            match next {
                c if c.is_ascii_whitespace() => {},
                c if c == '+' => return Some(Ok(Token::Plus)),
                c if c == '-' => return Some(Ok(Token::Minus)),
                c if c == '*' => return Some(Ok(Token::Mul)),
                c if c == '/' => return Some(Ok(Token::Div)),
                c if c == '%' => return Some(Ok(Token::Mod)),
                c if c == '^' => return Some(Ok(Token::Pow)),
                c if c == '(' => return Some(Ok(Token::LPar)),
                c if c == ')' => return Some(Ok(Token::RPar)),
                c if c == ',' => return Some(Ok(Token::Comma)),
                c if is_numeric(c) => {
                    let end_index = self.get_substring(index, |x| is_numeric(x));
                    let Ok(num) = &self.input[index..=end_index].parse::<f64>() else {
                        return Some(Err("Failed to parse number"));
                    };
                    return Some(Ok(Token::Num(*num)));
                }
                c if c.is_ascii_alphabetic() => {
                    let end_index = self.get_substring(index, |x| x.is_ascii_alphabetic());
                    return Some(Ok(Token::Id(&self.input[index..=end_index])));
                }
                _ => return Some(Err("Unknown character in input"))
            }
        }
    }
}

fn is_numeric(c: char) -> bool {
    c.is_ascii_digit() || c == '.'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_numeric() {
        assert!(is_numeric('0'));
        assert!(is_numeric('.'));
        assert!(!is_numeric('a'));
    }
}
