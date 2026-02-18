#[derive(Copy, Clone, Debug)]
pub enum Token<'a> {
    Num(f64),
    Id(&'a str),
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Exp,
    LPar,
    RPar,
    Comma,
    EOF,
}

impl<'a> Token<'a> {
    pub fn get_id(&self) -> &'a str {
        match self {
            Token::Id(s) => s,
            _ => panic!(),
        }
    }
}

struct Lexer {
    source: Vec<char>,
    cursor: usize,
}

impl Lexer {
    fn peek(&self) -> Option<char> {
        self.source.get(self.cursor).copied()
    }

    fn consume(&mut self) -> Option<char> {
        let consumed = self.peek();
        self.cursor += 1;
        consumed
    }

    fn back(&mut self) -> () {
        self.cursor -= 1;
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut stream = Lexer {
        source: input.chars().collect(),
        cursor: 0,
    };
    let mut tokens: Vec<Token> = Vec::new();
    loop {
        let next = stream.consume();
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
                while let Some(num) = stream.consume() {
                    if !(num.is_ascii_digit() || num == '.') {
                        break;
                    }
                    buff.push(num);
                }
                stream.back();
                let buff = buff.iter().cloned().collect::<String>();
                match buff.parse::<f64>() {
                    Ok(value) => tokens.push(Token::Num(value)),
                    Err(_) => panic!(),
                }
            }
            Some(c) if c.is_ascii_alphabetic() => {
                let start = stream.cursor - 1;
                while let Some(letter) = stream.consume() {
                    if !letter.is_ascii_alphabetic() {
                        break;
                    }
                }
                stream.back();
                tokens.push(Token::Id(&input[start..stream.cursor]));
            }
            _ => {}
        }
    }
}
