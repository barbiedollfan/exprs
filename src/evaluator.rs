use crate::parser;

pub fn eval(input: &str) -> Result<f64, &'static str> {
    Ok(parser::parse(input)?.eval())
}
