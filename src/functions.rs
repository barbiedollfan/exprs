pub fn abs(x: f64) -> f64 {
    x.abs()
}
pub fn ceil(x: f64) -> f64 {
    x.ceil()
}
pub fn floor(x: f64) -> f64 {
    x.floor()
}
pub fn min(x: f64, y: f64) -> f64 {
    x.min(y)
}
pub fn max(x: f64, y: f64) -> f64 {
    x.max(y)
}
pub fn ln(x: f64) -> f64 {
    x.ln()
}
pub fn log(base: f64, x: f64) -> f64 {
    x.log(base)
}
pub fn exp(x: f64) -> f64 {
    x.exp()
}
pub fn pow(base: f64, power: f64) -> f64 {
    base.powf(power)
}

pub fn sin(x: f64) -> f64 {
    x.sin()
}
pub fn cos(x: f64) -> f64 {
    x.cos()
}
pub fn tan(x: f64) -> f64 {
    x.tan()
}
pub fn csc(x: f64) -> f64 {
    1.0 / x.sin()
}
pub fn sec(x: f64) -> f64 {
    1.0 / x.cos()
}
pub fn cot(x: f64) -> f64 {
    1.0 / x.tan()
}

pub fn arcsin(x: f64) -> f64 {
    x.asin()
}
pub fn arccos(x: f64) -> f64 {
    x.acos()
}
pub fn arctan(x: f64) -> f64 {
    x.atan()
}
pub fn arccsc(x: f64) -> f64 {
    (1.0 / x).asin()
}
pub fn arcsec(x: f64) -> f64 {
    (1.0 / x).acos()
}
pub fn arccot(x: f64) -> f64 {
    (1.0 / x).atan()
}

pub fn sinh(x: f64) -> f64 {
    x.sinh()
}
pub fn cosh(x: f64) -> f64 {
    x.cosh()
}
pub fn tanh(x: f64) -> f64 {
    x.tanh()
}
pub fn csch(x: f64) -> f64 {
    1.0 / x.sinh()
}
pub fn sech(x: f64) -> f64 {
    1.0 / x.cosh()
}
pub fn coth(x: f64) -> f64 {
    1.0 / x.tanh()
}

pub fn arcsinh(x: f64) -> f64 {
    x.asinh()
}
pub fn arccosh(x: f64) -> f64 {
    x.acosh()
}
pub fn arctanh(x: f64) -> f64 {
    x.atanh()
}
pub fn arccsch(x: f64) -> f64 {
    arcsinh(1.0 / x)
}
pub fn arcsech(x: f64) -> f64 {
    arccosh(1.0 / x)
}
pub fn arccoth(x: f64) -> f64 {
    arctanh(1.0 / x)
}
