mod lexer;
mod parser;
mod structs;

use parser::parse;

fn main() {
    println!("{}", parse("10 * 0.2").eval());
}
