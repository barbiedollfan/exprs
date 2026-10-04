fn main() {
    let expression = "masch";
    match exprs::eval(expression) {
        Ok(res) => println!("{res}"),
        Err(msg) => println!("{msg}")
    }
}
