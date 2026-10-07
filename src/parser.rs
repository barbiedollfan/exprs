use crate::lexer::{Lexer};
use crate::structs::{Ast, Token, Node, Call, add, sub, mul, div, rem, neg, pow};
use std::iter::Peekable;

#[macro_export]
macro_rules! assert_next {
    ($next:expr, $cmp:pat) => {
        let Some(tok) = $next else {
            return Err("Incomplete expression");
        };
        let $cmp = tok? else {
            return Err("Missing something");
        };
    };

    ($next:expr) => {
        let Some(tok) = $next else {
            return Err("Incomplete expression");
        };
    }
}

fn parse_call(tree: &mut Ast, stream: &mut Peekable<Lexer>) -> Result<usize, &'static str> {
    let (op, arity) = stream.next().unwrap()?.call_type()?; // Should never be looking into EOF
    let mut args = Vec::with_capacity(arity);
    if arity > 0 { assert_next!(stream.next(), Token::LPar); };
    for i in 0..arity {
        args.push(parse_exp(tree, stream)?);
        if i != arity - 1 { assert_next!(stream.next(), Token::Comma); };
    };
    if arity > 0 { assert_next!(stream.next(), Token::RPar); };
    let node = Call {
        op,
        args,
        arity,
    };
    Ok(tree.add(Node::Branch(node)))
}

fn parse_factor(tree: &mut Ast, stream: &mut Peekable<Lexer>) -> Result<usize, &'static str> {
    let &tok = stream.peek().unwrap(); // Should never be looking into EOF
    match tok? {
        Token::LPar => {
            stream.next();
            let root = parse_exp(tree, stream)?;
            assert_next!(stream.next(), Token::RPar);
            return Ok(root);
        }
        Token::Num(n) => {
            stream.next();
            return Ok(tree.add(Node::Leaf(n)));
        }
        Token::Id(_) => return parse_call(tree, stream),
        _ => return Err("Idek what you did to get here")
    }
}

fn parse_power(tree: &mut Ast, stream: &mut Peekable<Lexer>) -> Result<usize, &'static str> {
    let mut root = parse_factor(tree, stream)?;
    while let Some(&tok) = stream.peek() {
        let Token::Pow = tok? else { break; };
        stream.next();
        let node = Call {
            op: pow,
            args: vec![root, parse_power(tree, stream)?],
            arity: 2,
        };
        root = tree.add(Node::Branch(node));
    }
    Ok(root)
}

fn parse_base(tree: &mut Ast, stream: &mut Peekable<Lexer>) -> Result<usize, &'static str> {
    match stream.peek() {
        Some(&tok) => { 
            if let Token::Minus = tok? {
                stream.next();
                let node = Call {
                    op: neg,
                    args: vec![parse_power(tree, stream)?],
                    arity: 1,
                };
                return Ok(tree.add(Node::Branch(node)));
            }
        }
        None => {}
    };
    parse_power(tree, stream)
}

fn parse_term(tree: &mut Ast, stream: &mut Peekable<Lexer>) -> Result<usize, &'static str> {
    let mut root = parse_base(tree, stream)?;
    while let Some(&tok) = stream.peek() {
        match tok? {
            Token::Mul => {
                stream.next();
                let node = Call {
                    op: mul,
                    args: vec![root, parse_base(tree, stream)?],
                    arity: 2,
                };
                root = tree.add(Node::Branch(node));
            }
            Token::Div => {
                stream.next();
                let node = Call {
                    op: div,
                    args: vec![root, parse_base(tree, stream)?],
                    arity: 2,
                };
                root = tree.add(Node::Branch(node));
            }
            Token::Mod => {
                stream.next();
                let node = Call {
                    op: rem,
                    args: vec![root, parse_base(tree, stream)?],
                    arity: 2,
                };
                root = tree.add(Node::Branch(node));
            }
            _ => break,
        }
    }
    Ok(root)
}

fn parse_exp(tree: &mut Ast, stream: &mut Peekable<Lexer>) -> Result<usize, &'static str> {
    let mut root = parse_term(tree, stream)?;
    while let Some(&tok) = stream.peek() {
        match tok? {
            Token::Plus => {
                stream.next();
                let node = Call {
                    op: add,
                    args: vec![root, parse_term(tree, stream)?],
                    arity: 2,
                };
                root = tree.add(Node::Branch(node));
            }
            Token::Minus => {
                stream.next();
                let node = Call {
                    op: sub,
                    args: vec![root, parse_term(tree, stream)?],
                    arity: 2,
                };
                root = tree.add(Node::Branch(node));
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
