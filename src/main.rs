mod lexer;
mod parser;
mod structs;

use parser::parse;

fn main() {
    println!("{}", parse("ln(e) + sin(pi) + phi").eval());
}
