use crate::lexer::tokenize;
use crate::structs::{Ast, Parser, Token};
use crate::structs::{BinaryExp, Operator, UnaryExp};
use crate::structs::{FnSig, Node};

fn get_signature(name: &str) -> FnSig {
    match name {
        // Unary functions
        "abs" => FnSig::Un(Operator::Abs),
        "ceil" => FnSig::Un(Operator::Ceil),
        "floor" => FnSig::Un(Operator::Floor),
        "ln" => FnSig::Un(Operator::Ln),
        "exp" => FnSig::Un(Operator::Exp),

        "sin" => FnSig::Un(Operator::Sin),
        "cos" => FnSig::Un(Operator::Cos),
        "tan" => FnSig::Un(Operator::Tan),
        "csc" => FnSig::Un(Operator::Csc),
        "sec" => FnSig::Un(Operator::Sec),
        "cot" => FnSig::Un(Operator::Cot),

        "asin" => FnSig::Un(Operator::Asin),
        "acos" => FnSig::Un(Operator::Acos),
        "atan" => FnSig::Un(Operator::Atan),
        "acsc" => FnSig::Un(Operator::Acsc),
        "asec" => FnSig::Un(Operator::Asec),
        "acot" => FnSig::Un(Operator::Acot),

        "sinh" => FnSig::Un(Operator::Sinh),
        "cosh" => FnSig::Un(Operator::Cosh),
        "tanh" => FnSig::Un(Operator::Tanh),
        "csch" => FnSig::Un(Operator::Csch),
        "sech" => FnSig::Un(Operator::Sech),
        "coth" => FnSig::Un(Operator::Coth),

        "asinh" => FnSig::Un(Operator::Asinh),
        "acosh" => FnSig::Un(Operator::Acosh),
        "atanh" => FnSig::Un(Operator::Atanh),
        "acsch" => FnSig::Un(Operator::Acsch),
        "asech" => FnSig::Un(Operator::Asech),
        "acoth" => FnSig::Un(Operator::Acoth),

        // Binary functions
        "min" => FnSig::Bin(Operator::Min),
        "max" => FnSig::Bin(Operator::Max),
        "log" => FnSig::Bin(Operator::Log),

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
        Some(Token::Id(_)) => {
            stream.iter.back();
            return parse_call(tree, stream);
        }
        _ => panic!("Idek what you did to get here"),
    }
}

fn parse_power(tree: &mut Ast, stream: &mut Parser) -> usize {
    let mut root: usize = parse_factor(tree, stream);
    loop {
        let Some(Token::Pow) = stream.iter.consume() else {
            stream.iter.back();
            return root;
        };
        let node = BinaryExp {
            op: Operator::Pow,
            left: root,
            right: parse_power(tree, stream),
        };
        root = tree.add(Node::Bin(node));
    }
}

fn parse_base(tree: &mut Ast, stream: &mut Parser) -> usize {
    if let Some(Token::Minus) = stream.iter.consume() {
        let node = UnaryExp {
            op: Operator::Minus,
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
                    op: Operator::Mul,
                    left: root,
                    right: parse_base(tree, stream),
                };
                root = tree.add(Node::Bin(node));
            }
            Some(Token::Div) => {
                let node = BinaryExp {
                    op: Operator::Div,
                    left: root,
                    right: parse_base(tree, stream),
                };
                root = tree.add(Node::Bin(node));
            }
            Some(Token::Mod) => {
                let node = BinaryExp {
                    op: Operator::Mod,
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
                    op: Operator::Plus,
                    left: root,
                    right: parse_term(tree, stream),
                };
                root = tree.add(Node::Bin(node));
            }
            Some(Token::Minus) => {
                let node = BinaryExp {
                    op: Operator::Minus,
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
