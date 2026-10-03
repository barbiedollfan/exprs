use crate::structs::{Token};
use std::iter::{Enumerate, Peekable};
use std::str::Chars;


#[derive(Debug)]
struct Lexer<'a> {
    iter: Peekable<Enumerate<Chars<'a>>>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer {
            iter: input.chars().enumerate().peekable(),
        }
    }

    fn peek(&mut self) -> Option<&(usize, char)> {
        self.iter.peek()
    }

    fn next(&mut self) -> Option<(usize, char)> {
        self.iter.next()
    }

    fn get_substring<F>(&mut self, start_index: usize, f: F) -> usize
    where 
        F: Fn(char) -> bool
    {
        let mut end_index = start_index;
        while let Some(&(next_index, next_element)) = self.peek() {
            if !f(next_element) {
                break;
            }
            end_index = next_index;
            self.next();
        }
        end_index
    }
}

fn is_numeric(c: char) -> bool {
    c.is_ascii_digit() || c == '.'
}

pub fn tokenize<'a>(input: &'a str) -> Result<Vec<Token<'a>>, &'static str> {
    if input.trim().is_empty() {
        return Err("Cannot parse an empty expression");
    }
    let mut stream = Lexer::new(input);
    let mut tokens: Vec<Token> = Vec::new();
    loop {
        let Some(&(index, next)) = stream.peek() else {
            tokens.push(Token::EOF);
            return Ok(tokens);
        };
        match next {
            c if c.is_ascii_whitespace() => {},
            c if c == '+' => tokens.push(Token::Plus),
            c if c == '-' => tokens.push(Token::Minus),
            c if c == '*' => tokens.push(Token::Mul),
            c if c == '/' => tokens.push(Token::Div),
            c if c == '%' => tokens.push(Token::Mod),
            c if c == '^' => tokens.push(Token::Pow),
            c if c == '(' => tokens.push(Token::LPar),
            c if c == ')' => tokens.push(Token::RPar),
            c if c == ',' => tokens.push(Token::Comma),
            c if is_numeric(c) => {
                let end_index = stream.get_substring(index, |x| is_numeric(x));
                let Ok(num) = &input[index..=end_index].parse::<f64>() else {
                    return Err("Failed to parse number");
                };
                tokens.push(Token::Num(*num));
                continue;
            }
            c if c.is_ascii_alphabetic() => {
                let end_index = stream.get_substring(index, |x| x.is_ascii_alphabetic());
                tokens.push(Token::Id(&input[index..=end_index]));
                continue;
            }
            _ => return Err("Unknown character in input")
        }
        stream.next();
    }
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
