mod lexer;
mod parser;
mod structs;

use parser::parse;

fn main() {
    println!("{}", parse("").eval());
}
