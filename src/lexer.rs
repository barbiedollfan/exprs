use crate::structs::{Lexer, Token};

pub fn tokenize<'a>(input: &'a str) -> Vec<Token<'a>> {
    let mut stream = Lexer::new(input);
    let mut tokens: Vec<Token> = Vec::new();
    loop {
        let next = stream.iter.consume();
        match next {
            None => {
                tokens.push(Token::EOF);
                return tokens;
            }
            Some(c) if c.is_ascii_whitespace() => {}
            Some(c) if c == '+' => tokens.push(Token::Plus),
            Some(c) if c == '-' => tokens.push(Token::Minus),
            Some(c) if c == '*' => tokens.push(Token::Mul),
            Some(c) if c == '/' => tokens.push(Token::Div),
            Some(c) if c == '%' => tokens.push(Token::Mod),
            Some(c) if c == '^' => tokens.push(Token::Pow),
            Some(c) if c == '(' => tokens.push(Token::LPar),
            Some(c) if c == ')' => tokens.push(Token::RPar),
            Some(c) if c == ',' => tokens.push(Token::Comma),
            Some(c) if c.is_ascii_digit() => {
                stream.iter.back();
                let mut current: u64 = 0;
                let mut decimals: Option<i32> = None;
                while let Some(num) = stream.iter.consume() {
                    if num.is_ascii_digit() {
                        current = current * 10 + num.to_digit(10).unwrap() as u64;
                        if let Some(num) = decimals {
                            decimals = Some(num + 1);
                        }
                    } else if num == '.' {
                        if decimals.is_some() {
                            panic!("Invalid number");
                        };
                        decimals = Some(0);
                    } else {
                        break;
                    }
                }
                stream.iter.back();
                let value = if let Some(d) = decimals {
                    current as f64 / 10f64.powi(d)
                } else {
                    current as f64
                };

                tokens.push(Token::Num(value));
            }
            Some(c) if c.is_ascii_alphabetic() => {
                let start = stream.iter.cursor - 1;
                while let Some(letter) = stream.iter.consume() {
                    if !letter.is_ascii_alphabetic() {
                        break;
                    }
                }
                stream.iter.back();
                tokens.push(Token::Id(&input[start..stream.iter.cursor]));
            }
            _ => {}
        }
    }
}
