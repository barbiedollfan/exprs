use crate::lexer;
use crate::structs::{Ast, Parser, Token, BinaryExp, Operator, UnaryExp, IdType, Node};

fn parse_call(tree: &mut Ast, stream: &mut Parser) -> Result<usize, &'static str> {
    let id_type = stream.iter.consume().unwrap().id_type();
    match id_type {
        IdType::Un(op) => {
            let Some(Token::LPar) = stream.iter.consume() else {
                return Err("Functions cannot be used as variable names");
            };
            let arg = parse_exp(tree, stream)?;
            match stream.iter.consume() {
                Some(Token::RPar) => {}
                Some(Token::Comma) => return Err("Is not a binary function"),
                _ => return Err("Unclosed parantheses on function call"),
            }
            let node = UnaryExp { op: op, child: arg };
            return Ok(tree.add(Node::Un(node)));
        }
        IdType::Bin(op) => {
            let Some(Token::LPar) = stream.iter.consume() else {
                return Err("Functions cannot be used as variable names");
            };
            let first_arg = parse_exp(tree, stream)?;
            let Some(Token::Comma) = stream.iter.consume() else {
                return Err("Expected two arguments to binary function");
            };
            let second_arg = parse_exp(tree, stream)?;
            let Some(Token::RPar) = stream.iter.consume() else {
                return Err("Unclosed parantheses on function calls");
            };
            let node = BinaryExp {
                op: op,
                left: first_arg,
                right: second_arg,
            };
            return Ok(tree.add(Node::Bin(node)));
        }
        IdType::Const(n) => {
            return Ok(tree.add(Node::Num(n)));
        }
        _ => {
            if let Some(Token::LPar) = stream.iter.peek() {
                return Err("Not a function");
            } else {
                return Err("Incorrect variable or function name");
            }
        }
    }
}

fn parse_factor(tree: &mut Ast, stream: &mut Parser) -> Result<usize, &'static str> {
    match stream.iter.consume() {
        Some(Token::LPar) => {
            let root = parse_exp(tree, stream)?;
            let Some(Token::RPar) = stream.iter.consume() else {
                return Err("Unclosed parenthesis on expression");
            };
            return Ok(root);
        }
        Some(Token::Num(n)) => return Ok(tree.add(Node::Num(n))),
        Some(Token::Id(_)) => {
            stream.iter.back();
            return parse_call(tree, stream);
        }
        _ => return Err("Idek what you did to get here"),
    }
}

fn parse_power(tree: &mut Ast, stream: &mut Parser) -> Result<usize, &'static str> {
    let mut root: usize = parse_factor(tree, stream)?;
    loop {
        let Some(Token::Pow) = stream.iter.consume() else {
            stream.iter.back();
            return Ok(root);
        };
        let node = BinaryExp {
            op: Operator::Pow,
            left: root,
            right: parse_power(tree, stream)?,
        };
        root = tree.add(Node::Bin(node));
    }
}

fn parse_base(tree: &mut Ast, stream: &mut Parser) -> Result<usize, &'static str> {
    if let Some(Token::Minus) = stream.iter.consume() {
        let node = UnaryExp {
            op: Operator::Minus,
            child: parse_power(tree, stream)?,
        };
        return Ok(tree.add(Node::Un(node)));
    }
    stream.iter.back();
    parse_power(tree, stream)
}

fn parse_term(tree: &mut Ast, stream: &mut Parser) -> Result<usize, &'static str> {
    let mut root: usize = parse_base(tree, stream)?;
    loop {
        match stream.iter.consume() {
            Some(Token::Mul) => {
                let node = BinaryExp {
                    op: Operator::Mul,
                    left: root,
                    right: parse_base(tree, stream)?,
                };
                root = tree.add(Node::Bin(node));
            }
            Some(Token::Div) => {
                let node = BinaryExp {
                    op: Operator::Div,
                    left: root,
                    right: parse_base(tree, stream)?,
                };
                root = tree.add(Node::Bin(node));
            }
            Some(Token::Mod) => {
                let node = BinaryExp {
                    op: Operator::Mod,
                    left: root,
                    right: parse_base(tree, stream)?,
                };
                root = tree.add(Node::Bin(node));
            }
            _ => {
                stream.iter.back();
                break;
            }
        }
    }
    Ok(root)
}

fn parse_exp(tree: &mut Ast, stream: &mut Parser) -> Result<usize, &'static str> {
    let mut root: usize = parse_term(tree, stream)?;
    loop {
        match stream.iter.consume() {
            Some(Token::Plus) => {
                let node = BinaryExp {
                    op: Operator::Plus,
                    left: root,
                    right: parse_term(tree, stream)?,
                };
                root = tree.add(Node::Bin(node));
            }
            Some(Token::Minus) => {
                let node = BinaryExp {
                    op: Operator::Minus,
                    left: root,
                    right: parse_term(tree, stream)?,
                };
                root = tree.add(Node::Bin(node));
            }
            Some(Token::EOF) => break,
            _ => return Err("Invalid expression"),
        }
    }
    Ok(root)
}

pub fn parse(input: &str) -> Result<Ast, &'static str> {
    let mut stream = Parser::new(lexer::tokenize(input)?);
    let mut tree: Ast = Ast(Vec::new());
    let _ = parse_exp(&mut tree, &mut stream)?;
    Ok(tree)
}
