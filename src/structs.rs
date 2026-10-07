use std::f64::consts::{E, PI};

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
}

#[derive(Debug)]
pub enum Node {
    Branch(Call),
    Leaf(f64)
}

#[derive(Debug)]
pub struct Call {
    pub op: fn(Vec<f64>) -> f64,
    pub args: Vec<usize>,
    pub arity: usize
}

pub fn neg(a: Vec<f64>) -> f64 { -a[0] }
pub fn add(a: Vec<f64>) -> f64 { a[0] + a[1] }
pub fn sub(a: Vec<f64>) -> f64 { a[0] - a[1] }
pub fn mul(a: Vec<f64>) -> f64 { a[0] * a[1] }
pub fn div(a: Vec<f64>) -> f64 { a[0] / a[1] }
pub fn rem(a: Vec<f64>) -> f64 { a[0] % a[1] }
pub fn pow(a: Vec<f64>) -> f64 { a[0].powf(a[1]) }

fn pi(_a: Vec<f64>) -> f64 { PI }
fn e(_a: Vec<f64>) -> f64 { E }
fn sin(a: Vec<f64>) -> f64 { a[0].sin() }
fn min(a: Vec<f64>) -> f64 { a[0].min(a[1]) }

impl<'a> Token<'a> {
    pub fn call_type(&self) -> Result<(fn(Vec<f64>) -> f64, usize), &'static str> {
        let Token::Id(name) = self else {
            panic!("Token does not contain an identifier");
        };
        match *name {
            "pi" => Ok((pi, 0)),
            "e" => Ok((e, 0)),
            "sin" => Ok((sin, 1)),
            "min" => Ok((min, 2)),
            _ => Err("Unknown identifier name"),
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
                Node::Branch(Call { op, args, arity }) => {
                    if *arity != args.len() { 
                        panic!("Incorrect number of arguments provided to function");
                    };
                    op(args.into_iter().map(|arg| results[*arg]).collect())
                },
                Node::Leaf(n) => *n,
            };
            results[index] = res;
        }
        results[self.0.len() - 1]
    }
}
