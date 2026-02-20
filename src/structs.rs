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
    Pow,
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

    pub fn eval(&self) -> f64 {
        let mut results: Vec<f64> = vec![0.; self.0.len()];
        for (index, node) in self.0.iter().enumerate() {
            let res = match node {
                Node::Un(UnaryExp { op, child }) => match op {
                    Operator::Minus => -results[*child],
                    Operator::Abs => results[*child].abs(),
                    Operator::Ceil => results[*child].ceil(),
                    Operator::Floor => results[*child].floor(),
                    Operator::Ln => results[*child].ln(),
                    Operator::Exp => results[*child].exp(),

                    Operator::Sin => results[*child].sin(),
                    Operator::Cos => results[*child].cos(),
                    Operator::Tan => results[*child].tan(),

                    Operator::Csc => 1.0 / results[*child].sin(),
                    Operator::Sec => 1.0 / results[*child].cos(),
                    Operator::Cot => 1.0 / results[*child].tan(),

                    Operator::Asin => results[*child].asin(),
                    Operator::Acos => results[*child].acos(),
                    Operator::Atan => results[*child].atan(),

                    Operator::Acsc => (1.0 / results[*child]).asin(),
                    Operator::Asec => (1.0 / results[*child]).acos(),
                    Operator::Acot => (1.0 / results[*child]).atan(),

                    Operator::Sinh => results[*child].sinh(),
                    Operator::Cosh => results[*child].cosh(),
                    Operator::Tanh => results[*child].tanh(),

                    Operator::Csch => 1.0 / results[*child].sinh(),
                    Operator::Sech => 1.0 / results[*child].cosh(),
                    Operator::Coth => 1.0 / results[*child].tanh(),

                    Operator::Asinh => results[*child].asinh(),
                    Operator::Acosh => results[*child].acosh(),
                    Operator::Atanh => results[*child].atanh(),

                    Operator::Acsch => (1.0 / results[*child]).asinh(),
                    Operator::Asech => (1.0 / results[*child]).acosh(),
                    Operator::Acoth => (1.0 / results[*child]).atanh(),
                    _ => unreachable!("Incorrect handling of unary expression"),
                },
                Node::Bin(BinaryExp { op, left, right }) => match op {
                    Operator::Plus => results[*left] + results[*right],
                    Operator::Minus => results[*left] - results[*right],
                    Operator::Mul => results[*left] * results[*right],
                    Operator::Div => results[*left] / results[*right],
                    Operator::Mod => results[*left] % results[*right],
                    Operator::Pow => results[*left].powf(results[*right]),
                    Operator::Min => results[*left].min(results[*right]),
                    Operator::Max => results[*left].max(results[*right]),
                    Operator::Log => results[*left].log(results[*right]),
                    _ => unreachable!("Incorrect handling of binary expression"),
                },
                Node::Num(n) => *n,
                Node::Var => {
                    panic!()
                }
            };
            results[index] = res;
        }
        results[self.0.len() - 1]
    }
}

#[derive(Debug)]
pub enum Operator {
    // Operators

    // Binary
    Plus,
    Mul,
    Div,
    Mod,
    Pow,
    // Arbitrary
    Minus,

    // Functions

    // Unary
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

    // Binary
    Min,
    Max,
    Log,
}

#[derive(Debug)]
pub enum Node {
    Un(UnaryExp),
    Bin(BinaryExp),
    Num(f64),
    Var,
}

#[derive(Debug)]
pub struct UnaryExp {
    pub op: Operator,
    pub child: usize,
}

#[derive(Debug)]
pub struct BinaryExp {
    pub op: Operator,
    pub left: usize,
    pub right: usize,
}

#[derive(Debug)]
pub enum FnSig {
    Bin(Operator),
    Un(Operator),
    None,
}
