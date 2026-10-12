fn main() {
    let expression = "cos(1) - cos(ln(e)) + sinh(1)";
    match exprs::eval(expression) {
        Ok(res) => println!("{res}"),
        Err(msg) => println!("{msg}")
    }
}
