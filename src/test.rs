fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

enum Sig {
    Binary,
    Unary,
}

fn main() {
    let op = Sig::Binary;
    let Sig::Unary = op else {
        println!("Hey");
        panic!();
    };
}
