fn main() {
    let expression = "10.0 * .1 + 1";
    match exprs::eval(expression) {
        Ok(res) => println!("{res}"),
        Err(msg) => println!("{msg}")
    }
}
