use crate::functions::*;
use crate::lexer::tokenize;
use crate::structs::{Ast, Parser, Token};
use crate::structs::{BinaryExp, BinaryOperator, UnaryExp, UnaryOperator};
use crate::structs::{FnSig, Node};

fn get_signature(name: &str) -> FnSig {
    match name {
        // Unary functions
        "abs" => FnSig::Un(UnaryOperator::Abs),
        "ceil" => FnSig::Un(UnaryOperator::Ceil),
        "floor" => FnSig::Un(UnaryOperator::Floor),
        "ln" => FnSig::Un(UnaryOperator::Ln),
        "exp" => FnSig::Un(UnaryOperator::Exp),

        "sin" => FnSig::Un(UnaryOperator::Sin),
        "cos" => FnSig::Un(UnaryOperator::Cos),
        "tan" => FnSig::Un(UnaryOperator::Tan),
        "csc" => FnSig::Un(UnaryOperator::Csc),
        "sec" => FnSig::Un(UnaryOperator::Sec),
        "cot" => FnSig::Un(UnaryOperator::Cot),

        "asin" => FnSig::Un(UnaryOperator::Asin),
        "acos" => FnSig::Un(UnaryOperator::Acos),
        "atan" => FnSig::Un(UnaryOperator::Atan),
        "acsc" => FnSig::Un(UnaryOperator::Acsc),
        "asec" => FnSig::Un(UnaryOperator::Asec),
        "acot" => FnSig::Un(UnaryOperator::Acot),

        "sinh" => FnSig::Un(UnaryOperator::Sinh),
        "cosh" => FnSig::Un(UnaryOperator::Cosh),
        "tanh" => FnSig::Un(UnaryOperator::Tanh),
        "csch" => FnSig::Un(UnaryOperator::Csch),
        "sech" => FnSig::Un(UnaryOperator::Sech),
        "coth" => FnSig::Un(UnaryOperator::Coth),

        "asinh" => FnSig::Un(UnaryOperator::Asinh),
        "acosh" => FnSig::Un(UnaryOperator::Acosh),
        "atanh" => FnSig::Un(UnaryOperator::Atanh),
        "acsch" => FnSig::Un(UnaryOperator::Acsch),
        "asech" => FnSig::Un(UnaryOperator::Asech),
        "acoth" => FnSig::Un(UnaryOperator::Acoth),

        // Binary functions
        "min" => FnSig::Bin(BinaryOperator::Min),
        "max" => FnSig::Bin(BinaryOperator::Max),
        "log" => FnSig::Bin(BinaryOperator::Log),

        _ => FnSig::None,
    }
}

fn parse_call(tree: &mut Ast, stream: &mut Parser) -> usize {
    let id = stream.iter.consume().unwrap().get_id();
    match get_signature(id) {
        FnSig::Un(op) => {
            let Some(Token::LPar) = stream.iter.consume() else {
                panic!("Functions cannot be used as variable names");
            };
            let arg = parse_exp(tree, stream);
            match stream.iter.consume() {
                Some(Token::RPar) => {}
                Some(Token::Comma) => panic!("Is not a binary function"),
                _ => panic!("Unclosed parantheses on function call"),
            }
            let node = UnaryExp { op: op, child: arg };
            return tree.add(Node::Un(node));
        }
        FnSig::Bin(op) => {
            let Some(Token::LPar) = stream.iter.consume() else {
                panic!("Functions cannot be used as variable names");
            };
            let first_arg = parse_exp(tree, stream);
            let Some(Token::Comma) = stream.iter.consume() else {
                panic!("Expected two arguments to binary function");
            };
            let second_arg = parse_exp(tree, stream);
            let Some(Token::RPar) = stream.iter.consume() else {
                panic!("Unclosed parantheses on function calls");
            };
            let node = BinaryExp {
                op: op,
                left: first_arg,
                right: second_arg,
            };
            return tree.add(Node::Bin(node));
        }
        _ => {
            if let Some(Token::LPar) = stream.iter.peek() {
                panic!("Not a function");
            } else {
                stream.iter.back();
                tree.add(Node::Var)
            }
        }
    }
}

fn parse_factor(tree: &mut Ast, stream: &mut Parser) -> usize {
    match stream.iter.consume() {
        Some(Token::LPar) => {
            let root = parse_exp(tree, stream);
            let Some(Token::RPar) = stream.iter.consume() else {
                panic!("Unclosed parenthesis on expression");
            };
            return root;
        }
        Some(Token::Num(n)) => return tree.add(Node::Num(n)),
        Some(Token::Id(s)) => {
            stream.iter.back();
            return parse_call(tree, stream);
        }
        _ => panic!("Idek what you did to get here"),
    }
}

fn parse_power(tree: &mut Ast, stream: &mut Parser) -> usize {
    let mut root: usize = parse_factor(tree, stream);
    loop {
        let Some(Token::Exp) = stream.iter.consume() else {
            stream.iter.back();
            return root;
        };
        let node = BinaryExp {
            op: BinaryOperator::Exp,
            left: root,
            right: parse_power(tree, stream),
        };
        root = tree.add(Node::Bin(node));
    }
}

fn parse_base(tree: &mut Ast, stream: &mut Parser) -> usize {
    if let Some(Token::Minus) = stream.iter.consume() {
        let node = UnaryExp {
            op: UnaryOperator::Minus,
            child: parse_power(tree, stream),
        };
        return tree.add(Node::Un(node));
    }
    stream.iter.back();
    parse_power(tree, stream)
}

fn parse_term(tree: &mut Ast, stream: &mut Parser) -> usize {
    let mut root: usize = parse_base(tree, stream);
    loop {
        match stream.iter.consume() {
            Some(Token::Mul) => {
                let node = BinaryExp {
                    op: BinaryOperator::Mul,
                    left: root,
                    right: parse_base(tree, stream),
                };
                root = tree.add(Node::Bin(node));
            }
            Some(Token::Div) => {
                let node = BinaryExp {
                    op: BinaryOperator::Div,
                    left: root,
                    right: parse_base(tree, stream),
                };
                root = tree.add(Node::Bin(node));
            }
            Some(Token::Mod) => {
                let node = BinaryExp {
                    op: BinaryOperator::Mod,
                    left: root,
                    right: parse_base(tree, stream),
                };
                root = tree.add(Node::Bin(node));
            }
            _ => {
                stream.iter.back();
                break;
            }
        }
    }
    root
}

fn parse_exp(tree: &mut Ast, stream: &mut Parser) -> usize {
    let mut root: usize = parse_term(tree, stream);
    loop {
        match stream.iter.consume() {
            Some(Token::Plus) => {
                let node = BinaryExp {
                    op: BinaryOperator::Plus,
                    left: root,
                    right: parse_term(tree, stream),
                };
                root = tree.add(Node::Bin(node));
            }
            Some(Token::Minus) => {
                let node = BinaryExp {
                    op: BinaryOperator::Minus,
                    left: root,
                    right: parse_term(tree, stream),
                };
                root = tree.add(Node::Bin(node));
            }
            _ => {
                stream.iter.back();
                break;
            }
        }
    }
    root
}

pub fn parse(input: &str) -> Ast {
    let mut stream = Parser::new(tokenize(input));
    let mut tree: Ast = Ast(Vec::new());
    let _ = parse_exp(&mut tree, &mut stream);
    tree
}
