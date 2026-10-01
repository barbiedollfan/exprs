fn main() {
    let expression = "pi t + 2";
    let res = exprs::eval(expression);
    match res {
        Ok(val) => println!("{val}"),
        Err(msg) => println!("{msg}")
    };
}
