fn main() {
    let expression = "";
    match exprs::eval(expression) {
        Ok(res) => println!("{res}"),
        Err(msg) => println!("{msg}")
    }
}
