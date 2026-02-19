use crate::structs::{Lexer, Token};

pub fn tokenize(input: &str) -> Vec<Token> {
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
            Some(c) if c == '^' => tokens.push(Token::Exp),
            Some(c) if c == '(' => tokens.push(Token::LPar),
            Some(c) if c == ')' => tokens.push(Token::RPar),
            Some(c) if c == ',' => tokens.push(Token::Comma),
            Some(c) if c.is_ascii_digit() || c == '.' => {
                let mut buff: Vec<char> = Vec::new();
                buff.push(c);
                while let Some(num) = stream.iter.consume() {
                    if !(num.is_ascii_digit() || num == '.') {
                        break;
                    }
                    buff.push(num);
                }
                stream.iter.back();
                let buff = buff.iter().cloned().collect::<String>();
                match buff.parse::<f64>() {
                    Ok(value) => tokens.push(Token::Num(value)),
                    Err(_) => panic!(),
                }
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
