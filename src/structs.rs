use std::f64::consts::{E, PI};
use std::{error::Error as StdErr, fmt};

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum TokenKind<'a> {
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
}

#[derive(Debug, Clone)]
pub struct Span {
    pub start: usize,
    pub len: usize,
}

#[derive(Debug, Clone)]
pub enum Error {
    // Lexer
    InvalidNum(String),
    UnknownChar(char),

    //Parser
    UnknownId(String),
    EmptyExpr,
    UnfinishedExpr,
    Unknown,
}

impl StdErr for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidNum(s) => write!(f, "Could not parse number '{s}'"),
            Error::UnknownChar(c) => write!(f, "Unknown character '{c}'"),
            Error::UnknownId(s) => write!(f, "Unknown variable or function '{s}'"),

            Error::EmptyExpr => write!(f, "Cannot parse an empty expression"),
            Error::UnfinishedExpr => write!(f, "Incomplete expression"),
            Error::Unknown => write!(f, "Unknown error"),
        }
    }
}

#[derive(Debug)]
pub enum Node {
    Call { proc: Op, args: Vec<usize> },
    Leaf(f64)
}

#[derive(Debug)]
pub enum Op {
    Func(Fptr), 
    Neg,
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Exp,
}

#[derive(Debug)]
pub struct Fptr {
    pub ptr: fn(Vec<f64>) -> f64,
    pub arity: usize,
}

fn pi(_a: Vec<f64>) -> f64 { PI }
fn e(_a: Vec<f64>) -> f64 { E }
fn sin(a: Vec<f64>) -> f64 { a[0].sin() }
fn min(a: Vec<f64>) -> f64 { a[0].min(a[1]) }

fn fptr(f: fn(Vec<f64>) -> f64, arity: usize) -> Result<Fptr, Error> { Ok(Fptr { ptr: f, arity }) }

impl<'a> Token<'a> {
    pub fn call_type(&self) -> Result<Fptr, Error> {
        let TokenKind::Id(name) = self.kind else {
            panic!("Token does not contain an identifier");
        };
        match name {
            "pi" => fptr(pi, 0),
            "e" => fptr(e, 0),
            "sin" => fptr(sin, 1),
            "min" => fptr(min, 2),
            _ => Err(Error::UnknownId(name.to_string()))
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

    pub fn fold(&self) -> f64 {
        let mut results: Vec<f64> = vec![0.; self.0.len()];
        for (index, node) in self.0.iter().enumerate() {
            let res = match node {
                Node::Call { proc, args } => match proc {
                    Op::Func(f) => {
                        if f.arity != args.len() { 
                            panic!("Incorrect number of arguments parsed for function");
                        };
                        (f.ptr)(args.into_iter().map(|arg| results[*arg]).collect())
                    },
                    Op::Neg if args.len() == 1 => -results[args[0]],
                    Op::Plus if args.len() == 2 => results[args[0]] + results[args[1]],
                    Op::Minus if args.len() == 2 => results[args[0]] - results[args[1]],
                    Op::Mul if args.len() == 2 => results[args[0]] * results[args[1]],
                    Op::Div if args.len() == 2 => results[args[0]] / results[args[1]],
                    Op::Mod if args.len() == 2 => results[args[0]] % results[args[1]],
                    Op::Exp if args.len() == 2 => results[args[0]].powf(results[args[1]]),
                    _ => panic!("Unknown operator or incorrect parsing of arguments")
                },
                Node::Leaf(n) => *n,
            };
            results[index] = res;
        }
        results[self.0.len() - 1]
    }
}
