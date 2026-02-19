#[derive(Debug)]
pub struct Scanner<T> {
    pub source: Vec<T>,
    pub cursor: usize,
}

impl<T: Copy + Clone + std::fmt::Debug> Scanner<T> {
    pub fn peek(&self) -> Option<T> {
        self.source.get(self.cursor).copied()
    }

    pub fn consume(&mut self) -> Option<T> {
        let consumed = self.peek();
        self.cursor += 1;
        consumed
    }

    pub fn back(&mut self) -> () {
        self.cursor -= 1;
    }
}

#[derive(Debug)]
pub struct Lexer {
    pub iter: Scanner<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            iter: Scanner {
                source: input.chars().collect(),
                cursor: 0,
            },
        }
    }
}

#[derive(Debug)]
pub struct Parser<'a> {
    pub iter: Scanner<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: Vec<Token>) -> Parser {
        Parser {
            iter: Scanner {
                source: input,
                cursor: 0,
            },
        }
    }
}

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
            _ => panic!("Token does not contain an identifier"),
        }
    }
}

#[derive(Debug)]
pub struct Ast(pub Vec<Node>);

impl Ast {
    pub fn add(&mut self, node: Node) -> usize {
        // Constant folding here
        let end = self.0.len();
        self.0.push(node);
        end
    }
}

#[derive(Debug)]
pub enum UnaryOperator {
    // Regular operators
    Minus,

    // Functions
    Abs,
    Ceil,
    Floor,
    Ln,
    Exp,

    Sin,
    Cos,
    Tan,
    Csc,
    Sec,
    Cot,
    Asin,
    Acos,
    Atan,
    Acsc,
    Asec,
    Acot,

    Sinh,
    Cosh,
    Tanh,
    Csch,
    Sech,
    Coth,
    Asinh,
    Acosh,
    Atanh,
    Acsch,
    Asech,
    Acoth,
}

#[derive(Debug)]
pub enum BinaryOperator {
    // Regular operators
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Exp,

    // Functions
    Min,
    Max,
    Log,
}

#[derive(Debug)]
pub struct UnaryExp {
    pub op: UnaryOperator,
    pub child: usize,
}

#[derive(Debug)]
pub struct BinaryExp {
    pub op: BinaryOperator,
    pub left: usize,
    pub right: usize,
}

#[derive(Debug)]
pub enum Node {
    Un(UnaryExp),
    Bin(BinaryExp),
    Num(f64),
    Var,
}

#[derive(Debug)]
pub enum FnSig {
    Bin(BinaryOperator),
    Un(UnaryOperator),
    None,
}
