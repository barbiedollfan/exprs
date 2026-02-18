mod functions;
mod lexer;
mod parser;

use functions::*;
use lexer::tokenize;
use parser::parse;

use statrs::function::beta::beta;
use statrs::function::erf::erf;
use statrs::function::erf::erfc;
use statrs::function::factorial::binomial;
use statrs::function::factorial::factorial;
use statrs::function::gamma::digamma;
use statrs::function::gamma::gamma;

use std::f64::consts::E;
use std::f64::consts::PI;
// TODO: Add Phi and Euler Mascheroni

fn main() {}
