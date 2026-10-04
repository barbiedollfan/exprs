use crate::lexer::{Lexer};
use crate::structs::{Ast, Token, BinaryExp, Operator, UnaryExp, IdType, Node};
use std::iter::Peekable;

#[macro_export]
macro_rules! assert_next {
    ($next:expr, $cmp:pat) => {
        let Some(res) = $next else {
            return Err("Incomplete expression");
        };
        let $cmp = res? else {
            return Err("Missing something");
        };
    };

    ($next:expr) => {
        let Some(res) = $next else {
            return Err("Incomplete expression");
        };
    }
}

fn parse_call(tree: &mut Ast, stream: &mut Peekable<Lexer>) -> Result<usize, &'static str> {
    let id_type = stream.next().unwrap()?.id_type(); // Should never be looking into EOF
    match id_type {
        IdType::Un(op) => {
            assert_next!(stream.next(), Token::LPar);
            let arg = parse_exp(tree, stream)?;
            assert_next!(stream.next(), Token::RPar);
            let node = UnaryExp { op, child: arg };
            return Ok(tree.add(Node::Un(node)));
        }
        IdType::Bin(op) => {
            assert_next!(stream.next(), Token::LPar);
            let first_arg = parse_exp(tree, stream)?;
            assert_next!(stream.next(), Token::Comma);
            let second_arg = parse_exp(tree, stream)?;
            assert_next!(stream.next(), Token::RPar);
            let node = BinaryExp {
                op,
                left: first_arg,
                right: second_arg,
            };
            return Ok(tree.add(Node::Bin(node)));
        }
        IdType::Const(n) => {
            return Ok(tree.add(Node::Num(n)));
        }
        _ => return Err("Incorrect identifier or something"),
    }
}

fn parse_factor(tree: &mut Ast, stream: &mut Peekable<Lexer>) -> Result<usize, &'static str> {
    let &res = stream.peek().unwrap(); // Should never be looking into EOF
    match res? {
        Token::LPar => {
            stream.next();
            let root = parse_exp(tree, stream)?;
            assert_next!(stream.next(), Token::RPar);
            return Ok(root);
        }
        Token::Num(n) => {
            stream.next();
            return Ok(tree.add(Node::Num(n)));
        }
        Token::Id(_) => return parse_call(tree, stream),
        _ => return Err("Idek what you did to get here")
    }
}

fn parse_power(tree: &mut Ast, stream: &mut Peekable<Lexer>) -> Result<usize, &'static str> {
    let mut root: usize = parse_factor(tree, stream)?;
    while let Some(&res) = stream.peek() {
        let Token::Pow = res? else { break; };
        stream.next();
        let node = BinaryExp {
            op: Operator::Pow,
            left: root,
            right: parse_power(tree, stream)?,
        };
        root = tree.add(Node::Bin(node));
    }
    Ok(root)
}

fn parse_base(tree: &mut Ast, stream: &mut Peekable<Lexer>) -> Result<usize, &'static str> {
    match stream.peek() {
        Some(&res) => { 
            if let Token::Minus = res? {
                stream.next();
                let node = UnaryExp {
                    op: Operator::Minus,
                    child: parse_power(tree, stream)?,
                };
                return Ok(tree.add(Node::Un(node)));
            }
        }
        None => {}
    };
    parse_power(tree, stream)
}

fn parse_term(tree: &mut Ast, stream: &mut Peekable<Lexer>) -> Result<usize, &'static str> {
    let mut root: usize = parse_base(tree, stream)?;
    while let Some(&res) = stream.peek() {
        match res? {
            Token::Mul => {
                stream.next();
                let node = BinaryExp {
                    op: Operator::Mul,
                    left: root,
                    right: parse_base(tree, stream)?,
                };
                root = tree.add(Node::Bin(node));
            }
            Token::Div => {
                stream.next();
                let node = BinaryExp {
                    op: Operator::Div,
                    left: root,
                    right: parse_base(tree, stream)?,
                };
                root = tree.add(Node::Bin(node));
            }
            Token::Mod => {
                stream.next();
                let node = BinaryExp {
                    op: Operator::Mod,
                    left: root,
                    right: parse_base(tree, stream)?,
                };
                root = tree.add(Node::Bin(node));
            }
            _ => break,
        }
    }
    Ok(root)
}

fn parse_exp(tree: &mut Ast, stream: &mut Peekable<Lexer>) -> Result<usize, &'static str> {
    let mut root: usize = parse_term(tree, stream)?;
    while let Some(&res) = stream.peek() {
        match res? {
            Token::Plus => {
                stream.next();
                let node = BinaryExp {
                    op: Operator::Plus,
                    left: root,
                    right: parse_term(tree, stream)?,
                };
                root = tree.add(Node::Bin(node));
            }
            Token::Minus => {
                stream.next();
                let node = BinaryExp {
                    op: Operator::Minus,
                    left: root,
                    right: parse_term(tree, stream)?,
                };
                root = tree.add(Node::Bin(node));
            }
            _ => break,
        }
    }
    Ok(root)
}

fn parse(input: &str) -> Result<Ast, &'static str> {
    if input.trim().is_empty() {
        return Err("Cannot parse empty expression");
    }
    let mut stream = Lexer::new(input).peekable();
    let mut tree = Ast(Vec::new());
    let _ = parse_exp(&mut tree, &mut stream)?;
    let None = stream.next() else {
        return Err("Invalid expression");
    };
    Ok(tree)
}

pub fn eval(input: &str) -> Result<f64, &'static str> {
    Ok(parse(input)?.fold())
}
