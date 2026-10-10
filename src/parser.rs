use crate::lexer::{Lexer};
use crate::structs::{Ast, Token, TokenKind, Error, Span, Node, Call, add, sub, mul, div, rem, neg, pow};
use std::iter::Peekable;

#[macro_export]
macro_rules! assert_next {
    ($next:expr, $cmp:pat) => {
        let Some(tok) = $next else {
            return Err(Error::UnfinishedExpr);
        };
        let $cmp = tok?.kind else {
            return Err(Error::UnfinishedExpr);
        };
    };

    ($next:expr) => {
        let Some(tok) = $next else {
            return Err(Error::UnfinishedExpr);
        };
    }
}

fn parse_call(tree: &mut Ast, stream: &mut Peekable<Lexer>) -> Result<usize, Error> {
    let (op, arity) = stream.next().unwrap()?.call_type()?; // Should never be looking into EOF
    let mut args = Vec::with_capacity(arity);
    if arity > 0 { assert_next!(stream.next(), TokenKind::LPar); };
    for i in 0..arity {
        args.push(parse_exp(tree, stream)?);
        if i != arity - 1 { assert_next!(stream.next(), TokenKind::Comma); };
    };
    if arity > 0 { assert_next!(stream.next(), TokenKind::RPar); };
    let node = Call {
        op,
        args,
        arity,
    };
    Ok(tree.add(Node::Branch(node)))
}

fn parse_factor(tree: &mut Ast, stream: &mut Peekable<Lexer>) -> Result<usize, Error> {
    let tok = stream.peek().unwrap(); // Should never be looking into EOF
    match tok.clone()?.kind {
        TokenKind::LPar => {
            stream.next();
            let root = parse_exp(tree, stream)?;
            assert_next!(stream.next(), TokenKind::RPar);
            return Ok(root);
        }
        TokenKind::Num(n) => {
            stream.next();
            return Ok(tree.add(Node::Leaf(n)));
        }
        TokenKind::Id(_) => return parse_call(tree, stream),
        _ => return Err(Error::Unknown)
    }
}

fn parse_power(tree: &mut Ast, stream: &mut Peekable<Lexer>) -> Result<usize, Error> {
    let mut root = parse_factor(tree, stream)?;
    while let Some(tok) = stream.peek() {
        let TokenKind::Pow = tok.clone()?.kind else { break; };
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

fn parse_base(tree: &mut Ast, stream: &mut Peekable<Lexer>) -> Result<usize, Error> {
    match stream.peek() {
        Some(tok) => { 
            if let TokenKind::Minus = tok.clone()?.kind {
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

fn parse_term(tree: &mut Ast, stream: &mut Peekable<Lexer>) -> Result<usize, Error> {
    let mut root = parse_base(tree, stream)?;
    while let Some(tok) = stream.peek() {
        match tok.clone()?.kind {
            TokenKind::Mul => {
                stream.next();
                let node = Call {
                    op: mul,
                    args: vec![root, parse_base(tree, stream)?],
                    arity: 2,
                };
                root = tree.add(Node::Branch(node));
            }
            TokenKind::Div => {
                stream.next();
                let node = Call {
                    op: div,
                    args: vec![root, parse_base(tree, stream)?],
                    arity: 2,
                };
                root = tree.add(Node::Branch(node));
            }
            TokenKind::Mod => {
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

fn parse_exp(tree: &mut Ast, stream: &mut Peekable<Lexer>) -> Result<usize, Error> {
    let mut root = parse_term(tree, stream)?;
    while let Some(tok) = stream.peek() {
        match tok.clone()?.kind {
            TokenKind::Plus => {
                stream.next();
                let node = Call {
                    op: add,
                    args: vec![root, parse_term(tree, stream)?],
                    arity: 2,
                };
                root = tree.add(Node::Branch(node));
            }
            TokenKind::Minus => {
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

fn parse(input: &str) -> Result<Ast, Error> {
    if input.trim().is_empty() {
        return Err(Error::EmptyExpr);
    }
    let mut stream = Lexer::new(input).peekable();
    let mut tree = Ast(Vec::new());
    let _ = parse_exp(&mut tree, &mut stream)?;
    let None = stream.next() else {
        return Err(Error::UnfinishedExpr);
    };
    Ok(tree)
}

pub fn eval(input: &str) -> Result<f64, Error> {
    Ok(parse(input)?.fold())
}
