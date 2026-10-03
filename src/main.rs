fn main() {
    let expression = "pi^2 / 6";
    match exprs::eval(expression) {
        Ok(res) => println!("{res}"),
        Err(msg) => println!("{msg}")
    }
}
