use std::f64::consts::{E, PI};
use std::{error::Error as StdErr, fmt, ops::Range};

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub span: Range<usize>,
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
pub struct Error {
    pub kind: ErrorKind,
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    // Lexer
    InvalidNum(String),
    UnknownChar(char),

    //Parser
    UnknownId(String),
    UnfinishedExpr,
    UnexpectedExpr,
    Unknown,
}

impl StdErr for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::InvalidNum(ref s) => write!(f, "Could not parse number '{s}' ({:?})", self.span),
            ErrorKind::UnknownChar(c) => write!(f, "Unknown character '{c}' ({:?})", self.span),
            ErrorKind::UnknownId(ref s) => write!(f, "Unknown variable or function '{s}' ({:?})", self.span),

            ErrorKind::UnfinishedExpr => write!(f, "Incomplete expression"),
            ErrorKind::UnexpectedExpr => write!(f, "Unexpected expression"),
            ErrorKind::Unknown => write!(f, "Unknown error"),
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
fn asin(a: Vec<f64>) -> f64 { a[0].asin() }
fn sinh(a: Vec<f64>) -> f64 { a[0].sinh() }
fn asinh(a: Vec<f64>) -> f64 { a[0].asinh() }
fn cos(a: Vec<f64>) -> f64 { a[0].cos() }
fn acos(a: Vec<f64>) -> f64 { a[0].acos() }
fn cosh(a: Vec<f64>) -> f64 { a[0].cosh() }
fn acosh(a: Vec<f64>) -> f64 { a[0].acosh() }
fn tan(a: Vec<f64>) -> f64 { a[0].tan() }
fn atan(a: Vec<f64>) -> f64 { a[0].atan() }
fn tanh(a: Vec<f64>) -> f64 { a[0].tanh() }
fn atanh(a: Vec<f64>) -> f64 { a[0].atanh() }
fn exp(a: Vec<f64>) -> f64 { a[0].exp() }
fn ln(a: Vec<f64>) -> f64 { a[0].ln() }
fn log(a: Vec<f64>) -> f64 { a[0].log(a[1]) }
fn min(a: Vec<f64>) -> f64 { a[0].min(a[1]) }
fn max(a: Vec<f64>) -> f64 { a[0].max(a[1]) }

fn fptr(ptr: fn(Vec<f64>) -> f64, arity: usize) -> Result<Fptr, Error> { Ok(Fptr { ptr, arity }) }

impl<'a> Token<'a> {
    pub fn call_type(&self) -> Result<Fptr, Error> {
        let TokenKind::Id(name) = self.kind else {
            panic!("Token does not contain an identifier");
        };
        match name {
            "pi" => fptr(pi, 0),
            "e" => fptr(e, 0),
            "sin" => fptr(sin, 1),
            "asin" => fptr(asin, 1),
            "sinh" => fptr(sinh, 1),
            "asinh" => fptr(asinh, 1),
            "cos" => fptr(cos, 1),
            "acos" => fptr(acos, 1),
            "cosh" => fptr(cosh, 1),
            "acosh" => fptr(acosh, 1),
            "tan" => fptr(tan, 1),
            "atan" => fptr(atan, 1),
            "tanh" => fptr(tanh, 1),
            "atanh" => fptr(atanh, 1),
            "exp" => fptr(exp, 1),
            "ln" => fptr(ln, 1),
            "log" => fptr(log, 2),
            "min" => fptr(min, 2),
            "max" => fptr(max, 2),
            _ => Err(Error {kind: ErrorKind::UnknownId(name.to_string()), span: self.span.clone()})
        }
    }
}


#[derive(Debug)]
pub struct Ast(pub Vec<Node>);

impl Ast {
    pub fn new() -> Self {
        Ast(Vec::new())
    }

    pub fn add(&mut self, node: Node) -> usize {
        // Constant folding here
        let end = self.0.len();
        self.0.push(node);
        end
    }

    pub fn fold(&self) -> f64 {
        if self.0.len() == 0 { return 0. };
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
