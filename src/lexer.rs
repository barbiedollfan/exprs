use crate::structs::{Token};

fn is_numeric(c: char) -> bool {
    c.is_ascii_digit() || c == '.'
}

pub fn tokenize<'a>(input: &'a str) -> Result<Vec<Token<'a>>, &'static str> {
    if input.trim().is_empty() {
        return Err("Cannot parse an empty expression");
    }
    let mut stream = input.chars().enumerate().peekable();
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
                stream.next();
                let mut end_index = index;
                while let Some(&(next_index, next_num)) = stream.peek() {
                    if !is_numeric(next_num) {
                        break;
                    }
                    end_index = next_index;
                    stream.next();
                }
                let Ok(num) = &input[index..=end_index].parse::<f64>() else {
                    return Err("Failed to parse number");
                };
                tokens.push(Token::Num(*num));
                continue;
            }
            c if c.is_ascii_alphabetic() => {
                stream.next();
                let mut end_index = index;
                while let Some(&(next_index, next_char)) = stream.peek() {
                    if !next_char.is_ascii_alphabetic() {
                        break;
                    }
                    end_index = next_index;
                    stream.next();
                }
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
